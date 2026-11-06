//! Output steps: write reads (and reports) to files as part of the pipeline.
//!
//! These steps replicate the behaviour of the legacy, configuration-driven
//! `[output]` section (see `fastqrab/src/output.rs`) but expressed as regular
//! pipeline transformations, so that output can be placed, ordered and
//! configured like any other step.
//!
//! This module holds functionality shared between the individual output steps;
//! the steps themselves live in the `output/` sub-directory.

mod output_bam;
mod output_fasta;
mod output_fastq;
mod output_report;

pub use output_bam::{OutputBAM, PartialOutputBAM};
pub(crate) use output_bam::{resolve_output_bam, verify_output_bam_merge};
pub use output_fasta::{OutputFASTA, PartialOutputFASTA};
pub use output_fastq::{OutputFASTQ, PartialOutputFASTQ};
pub use output_report::{OutputReport, PartialOutputReport};

use crate::transformations::prelude::*;
use fastqrab_io::CompressionFormat;
use fastqrab_io::io::output::chunked_writer::BamSinkOptions;

use crate::demultiplex::OptDemultiplex;
use crate::transformations::FinalizeReportResult;

/// Declaration id used for the (optional) interleaved / stdout writer.
pub(crate) const INTERLEAVED_ID: &str = "interleaved";

/// The magic infix understood by [`WriteTargetConfig::new`] to mean "stdout".
pub(crate) const STDOUT_INFIX: &str = "--stdout--";

/// Declaration id for a per-segment writer, keyed by the segment name.
pub(crate) fn segment_id(segment_name: &str) -> String {
    format!("seg:{segment_name}")
}

/// Build a [`SinkConfig`] from the common record-output options.
// TOdo: this should be a SinkConfig Constructor!
pub(crate) fn sink_config(
    compression: CompressionFormat,
    compression_level: Option<u8>,
    hash_uncompressed: bool,
    hash_compressed: bool,
) -> SinkConfig {
    SinkConfig {
        compression,
        compression_level,
        hash_uncompressed,
        hash_compressed,
        simulated_failure: None,
    }
}

/// Parameters shared by [`OutputFASTQ`]/[`OutputFASTA`]/[`OutputBAM`] when they
/// declare their output files. Built from the (already verified) step config.
pub(crate) struct RecordOutputDeclSpec<'a> {
    pub format: FileFormat,
    pub suffix: String,
    /// Segment names to write to individual files, in any order.
    pub segments: &'a [String],
    /// When set, the segments to interleave into a single file/stdout.
    pub interleave: Option<&'a [String]>,
    pub stdout: bool,
    pub sink_config: SinkConfig,
    pub chunksize: Option<usize>,
    pub bam_options: Option<BamSinkOptions>,
    /// Span pointing at the field most responsible for the filenames.
    pub span: std::ops::Range<usize>,
}

/// Build the [`OutputDeclaration`]s for a record-writing output step.
///
/// Emits one declaration per output segment, plus one for the interleaved /
/// stdout writer when requested. Mirrors the file layout produced by the legacy
/// `open_one_set_of_output_files`.
pub(crate) fn build_record_declarations(spec: &RecordOutputDeclSpec) -> Vec<OutputDeclaration> {
    let mut decls = Vec::new();

    if spec.stdout {
        // Stdout: a single interleaved writer; chunking and hashing are
        // meaningless and disabled by validation.
        let mut sink = spec.sink_config.clone();
        sink.hash_uncompressed = false;
        sink.hash_compressed = false;
        decls.push(OutputDeclaration {
            id: INTERLEAVED_ID.to_string(),
            target: WriteTargetConfig::new(
                vec![STDOUT_INFIX.to_string()],
                None,
                spec.suffix.clone(),
            ),
            sink_config: sink,
            format: spec.format,
            chunk_policy: ChunkPolicy::default(),
            bam_options: spec.bam_options.clone(),
            singleton: false,
            span: spec.span.clone(),
        });
        return decls;
    }

    if let Some(interleave) = spec.interleave {
        // When interleaving, chunk_size counts molecules — multiply by the
        // number of segments so files line up with non-interleaved output.
        let interleave_count = interleave.len();
        decls.push(OutputDeclaration {
            id: INTERLEAVED_ID.to_string(),
            target: WriteTargetConfig::new(
                vec![],
                Some("interleaved".to_string()),
                spec.suffix.clone(),
            ),
            sink_config: spec.sink_config.clone(),
            format: spec.format,
            chunk_policy: ChunkPolicy {
                records_per_chunk: spec.chunksize.map(|x| x * interleave_count),
            },
            bam_options: spec.bam_options.clone(),
            singleton: false,
            span: spec.span.clone(),
        });
    }

    for name in spec.segments {
        decls.push(OutputDeclaration {
            id: segment_id(name),
            target: WriteTargetConfig::new(vec![], Some(name.clone()), spec.suffix.clone()),
            sink_config: spec.sink_config.clone(),
            format: spec.format,
            chunk_policy: ChunkPolicy {
                records_per_chunk: spec.chunksize,
            },
            bam_options: spec.bam_options.clone(),
            singleton: false,
            span: spec.span.clone(),
        });
    }

    decls
}

