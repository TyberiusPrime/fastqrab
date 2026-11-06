use anyhow::Result;
use bio::alignment::{
    AlignmentOperation,
    pairwise::{Aligner, MIN_SCORE, Scoring},
};
use bstr::{BStr, BString, ByteSlice};
use hamming_resonate::HammingResonator;
use indexmap::IndexMap;
use schemars::JsonSchema;
use smallvec::SmallVec;
use std::{borrow::Cow, ops::Range};
use stringpod::{DualStringPodMultiLocation, StringPod};
use toml_pretty_deser::prelude::*;

use crate::segments::SegmentIndex;

use bitvec::prelude as bv;
pub use triple_accel::hamming; //todo: do we need this. Profile.

#[derive(Debug, Clone)]
pub struct StringColumn(StringPod, bv::BitBox);

//todo: move this into it's own file
#[derive(Debug, Clone)]
pub enum TagColumn {
    Location(DualStringPodMultiLocation),
    String(StringColumn),
    Numeric(Vec<f64>),
    Bool(bv::BitBox),
}

impl TagColumn {
    pub fn truncate(&mut self, len: usize) {
        match self {
            TagColumn::Location(col) => col.truncate(len),
            TagColumn::String(StringColumn(items, _validity)) => {
                items.truncate(len);
                // we ignore the size change, in exchange of not using a BitVec
                // here. Only the string pod is the source
                // of truth for how-many-elements are there
                //    validity.truncate(len);
            }
            TagColumn::Numeric(items) => items.truncate(len),
            TagColumn::Bool(items) => *items = items.iter().take(len).collect(),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            TagColumn::Location(col) => col.row_count(),
            TagColumn::String(StringColumn(items, _validity)) => items.len(), //counts missing
            TagColumn::Numeric(items) => items.len(),
            TagColumn::Bool(items) => items.len(),
        }
    }

    #[must_use]
    #[mutants::skip]
    //cov:excl-start
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    //cov:excl-stop

    /// Retain just the values where keep is true.
    ///
    /// # Panics
    /// if keep has the wrong length.
    pub fn retain_by_bool(&mut self, keep: &[bool]) {
        assert!(keep.len() == self.len());
        match self {
            TagColumn::String(StringColumn(items, validity)) => {
                items.retain_by_bools(keep);
                let new_valid = validity
                    .iter()
                    .zip(keep.iter())
                    .filter_map(|(is_some, &do_keep)| if do_keep { Some(is_some) } else { None })
                    .collect::<bv::BitBox>();
                *validity = new_valid;
            }
            TagColumn::Location(col) => {
                col.retain_by_bools(keep);
            }
            TagColumn::Numeric(items) => {
                let mut iter = keep.iter();
                let mut keep_func = || iter.next().expect("Length was checked");
                items.retain(|_| *keep_func());
            }
            TagColumn::Bool(items) => {
                *items = items
                    .iter()
                    .zip(keep.iter())
                    .filter_map(|(is_some, &do_keep)| if do_keep { Some(is_some) } else { None })
                    .collect::<bv::BitBox>();
            }
        }
    }

    #[must_use]
    pub fn as_locations(&self) -> Option<&DualStringPodMultiLocation> {
        if let TagColumn::Location(col) = self {
            Some(col)
        } else {
            None //cov:excl-line
        }
    }

    /// Iterate a numeric column
    /// # Panics
    /// When it's not a numeric column
    pub fn iter_numeric(&self) -> impl Iterator<Item = &f64> {
        if let TagColumn::Numeric(items) = self {
            items.iter()
        } else {
            panic!("iter_numeric called on a non-numeric tag column");
        }
    }

