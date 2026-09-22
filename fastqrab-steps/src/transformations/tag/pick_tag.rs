use crate::transformations::prelude::*;

/// The policy `PickTag` uses to choose one of several tags' values per read.
///
/// Every tag in `in_labels` must have the same underlying type (Location,
/// String or Numeric - Bool tags are not supported). Which of those a given
/// policy accepts is noted below.
#[derive(Clone, JsonSchema, PartialEq, Eq, Copy)]
#[tpd]
#[derive(Debug)]
pub enum PickPolicy {
    /// Pick the left-most (first in `in_labels`) tag that has a value for
    /// this read. Location, String or Numeric tags only - Numeric tags use
    /// `NaN` as their "missing" value, same as `equal_only` below.
    FirstAvailable,
    /// Pick a value only when every input tag agrees for this read;
    /// otherwise the output is missing. Works with Location, String or
    /// Numeric tags:
    /// - Location / String: compares the extracted text.
    /// - Numeric: compares the values; disagreement outputs `NaN` (there is
    ///   no "missing" numeric value).
    EqualOnly,
    /// Location or Numeric tags only. Pick the tag with the largest measure
    /// for this read:
    /// - Location: the sum of that tag's region lengths.
    /// - Numeric: the value itself.
    Largest,
    /// Location or Numeric tags only. Pick the tag with the smallest measure
    /// for this read - the inverse of `largest`:
    /// - Location: the sum of that tag's region lengths.
    /// - Numeric: the value itself.
    Smallest,
    /// Location tags only. Pick the tag whose first region starts earliest,
    /// comparing `(segment, start coordinate)` tuples - so this also picks
    /// between tags living on different segments.
    LeftMostRegion,
}

/// Pick one value out of several tags per read, according to `policy`.
///
/// # Examples
///
/// ```toml
/// [[step]]
/// action = "PickTag"
/// in_labels = ["barcode1", "barcode2"]
/// out_label = "barcode"
/// policy = "first_available"
/// ```
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct PickTag {
    /// Input tag labels to choose between. Must all have the same tag type.
    in_labels: Vec<TagLabel>,

    /// Output tag label for the picked value.
    out_label: TagLabel,

    /// The policy used to choose which tag's value to output for each read.
    /// - `first_available`: the left-most tag (in `in_labels`) that has a value.
    /// - `equal_only`: a value, but only if every input tag agrees; else missing.
    /// - `largest`: the tag with the largest measure (region length sum, or numeric value).
    /// - `smallest`: the tag with the smallest measure - the inverse of `largest`.
    /// - `left_most_region`: Location tags only - the tag starting earliest, comparing
    ///   `(segment, start)` across all input tags.
    policy: PickPolicy,
}