/// Runtime state for a record-writing output step, built in `init` from the
/// writers handed over by the pipeline. Holds, per input segment, the per-tag
/// writers (or `None` when that segment is not written), plus the optional
/// interleaved writer and the resolved interleave order.
pub(crate) struct RecordOutputState {
    /// Indexed by input segment order. `None` means the segment is not written.
    segment_writers: Vec<Option<DemultiplexedData<Option<ChunkedRecordWriter>>>>,
    interleaved_writers: Option<DemultiplexedData<Option<ChunkedRecordWriter>>>,
    /// Indices into the segment order, in interleaving order.
    interleave_order: Vec<usize>,
}

impl std::fmt::Debug for RecordOutputState {
    //cov:excl-start
    #[mutants::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RecordOutputState")
            .field("segments", &self.segment_writers.len())
            .field("interleaved", &self.interleaved_writers.is_some())
            .field("interleave_order", &self.interleave_order)
            .finish()
    }
    //cov:excl-stop
}

impl RecordOutputState {
    /// Collect the writers handed to us in `init`.
    ///
    /// `segment_order` is the full input segment order; `interleave_names` are
    /// the segment names to interleave (in order). `stdout` selects the single
    /// stdout/interleaved writer path.
    pub(crate) fn from_step_output_files(
        files: &mut StepOutputFiles,
        segment_order: &[String],
        output_segments: &[String],
        interleave_names: Option<&[String]>,
        stdout: bool,
    ) -> Self {
        let mut segment_writers: Vec<Option<DemultiplexedData<Option<ChunkedRecordWriter>>>> =
            segment_order.iter().map(|_| None).collect();

        if !stdout {
            for (idx, name) in segment_order.iter().enumerate() {
                if output_segments.iter().any(|s| s == name) {
                    let per_tag = files.take(&segment_id(name));
                    segment_writers[idx] =
                        Some(per_tag.into_iter().map(|(t, w)| (t, Some(w))).collect());
                }
            }
        }

        let interleaved_writers = if stdout || interleave_names.is_some() {
            let per_tag = files.take(INTERLEAVED_ID);
            Some(per_tag.into_iter().map(|(t, w)| (t, Some(w))).collect())
        } else {
            None
        };

        let interleave_order = interleave_names
            .unwrap_or(&[])
            .iter()
            .map(|name| {
                segment_order
                    .iter()
                    .position(|s| s == name)
                    .expect("interleave segment must exist in segment order")
            })
            .collect();

        RecordOutputState {
            segment_writers,
            interleaved_writers,
            interleave_order,
        }
    }

