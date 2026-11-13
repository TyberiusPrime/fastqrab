use std::borrow::Cow;

use super::merge_reads::{Algorithm, find_best_overlap_fastp, verify_overlap_parameters};
use crate::transformations::prelude::*;
use fastqrab_dna::dna::reverse_complement;

/// Trim paired reads to their overlap (the `MergeReads` overlap, without merging)
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct TrimToOverlap {
    /// Algorithm to use for overlap detection
    pub algorithm: Algorithm,

    /// Minimum overlap length required (suggested: 30, minimum: 5)
    pub min_overlap: usize,

    /// Maximum allowed mismatch rate (0.0 to 1.0, suggested: 0.2)
    pub max_mismatch_rate: f64,

    /// Maximum allowed absolute number of mismatches (suggested: 5)
    pub max_mismatch_count: usize,

    /// Whether segment2 is reverse complemented for the overlap search
    /// (the read itself stays as is)
    pub reverse_complement_segment2: bool,

    /// Optional bool tag: true if an overlap was found (and the reads trimmed to it)
    pub out_label: Option<TagLabel>,

    #[schemars(with = "String")]
    #[tpd(adapt_in_verify(String))]
    pub segment1: SegmentIndex,
    #[schemars(with = "String")]
    #[tpd(adapt_in_verify(String))]
    pub segment2: SegmentIndex,
}

impl VerifyIn<PartialConfig> for PartialTrimToOverlap {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        verify_overlap_parameters(
            &mut self.segment1,
            &mut self.segment2,
            &mut self.min_overlap,
            &mut self.max_mismatch_rate,
            parent,
        );
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialTrimToOverlap> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        // Unlike MergeReads, location tags survive: trimming is a `resize`,
        // which records the cuts in the segments' edit logs, so tags lift
        // forward (or are dropped per read if the tagged bases were cut away).
        if let Some(inner) = self.toml_value.value.as_mut() {
            Some(TagUsageInfo {
                declared_tag: inner.out_label.to_declared_tag(TagValueType::Bool),
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

/// Keep-windows `(start, len)` for (read1, read2), in each read's own orientation.
type OverlapWindows = ((usize, usize), (usize, usize));

impl TrimToOverlap {
    /// Convert an overlap (as reported by `find_best_overlap_fastp`, with
    /// `offset` = position of the (processed) read2 start in read1) into
    /// keep-windows for both reads.
    #[expect(clippy::cast_sign_loss, reason = "sign checked")]
    fn windows(
        len2: usize,
        offset: isize,
        overlap_len: usize,
        reverse_complement_segment2: bool,
    ) -> OverlapWindows {
        let (start1, start2_processed) = if offset >= 0 {
            (offset as usize, 0)
        } else {
            (0, (-offset) as usize)
        };
        let start2 = if reverse_complement_segment2 {
            // position i in the reverse complement is len2 - 1 - i in the read
            len2 - start2_processed - overlap_len
        } else {
            start2_processed
        };
        ((start1, overlap_len), (start2, overlap_len))
    }
}

impl Step for TrimToOverlap {
    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let seg1_idx = self.segment1.as_index();
        let seg2_idx = self.segment2.as_index();

        let windows: Vec<Option<OverlapWindows>> = block.segments[seg1_idx]
            .iter()
            .zip(block.segments[seg2_idx].iter())
            .map(|(read1, read2)| {
                let seq2: Cow<[u8]> = if self.reverse_complement_segment2 {
                    Cow::Owned(reverse_complement(read2.seq))
                } else {
                    Cow::Borrowed(read2.seq)
                };
                let overlap = match self.algorithm {
                    Algorithm::Fastp => find_best_overlap_fastp(
                        read1.seq,
                        &seq2,
                        self.min_overlap,
                        self.max_mismatch_rate,
                        self.max_mismatch_count,
                    ),
                };
                // fastp reports overlaps shorter than min_overlap when read2 is
                // shorter than that (down to 0 bases for an empty read2).
                // Trimming both reads to that would be nonsense.
                overlap
                    .filter(|(_offset, overlap_len)| *overlap_len >= self.min_overlap)
                    .map(|(offset, overlap_len)| {
                        Self::windows(
                            read2.seq.len(),
                            offset,
                            overlap_len,
                            self.reverse_complement_segment2,
                        )
                    })
            })
            .collect();

        // `resize` narrows without copying and records the cuts in the edit log,
        // so location tags on either segment stay valid.
        block
            .member_mut(seg1_idx)
            .seq_quals
            .resize(|row, _seq, _qual| windows[row].map(|w| w.0));
        block
            .member_mut(seg2_idx)
            .seq_quals
            .resize(|row, _seq, _qual| windows[row].map(|w| w.1));

        if let Some(label) = &self.out_label {
            block.tags.insert(
                label.clone(),
                TagColumn::Bool(windows.iter().map(Option::is_some).collect()),
            );
        }

        Ok((block, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_forward_offset() {
        // read1 = insert[0..10], rc(read2) = insert[4..14] -> overlap insert[4..10]
        assert_eq!(TrimToOverlap::windows(10, 4, 6, true), ((4, 6), (4, 6)));
        // without rc, read2 itself starts at the overlap
        assert_eq!(TrimToOverlap::windows(10, 4, 6, false), ((4, 6), (0, 6)));
    }

    #[test]
    fn windows_negative_offset() {
        // read-through: read1 = insert(6) + adapter(4), rc(read2) = adapter'(4) + insert(6)
        assert_eq!(TrimToOverlap::windows(10, -4, 6, true), ((0, 6), (0, 6)));
        assert_eq!(TrimToOverlap::windows(10, -4, 6, false), ((0, 6), (4, 6)));
    }
}
