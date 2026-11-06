use anyhow::{Context, Result};

use fastqrab_io::io::{
    DetectedInputFormat, InputFiles, SegmentsCombined, open_input_file,
    total_file_size,
};
use indexmap::IndexMap;

/// # Panics
/// `Sgement_order` / segments mismatch
pub fn open_input_files(
    input_config: &crate::config::Input,
    input_format_per_segment: &IndexMap<String, DetectedInputFormat>,
) -> Result<InputFiles> {
    match &input_config.structured {
        crate::config::StructuredInput::Interleaved {
            files,
            segment_order,
        } => {
            // we need the optional format, since this is where we complain about
            // wrongly formatted files eventually
            let format = input_format_per_segment.get("--interleaved--").copied();
            let readers: Result<Vec<_>> = files
                .iter()
                .map(|x| {
                    open_input_file(x, format).with_context(|| {
                        format!("Problem in interleaved segment while opening '{x}'")
                    })
                })
                .collect();
            let readers = vec![readers?];
            //since there is only one segment, it's by default the largest
            let total_size_of_largest_segment =
                total_file_size(&readers[0]).map(|x| x / segment_order.len() as u64); //mutants::skip

            Ok(InputFiles {
                segment_files: SegmentsCombined { segments: readers },
                total_size_of_largest_segment,
                largest_segment_idx: 0, // does not matter.
            })
        }
        crate::config::StructuredInput::Segmented {
            segment_order,
            segment_files,
        } => {
            let mut segments = Vec::new();
            let mut sizes = Vec::new();
            for key in segment_order {
                let filenames = segment_files
                    .get(key)
                    .expect("Segment order / segments mismatch");

                // we need the optional format, since this is where we complain about
                // wrongly formatted files eventually
                let format = input_format_per_segment.get(key).copied();
                let readers: Result<Vec<_>> = filenames
                    .iter()
                    .map(|x| {
                        open_input_file(x, format).with_context(|| {
                            format!("Problem in segment {key} while opening '{x}'")
                        })
                    })
                    .collect();
                let readers = readers?;
                sizes.push(total_file_size(&readers));
                segments.push(readers);
            }
            let total_size_of_largest_segment = sizes.iter().filter_map(|x| *x).max();
            let largest_segment_idx = sizes
                .iter()
                .filter_map(|x| *x)
                .enumerate()
                .max_by_key(|&(_idx, size)| size)
                .map_or(0, |(idx, _size)| idx);
            Ok(InputFiles {
                segment_files: SegmentsCombined { segments },
                total_size_of_largest_segment,
                largest_segment_idx,
            })
        }
    }
}