    /// Write one block to all configured writers. Ported from the legacy
    /// `output_block_demultiplex` / `output_block_inner` / `output_block_interleaved`.
    pub(crate) fn write_block(
        &mut self,
        block: &FastQBlocksCombined,
        demultiplex_info: &OptDemultiplex,
    ) -> Result<()> {
        block.sanity_check()?;
        let demultiplexed = matches!(demultiplex_info, OptDemultiplex::Yes(_));

        for (seg_idx, seg_writers) in self.segment_writers.iter_mut().enumerate() {
            let Some(seg_writers) = seg_writers else {
                continue;
            };
            for (tag, writer_opt) in seg_writers.iter_mut() {
                let Some(writer) = writer_opt else { continue };
                let target_tag = if demultiplexed { Some(tag) } else { None };
                write_segment(writer, &block.segments[seg_idx], target_tag, block)?;
            }
        }

        if let Some(interleaved) = &mut self.interleaved_writers {
            for (tag, writer_opt) in interleaved.iter_mut() {
                let Some(writer) = writer_opt else { continue };
                let target_tag = if demultiplexed { Some(tag) } else { None };
                let blocks_to_interleave: Vec<_> = self
                    .interleave_order
                    .iter()
                    .map(|&i| &block.segments[i])
                    .collect();
                write_interleaved(writer, &blocks_to_interleave, target_tag, block)?;
            }
        }
        Ok(())
    }

    /// Finish (flush + close) every writer. Idempotent: takes each writer out,
    /// so calling twice is harmless.
    pub(crate) fn finish(&mut self) -> Result<()> {
        for seg in self.segment_writers.iter_mut().flatten() {
            for (_tag, writer) in seg.iter_mut() {
                let writer = writer.take().expect("Writer already taken?");
                let () = writer.finish()?;
            }
        }
        if let Some(interleaved) = self.interleaved_writers.as_mut() {
            for (_tag, writer) in interleaved.iter_mut() {
                let writer = writer.take().expect("Writer already taken?");
                let () = writer.finish()?;
            }
        }
        Ok(())
    }
}

fn write_segment(
    writer: &mut ChunkedRecordWriter,
    block: &fastqrab_io::blocks::FastQChunk,
    target_tag: Option<DemultiplexTag>,
    combined: &FastQBlocksCombined,
) -> Result<()> {
    let output_tags = if target_tag.is_some() {
        combined.output_tags.as_ref()
    } else {
        None
    };
    let format = writer.format();
    let iter = block.iter_filtered_to_tag(target_tag, output_tags);
    let mut buf = Vec::<u8>::with_capacity(256);
    match format {
        FileFormat::Fastq => {
            for (_idx, read) in iter {
                buf.clear();
                read.append_as_fastq(&mut buf);
                writer.write_text_record(&buf)?;
            }
        }
        FileFormat::Fasta => {
            for (_idx, read) in iter {
                buf.clear();
                read.append_as_fasta(&mut buf);
                writer.write_text_record(&buf)?;
            }
        }
        FileFormat::Bam => {
            for (read_index, read) in iter {
                writer.write_bam_record(&read, read_index, 0, 1, &combined.tags)?;
            }
        }
        FileFormat::Text | FileFormat::None => {
            unreachable!("Cannot output reads with format 'Text' or 'None'")
        }
    }
    Ok(())
}

fn write_interleaved(
    writer: &mut ChunkedRecordWriter,
    blocks_to_interleave: &[&fastqrab_io::blocks::FastQChunk],
    target_tag: Option<DemultiplexTag>,
    combined: &FastQBlocksCombined,
) -> Result<()> {
    let output_tags = if target_tag.is_some() {
        combined.output_tags.as_ref()
    } else {
        None
    };
    let format = writer.format();
    let mut iters: Vec<_> = blocks_to_interleave
        .iter()
        .map(|block| block.iter_filtered_to_tag(target_tag, output_tags))
        .collect();
    let segment_count = iters.len();
    assert!(segment_count > 0, "Interleave output but no blocks?");
    let mut buf = Vec::<u8>::with_capacity(256);
    'outer: loop {
        for (segment_index, iter) in iters.iter_mut().enumerate() {
            match format {
                FileFormat::Fastq => {
                    let Some((_read_idx, read)) = iter.next() else {
                        break 'outer;
                    };
                    buf.clear();
                    read.append_as_fastq(&mut buf);
                    writer.write_text_record(&buf)?;
                }
                FileFormat::Fasta => {
                    let Some((_read_idx, read)) = iter.next() else {
                        break 'outer;
                    };
                    buf.clear();
                    read.append_as_fasta(&mut buf);
                    writer.write_text_record(&buf)?;
                }
                FileFormat::Bam => {
                    let Some((read_index, read)) = iter.next() else {
                        break 'outer;
                    };
                    writer.write_bam_record(
                        &read,
                        read_index,
                        segment_index,
                        segment_count,
                        &combined.tags,
                    )?; // cov:excl-line
                }
                FileFormat::Text | FileFormat::None => {
                    unreachable!("Cannot output reads with format 'Text' or 'None'")
                }
            }
        }
    }
    Ok(())
}

