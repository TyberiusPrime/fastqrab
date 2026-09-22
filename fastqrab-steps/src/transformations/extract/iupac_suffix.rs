use super::extract_region_tags_from_seq;
use crate::transformations::prelude::*;
use fastqrab_config::{StringOrVecString, dna::iupac_hamming_distance, tpd_adapt_iupac_bstring};

/// Extract a IUPAC sequence (or a prefix of it) at the end of a read into a tag.
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct IUPACSuffix {
    #[schemars(with = "String")]
    #[tpd(adapt_in_verify(String))]
    segment: SegmentIndex,

    pub out_label: TagLabel,
    pub min_length: usize,
    pub max_mismatches: usize,
    #[tpd(with = "tpd_adapt_iupac_bstring")]
    #[tpd(alias = "query")]
    #[tpd(alias = "pattern")]
    #[schemars(with = "StringOrVecString")]
    search: Vec<BString>,
}

impl VerifyIn<PartialConfig> for PartialIUPACSuffix {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.segment.validate_segment(parent);
        let ml_span = self.min_length.span();
        self.search.verify(|v| {
            if v.is_empty() {
                Err(ValidationFailure::new(
                    "Search must not be an empty list",
                    Some("Fill in at least one search pattern"),
                ))
            } else {
                let lengths: Vec<usize> = v
                    .iter()
                    .map(|x| x.as_ref().map_or(0, |y| y.len()))
                    .collect();
                if lengths.iter().any(|x| *x != lengths[0]) {
                    Err(ValidationFailure::new(
                        "Search patterns must all be of the same length".to_string(),
                        Some(format!("Observed: {:?}", lengths)),
                    ))
                } else {
                    Ok(())
                }
            }
        });
        self.min_length.verify(|v| {
            if *v == 0 {
                Err(ValidationFailure::new(
                    "min_length must be > 0",
                    Some("Set to a positive integer."),
                ))
            } else {
                if let Some(search) = self.search.as_ref()
                    && *v
                        > search
                            .iter()
                            .map(|y| y.as_ref().map(|z| z.len()).unwrap_or(0))
                            .min()
                            .unwrap_or(0)
                {
                    let spans = vec![
                        (
                            ml_span,
                            "Too large, can not be longer than the search pattern".to_string(),
                        ),
                        (self.search.span(), "or this too short?".to_string()),
                    ];
                    self.search.state = TomlValueState::Custom { spans };
                    self.search.help = Some(
                        "min_length cannot be greater than the length of the search pattern"
                            .to_string(),
                    );
                }
                Ok(())
            }
        });
        Ok(())
    }
}

impl IUPACSuffix {
    fn longest_suffix_that_is_a_prefix(
        seq: &[u8],
        query: &[u8],
        max_mismatches: usize,
        min_length: usize,
    ) -> Option<usize> {
        assert!(min_length >= 1);
        let max_len = std::cmp::min(seq.len(), query.len());
        for prefix_len in (min_length..=max_len).rev() {
            let suffix_start = seq.len() - prefix_len;
            let dist = iupac_hamming_distance(&query[..prefix_len], &seq[suffix_start..]);
            if dist <= max_mismatches {
                return Some(prefix_len);
            }
        }
        None
    }
    fn longest_suffix_that_is_a_prefix_multiple(
        seq: &[u8],
        queries: &[BString],
        max_mismatches: usize,
        min_length: usize,
    ) -> Option<usize> {
        let mut best = None;
        for query in queries {
            let here = IUPACSuffix::longest_suffix_that_is_a_prefix(
                seq,
                query,
                max_mismatches,
                min_length,
            );
            if here > best {
                best = here;
            }
            // match (best, here) {
            //             (None, here) => here,
            //             (Some(old_prefix_len), None) => Some(old_prefix_len),
            //              Some(old_prefix_len), Some(new_prefix_len) => {
            //                 if new_prefix_len > old_prefix_len {
            //                     Some(new_prefix_len)
            //                 } else {
            //                     Some(old_prefix_len)
            //                 }
            //         }
            //
            //     }
        }
        best
    }
}

impl TagUser for PartialTaggedVariant<PartialIUPACSuffix> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut()
            && let Some(Some(segment)) = inner.segment.as_ref().map(|m| m.as_ref_post())
        {
            // Record the segment this location tag lives on, so a conditional
            // `Swap` on that segment can forget it (a per-read swap would split it
            // across two segments).
            Some(TagUsageInfo {
                declared_tag: inner
                    .out_label
                    .to_declared_tag(TagValueType::Location)
                    .map(|dt| dt.with_segment(*segment)),
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

impl Step for IUPACSuffix {
    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        extract_region_tags_from_seq(&mut block, self.segment, &self.out_label, |seq| {
            //cheap empty range if read length too short no need for explicit check
            #[expect(
                clippy::cast_possible_truncation,
                reason = "lengths are guaranteed to be within u32 range"
            )]
            Self::longest_suffix_that_is_a_prefix_multiple(
                seq,
                &self.search,
                self.max_mismatches,
                self.min_length,
            )
            .map(|suffix_len| (seq.len() - suffix_len) as u32..seq.len() as u32)
        });
        Ok((block, true))
    }
}