    #[must_use]
    pub fn iter_stringified<'a>(&'a self) -> Box<dyn Iterator<Item = Option<Cow<'a, BStr>>> + 'a> {
        match self {
            TagColumn::Numeric(_) => unreachable!("Cant stringify numeric values"),
            TagColumn::Bool(bools) => Box::new(bools.iter().map(|x| {
                if *x {
                    Some(Cow::Borrowed(BStr::new(b"true")))
                } else {
                    Some(Cow::Borrowed(BStr::new(b"false")))
                }
            })),
            TagColumn::Location(col) => {
                Box::new(col.iter_seq().map(|x| (!x.is_empty()).then_some(x)))
            }
            TagColumn::String(StringColumn(items, validity)) => {
                Box::new(items.iter().zip(validity.iter()).map(|(value, valid)| {
                    if *valid {
                        Some(Cow::Borrowed(BStr::new(value)))
                    } else {
                        None
                    }
                }))
            }
        }
    }

    #[must_use]
    pub fn iter_truthy(&self) -> Box<dyn Iterator<Item = bool> + '_> {
        match self {
            TagColumn::Numeric(_) => unreachable!("Cant get truthy of numeric values."), //cov:excl-line
            TagColumn::Bool(bools) => Box::new(bools.iter().by_vals()),
            TagColumn::Location(col) => {
                Box::new((0..col.row_count()).map(|idx| !col.row_is_empty(idx)))
            }

            TagColumn::String(StringColumn(_items, valid)) => Box::new(valid.iter().map(|x| *x)),
        }
    }

    /// Get the regions of an entry for a  Location column
    /// # Panics
    /// If used with a non location column
    #[must_use]
    pub fn get_location(&self, index: usize) -> SmallVec<[(u32, u32); 1]> {
        if let TagColumn::Location(items) = self {
            items.row_regions(index).collect()
        } else {
            panic!("get_location called on a non-location tag column");
        }
    }

    /// The segment this location column was aliased from, recovered from the
    /// snapshot's `source_id` (stamped by
    /// [`FastQBlocksCombined::location_column_builder`]). Steps that rebuild a
    /// location column (e.g. `ReservoirSample`) need it to target the right
    /// segment's buffer.
    ///
    /// # Panics
    /// If `self` is not a `Location` column.
    #[must_use]
    pub fn location_segment(&self) -> SegmentIndex {
        if let TagColumn::Location(col) = self {
            SegmentIndex::new(col.source_id() as usize)
        } else {
            panic!("location_segment called on a non-location tag column");
        }
    }

    /// Get the string for one entry for a String column
    /// # Panics
    /// when used on a non `TagColumn::String` column
    #[must_use]
    pub fn get_string(&self, index: usize) -> Option<&BStr> {
        if let TagColumn::String(StringColumn(items, valid)) = self {
            if valid[index] {
                Some(items.get(index))
            } else {
                None
            }
        } else {
            panic!("get_string called on a non-string tag column");
        }
    }

    /// Get the value for one entry for a Numeric column
    /// # Panics
    /// when used on a non `TagColumn::Numeric` column
    #[must_use]
    pub fn get_numeric(&self, index: usize) -> f64 {
        if let TagColumn::Numeric(items) = self {
            items[index]
        } else {
            panic!("get_numeric called on a non-numeric tag column");
        }
    }

    /// Get the value for one entry for a bool column
    /// # Panics
    /// when used on a non `TagColumn::bool` column
    #[must_use]
    pub fn get_bool(&self, index: usize) -> bool {
        if let TagColumn::Bool(items) = self {
            items[index]
        } else {
            panic!("get_bool called on a non-bool tag column");
        }
    }

    #[must_use]
    pub fn to_bstr<'a, F>(
        &'a self,
        index: usize,
        number_formatter: F,
        region_seperator: Option<&[u8]>,
    ) -> Cow<'a, BStr>
    where
        F: Fn(f64) -> String,
    {
        match &self {
            TagColumn::Location(col) => col.joined_seq(index, region_seperator),
            TagColumn::String(StringColumn(items, valid)) => {
                if valid[index] {
                    Cow::Borrowed(items.get(index))
                } else {
                    Cow::Borrowed(BStr::new(b""))
                }
            }
            TagColumn::Numeric(items) => Cow::Owned(number_formatter(items[index]).into()),
            TagColumn::Bool(items) => Cow::Borrowed(if items[index] {
                BStr::new(b"1")
            } else {
                BStr::new(b"0")
            }),
        }
    }
    pub fn iter_bstr<'a>(
        &'a self,
        number_formatter: impl Fn(f64) -> String + 'a,
        region_seperator: Option<&'a [u8]>,
    ) -> Box<dyn Iterator<Item = Cow<'a, BStr>> + 'a> {
        Box::new(
            (0..self.len()).map(move |idx| self.to_bstr(idx, &number_formatter, region_seperator)),
        )
    }

    // pub fn extend(&mut self, other: &TagColumn) {
    //     match self {
    //         TagColumn::Location(col) => {
    //             if let TagColumn::Location(other_col) = other {
    //                 col.extend_from(other_col);
    //             } else {
    //                 panic!("Cannot extend Location column from non-Location column");
    //             }
    //         }
    //         TagColumn::String(items) => items.extend(
    //             other
    //                 .iter_stringified()
    //                 .map(|x| x.map(|cow| cow.into_owned())),
    //         ),
    //         TagColumn::Numeric(items) => items.extend(other.iter_numeric().copied()),
    //         TagColumn::Bool(items) => items.extend(other.iter_truthy()),
    //     }
    // }

    /// Removes elements without returning them.
    pub fn drain(&mut self, range: std::ops::Range<usize>) {
        match self {
            TagColumn::Location(col) => col.drain(range),
            TagColumn::String(StringColumn(items, valid)) => {
                items.drain(range.clone());
                *valid = valid
                    .iter()
                    .enumerate()
                    .filter_map(|(i, is_valid)| {
                        if range.contains(&i) {
                            None
                        } else {
                            Some(*is_valid)
                        }
                    })
                    .collect::<bv::BitBox>();
            }
            TagColumn::Numeric(items) => {
                items.drain(range);
            }
            TagColumn::Bool(items) => {
                *items = items
                    .iter()
                    .enumerate()
                    .filter_map(|(i, is_valid)| {
                        if range.contains(&i) {
                            None
                        } else {
                            Some(*is_valid)
                        }
                    })
                    .collect::<bv::BitBox>();
            }
        }
    }
}

impl StringColumn {
    #[must_use]
    pub fn empty() -> Self {
        StringColumn(StringPod::empty(), bv::BitVec::new().into_boxed_bitslice())
    }

