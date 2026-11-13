use std::borrow::Cow;

use super::merge_reads::{Algorithm, find_best_overlap_fastp, verify_overlap_parameters};
use crate::transformations::prelude::*;
use fastqrab_dna::dna::reverse_complement;

/// When to trim a pair to its overlap
#[derive(Clone, Copy, PartialEq, Eq, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub enum TrimMode {
    /// Trim whenever the reads overlap, keeping exactly the overlapping bases
    AnyOverlap,
    /// Trim only when the reads run past the fragment end into adapter
    /// (segment2 starts before segment1 = fragment shorter than the reads).
    /// Reproduces fastp's overlap based adapter trimming:
    /// both reads keep their first `overlap length` bases.
    ReadThrough,
}

/// Trim paired reads to their overlap (the `MergeReads` overlap, without merging)
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct TrimToOverlap {
    /// Algorithm to use for overlap detection
    pub algorithm: Algorithm,

    /// `AnyOverlap`: trim whenever the reads overlap.
    /// `ReadThrough`: only trim reads that run into adapter (like fastp)
    pub trim_mode: TrimMode,

    /// Minimum overlap length required (suggested: 30, minimum: 5)
    pub min_overlap: usize,

    /// Maximum allowed mismatch rate (0.0 to 1.0, suggested: 0.2)
    pub max_mismatch_rate: f64,

    /// Maximum allowed absolute number of mismatches (suggested: 5)
    pub max_mismatch_count: usize,

    /// Whether segment2 is reverse complemented for the overlap search
    /// (the read itself stays as is)
    pub reverse_complement_segment2: bool,

    /// Optional bool tag: true if the reads were trimmed
    /// (`AnyOverlap`: an overlap was found, `ReadThrough`: the reads ran into adapter)
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
    /// keep-windows for both reads - or None if this mode doesn't trim it.
    #[expect(clippy::cast_sign_loss, reason = "sign checked")]
    fn windows(
        trim_mode: TrimMode,
        len2: usize,
        offset: isize,
        overlap_len: usize,
        reverse_complement_segment2: bool,
    ) -> Option<OverlapWindows> {
        if trim_mode == TrimMode::ReadThrough {
            if offset >= 0 {
                return None; // regular overlap, no adapter read-through
            }
            // fastp (AdapterTrimmer::trimByOverlapAnalysis) keeps the first
            // overlap_len bases of both reads. That equals the overlap unless
            // read1 ends before read2's (processed) end, in which case read2
            // keeps its 5' end instead.
            let start2 = if reverse_complement_segment2 {
                0
            } else {
                len2 - overlap_len
            };
            return Some(((0, overlap_len), (start2, overlap_len)));
        }
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
        Some(((start1, overlap_len), (start2, overlap_len)))
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
                    .and_then(|(offset, overlap_len)| {
                        Self::windows(
                            self.trim_mode,
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
        let w = |rc| TrimToOverlap::windows(TrimMode::AnyOverlap, 10, 4, 6, rc);
        assert_eq!(w(true), Some(((4, 6), (4, 6))));
        // without rc, read2 itself starts at the overlap
        assert_eq!(w(false), Some(((4, 6), (0, 6))));
        // no read-through, fastp would not trim
        assert_eq!(
            TrimToOverlap::windows(TrimMode::ReadThrough, 10, 4, 6, true),
            None
        );
    }

    #[test]
    fn windows_negative_offset() {
        // read-through: read1 = insert(6) + adapter(4), rc(read2) = adapter'(4) + insert(6)
        for mode in [TrimMode::AnyOverlap, TrimMode::ReadThrough] {
            let w = |rc| TrimToOverlap::windows(mode, 10, -4, 6, rc);
            assert_eq!(w(true), Some(((0, 6), (0, 6))));
            assert_eq!(w(false), Some(((0, 6), (4, 6))));
        }
    }

    #[test]
    fn windows_read_through_short_read1() {
        // read1 = insert[0..5], rc(read2) = adapter'(2) + insert[0..8]
        // -> overlap insert[0..5]. The overlap in read2 is read2[3..8],
        // fastp keeps read2's 5' end, read2[0..5]
        let w = |mode| TrimToOverlap::windows(mode, 10, -2, 5, true);
        assert_eq!(w(TrimMode::AnyOverlap), Some(((0, 5), (3, 5))));
        assert_eq!(w(TrimMode::ReadThrough), Some(((0, 5), (0, 5))));
    }
}
