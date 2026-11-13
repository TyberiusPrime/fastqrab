// Module declarations

mod _change_case;
mod convert_quality;
mod cut_end;
mod cut_start;
mod lowercase;
mod merge_reads;
mod postfix;
mod prefix;
mod rename;
mod reverse_complement;
mod swap;
mod trim_at_tag;
mod trim_to_overlap;
mod truncate;
mod uppercase;

// Re-exports
pub use _change_case::{_ChangeCase, CaseType, Partial_ChangeCase};
pub use convert_quality::{ConvertQuality, PartialConvertQuality};
pub use cut_end::{CutEnd, PartialCutEnd};
pub use cut_start::{CutStart, PartialCutStart};
pub use lowercase::{Lowercase, PartialLowercase};
pub use merge_reads::{MergeReads, PartialMergeReads};
pub use postfix::{PartialPostfix, Postfix};
pub use prefix::{PartialPrefix, Prefix};
pub use rename::{PartialRename, Rename};
pub use reverse_complement::{PartialReverseComplement, ReverseComplement};
pub use swap::{PartialSwap, Swap};
pub use trim_at_tag::{PartialTrimAtTag, TrimAtTag};
pub use trim_to_overlap::{PartialTrimToOverlap, TrimToOverlap};
pub use truncate::{PartialTruncate, Truncate};
pub use uppercase::{PartialUppercase, Uppercase};

use fastqrab_config::ConditionalTagLabel;
use fastqrab_io::io::FastQBlocksCombined;

/// Helper function to extract a boolean Vec from tags
/// Converts any tag value to its truthy representation, with optional inversion
pub(crate) fn get_bool_vec_from_tag(
    block: &FastQBlocksCombined,
    cond_tag: &ConditionalTagLabel,
) -> Vec<bool> {
    block
        .tags
        .get(&cond_tag.tag)
        .expect("Tag not found - should have been caught in validation")
        .iter_truthy()
        .map(|val| if cond_tag.invert { !val } else { val })
        .collect()
}