/// A list of segment names as carried by a partial output step.
pub(crate) type SegmentList = TomlValue<Option<Vec<TomlValue<String>>>>;

/// Default + validate the `output` (segments), `interleave` and `stdout` fields
/// of a record-writing output step. Ported from the legacy
/// `PartialOutput::verify_stdout` / `verify_output_segments`.
///
/// `allow_stdout` is false for BAM (which cannot be written to stdout).
pub(crate) fn verify_record_targets(
    parent: &PartialConfig,
    output: &mut SegmentList,
    interleave: &mut SegmentList,
    stdout: &mut TomlValue<bool>,
    allow_stdout: bool,
) {
    // ---- defaulting (verify_stdout) ----
    if allow_stdout && matches!(stdout.as_ref(), Some(true)) {
        if let Some(Some(_)) = output.as_ref() {
            let spans = vec![
                (stdout.span(), "Conflict with 'output' option".to_string()),
                (output.span(), "Conflict with 'stdout' option".to_string()),
            ];
            stdout.state = TomlValueState::Custom { spans };
            stdout.help = Some("Remove either `output` or `stdout`".to_string());
            return;
        }
        if let Some(None) = interleave.as_ref()
            && let Some(seg_order) = segment_order(parent)
        {
            *interleave = TomlValue::new_ok(
                Some(
                    seg_order
                        .iter()
                        .map(|x| TomlValue::new_ok(x.clone(), 0..0))
                        .collect(),
                ),
                interleave.span(),
            );
        }
    } else if let Some(None) = output.as_ref() {
        //default unset output to all segments if no interleave is set.
        if let Some(Some(_)) = interleave.as_ref() {
            *output = TomlValue::new_ok(Some(Vec::new()), 0..0);
        } else if let Some(seg_order) = segment_order(parent) {
            *output = TomlValue::new_ok(
                Some(
                    seg_order
                        .iter()
                        .map(|x| TomlValue::new_ok(x.clone(), 0..0))
                        .collect(),
                ),
                0..0,
            );
        }
    }

    // ---- validation (verify_output_segments) ----
    let Some(seg_order) = segment_order(parent) else {
        return;
    };
    let valid: std::collections::HashSet<&String> = seg_order.iter().collect();
    let stdout_set = allow_stdout && matches!(stdout.as_ref(), Some(true));

    if let Some(Some(output_segments)) = output.as_mut()
        && validate_segment_names(output_segments, &valid, "output segments")
    {
        output.state = TomlValueState::Nested;
    }
    //we still need to check if output is set to empty & interleave is set to empty
    let interleave_is_empty = if let Some(Some(interleaved_segments)) = interleave.as_ref()
        && interleaved_segments.is_empty()
    {
        true
    } else {
        matches!(interleave.as_ref(), Some(None))
    };
    if let Some(Some(output_segments)) = output.as_ref()
        && output_segments.is_empty()
        && interleave_is_empty
    {
        let mut spans = vec![(
            output.span(),
            "Add segments here (or at `interleave =`) or remove 'output = ...`".to_string(),
        )];
        if let Some(Some(_)) = interleave.as_ref() {
            spans.push((
                interleave.span(),
                "Add segments here / remove `interleave = ..` (and fill `output =`) =".to_string(),
            ));
        }
        output.state = TomlValueState::Custom { spans };
        output.help = Some(
            "Either add segments to `output` or to `interleave`, \
                or remove both options to get the default behaviour."
                .to_string(),
        );
    } else if let Some(Some(interleave_order)) = interleave.as_mut() {
        let failed = validate_segment_names(interleave_order, &valid, "interleave order");
        if failed {
            if matches!(interleave.state, TomlValueState::Ok) {
                interleave.state = TomlValueState::Nested;
            }
        } else if interleave_order.len() < 2 && !stdout_set {
            interleave.state = TomlValueState::new_validation_failed(
                "Must contain at least two segments to interleave.",
            );
            interleave.help = Some(
                "Either add another segment to interleave, \
                    or remove interleave."
                    .to_string(),
            );
        } else if let Some(Some(output_segments)) = output.as_ref() {
            // no overlap between interleave and output
            for segment in output_segments {
                if let Some(segment_str) = segment.as_ref()
                    && let Some(found) = interleave_order
                        .iter_mut()
                        .find(|x| x.as_ref() == Some(segment_str))
                {
                    let spans = vec![
                        (found.span(), "Duplicate output & interleave".to_string()),
                        (segment.span(), "Duplicate output & interleave".to_string()),
                    ];
                    found.state = TomlValueState::Custom { spans };
                    found.help =
                        Some("Remove from either 'interleave' or from 'output'".to_string());
                }
            }
        }
    }
}

