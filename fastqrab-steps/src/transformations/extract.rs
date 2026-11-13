use std::{borrow::Cow, ops::Range};

use bstr::{BStr, BString};
use fastqrab_dna::dna::TagColumn;
use fastqrab_io::blocks::Molecule;
use indexmap::IndexMap;
use stringpod::CrossPods;

use super::prelude::{DemultiplexTag, StringColumnBuilder};
use fastqrab_config::{
    TagLabel,
    segments::{SegmentIndex, SegmentIndexOrAll},
};
use fastqrab_io::io::FastQBlocksCombined;

mod adapter_like_fastp;
mod iupac;
mod iupac_suffix;
mod iupac_with_indel;
mod longest_poly_x;
mod low_quality_end;
mod low_quality_start;
mod poly_tail;
mod regex;
mod region;
mod regions;
mod regions_of_low_quality;
pub mod tag;

pub use adapter_like_fastp::{AdapterLikeFastP, PartialAdapterLikeFastP};
pub use iupac::{IUPAC, PartialIUPAC};
pub use iupac_suffix::{IUPACSuffix, PartialIUPACSuffix};
pub use iupac_with_indel::{IUPACWithIndel, PartialIUPACWithIndel};
pub use longest_poly_x::{LongestPolyX, PartialLongestPolyX};
pub use low_quality_end::{LowQualityEnd, PartialLowQualityEnd};
pub use low_quality_start::{LowQualityStart, PartialLowQualityStart};
pub use poly_tail::{PartialPolyTail, PolyTail};
pub use regex::{PartialRegex, Regex};
pub use region::{PartialRegion, Region};
pub use regions::{PartialRegions, Regions};
pub use regions_of_low_quality::{PartialRegionsOfLowQuality, RegionsOfLowQuality};

pub(crate) fn extract_region_tags_from_seq(
    block: &mut FastQBlocksCombined,
    segment: SegmentIndex,
    label: &TagLabel,
    f: impl Fn(&BStr) -> Option<Range<u32>>,
) {
    let mut col = block.location_column_builder(segment);
    for seq in block.segments[segment.as_index()].seq_quals.iter_seq() {
        match f(seq) {
            Some(region_range) => col.push_row_from_ranges(&[region_range]),
            None => col.push_row(&[]),
        }
    }

    block
        .tags
        .insert(label.clone(), TagColumn::Location(col.finish()));
}

pub(crate) fn extract_region_tags_from_qual(
    block: &mut FastQBlocksCombined,
    segment: SegmentIndex,
    label: &TagLabel,
    f: impl Fn(&BStr) -> Option<Range<u32>>,
) {
    let mut col = block.location_column_builder(segment);
    for qual in block.segments[segment.as_index()].seq_quals.iter_qual() {
        match f(qual) {
            Some(region_range) => col.push_row_from_ranges(&[region_range]),
            None => col.push_row(&[]),
        }
    }

    block
        .tags
        .insert(label.clone(), TagColumn::Location(col.finish()));
}

/// What an [`extract_region_or_value_tags_using_tags`] closure produces for one
/// read: nothing, a live alias window, or owned divergent content.
pub(crate) enum RegexExtraction {
    /// No match — a no-hit row.
    None,
    /// A single contiguous window into the live read (an alias): its coordinates
    /// follow later edits and its quality stays the read's own.
    Region(Range<u32>),
    /// Content the read does not contain as one slice (a regex replacement that
    /// conjures, repeats, or reorders bytes). The bytes are owned in the column's
    /// arena; `anchor` is the read-relative span they stand in for — what
    /// write-back overwrites and what liftover lifts (its length may differ from
    /// the content's).
    Owned {
        anchor: Range<u32>,
        seq: Vec<u8>,
        qual: Vec<u8>,
    },
}