impl VerifyIn<PartialConfig> for PartialPickTag {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.in_labels.verify_mut(|v| {
            let mut seen: IndexMap<&TagLabel, std::ops::Range<usize>> = IndexMap::new();
            for label in v.iter_mut() {
                let lv = label.value.as_ref().expect("Parent was ok?");
                match seen.entry(lv) {
                    indexmap::map::Entry::Occupied(occupied_entry) => {
                        let spans = vec![
                            (label.span(), "Duplicate input label".to_string()),
                            (occupied_entry.get().clone(), "First occurrence".to_string()),
                        ];
                        label.state = TomlValueState::Custom { spans };
                    }
                    indexmap::map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(label.span());
                    }
                }
            }
            Ok(())
        });

        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialPickTag> {
    fn get_tag_usage(
        &mut self,
        tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            let policy = inner.policy.as_ref().copied();
            let accepted_tag_types: &[TagValueType] = match policy {
                Some(PickPolicy::FirstAvailable | PickPolicy::EqualOnly) | None => &[
                    TagValueType::Location,
                    TagValueType::String,
                    TagValueType::Numeric((None, None)),
                ],
                Some(PickPolicy::Largest | PickPolicy::Smallest) => {
                    &[TagValueType::Location, TagValueType::Numeric((None, None))]
                }
                Some(PickPolicy::LeftMostRegion) => &[TagValueType::Location],
            };

            let mut common_type: Option<TagValueType> = None;
            let mut common_span: Option<std::ops::Range<usize>> = None;
            let mut common_segment: Option<SegmentIndex> = None;
            let mut segment_mismatch = false;

            let used_tags: Vec<_> = if let Some(tv_in_labels) = inner.in_labels.value.as_mut() {
                tv_in_labels
                    .iter_mut()
                    .filter(|x| x.is_ok())
                    .map(|x| {
                        if let Some(label) = x.as_ref()
                            && let Some(meta) = tags_available.get(label)
                        {
                            match common_type {
                                None => {
                                    common_type = Some(meta.tag_type);
                                    common_span = Some(x.span());
                                }
                                Some(first_type) if first_type.compatible(meta.tag_type) => {}
                                Some(first_type) => {
                                    let spans = vec![
                                        (x.span(), format!("This tag is {}", meta.tag_type)),
                                        (
                                            common_span.clone().expect("set alongside common_type"),
                                            format!("First tag is {first_type}"),
                                        ),
                                    ];
                                    x.state = TomlValueState::Custom { spans };
                                }
                            }
                            if matches!(meta.tag_type, TagValueType::Location) {
                                match (common_segment, meta.segment) {
                                    (None, seg) => common_segment = seg,
                                    (Some(a), Some(b)) if a == b => {}
                                    _ => segment_mismatch = true,
                                }
                            }
                        }
                        x.to_used_tag(accepted_tag_types)
                    })
                    .collect()
            } else {
                vec![]
            };

            let output_type = match common_type {
                Some(TagValueType::Location) if segment_mismatch => TagValueType::String,
                Some(other) => other,
                None => TagValueType::String,
            };

            Some(TagUsageInfo {
                used_tags,
                declared_tag: inner.out_label.to_declared_tag(output_type).map(|dt| {
                    match (output_type, common_segment) {
                        (TagValueType::Location, Some(seg)) => dt.with_segment(seg),
                        _ => dt,
                    }
                }),
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

/// Which of `tag_columns` (all `Location`) wins row `i` under `policy`, or
/// `None` if no tag has a hit.
fn location_winner(policy: PickPolicy, tag_columns: &[&TagColumn], i: usize) -> Option<usize> {
    match policy {
        PickPolicy::FirstAvailable => tag_columns.iter().position(|col| {
            let TagColumn::Location(loc) = col else {
                unreachable!("location_winner only sees Location tags") // cov:excl-line
            };
            !loc.row_is_empty(i)
        }),
        PickPolicy::EqualOnly => {
            let mut values = tag_columns.iter().map(|col| {
                let TagColumn::Location(loc) = col else {
                    unreachable!("location_winner only sees Location tags") // cov:excl-line
                };
                (!loc.row_is_empty(i)).then(|| loc.joined_seq(i, None))
            });
            let first = values.next().expect("in_labels is non-empty");
            if values.all(|v| v == first) {
                first.is_some().then_some(0)
            } else {
                None
            }
        }
        PickPolicy::Largest => {
            let mut best: Option<(usize, usize)> = None; // (winning index, total region length)
            for (idx, col) in tag_columns.iter().enumerate() {
                let TagColumn::Location(loc) = col else {
                    unreachable!("location_winner only sees Location tags") // cov:excl-line
                };
                if loc.row_is_empty(i) {
                    continue;
                }
                let len = loc.row_length(i, None);
                if best.is_none_or(|(_, best_len)| len > best_len) {
                    best = Some((idx, len));
                }
            }
            best.map(|(idx, _)| idx)
        }
        PickPolicy::Smallest => {
            let mut best: Option<(usize, usize)> = None; // (winning index, total region length)
            for (idx, col) in tag_columns.iter().enumerate() {
                let TagColumn::Location(loc) = col else {
                    unreachable!("location_winner only sees Location tags") // cov:excl-line
                };
                if loc.row_is_empty(i) {
                    continue;
                }
                let len = loc.row_length(i, None);
                if best.is_none_or(|(_, best_len)| len < best_len) {
                    best = Some((idx, len));
                }
            }
            best.map(|(idx, _)| idx)
        }
        PickPolicy::LeftMostRegion => {
            let mut best: Option<(usize, u32, u32)> = None; // (winning index, segment, start)
            for (idx, col) in tag_columns.iter().enumerate() {
                let TagColumn::Location(loc) = col else {
                    unreachable!("location_winner only sees Location tags") // cov:excl-line
                };
                if loc.row_is_empty(i) {
                    continue;
                }
                let start = loc
                    .row_regions(i)
                    .next()
                    .expect("non-empty row has at least one region")
                    .0;
                let segment = loc.source_id();
                if best.is_none_or(|(_, best_seg, best_start)| {
                    (segment, start) < (best_seg, best_start)
                }) {
                    best = Some((idx, segment, start));
                }
            }
            best.map(|(idx, _, _)| idx)
        }
    }
}

impl Step for PickTag {
    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let num_reads = block.segments[0].len();

        let tag_columns: Vec<&TagColumn> = self
            .in_labels
            .iter()
            .map(|label| {
                block
                    .tags
                    .get(label)
                    .ok_or_else(|| anyhow::anyhow!("Tag '{label}' not found in block"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        // `in_labels` is non-empty and all its tags share the same type - both
        // guaranteed by config-time validation - so the first column tells us
        // which branch below applies. Bool is never accepted, so that branch
        // is unreachable.
        match tag_columns[0] {
            TagColumn::Location(_) => {
                let first_segment = tag_columns[0].location_segment();
                let same_segment = tag_columns
                    .iter()
                    .all(|col| col.location_segment() == first_segment);

                if same_segment {
                    let mut builder = block.location_column_builder(first_segment);
                    for i in 0..num_reads {
                        match location_winner(self.policy, &tag_columns, i) {
                            Some(idx) => {
                                let TagColumn::Location(loc) = tag_columns[idx] else {
                                    unreachable!("all inputs are Location here") // cov:excl-line
                                };
                                let regions: SmallVec<[(u32, u32); 1]> =
                                    loc.row_regions(i).collect();
                                builder.push_row(&regions);
                            }
                            None => builder.push_row(&[]),
                        }
                    }
                    block.tags.insert(
                        self.out_label.clone(),
                        TagColumn::Location(builder.finish()),
                    );
                } else {
                    // A single Location column can only live on one segment, so
                    // when the input tags don't all share one, fall back to the
                    // winner's extracted text.
                    let mut out = StringColumnBuilder::new();
                    for i in 0..num_reads {
                        let value = location_winner(self.policy, &tag_columns, i).map(|idx| {
                            let TagColumn::Location(loc) = tag_columns[idx] else {
                                unreachable!("all inputs are Location here") // cov:excl-line
                            };
                            loc.joined_seq(i, None)
                        });
                        out.push(value);
                    }
                    block
                        .tags
                        .insert(self.out_label.clone(), TagColumn::String(out.finish()));
                }
            }
            TagColumn::String(_) => {
                let mut out = StringColumnBuilder::new();
                for i in 0..num_reads {
                    let value = match self.policy {
                        PickPolicy::FirstAvailable => {
                            tag_columns.iter().find_map(|col| col.get_string(i))
                        }
                        PickPolicy::EqualOnly => {
                            let mut values = tag_columns.iter().map(|col| col.get_string(i));
                            let first = values.next().expect("in_labels is non-empty");
                            if values.all(|v| v == first) {
                                first
                            } else {
                                None
                            }
                        }
                        PickPolicy::Largest | PickPolicy::Smallest | PickPolicy::LeftMostRegion => {
                            unreachable!("rejected at config time for String tags") // cov:excl-line
                        }
                    };
                    out.push(value.map(Cow::Borrowed));
                }
                block
                    .tags
                    .insert(self.out_label.clone(), TagColumn::String(out.finish()));
            }
            TagColumn::Numeric(_) => {
                let mut out: Vec<f64> = Vec::with_capacity(num_reads);
                for i in 0..num_reads {
                    let value = match self.policy {
                        // NaN is a Numeric tag's "missing" value (see `equal_only`
                        // below), so the first available value is the first one
                        // that isn't NaN.
                        PickPolicy::FirstAvailable => tag_columns
                            .iter()
                            .map(|col| col.get_numeric(i))
                            .find(|v| !v.is_nan())
                            .unwrap_or(f64::NAN),
                        PickPolicy::EqualOnly => {
                            let mut values = tag_columns.iter().map(|col| col.get_numeric(i));
                            let first = values.next().expect("in_labels is non-empty");
                            if values.all(|v| v.total_cmp(&first).is_eq()) {
                                first
                            } else {
                                f64::NAN
                            }
                        }
                        PickPolicy::Largest => tag_columns
                            .iter()
                            .map(|col| col.get_numeric(i))
                            .fold(f64::NEG_INFINITY, f64::max),
                        PickPolicy::Smallest => tag_columns
                            .iter()
                            .map(|col| col.get_numeric(i))
                            .fold(f64::INFINITY, f64::min),
                        PickPolicy::LeftMostRegion => {
                            unreachable!("rejected at config time for Numeric tags") // cov:excl-line
                        }
                    };
                    out.push(value);
                }
                block
                    .tags
                    .insert(self.out_label.clone(), TagColumn::Numeric(out));
            }
            TagColumn::Bool(_) => {
                unreachable!("Bool tags are rejected at config time for PickTag") // cov:excl-line
            }
        }

        Ok((block, true))
    }
}