fn segment_order(parent: &PartialConfig) -> Option<&Vec<String>> {
    //todo: turn into a mothed on PartialConfig!
    parent
        .input
        .as_ref()
        .map(crate::config::PartialInput::get_segment_order)
}

/// Validate a list of segment names against the valid set, marking duplicates
/// and unknown names in place. Returns true if any entry failed.
fn validate_segment_names(
    list: &mut [TomlValue<String>],
    valid: &std::collections::HashSet<&String>,
    container: &str,
) -> bool {
    let mut seen = std::collections::HashSet::new();
    let all_seen: std::collections::HashSet<String> =
        list.iter().filter_map(|x| x.as_ref()).cloned().collect();
    let mut any_failed = false;
    for segment in list.iter_mut() {
        if let Some(segment_str) = segment.as_ref() {
            if valid.contains(segment_str) {
                if !seen.insert(segment_str.clone()) {
                    segment.help = Some(format!("Remove all but one '{segment_str}'"));
                    segment.state = TomlValueState::new_validation_failed(format!(
                        "Segment is duplicated in {container}"
                    ));
                    any_failed = true;
                }
            } else {
                let available: Vec<&String> = valid
                    .iter()
                    .filter_map(|x| {
                        if all_seen.contains(*x) {
                            None
                        } else {
                            Some(*x)
                        }
                    })
                    .collect();
                segment.help = Some(offer_alternatives(segment_str, &available));
                segment.state =
                    TomlValueState::new_validation_failed("Not found in input segments");
                any_failed = true;
            }
        } // cov:excl-line
    }
    any_failed
}

/// Convenience to read a verified segment list into a plain `Vec<String>`.
#[must_use]
pub(crate) fn collect_segment_list(list: &SegmentList) -> Vec<String> {
    match list.as_ref() {
        Some(Some(items)) => items.iter().filter_map(|x| x.as_ref().cloned()).collect(),
        _ => Vec::new(),
    }
}

