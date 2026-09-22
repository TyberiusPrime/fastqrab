use anyhow::{Result, bail};
use bstr::{BStr, BString};
use toml_pretty_deser::{TomlValue, TomlValueState};
// Common functionality shared by multiple tag transformations

// Individual transformation modules
pub mod assign_by_halves;
pub mod concat_tags;
pub mod fill_missing;
pub mod forget_all_tags;
pub mod forget_tag;
pub mod lifted_writeback;
pub mod pick_tag;
pub mod quantify_tag;
pub mod replace_tag_with_letter;
pub mod store_single_cell_matrix;
pub mod store_tag_back_in_sequence;
pub mod store_tag_in_comment;
pub mod store_tag_in_fastq;
pub mod store_tag_in_sequence;
pub mod store_tags_in_table;

// Re-exports
pub use assign_by_halves::{AssignByHalves, PartialAssignByHalves};
pub use concat_tags::{ConcatTags, PartialConcatTags};
use fastqrab_io::io::bam_read_count_from_index;
pub use fill_missing::{FillMissing, PartialFillMissing};
pub use forget_all_tags::{ForgetAllTags, PartialForgetAllTags};
pub use forget_tag::{ForgetTag, PartialForgetTag};
pub use pick_tag::{PartialPickTag, PickTag};
pub use quantify_tag::{PartialQuantifyTag, QuantifyTag};
pub use replace_tag_with_letter::{PartialReplaceTagWithLetter, ReplaceTagWithLetter};
pub use store_single_cell_matrix::{PartialStoreSingleCellMatrix, StoreSingleCellMatrix};
pub use store_tag_back_in_sequence::{PartialStoreTagBackInSequence, StoreTagBackInSequence};
pub use store_tag_in_comment::{PartialStoreTagInComment, StoreTagInComment};
pub use store_tag_in_fastq::{PartialStoreTagInFastQ, StoreTagInFastQ};
pub use store_tag_in_sequence::{PartialStoreTagInSequence, StoreTagInSequence};
pub use store_tags_in_table::{PartialStoreTagsInTable, StoreTagsInTable};

// pub(crate) fn apply_in_place_wrapped_with_tag(
//     segment_index: &SegmentIndexOrAll,
//     label: &TagLabel,
//     block: &mut FastQBlocksCombined,
//     f: impl Fn(&mut WrappedFastQReadMut, &TagValue),
// ) {
//     match segment_index {
//         SegmentIndexOrAll::Indexed(idx) => {
//             block.segments[*idx].apply_mut_with_tag(&block.tags, label, f);
//         }
//         SegmentIndexOrAll::All => {
//             for segment_block in &mut block.segments {
//                 segment_block.apply_mut_with_tag(&block.tags, label, &f);
//             }
//         }
//     }
// }

pub const DEFAULT_INITIAL_FILTER_CAPACITY: usize = 134_217_728; // 2^27. Scaleable cuckoo filters
// always need a power of 2, and we want to be north of a 'typical' danaset with 100 million reads

/// Calculate the optimal initial filter capacity based on:
/// - Configured capacity (local to the step. if provided)
/// - `InputInfo`'s `initial_filter_capacity` (if available)
/// - Demultiplexing factor (for demultiplexed filters)
///
/// # Arguments
/// * `configured_capacity` - Explicitly configured capacity (highest priority)
/// * `input_info` - Input configuration including optional `initial_filter_capacity`
/// * `demultiplex_count` - Number of demultiplex tags (for adjusting per-filter size)
/// * `debug_reproducibility` - Use small capacity for testing
///
/// # Returns
/// Capacity adjusted for demultiplexing
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Range is too low for precision loss, precision loss is irrelevant."
)]
#[mutants::skip] // changing the base_capacity will just make things slow, not fail
pub(crate) fn calculate_filter_capacity(
    configured_capacity: Option<usize>,
    input_info: &crate::transformations::InputInfo,
    demultiplex_count: usize,
) -> usize {
    // If explicitly configured, use that value
    if let Some(capacity) = configured_capacity {
        return if demultiplex_count > 1 {
            // For demultiplexed: give each filter 1.5/n of the total
            ((capacity as f64 * 1.5) / demultiplex_count as f64).ceil() as usize
        } else {
            capacity
        };
    }

    // Use InputInfo's configured capacity if available
    let base_capacity = input_info
        .initial_filter_capacity
        .unwrap_or(DEFAULT_INITIAL_FILTER_CAPACITY);

    // Adjust for demultiplexing
    if demultiplex_count > 1 {
        ((base_capacity as f64 * 1.5) / demultiplex_count as f64).ceil() as usize
    } else {
        base_capacity
    }
}