    #[must_use]
    pub fn get_string(&self, index: usize) -> Option<&BStr> {
        if self.1[index] {
            Some(self.0.get(index))
        } else {
            None
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = Option<&BStr>> {
        self.0
            .iter()
            .zip(self.1.iter())
            .map(|(item, valid)| if *valid { Some(item) } else { None })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = Option<&mut BStr>> {
        self.0.make_exclusive();
        self.0
            .iter_mut()
            .zip(self.1.iter())
            .map(|(item, valid)| if *valid { Some(item) } else { None })
    }
}

pub struct StringColumnBuilder {
    pod_builder: stringpod::StringPodBuilder,
    valid: Vec<bool>,
}

impl StringColumnBuilder {
    #[expect(
        clippy::new_without_default,
        reason = "We want to be explicit about the builder pattern"
    )]
    #[must_use]
    pub fn new() -> Self {
        StringColumnBuilder {
            pod_builder: stringpod::StringPodBuilder::new(),
            valid: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Option<Cow<BStr>>) {
        match value {
            Some(value) => {
                self.pod_builder.push(&value);
                self.valid.push(true);
            }
            None => {
                self.pod_builder.push(b"");
                self.valid.push(false);
            }
        }
    }

    #[must_use]
    pub fn finish(self) -> StringColumn {
        StringColumn(self.pod_builder.finish(), self.valid.into_iter().collect())
    }
}

impl FromIterator<Option<BString>> for StringColumn {
    fn from_iter<T: IntoIterator<Item = Option<BString>>>(iter: T) -> Self {
        let mut builder = StringColumnBuilder::new();
        for item in iter {
            builder.push(item.map(Cow::Owned));
        }
        builder.finish()
    }
}

impl<'a> FromIterator<Option<&'a BStr>> for StringColumn {
    fn from_iter<T: IntoIterator<Item = Option<&'a BStr>>>(iter: T) -> Self {
        let mut builder = StringColumnBuilder::new();
        for item in iter {
            builder.push(item.map(Cow::Borrowed));
        }
        builder.finish()
    }
}

impl<'a> FromIterator<Option<Cow<'a, BStr>>> for StringColumn {
    fn from_iter<T: IntoIterator<Item = Option<Cow<'a, BStr>>>>(iter: T) -> Self {
        let mut builder = StringColumnBuilder::new();
        for item in iter {
            builder.push(item);
        }
        builder.finish()
    }
}

// ── Search anchor ─────────────────────────────────────────────────────────────

/// Where to search
#[derive(Copy, Clone, Debug, JsonSchema)]
#[tpd]
pub enum Anchor {
    #[tpd(alias = "left")]
    Left,
    #[tpd(alias = "right")]
    Right,
    #[tpd(alias = "anywhere")]
    Anywhere,
}

#[must_use]
/// find one hit of the iupac pattern/query.
///
/// # Panics
/// If the coordinates exceed an u32
pub fn find_iupac(
    reference: &[u8],
    pattern: &[u8],
    anchor: Anchor,
    max_mismatches: u8,
    max_anchor_distance: usize,
) -> Option<std::ops::Range<u32>> {
    if reference.len() < pattern.len() {
        return None;
    }
    let make_draft = |start: usize| -> std::ops::Range<u32> {
        start.try_into().expect("start would not fit u32")
            ..(start + pattern.len())
                .try_into()
                .expect("Start+pattern len would not fit into u32")
    };
    match anchor {
        Anchor::Left => iupac_find_best(
            pattern,
            &reference[..pattern.len() + max_anchor_distance],
            max_mismatches as usize,
            Direction::Forward,
        )
        .map(make_draft),
        Anchor::Right => {
            let offset = reference.len() - pattern.len() - max_anchor_distance;
            iupac_find_best(
                pattern,
                &reference[offset..],
                max_mismatches as usize,
                Direction::Reverse,
            )
            .map(|start| -> std::ops::Range<u32> {
                (offset + start)
                    .try_into()
                    .expect("offset would not fit u32")
                    ..(offset + start + pattern.len())
                        .try_into()
                        .expect("offset + start + len would not fit  u32")
            })
        }
        Anchor::Anywhere => iupac_find_best(
            pattern,
            reference,
            max_mismatches as usize,
            Direction::Forward,
        )
        .map(make_draft),
    }
}

#[inline]
fn iupac_alignment_score(a: u8, b: u8) -> i32 {
    if iupac_hamming_distance(&[a], &[b]) == 0 {
        1
    } else {
        -1
    }
}

/// # Panics
/// When the aligner doesn't stick to the start & end requirements (bug)
pub fn find_iupac_with_indel(
    reference: &[u8],
    query: &[u8],
    anchor: Anchor,
    max_mismatches: usize,
    max_indel_bases: usize,
    max_total_edits: Option<usize>,
) -> Option<Range<u32>> {
    if query.is_empty() || reference.is_empty() {
        return None;
    }

    let total_limit = max_total_edits.unwrap_or(max_mismatches + max_indel_bases);

    if reference.len() + max_indel_bases < query.len() {
        return None; // cov:excl-line
    }

    let query_upper: Vec<u8> = query.iter().map(u8::to_ascii_uppercase).collect();
    let reference_upper: Vec<u8> = reference.iter().map(u8::to_ascii_uppercase).collect();

    let base_scoring = Scoring::new(0, -1, iupac_alignment_score);
    let scoring = match anchor {
        Anchor::Left => base_scoring.yclip_prefix(MIN_SCORE).yclip_suffix(0),
        Anchor::Right => base_scoring.yclip_prefix(0).yclip_suffix(MIN_SCORE),
        Anchor::Anywhere => base_scoring.yclip(0),
    };

    let mut aligner = Aligner::with_scoring(scoring);
    let alignment = aligner.custom(&query_upper, &reference_upper);

    if alignment.operations.is_empty() {
        return None; // cov:excl-line
    }

    assert!(
        (matches!(anchor, Anchor::Left) && alignment.ystart == 0)
            || (matches!(anchor, Anchor::Right) && alignment.yend == reference.len())
            || matches!(anchor, Anchor::Anywhere),
        "Alignment produced invalid coordinates for the specified anchor"
    );

    let mut mismatches = 0usize;
    let mut insertions = 0usize;
    let mut deletions = 0usize;

    for op in &alignment.operations {
        match op {
            AlignmentOperation::Subst => mismatches += 1,
            AlignmentOperation::Del => insertions += 1,
            AlignmentOperation::Ins => deletions += 1,
            AlignmentOperation::Match
            | AlignmentOperation::Xclip(_)
            | AlignmentOperation::Yclip(_) => {}
        }
    }

    let total_indels = insertions + deletions;
    if mismatches > max_mismatches
        || total_indels > max_indel_bases
        || mismatches + total_indels > total_limit
    {
        return None;
    }

    let start = alignment.ystart;
    let end = alignment.yend;

    assert!(
        end >= start,
        "Alignment produced invalid coordinates (end < start)"
    );
    assert!(
        end <= reference.len(),
        "Alignment produced invalid coordinates (end > reference length)"
    );

    Some(
        start.try_into().expect("Start would not fit u32")
            ..end.try_into().expect("end would not fit u32"),
    )
}

