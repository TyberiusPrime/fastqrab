use crate::transformations::prelude::*;

/// Return the primary tag if it is not missing, otherwise return the secondary tag.
///
/// Both tags must have the same kind (Location or String), or be a
/// Location/String mixture — in that case the output is always a String tag
/// (Location values are converted to their sequence representation).
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct FillMissing {
    /// Primary tag: used when not missing.
    #[tpd(alias = "primary")]
    #[tpd(alias = "first")]
    in_label_primary: TagLabel,

    /// Secondary tag: used when the primary tag is missing.
    #[tpd(alias = "secondary")]
    #[tpd(alias = "second")]
    in_label_secondary: TagLabel,

    /// Output tag label for the result.
    out_label: TagLabel,

    #[tpd(skip)]
    #[schemars(skip)]
    output_type: TagValueType,
}

impl VerifyIn<PartialConfig> for PartialFillMissing {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialFillMissing> {
    fn get_tag_usage(
        &mut self,
        tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            let type0 = inner
                .in_label_primary
                .as_ref()
                .and_then(|l| tags_available.get(l))
                .map(|m| &m.tag_type);
            let type1 = inner
                .in_label_secondary
                .as_ref()
                .and_then(|l| tags_available.get(l))
                .map(|m| &m.tag_type);

            let output_type = match (type0, type1) {
                (Some(TagValueType::Location), Some(TagValueType::Location)) => {
                    let segment_0 = inner
                        .in_label_primary
                        .as_ref()
                        .and_then(|l| tags_available.get(l))
                        .map(|m| m.segment)
                        .expect("checked above");
                    let segment_1 = inner
                        .in_label_secondary
                        .as_ref()
                        .and_then(|l| tags_available.get(l))
                        .map(|m| m.segment)
                        .expect("checked above");
                    if segment_0 == segment_1 {
                        TagValueType::Location
                    } else {
                        TagValueType::String
                    }
                }
                // (Some(TagValueType::Location), Some(TagValueType::String))
                // | (Some(TagValueType::String), Some(TagValueType::Location))
                | _ //doesn't mater, will get rejected
                =>  {
                    TagValueType::String
                },
            };
            inner.output_type = Some(output_type);

            Some(TagUsageInfo {
                used_tags: vec![
                    inner
                        .in_label_primary
                        .to_used_tag(&[TagValueType::Location, TagValueType::String]),
                    inner
                        .in_label_secondary
                        .to_used_tag(&[TagValueType::Location, TagValueType::String]),
                ],
                declared_tag: inner.out_label.to_declared_tag(output_type),
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

impl Step for FillMissing {
    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let primary_vec = block
            .tags
            .get(&self.in_label_primary)
            .expect("Tag not found in block. Verification failure");
        let secondary_vec = block
            .tags
            .get(&self.in_label_secondary)
            .expect("Secondary tag not found in block. Verification failure");

        let output_col = match (self.output_type, primary_vec, secondary_vec) {
            (TagValueType::Location, TagColumn::Location(prim), TagColumn::Location(sec)) => {
                assert!(
                    prim.source_id() == sec.source_id(),
                    "Validation failed to detect different sources"
                );
                let mut out = block.location_column_builder(prim.source_id().into());
                for (prim_hit, sec_hit) in prim.iter_row_regions().zip(sec.iter_row_regions()) {
                    if !prim_hit.is_empty() {
                        out.push_row(&prim_hit);
                    } else {
                        out.push_row(&sec_hit);
                    }
                }
                TagColumn::Location(out.finish())
            }
            (TagValueType::String, TagColumn::Location(prim), TagColumn::Location(sec)) => {
                TagColumn::String(
                    prim.iter_seq()
                        .zip(sec.iter_seq())
                        .map(|(prim_hit, sec_hit)| {
                            match (prim_hit.is_empty(), sec_hit.is_empty()) {
                                (false, _) => Some(BString::new(prim_hit.to_vec())),
                                (true, false) => Some(BString::new(sec_hit.to_vec())),
                                (true, true) => None,
                            }
                        })
                        .collect(),
                )
            }
            (TagValueType::String, TagColumn::String(prim), TagColumn::String(sec)) => {
                TagColumn::String(prim.iter().zip(sec.iter()).map(|(p, s)| p.or(s)).collect())
            }
            (TagValueType::String, TagColumn::Location(prim), TagColumn::String(sec)) => {
                TagColumn::String(
                    prim.iter_seq()
                        .zip(sec.iter())
                        .map(|(prim_hit, sec_str)| {
                            (!prim_hit.is_empty())
                                .then(|| BString::new(prim_hit.to_vec()))
                                .or(sec_str.as_ref().map(|x| BString::new(x.to_vec())))
                        })
                        .collect(),
                )
            }
            (TagValueType::String, TagColumn::String(prim), TagColumn::Location(sec)) => {
                TagColumn::String(
                    prim.iter()
                        .zip(sec.iter_seq())
                        .map(|(prim_str, sec_hit)| {
                            prim_str
                                .map(Cow::Borrowed)
                                .or_else(|| (!sec_hit.is_empty()).then_some(sec_hit))
                        })
                        .collect(),
                )
            }
            _ => {
                //cov:excl-start
                unreachable!(
                    "FillMissing: unsupported tag type combination {:?} {:?} {:?}",
                    &self.output_type,
                    primary_vec,
                    secondary_vec // cov:excl-line
                )
                //cov:excl-stop
            }
        };

        block.tags.insert(self.out_label.clone(), output_col);

        Ok((block, true))
    }
}