pub(crate) fn initial_filter_elements(
    filename: &str,
    include_mapped: bool,
    include_unmapped: bool,
) -> usize {
    let bam_read_count = bam_read_count_from_index(filename, include_mapped, include_unmapped);
    bam_read_count.unwrap_or(DEFAULT_INITIAL_FILTER_CAPACITY)
}

/// Format a numeric value for use in read comments, truncating floats to 4 decimal places
/// using scientific format
#[expect(clippy::cast_possible_truncation, reason = "Loss is ok")]
pub(crate) fn format_numeric_for_comment(value: f64) -> String {
    // Check if the value is effectively an integer
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else if (1e-3..=1e6).contains(&value.abs()) {
        format!("{value:.4}")
    } else {
        format!("{value:.4e}")
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

pub(crate) fn store_tag_in_comment(
    name: &[u8],
    label: &[u8],
    tag_value: &[u8],
    comment_separator: u8,
    comment_insert_char: u8,
) -> Result<Vec<u8>> {
    //todo: should we check region_separator as well?!
    if tag_value
        .iter()
        .any(|x| *x == comment_separator || *x == comment_insert_char)
    {
        bail!(
            "Tag value must not contain the comment separator ('{}'), nor the comment insert char ('{}').\n\
            Observed tag value for label '{}': '{}'.\n\
            Check `comment_separator` and `comment_insert_char` on StoreTagInComment steps",
            BString::new(vec![comment_separator]),
            BString::new(vec![comment_insert_char]),
            std::str::from_utf8(label).unwrap_or("utf-8 error"),
            BStr::new(tag_value)
        )
    }
    let insert_pos = name
        .iter()
        .position(|&x| x == comment_insert_char)
        .unwrap_or(name.len());

    let mut new_name = Vec::with_capacity(name.len() + 1 + label.len() + 1 + tag_value.len());
    new_name.extend_from_slice(&name[..insert_pos]);
    new_name.push(comment_separator);
    new_name.extend_from_slice(label);
    new_name.push(b'=');
    new_name.extend_from_slice(tag_value);
    new_name.extend_from_slice(&name[insert_pos..]);

    Ok(new_name)
}

pub fn validate_seed(
    tv_seed: &mut TomlValue<Option<u64>>,
    tv_false_positive_rate: &mut TomlValue<f64>,
) {
    if let Some(seed) = tv_seed.as_ref()
        && let Some(false_positive_rate) = tv_false_positive_rate.as_ref()
    {
        if *false_positive_rate < 0.0 {
            tv_false_positive_rate.state = TomlValueState::new_validation_failed("Too low");
            tv_false_positive_rate.help = Some(
                "False positive rate must be >= 0. Change `false_positive_rate` to a valid value."
                    .to_string(),
            );
        } else if *false_positive_rate >= 1.0 {
            tv_false_positive_rate.state = TomlValueState::new_validation_failed("Too high");
            tv_false_positive_rate.help = Some(
                "False positive rate must be < 1.0 Change `false_positive_rate` to a valid value."
                    .to_string(),
            );
        } else if *false_positive_rate > 0.0 && seed.is_none() {
            tv_seed.state =
                TomlValueState::new_validation_failed("Seed required for approximate filtering");
            tv_seed.help = Some(
                "Seed is required when false_positive_rate > 0.0 (approximate filtering). Add `seed=42` for example.".to_string(),
            );
        }
    } // cov:excl-line
}