#[derive(Clone, Copy)]
pub enum Direction {
    Forward,
    Reverse,
}

/// Find the best hit for this IUPAC string, on parity, earlier hits preferred.
/// Optimized pure Rust implementation with early exit on perfect matches.
/// Returns the start position of the best match, or None if no match within `max_mismatches`.
#[inline]
#[must_use]
pub fn iupac_find_best(
    pattern: &[u8],
    reference: &[u8],
    max_mismatches: usize,
    direction: Direction,
) -> Option<usize> {
    let query_len = pattern.len();
    let mut best_pos = None;
    let mut best_so_far = max_mismatches + 1;

    let iter: Box<dyn Iterator<Item = usize>> = match direction {
        Direction::Forward => Box::new(0..=reference.len() - query_len),
        Direction::Reverse => Box::new((0..=reference.len() - query_len).rev()),
    };

    for start in iter {
        let hd = iupac_hamming_distance_with_limit(
            pattern,
            &reference[start..start + query_len],
            best_so_far,
        );

        if hd == 0 {
            return Some(start);
        } else if hd < best_so_far {
            best_so_far = hd;
            best_pos = Some(start);
        }
    }

    if best_so_far <= max_mismatches {
        best_pos
    } else {
        None
    }
}

//check the complete string is valid dna + iupac, upper case only
#[must_use]
pub fn first_non_iupac(input: &[u8]) -> Option<u8> {
    input.iter().map(u8::to_ascii_uppercase).find(|char| {
        !matches!(
            char,
            b'A' | b'T'
                | b'U'
                | b'C'
                | b'G'
                | b'R'
                | b'Y'
                | b'S'
                | b'W'
                | b'K'
                | b'M'
                | b'B'
                | b'V'
                | b'D'
                | b'H'
                | b'N'
        )
    })
}

#[must_use]
pub fn all_iupac_or_underscore(input: &[u8]) -> bool {
    input.iter().all(|&char| {
        matches!(
            char,
            b'A' | b'T'
                | b'U'
                | b'C'
                | b'G'
                | b'R'
                | b'Y'
                | b'S'
                | b'W'
                | b'K'
                | b'M'
                | b'B'
                | b'V'
                | b'D'
                | b'H'
                | b'N'
                | b'_'
        )
    })
}

/// Reverse complement a DNA sequence.
/// Handles standard bases (ATCGN) in upper and lowercase;
/// non-DNA characters are passed through unchanged.
#[must_use]
pub fn reverse_complement(seq: &[u8]) -> Vec<u8> {
    seq.iter()
        .rev()
        .map(|&base| match base {
            b'A' => b'T',
            b'T' => b'A',
            b'C' => b'G',
            b'G' => b'C',
            b'a' => b't',
            b't' => b'a',
            b'c' => b'g',
            b'g' => b'c',
            _ => base,
        })
        .collect()
}

///
/// Reverse complement a DNA sequence considering IUPAC ambiguity codes.
///
/// # Panics
/// On newline (our parsers never have newlines)
#[must_use]
pub fn reverse_complement_iupac(input: &[u8]) -> Vec<u8> {
    let mut new_seq = Vec::new();
    for char in input.iter().rev() {
        new_seq.push(match char {
            b'A' => b'T',
            b'T' | b'U' => b'A',
            b'C' => b'G',
            b'G' => b'C',

            b'a' => b't',
            b't' | b'u' => b'a',
            b'c' => b'g',
            b'g' => b'c',

            b'R' => b'Y',
            b'Y' => b'R',
            b'K' => b'M',
            b'M' => b'K',
            b'B' => b'V',
            b'V' => b'B',
            b'D' => b'H',
            b'H' => b'D',

            b'r' => b'y',
            b'y' => b'r',
            b'k' => b'm',
            b'm' => b'k',
            b'b' => b'v',
            b'v' => b'b',
            b'd' => b'h',
            b'h' => b'd',
            b'\n' => panic!("New line in DNA sequence"),
            _ => *char,
        });
    }
    new_seq
}

/// Straight up hamming distance. No frills.
///
/// # Panics
///
/// on unequal lengths
#[must_use]
pub fn hamming_distance(a: &[u8], b: &[u8]) -> usize {
    assert_eq!(
        a.len(),
        b.len(),
        "Hamming distance requires sequences of equal length"
    );
    let mut dist = 0;
    for (letter_a, letter_b) in a.iter().zip(b.iter()) {
        if letter_a != letter_b {
            dist += 1;
        }
    }
    dist
}

