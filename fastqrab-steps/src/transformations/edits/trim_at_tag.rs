use crate::transformations::prelude::*;
use fastqrab_io::blocks::FastQChunk;
use stringpod::{DualStringPodMultiLocation, Lifted, RegionLift};

#[derive(Clone, Eq, PartialEq, Copy, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub enum Direction {
    #[tpd(alias = "start")]
    Start,
    #[tpd(alias = "end")]
    End,
}

/// Trim reads at a tag's position
#[derive(Clone, JsonSchema)]
#[tpd(no_verify)]
#[derive(Debug)]
pub struct TrimAtTag {
    in_label: TagLabel,
    direction: Direction,
    keep_tag: bool,
    #[tpd(default)]
    #[tpd(alias = "if_label")]
    if_tag: Option<ConditionalTagLabel>,
}

impl TagUser for PartialTaggedVariant<PartialTrimAtTag> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            Some(TagUsageInfo {
                used_tags: vec![
                    inner.in_label.to_used_tag(&[TagValueType::Location]),
                    inner.if_tag.to_used_tag(&[]),
                ],
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

impl Step for TrimAtTag {
    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let condition = self
            .if_tag
            .as_ref()
            .map(|tag| get_bool_vec_from_tag(&block, tag));
        let col = block
            .tags
            .get(&self.in_label)
            .expect("in_label tag must exist in block")
            .as_locations()
            .expect("Must be a location tag");
        let segment_index = col.source_id() as usize;
        let row_count = col.row_count();

        // Pass 1: for each read, work out the keep-window in the read's *current*
        // frame. The tag stores born-frame coordinates, so each region is lifted
        // forward through whatever edits ran since the tag was captured. This
        // borrows `block.tags` and `block.segments` immutably, so it must finish
        // before the mutable `member_mut` below.
        let segment = &block.segments[segment_index];
        let windows: Vec<Option<(usize, usize)>> = (0..row_count)
            .map(|row| {
                if condition.as_ref().is_some_and(|c| !c[row]) {
                    None // if_tag false: leave the read untouched
                } else {
                    self.keep_window(segment, col, row)
                }
            })
            .collect();

        // Pass 2: narrow each read to its window. `resize` is a no-copy narrowing
        // (it only rewrites the per-entry offsets) and records the implied
        // cut_start/cut_end into the segment's edit log, so any tag still pointing
        // into these reads — including `in_label` itself — lifts forward correctly
        // afterwards. No need to rewrite the tag column by hand.
        block
            .member_mut(segment_index)
            .seq_quals
            .resize(|row, _seq, _qual| windows[row]);

        Ok((block, true))
    }
}

impl TrimAtTag {
    /// The sub-range `[start..start + len]` (in the read's *current* frame) that
    /// this trim keeps for `row`, or `None` to leave the read untouched — used
    /// when the read has no hit, or when the captured location was cut away by an
    /// earlier edit and can no longer be located.
    fn keep_window(
        &self,
        segment: &FastQChunk,
        col: &DualStringPodMultiLocation,
        row: usize,
    ) -> Option<(usize, usize)> {
        if col.row_is_empty(row) {
            return None;
        }
        let (born_generation, born_len) = col.row_born(row);
        let view = segment
            .seq_quals
            .ops_since(born_generation, row)
            .expect("born generation captured from this pod; row in range. Bug");

        // Lift every region into the current frame; if any was cut away or split
        // we can no longer place the trim point, so leave the read as-is. A tag
        // with several regions is trimmed at its outermost edge — `min_start` /
        // `max_end` span all of them, so the whole tag is kept or removed as one.
        let mut min_start = usize::MAX;
        let mut max_end = 0usize;
        for (start, len) in col.row_regions(row) {
            match view.map_region(start as usize, len as usize, born_len) {
                Ok(RegionLift::Kept { start, len }) => {
                    min_start = min_start.min(start);
                    max_end = max_end.max(start + len);
                }
                Ok(RegionLift::Dropped) | Err(_) => {
                    return None;
                }
            }
        }

        let cur_len = segment.seq_quals.entry_len(row);
        // Keep/drop at the tag's far edge in the cut direction, so the whole tag
        // (all its regions) is either entirely kept or entirely removed.
        let (start, end) = match (self.direction, self.keep_tag) {
            (Direction::Start, true) => (min_start, cur_len),
            (Direction::Start, false) => (max_end, cur_len),
            (Direction::End, true) => (0, max_end),
            (Direction::End, false) => (0, min_start),
        };
        Some((start, end - start))
    }
}