/// Assemble the JSON report object from the collected per-step reports.
///
/// Build the combined run report JSON, a port of the legacy `output_json_report`.
///
/// Reports are keyed by their config label (`metadata.report_labels[report_no]`)
/// and emitted in declaration order, alongside the `__` run/input metadata, the
/// `run_info` block and the `report_order` array the HTML template consumes.
pub(crate) fn build_report_json(
    reports: &[FinalizeReportResult],
    metadata: &ReportMetadata,
) -> Result<String> {
    use json_value_merge::Merge;
    let mut output: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    let input_files: serde_json::Value = serde_json::from_str(&metadata.input_files)
        .expect("input_files was serialized from a valid value");
    // Store run info such as version in "__".
    output.insert(
        "__".to_string(),
        serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "cwd": std::env::current_dir().expect("Failed to retrieve current working directory"),
            "input_files": input_files,
            "repository": env!("CARGO_PKG_HOMEPAGE"),
        }),
    );

    let report_order: Vec<serde_json::Value> = metadata
        .report_labels
        .iter()
        .map(|label| serde_json::Value::String(label.clone()))
        .collect();

    // Collect reports by label, merging any that share a label (e.g. a report
    // split into multiple `report_no`s).
    let mut report_output: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
    for report in reports {
        let key = metadata
            .report_labels
            .get(report.report_no)
            .expect("report_no must have a corresponding label")
            .clone();
        match report_output.entry(key) {
            serde_json::map::Entry::Vacant(entry) => {
                entry.insert(report.contents.clone());
            }
            serde_json::map::Entry::Occupied(mut entry) => {
                entry.get_mut().merge(&report.contents);
            }
        }
    }

    // Emit reports in declaration order.
    for key in &metadata.report_labels {
        if let Some(contents) = report_output.remove(key) {
            output.insert(key.clone(), contents);
        }
    }

    let mut run_info = serde_json::Map::new();
    run_info.insert(
        "program_version".to_string(),
        serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string()),
    );
    run_info.insert(
        "input_toml".to_string(),
        serde_json::Value::String(metadata.raw_config.to_string()),
    );
    run_info.insert(
        "working_directory".to_string(),
        serde_json::Value::String(
            std::env::current_dir()
                .expect("Failed to retrieve current working directory")
                .to_string_lossy()
                .into_owned(),
        ),
    );
    output.insert("run_info".to_string(), serde_json::Value::Object(run_info));

    // Maintain the report order as defined in the config.
    output.insert(
        "report_order".to_string(),
        serde_json::Value::Array(report_order),
    );

    Ok(serde_json::to_string_pretty(&serde_json::Value::Object(
        output,
    ))?)
}

/// Render the HTML report by injecting the JSON into the shared template.
///
/// Reuses the same template/chart assets as the legacy renderer.
pub(crate) fn render_html_report(json_report_string: &str) -> Result<String> {
    if json_report_string
        .to_ascii_lowercase()
        .contains("</script>")
    {
        bail!("JSON output contained </script> which will break html parsing."); // cov:excl-line
    }
    let template = include_str!("../../../fastqrab/src/html/template.html");
    let chartjs = include_str!("../../../fastqrab/src/html/chart/chart.umd.min.js");
    let html = template
        .replace("%TITLE%", "fastqrab-report")
        .replace("\"%DATA%\"", json_report_string)
        .replace("/*%CHART%*/", chartjs);
    Ok(html)
}

fn verify_chunk_size(
    chunk_size: Option<&usize>,
    stdout: &TomlValue<bool>,
) -> Result<(), ValidationFailure> {
    if let Some(chunk_size) = chunk_size {
        if *chunk_size == 0 {
            return Err(ValidationFailure::new(
                "Must not be 0.",
                Some(
                    "'Chunksize' must be greater than zero when specified. Increase or remove setting.",
                ),
            ));
        } else if let Some(true) = stdout.as_ref() {
            return Err(ValidationFailure::new(
                "Invalid when stdout = true",
                Some("Either remove 'chunksize' or set 'stdout' to false"),
            ));
        }
    }
    Ok(())
}

/// Validates that the compression level is within the expected range for the given compression format
pub fn validate_compression_level_u8(
    compression: &TomlValue<CompressionFormat>,
    compression_level: &mut TomlValue<Option<u8>>,
) {
    if let Some(Some(level)) = compression_level.as_ref() {
        match compression.as_ref() {
            None | Some(CompressionFormat::Uncompressed) => {
                if *level != 0 {
                    compression_level.state = TomlValueState::ValidationFailed {
                        message: "Compression level specified for uncompressed output".to_string(),
                    };
                    compression_level.help = Some(
                        "Remove compression_level, or set compressed='gzip' or 'zstd'".to_string(),
                    );
                }
            }
            Some(CompressionFormat::Gzip) => {
                if *level > 9 {
                    compression_level.state = TomlValueState::ValidationFailed {
                        message: "Invalid Value".to_string(),
                    };
                    compression_level.help = Some("Valid range is 0-9 for gzip.".to_string());
                }
            }
            Some(CompressionFormat::Zstd) => {
                if *level == 0 || *level > 22 {
                    compression_level.state = TomlValueState::ValidationFailed {
                        message: ("Invalid Value".to_string()),
                    };
                    compression_level.help = Some("Valid range is 1-22 for zstd.".to_string());
                }
            }
        }
    } else {
        // No compression level specified - rapidgzip is still invalid for output
    }
}