/// Calculate IUPAC-aware Hamming distance between a pattern and a sequence.
///
/// # Panics
/// on unequal lengths
#[inline]
#[must_use]
pub fn iupac_hamming_distance(iupac_reference: &[u8], atcg_query: &[u8]) -> usize {
    assert_eq!(
        iupac_reference.len(),
        atcg_query.len(),
        "Hamming distance problem: Reference and query must have same length.\n\
        Reference='{iupac_reference:?}' query='{atcg_query:?}'"
    );
    iupac_hamming_distance_with_limit(iupac_reference, atcg_query, usize::MAX)
}

/// Optimized IUPAC Hamming distance with early exit when distance exceeds limit.
#[inline]
fn iupac_hamming_distance_with_limit(
    iupac_pattern: &[u8],
    atcg_query: &[u8],
    limit: usize,
) -> usize {
    let mut dist = 0;

    for (a, b) in iupac_pattern.iter().zip(atcg_query.iter()) {
        if a == b {
            continue;
        }

        let is_match = matches!(
            (a, b),
            (b'A', b'a')
                | (b'a', b'A')
                | (b'C', b'c')
                | (b'c', b'C')
                | (b'G', b'g')
                | (b'g', b'G')
                | (b'T', b't')
                | (b't', b'T')
                | (b'R' | b'r', b'A' | b'G' | b'a' | b'g')
                | (b'Y' | b'y', b'C' | b'T' | b'c' | b't')
                | (b'S' | b's', b'G' | b'C' | b'g' | b'c')
                | (b'W' | b'w', b'A' | b'T' | b'a' | b't')
                | (b'K' | b'k', b'G' | b'T' | b'g' | b't')
                | (b'M' | b'm', b'A' | b'C' | b'a' | b'c')
                | (b'B' | b'b', b'C' | b'G' | b'T' | b'c' | b'g' | b't')
                | (b'D' | b'd', b'A' | b'G' | b'T' | b'a' | b'g' | b't')
                | (b'H' | b'h', b'A' | b'C' | b'T' | b'a' | b'c' | b't')
                | (b'V' | b'v', b'A' | b'C' | b'G' | b'a' | b'c' | b'g')
                | (
                    b'N' | b'n',
                    b'A' | b'C' | b'G' | b'T' | b'a' | b'c' | b'g' | b't' | b'N' | b'n'
                )
        );
        if !is_match {
            dist += 1;
            if dist >= limit {
                return dist;
            }
        }
    }

    dist
}

/// Check if two IUPAC barcode patterns can accept the same sequence
#[must_use]
pub fn iupac_overlapping(pattern1: &[u8], pattern2: &[u8]) -> bool {
    if pattern1.len() != pattern2.len() {
        return false;
    }
    for (c1, c2) in pattern1.iter().zip(pattern2.iter()) {
        if !positions_compatible(*c1, *c2) {
            return false;
        }
    }
    true
}

fn positions_compatible(c1: u8, c2: u8) -> bool {
    let set1 = iupac_to_bases(c1);
    for base2 in iupac_to_bases(c2) {
        if set1.contains(base2) {
            return true;
        }
    }
    false
}

/// # panics
/// on non-iupac bases
fn iupac_to_bases(c: u8) -> &'static [u8] {
    match c.to_ascii_uppercase() {
        b'A' => b"A",
        b'C' => b"C",
        b'G' => b"G",
        b'T' | b'U' => b"T",
        b'R' => b"AG",
        b'Y' => b"CT",
        b'S' => b"CG",
        b'W' => b"AT",
        b'K' => b"GT",
        b'M' => b"AC",
        b'B' => b"CGT",
        b'D' => b"AGT",
        b'H' => b"ACT",
        b'V' => b"ACG",
        b'N' => b"ACGT",
        b'_' => b"",
        _ => panic!("non iupac string passed to iupac_to_bases"),
    }
}

pub struct IupacExpander {
    positions: Vec<&'static [u8]>,
    indices: Option<Vec<usize>>,
}

impl IupacExpander {
    pub fn new(iupac: &BStr) -> Self {
        let positions: Vec<&'static [u8]> = iupac
            .bytes()
            .map(iupac_to_bases)
            .filter(|bases| !bases.is_empty())
            .collect();

        let indices = Some(vec![0usize; positions.len()]);
        Self { positions, indices }
    }

    fn advance(&mut self) -> bool {
        let indices = self
            .indices
            .as_mut()
            .expect("Inner advance called after iterator was exhausted");
        for i in (0..indices.len()).rev() {
            indices[i] += 1;
            if indices[i] < self.positions[i].len() {
                return true;
            }
            indices[i] = 0;
        }
        false
    }
}

impl Iterator for IupacExpander {
    type Item = BString;

    fn next(&mut self) -> Option<Self::Item> {
        let indices = self.indices.as_ref()?;

        let seq: BString = indices
            .iter()
            .zip(&self.positions)
            .map(|(&i, bases)| bases[i])
            .collect();

        if !self.advance() {
            self.indices = None;
        }

        Some(seq)
    }
}

///
/// # Errors
///
/// when seqs are of unequal length
pub fn init_hamming_resonator(
    seq_to_name: &IndexMap<BString, String>,
    max_dist: u8,
) -> Result<HammingResonator, ValidationFailure> {
    let seqs: Vec<BString> = seq_to_name.keys().cloned().collect();

    HammingResonator::new(seqs, max_dist.into())
        .map_err(|e| ValidationFailure::new(format!("Hamming resonator failed to init: {e}"), None))
}