/// Build a `TagColumn::Location` from a regex-style closure that, per read, may
/// return owned content (a [`RegexExtraction::Owned`]) when the result can't be
/// expressed as a slice of the read — it is given the read's `seq` *and* `qual`
/// so it can carry real or synthesized quality with that content. Alias rows
/// (live windows) and owned rows coexist in the one column.
pub(crate) fn extract_region_or_value_tags_using_tags(
    block: &mut FastQBlocksCombined,
    segment_index: SegmentIndex,
    label: &TagLabel,
    f: impl Fn(&BStr, &BStr, usize, &IndexMap<TagLabel, TagColumn>) -> RegexExtraction,
) {
    let read_no = block.first_read_sequential_number;
    let (mut col, tags, segment) =
        block.location_column_builder_and_tags_and_segment(segment_index);

    for (ii, read) in segment.seq_quals.iter().enumerate() {
        match f(read.seq, read.qual, read_no + ii, tags) {
            RegexExtraction::None => col.push_row(&[]),
            RegexExtraction::Region(region_range) => col.push_row_from_ranges(&[region_range]),
            RegexExtraction::Owned { anchor, seq, qual } => {
                col.push_owned_row(&[(anchor.start, anchor.end - anchor.start)], &seq, &qual);
            }
        }
    }

    tags.insert(label.clone(), TagColumn::Location(col.finish()));
}

pub(crate) fn extract_string_tags_using_tags(
    block: &mut FastQBlocksCombined,
    segment: SegmentIndex,
    label: &TagLabel,
    f: impl Fn(&BStr, usize, &IndexMap<TagLabel, TagColumn>) -> Option<BString>,
) {
    let mut out = StringColumnBuilder::new();

    let read_no = block.first_read_sequential_number;
    for (ii, seq) in block.segments[segment.as_index()]
        .seq_quals
        .iter_seq()
        .enumerate()
    {
        out.push(f(seq, read_no + ii, &mut block.tags).map(Cow::Owned));
    }

    block
        .tags
        .insert(label.clone(), TagColumn::String(out.finish()));
}

pub(crate) fn extract_bool_tags<F>(
    block: &mut FastQBlocksCombined,
    segment: SegmentIndex,
    label: &TagLabel,
    mut extractor: F,
) where
    F: FnMut(&fastqrab_io::blocks::FastQRead, DemultiplexTag) -> bool,
{
    let mut values = Vec::new();
    for (idx, read) in block.segments[segment.as_index()].iter().enumerate() {
        let output_tag = block
            .output_tags
            .as_ref()
            .map(|x| x[idx])
            .unwrap_or_default();
        values.push(extractor(&read, output_tag));
    }

    block
        .tags
        .insert(label.clone(), TagColumn::Bool(values.into_iter().collect()));
}

pub(crate) fn extract_bool_tags_plus_all<F, G>(
    block: &mut FastQBlocksCombined,
    segment: SegmentIndexOrAll,
    label: &TagLabel,
    extractor_single: F,
    mut extractor_all: G,
) where
    F: FnMut(&fastqrab_io::blocks::FastQRead, DemultiplexTag) -> bool,
    G: FnMut(&Molecule<'_>, DemultiplexTag) -> bool,
{
    let target: Result<SegmentIndex, _> = segment.try_into();
    if let Ok(target) = target {
        // Handle single target case
        extract_bool_tags(block, target, label, extractor_single);
    } else {
        // Handle "All" target case
        let mut values = Vec::new();
        for (idx, molecule) in block.molecules().enumerate() {
            let output_tag = block
                .output_tags
                .as_ref()
                .map(|x| x[idx])
                .unwrap_or_default();
            let value = extractor_all(&molecule, output_tag);
            values.push(value);
        }
        block
            .tags
            .insert(label.clone(), TagColumn::Bool(values.into_iter().collect()));
    }
}

pub(crate) fn extract_bool_tags_from_tag<F>(
    block: &mut FastQBlocksCombined,
    out_label: &TagLabel,
    input_label: &TagLabel,
    mut extractor: F,
) where
    F: FnMut(Option<Cow<'_, BStr>>, DemultiplexTag) -> bool,
{
    let input_tags = block
        .tags
        .get(input_label)
        .expect("Input tag missing, validation bug");

    let mut values = Vec::new();
    for (pos, tag_value) in input_tags.iter_stringified().enumerate() {
        let output_tag = block
            .output_tags
            .as_ref()
            .map(|x| x[pos])
            .unwrap_or_default();
        values.push(extractor(tag_value, output_tag));
    }

    block.tags.insert(
        out_label.clone(),
        TagColumn::Bool(values.into_iter().collect()),
    );
}