/// Find out exactly what's the minimum number of bits to represent a number in binary
#[expect(
    clippy::cast_possible_truncation,
    reason = "Can not be more than usize::BITS, so 32, no truncation possible"
)]
#[must_use]
pub fn bits_needed_to_represent(count: usize) -> u16 {
    if count <= 1 {
        1u16
    } else {
        (usize::BITS - (count).leading_zeros()) as u16
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod test {
    fn check(should: &[u8], input: &[u8]) {
        let s: Vec<u8> = should.to_vec();
        assert_eq!(
            std::str::from_utf8(&s).expect("test DNA string should be valid UTF-8"),
            std::str::from_utf8(&super::reverse_complement_iupac(input))
                .expect("test DNA string should be valid UTF-8")
        );
    }

    #[test]
    fn test_rev_complement() {
        check(b"AGCT", b"AGCT");
        check(b"DHBVNKMWSRYAAGCT", b"AGCTURYSWKMNBVDH");
        check(b"dhbvnkmwsryaagct", b"agcturyswkmnbvdh");
    }

    #[test]
    #[should_panic(expected = "New line in DNA sequence")]
    fn test_rev_complement_panics_on_newline() {
        let _ = super::reverse_complement_iupac(b"AGCT\n");
    }

    #[test]
    fn test_reverse_complement_uupac() {
        let input = b"AGCTRYSWKMNBVDH";
        let rev = super::reverse_complement_iupac(input);
        let rev_rev = super::reverse_complement_iupac(&rev);
        let rev2 = super::reverse_complement_iupac(&input.to_ascii_lowercase());
        let rev_rev2 = super::reverse_complement_iupac(&rev2);

        assert!(rev_rev == input);
        assert!(rev_rev2 == input.to_ascii_lowercase());
        assert!(rev_rev != rev);
    }

    #[test]
    fn test_reverse_complement_upac_lower() {
        let input = b"agctryswkmnbvdh";
        let rev = super::reverse_complement_iupac(input);
        let rev_rev = super::reverse_complement_iupac(&rev);

        assert!(rev_rev == input);
        assert!(rev_rev != rev);
    }

    #[test]
    fn test_reverse_complement() {
        assert_eq!(super::reverse_complement(b"ATCG"), b"CGAT");
        assert_eq!(super::reverse_complement(b"AAAA"), b"TTTT");
        assert_eq!(super::reverse_complement(b"CGCG"), b"CGCG");
        assert_eq!(super::reverse_complement(b"ATCGN"), b"NCGAT");
        assert_eq!(super::reverse_complement(b"atcg"), b"cgat");
        assert_eq!(super::reverse_complement(b"AtCg"), b"cGaT");
        assert_eq!(super::reverse_complement(b""), b"");
        assert_eq!(super::reverse_complement(b"atcg"), b"cgat");
        assert_eq!(super::reverse_complement(b"aaaa"), b"tttt");
        assert_eq!(super::reverse_complement(b"cgcg"), b"cgcg");
        assert_eq!(super::reverse_complement(b"cgcgn"), b"ncgcg");
    }

    #[test]
    fn test_iupac_hamming_distance() {
        assert_eq!(super::iupac_hamming_distance(b"AGCT", b"AGCT"), 0);
        assert_eq!(super::iupac_hamming_distance(b"AGCT", b"AGCA"), 1);
        assert_eq!(super::iupac_hamming_distance(b"AGCT", b"AGCG"), 1);
        assert_eq!(super::iupac_hamming_distance(b"NGCC", b"AGCC"), 0);
        assert_eq!(super::iupac_hamming_distance(b"NGCC", b"AGCT"), 1);
        assert_eq!(super::iupac_hamming_distance(b"NGCC", b"cGCT"), 1);

        assert_eq!(super::iupac_hamming_distance(b"AGKC", b"agKc"), 0);
        assert_eq!(super::iupac_hamming_distance(b"AGKC", b"agkc"), 1);
        let should = vec![
            (b'R', (0, 1, 0, 1)),
            (b'Y', (1, 0, 1, 0)),
            (b'S', (1, 0, 0, 1)),
            (b'W', (0, 1, 1, 0)),
            (b'K', (1, 1, 0, 0)),
            (b'M', (0, 0, 1, 1)),
            (b'B', (1, 0, 0, 0)),
            (b'D', (0, 1, 0, 0)),
            (b'H', (0, 0, 1, 0)),
            (b'V', (0, 0, 0, 1)),
            (b'N', (0, 0, 0, 0)),
        ];
        for (letter, actg) in &should {
            let str_letter = std::str::from_utf8(&[*letter])
                .expect("single ASCII letter should be valid UTF-8")
                .to_string();
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"A"),
                actg.0,
                "wrong result {str_letter} vs A"
            );
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"C"),
                actg.1,
                "wrong result {str_letter} vs C"
            );
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"G"),
                actg.2,
                "wrong result {str_letter} vs G"
            );
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"T"),
                actg.3,
                "wrong result {str_letter} vs T"
            );
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"a"),
                actg.0,
                "wrong result {str_letter} vs a"
            );
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"c"),
                actg.1,
                "wrong result {str_letter} vs c"
            );
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"g"),
                actg.2,
                "wrong result {str_letter} vs g"
            );
            assert_eq!(
                super::iupac_hamming_distance(&[*letter], b"t"),
                actg.3,
                "wrong result {str_letter} vs t"
            );
        }
    }

    fn draft_for(start: usize, len: usize) -> Range<u32> {
        start.try_into().expect("start would not fit an u32")
            ..(start + len)
                .try_into()
                .expect("start + len would not fit an u32")
    }

    #[test]
    fn test_find_iupac() {
        assert_eq!(
            super::find_iupac(b"AGTTC", b"AGT", super::Anchor::Left, 0, 0),
            Some(draft_for(0, 3))
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"TTC", super::Anchor::Right, 0, 0),
            Some(draft_for(2, 3))
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"GT", super::Anchor::Anywhere, 0, 0),
            Some(draft_for(1, 2))
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"AGT", super::Anchor::Anywhere, 0, 0),
            Some(draft_for(0, 3))
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"TTC", super::Anchor::Anywhere, 0, 0),
            Some(draft_for(2, 3))
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"GT", super::Anchor::Left, 0, 0),
            None
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"GT", super::Anchor::Right, 0, 0),
            None
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"GG", super::Anchor::Anywhere, 0, 0),
            None,
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"T", super::Anchor::Anywhere, 0, 0),
            Some(draft_for(2, 1))
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"AA", super::Anchor::Left, 1, 0),
            Some(draft_for(0, 2))
        );
        assert_eq!(
            super::find_iupac(b"AGTTC", b"AGTTN", super::Anchor::Left, 0, 0),
            Some(draft_for(0, 5))
        );
    }

    #[test]
    fn test_find_iupac_with_indel() {
        assert_eq!(
            super::find_iupac_with_indel(b"AGTTC", b"AGT", super::Anchor::Anywhere, 0, 0, None,),
            Some(draft_for(0, 3))
        );

        assert_eq!(
            super::find_iupac_with_indel(b"AGTTC", b"AAT", super::Anchor::Left, 1, 0, None,),
            Some(draft_for(0, 3))
        );

        assert_eq!(
            super::find_iupac_with_indel(b"AGGTC", b"AGTC", super::Anchor::Anywhere, 0, 1, None,),
            Some(draft_for(0, 5))
        );

        assert_eq!(
            super::find_iupac_with_indel(b"AGTC", b"AGGTC", super::Anchor::Anywhere, 0, 1, None,),
            Some(draft_for(0, 4))
        );

        assert_eq!(
            super::find_iupac_with_indel(b"CCAGTTC", b"AGT", super::Anchor::Left, 0, 1, None,),
            None
        );

        assert_eq!(
            super::find_iupac_with_indel(b"CCAGTTC", b"AGT", super::Anchor::Right, 0, 1, None,),
            None
        );

        assert_eq!(
            super::find_iupac_with_indel(b"CCAGTTC", b"TTC", super::Anchor::Right, 0, 1, None,),
            Some(draft_for(4, 3))
        );

        assert_eq!(
            super::find_iupac_with_indel(b"AGGTC", b"AATC", super::Anchor::Anywhere, 0, 1, None,),
            None
        );

        assert_eq!(
            super::find_iupac_with_indel(b"AGGTC", b"AATC", super::Anchor::Anywhere, 1, 1, Some(1),),
            None
        );

        assert_eq!(
            super::find_iupac_with_indel(
                b"AGNNNNNNNNTC",
                b"AGTC",
                super::Anchor::Anywhere,
                1,
                20,
                Some(5),
            ),
            None
        );

        assert_eq!(
            super::find_iupac_with_indel(b"", b"AGT", super::Anchor::Anywhere, 0, 0, None,),
            None
        );
        assert_eq!(
            super::find_iupac_with_indel(b"AG", b"", super::Anchor::Anywhere, 0, 0, None,),
            None
        );
    }

    use super::*;

    #[test]
    fn test_positions_compatible() {
        assert!(positions_compatible(b'A', b'A'));
        assert!(positions_compatible(b'T', b'T'));
        assert!(!positions_compatible(b'A', b'T'));
        assert!(!positions_compatible(b'C', b'G'));
        assert!(positions_compatible(b'R', b'A'));
        assert!(positions_compatible(b'R', b'G'));
        assert!(!positions_compatible(b'R', b'C'));
        assert!(!positions_compatible(b'R', b'T'));
        assert!(positions_compatible(b'N', b'A'));
        assert!(positions_compatible(b'N', b'T'));
        assert!(positions_compatible(b'N', b'C'));
        assert!(positions_compatible(b'N', b'G'));
    }

    #[test]
    fn test_iupac_overlapping() {
        assert!(!iupac_overlapping(b"AT", b"ATC"));
        assert!(iupac_overlapping(b"ATCG", b"ATCG"));
        assert!(!iupac_overlapping(b"ATCG", b"GGCC"));
        assert!(iupac_overlapping(b"NNNN", b"ATCG"));
        assert!(iupac_overlapping(b"ATCG", b"NNNN"));
        assert!(iupac_overlapping(b"ATVG", b"ATCG"));
        assert!(iupac_overlapping(b"ATCG", b"ATCN"));
        assert!(iupac_overlapping(b"N", b"A"));
        assert!(iupac_overlapping(b"N", b"G"));
        assert!(iupac_overlapping(b"N", b"C"));
        assert!(iupac_overlapping(b"N", b"T"));
        assert!(iupac_overlapping(b"R", b"A"));
        assert!(iupac_overlapping(b"R", b"G"));
        assert!(!iupac_overlapping(b"R", b"C"));
        assert!(!iupac_overlapping(b"R", b"T"));
        assert!(iupac_overlapping(b"Y", b"C"));
        assert!(iupac_overlapping(b"Y", b"T"));
        assert!(!iupac_overlapping(b"Y", b"A"));
        assert!(!iupac_overlapping(b"Y", b"G"));
        assert!(iupac_overlapping(b"S", b"G"));
        assert!(iupac_overlapping(b"S", b"C"));
        assert!(!iupac_overlapping(b"S", b"A"));
        assert!(!iupac_overlapping(b"S", b"T"));
        assert!(iupac_overlapping(b"W", b"A"));
        assert!(iupac_overlapping(b"W", b"T"));
        assert!(!iupac_overlapping(b"W", b"G"));
        assert!(!iupac_overlapping(b"W", b"C"));
        assert!(iupac_overlapping(b"K", b"G"));
        assert!(iupac_overlapping(b"K", b"T"));
        assert!(!iupac_overlapping(b"K", b"A"));
        assert!(!iupac_overlapping(b"K", b"C"));
        assert!(iupac_overlapping(b"M", b"A"));
        assert!(iupac_overlapping(b"M", b"C"));
        assert!(!iupac_overlapping(b"M", b"G"));
        assert!(!iupac_overlapping(b"M", b"T"));
        assert!(iupac_overlapping(b"B", b"C"));
        assert!(iupac_overlapping(b"B", b"G"));
        assert!(iupac_overlapping(b"B", b"T"));
        assert!(!iupac_overlapping(b"B", b"A"));
        assert!(iupac_overlapping(b"D", b"A"));
        assert!(iupac_overlapping(b"D", b"G"));
        assert!(iupac_overlapping(b"D", b"T"));
        assert!(!iupac_overlapping(b"D", b"C"));
        assert!(iupac_overlapping(b"H", b"A"));
        assert!(iupac_overlapping(b"H", b"C"));
        assert!(iupac_overlapping(b"H", b"T"));
        assert!(!iupac_overlapping(b"H", b"G"));
        assert!(iupac_overlapping(b"V", b"A"));
        assert!(iupac_overlapping(b"V", b"C"));
        assert!(iupac_overlapping(b"V", b"G"));
        assert!(!iupac_overlapping(b"V", b"T"));
        assert!(iupac_overlapping(b"U", b"T"));
        assert!(iupac_overlapping(b"U", b"U"));
        assert!(!iupac_overlapping(b"U", b"C"));
        assert!(!iupac_overlapping(b"U", b"G"));
        assert!(!iupac_overlapping(b"U", b"A"));
        assert!(!iupac_overlapping(b"RYRY", b"ATCG"));
    }

    #[test]
    fn test_all_iupac() {
        assert_eq!(first_non_iupac(b"A"), None);
        assert_eq!(first_non_iupac(b"AAAA"), None);
        assert_eq!(first_non_iupac(b"aaaycn"), None);
        assert_eq!(first_non_iupac(b"AAAx"), Some(b'X'));
        assert_eq!(first_non_iupac(b"R"), None);
        assert_eq!(first_non_iupac(b"AGCTURYSWKMNBVDH"), None);
        assert_eq!(first_non_iupac(b"aGCTURYSWKMNBVDH"), None);
    }

    #[test]
    #[should_panic(expected = "non iupac string passed to iupac_to_bases")]
    fn test_iupac_to_bases_panics_on_non_iupac() {
        iupac_to_bases(b'X');
    }

    #[test]
    fn test_iupac_expander() {
        assert_eq!(
            IupacExpander::new(b"A".into()).collect::<Vec<BString>>(),
            vec![BString::from(b"A")]
        );
        assert_eq!(
            IupacExpander::new(b"N".into()).collect::<Vec<BString>>(),
            vec![
                BString::from(b"A"),
                BString::from(b"C"),
                BString::from(b"G"),
                BString::from(b"T"),
            ]
        );
        assert_eq!(
            IupacExpander::new(b"AGTRA".into()).collect::<Vec<BString>>(),
            vec![BString::from(b"AGTAA"), BString::from(b"AGTGA"),]
        );
        assert_eq!(
            IupacExpander::new(b"NAGTRA".into()).collect::<Vec<BString>>(),
            vec![
                BString::from(b"AAGTAA"),
                BString::from(b"AAGTGA"),
                BString::from(b"CAGTAA"),
                BString::from(b"CAGTGA"),
                BString::from(b"GAGTAA"),
                BString::from(b"GAGTGA"),
                BString::from(b"TAGTAA"),
                BString::from(b"TAGTGA"),
            ]
        );
        assert_eq!(
            IupacExpander::new(b"VAGTRA".into()).collect::<Vec<BString>>(),
            vec![
                BString::from(b"AAGTAA"),
                BString::from(b"AAGTGA"),
                BString::from(b"CAGTAA"),
                BString::from(b"CAGTGA"),
                BString::from(b"GAGTAA"),
                BString::from(b"GAGTGA"),
            ]
        );
        assert_eq!(
            IupacExpander::new(b"SAGTRA".into()).collect::<Vec<BString>>(),
            vec![
                BString::from(b"CAGTAA"),
                BString::from(b"CAGTGA"),
                BString::from(b"GAGTAA"),
                BString::from(b"GAGTGA"),
            ]
        );
        assert_eq!(
            IupacExpander::new(b"_S_AGT__RA".into()).collect::<Vec<BString>>(),
            vec![
                BString::from(b"CAGTAA"),
                BString::from(b"CAGTGA"),
                BString::from(b"GAGTAA"),
                BString::from(b"GAGTGA"),
            ]
        );
    }
}
