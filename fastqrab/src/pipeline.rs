use anyhow::{Context, Result, bail};
use crossbeam::channel::{bounded, unbounded};
use fastqrab_config::dna::bits_needed_to_represent;
use fastqrab_steps::no_barcode_infix;
use indexmap::IndexMap;
use std::{
    cell::OnceCell,
    collections::BTreeMap,
    num::NonZero,
    panic,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    bam_merge::DemultiplexStepInfo,
    config::{CheckedConfig, StructuredInput},
    demultiplex::{DemultiplexBarcodes, DemultiplexInfo, DemultiplexedData, OptDemultiplex},
    transformations::{self, Step, Transformation},
};
use fastqrab_io::{
    blocks::FastQChunk,
    ensure_output_destination_available,
    io::{
        self,
        input::InputOptions,
        output::chunked_writer::{ChunkPaths, ChunkedRecordWriter, WriteTarget, WriteTargetConfig},
        parsers::{ChainedParser, ParserThreadCounts, ThreadCount},
    },
};
use fastqrab_steps::{
    demultiplex::StepOutputFiles,
    input_files::{InputDeclaration, StepInputFiles},
    join_nonempty,
};

/// Open every file a step declared via `TagUser::declare_input_files`, keyed by
/// the declaration's `id`, and hand them to `Step::init` as [`StepInputFiles`].
/// Opening up front (here, not lazily inside the step) keeps all out-of-tree
/// reads in one place — the seam a future filesystem sandbox hooks into.
fn build_step_input_files(declarations: Option<Vec<InputDeclaration>>) -> Result<StepInputFiles> {
    let mut files = StepInputFiles::empty();
    for decl in declarations.into_iter().flatten() {
        // `ex::fs` names the path; we add the declaration id (the config field)
        // so the user knows which setting points at the file; the call site adds
        // the step.
        let file = ex::fs::File::open(&decl.path)
            .with_context(|| format!("Failed to open input file declared by '{}'", decl.id))?;
        files.0.insert(decl.id, file);
    }
    Ok(files)
}

/// The directory- and chunk-free name of one output file a declaration would
/// produce for a given demultiplex tag. The runtime turns this into a
/// [`WriteTarget`] (adding the output directory) while the `output-files`
/// command renders it as a plain `basename.suffix` string. Shared so both paths
/// agree on the prefix / infix / demux-name / second-infix layout.
pub(crate) enum ResolvedOutputName {
    File { basename: String, suffix: String },
    Stdout,
}

/// Enumerate the `(tag, name)` pairs one output declaration produces under the
/// given demultiplex info. This is the single source of truth for output
/// filenames; [`build_step_output_files`] (runtime) and
/// `list_config_output_files` (the `output-files` command) both go through it.
pub(crate) fn enumerate_declaration_outputs(
    singleton: bool,
    target: &WriteTargetConfig,
    demultiplex_info: &OptDemultiplex,
    output_prefix: &str,
    output_ix_separator: &str,
) -> Vec<(crate::demultiplex::Tag, ResolvedOutputName)> {
    let mut out = Vec::new();
    // Singleton steps (Inspect, Progress) always get exactly one output (tag 0).
    let tags_to_create: Vec<_> = if singleton {
        vec![0u64]
    } else {
        demultiplex_info.iter_tags()
    };

    for tag in tags_to_create {
        // Singletons never get a demux suffix. For demultiplexed non-singleton
        // outputs, skip tags with no name (the no-match bucket).
        let demux_name: Option<&str> = if singleton {
            None
        } else {
            match demultiplex_info {
                OptDemultiplex::No => None,
                OptDemultiplex::Yes(info) => {
                    let name_opt = info.tag_to_name.get(&tag).and_then(|n| n.as_deref());
                    if name_opt.is_none() {
                        continue; // skip no-match demultiplex tag
                    }
                    name_opt
                }
            }
        };

        let resolved = match target {
            WriteTargetConfig::Stdout => ResolvedOutputName::Stdout,
            WriteTargetConfig::File(ft) => {
                let basename = join_nonempty(
                    std::iter::once(output_prefix)
                        .chain(ft.infix_parts().iter().map(String::as_str))
                        .chain(demux_name)
                        .chain(ft.second_infix().map(String::as_str)),
                    output_ix_separator,
                );
                ResolvedOutputName::File {
                    basename,
                    suffix: ft.suffix().to_string(),
                }
            }
        };
        out.push((tag, resolved));
    }
    out
}

fn build_step_output_files(
    declarations: Option<Vec<fastqrab_io::io::output::chunked_writer::OutputDeclaration>>,
    demultiplex_info: &OptDemultiplex,
    output_directory: &Path,
    output_prefix: &str,
    output_ix_separator: &str,
    allow_overwrite: bool,
    simulated_failure: Option<&fastqrab_io::io::output::simulated_failure::SimulatedWriteFailure>,
) -> Result<StepOutputFiles> {
    let mut result = StepOutputFiles::empty();
    if let Some(declarations) = declarations {
        for decl in declarations {
            let mut per_tag: DemultiplexedData<ChunkedRecordWriter> = DemultiplexedData::new();

            for (tag, resolved) in enumerate_declaration_outputs(
                decl.singleton,
                &decl.target,
                demultiplex_info,
                output_prefix,
                output_ix_separator,
            ) {
                let target = match resolved {
                    ResolvedOutputName::Stdout => WriteTarget::Stdout,
                    ResolvedOutputName::File { basename, suffix } => {
                        WriteTarget::Files(ChunkPaths {
                            directory: output_directory.to_path_buf(),
                            basename,
                            suffix,
                        })
                    }
                };
                // The output-failure simulation comes from [options.debug_failures];
                // the declaration itself can't see the global options, so inject it
                // here (mirrors the legacy build_sink_config behaviour).
                let mut sink_config = decl.sink_config.clone();
                if let Some(failure) = simulated_failure {
                    sink_config.simulated_failure = Some(failure.clone());
                }
                let writer = ChunkedRecordWriter::new(
                    decl.format,
                    target,
                    sink_config,
                    decl.chunk_policy,
                    decl.bam_options.clone(),
                    NonZero::<usize>::MIN,
                    allow_overwrite,
                )?;
                per_tag.insert(tag, writer);
            }
            result.insert(decl.id, per_tag);
        }
    }
    Ok(result)
}

/// Check, up front, that every output file the config would produce is writable
/// (does not already exist unless `--allow-overwrite`), *before* any step is
/// initialized.
///
/// The per-stage writer construction in [`build_step_output_files`] also runs
/// this check (via [`ChunkedRecordWriter::new`]), but only when the init loop
/// reaches that stage — so a clobber error on a late output step would surface
/// only after earlier steps' `init` has done its (potentially slow) work, e.g.
/// `TagOtherFile` reading its entire reference file. Running the check here
/// means the error is reported immediately.
///
/// Output filenames are fully determined by the config (demultiplex names come
/// from [`Transformation::declared_barcodes`], not from `init`), so this walks
/// the stages exactly like `list_config_output_files` / the runtime init loop —
/// accumulating the demultiplex info via [`DemultiplexChain`] — and mirrors the
/// path set and `allow_fifo` rule that [`ChunkedRecordWriter::new`] applies.
fn preflight_check_output_files(
    stages: &[crate::config::Stage],
    output_directory: &Path,
    output_prefix: &str,
    output_ix_separator: &str,
    allow_overwrite: bool,
) -> Result<()> {
    let mut chain = DemultiplexChain::new();
    for (index, stage) in stages.iter().enumerate() {
        // A step sees the demultiplex info from every Demultiplex step before it.
        let demultiplex_info = chain.current_info();
        if let Some(declarations) = &stage.output_declarations {
            for decl in declarations {
                for (_tag, resolved) in enumerate_declaration_outputs(
                    decl.singleton,
                    &decl.target,
                    demultiplex_info,
                    output_prefix,
                    output_ix_separator,
                ) {
                    let ResolvedOutputName::File { basename, suffix } = resolved else {
                        continue; // stdout has nothing to clobber
                    };
                    let paths = ChunkPaths {
                        directory: output_directory.to_path_buf(),
                        basename,
                        suffix,
                    };
                    // Mirror the path set ChunkedRecordWriter::new checks: for
                    // chunked output both the bare name and the `.0` chunk, and
                    // fifos are only tolerated for non-chunked output.
                    let chunked = decl.chunk_policy.records_per_chunk.is_some();
                    let check_paths = if chunked {
                        vec![paths.nth(0, 0), paths.nth(0, 1)]
                    } else {
                        vec![paths.nth(0, 0)]
                    };
                    let allow_fifo = !chunked;
                    for path in &check_paths {
                        ensure_output_destination_available(path, allow_overwrite, allow_fifo)
                            .with_context(|| {
                                format!(
                                    "Output file for step index {index} is not available: {:?}",
                                    stage.transformation
                                )
                            })?;
                    }
                }
            }
        }
        // Advance the chain *after* this step's own outputs are enumerated, so a
        // Demultiplex step's split only affects steps downstream of it.
        if let Transformation::Demultiplex(d) = &stage.transformation {
            chain.push(
                index,
                d.declared_barcodes(),
                d.in_label.to_string(),
                output_ix_separator,
            )?; // cov:excl-line
        }
    }
    Ok(())
}

/// Accumulates the demultiplex tag→name mapping as the pipeline walks its
/// stages. Each `Demultiplex` step contributes a new set of barcodes; the chain
/// combinatorially combines them with everything seen so far, allocating a fresh
/// range of tag bits per step. Shared between the runtime
/// ([`RunStage0::configure_demultiplex_and_init_stages`]) and the `output-files`
/// command, so both derive identical demux-aware filenames.
pub(crate) struct DemultiplexChain {
    infos: Vec<(usize, OptDemultiplex)>,
    step_infos: Vec<DemultiplexStepInfo>,
    current_bit_start: u16,
}

impl DemultiplexChain {
    pub(crate) fn new() -> Self {
        Self {
            infos: Vec::new(),
            step_infos: Vec::new(),
            current_bit_start: 0,
        }
    }

    /// The demultiplex info a step sees: the combination of every `Demultiplex`
    /// step pushed *before* it. `OptDemultiplex::No` until the first push.
    pub(crate) fn current_info(&self) -> &OptDemultiplex {
        self.infos
            .iter()
            .last()
            .map_or(&OptDemultiplex::No, |x| &x.1)
    }

    /// Fold one more `Demultiplex` step (at stage `index`) into the chain.
    pub(crate) fn push(
        &mut self,
        index: usize,
        new_demultiplex_barcodes: DemultiplexBarcodes,
        in_label: String,
        output_ix_separator: &str,
    ) -> Result<()> {
        let barcode_count = new_demultiplex_barcodes.barcode_to_name.len()
            + usize::from(new_demultiplex_barcodes.include_no_barcode);
        let bits_needed = bits_needed_to_represent(barcode_count);
        let mut tag_to_name = BTreeMap::new();
        if new_demultiplex_barcodes.include_no_barcode {
            tag_to_name.insert(0, Some(no_barcode_infix().to_string()));
        } else {
            tag_to_name.insert(0, None);
        }

        let unique_names = new_demultiplex_barcodes
            .barcode_to_name
            .values()
            .collect::<std::collections::BTreeSet<_>>();
        let unique_names = unique_names.into_iter().cloned().collect::<Vec<_>>();
        let mut local_name_to_tag = BTreeMap::new();
        let mut tag_value: crate::demultiplex::Tag = 1;
        for name in unique_names {
            let bitpattern = tag_value << self.current_bit_start;
            tag_to_name.insert(bitpattern, Some(name.clone()));
            local_name_to_tag.insert(name.clone(), bitpattern);
            tag_value += 1;
        }
        let local_barcode_to_tag = new_demultiplex_barcodes
            .barcode_to_name
            .into_iter()
            .map(|(k, v)| {
                let tag = local_name_to_tag
                    .get(&v)
                    .expect("tag must exist in local_name_to_tag map");
                (k, *tag)
            })
            .collect();

        self.step_infos.push(DemultiplexStepInfo { in_label });

        if self.infos.is_empty() {
            self.infos.push((
                index,
                OptDemultiplex::Yes(DemultiplexInfo::new(
                    tag_to_name,
                    local_barcode_to_tag,
                    local_name_to_tag,
                )),
            ));
        } else {
            let mut next = BTreeMap::new();
            {
                let last_demultiplex_info = self
                    .infos
                    .iter()
                    .last()
                    .map_or(&OptDemultiplex::No, |x| &x.1);

                for (old_tag, old_name) in &last_demultiplex_info
                    .expect("last_demultiplex_info must be Some when iterating over tag_to_name")
                    .tag_to_name
                {
                    for (new_tag, new_name) in &tag_to_name {
                        debug_assert!(old_tag & new_tag == 0);
                        let combined_tag = old_tag | new_tag; //mutants::skip ^ is just as valid)
                        let out_name: Option<String> = {
                            if let Some(old_name) = old_name {
                                new_name.as_ref().map(|new_name| {
                                    format!("{}{}{}", old_name, &output_ix_separator, new_name)
                                })
                            } else {
                                None
                            }
                        };
                        next.insert(combined_tag, out_name);
                    }
                }
            }
            self.infos.push((
                index,
                OptDemultiplex::Yes(DemultiplexInfo::new(
                    next,
                    local_barcode_to_tag,
                    local_name_to_tag,
                )),
            ));
        }
        self.current_bit_start += bits_needed;
        if self.current_bit_start > 64 {
            //mutants::skip
            // not covered in tests, will alert in mutation testing.
            // There's an O(2^n) runtime above, and anything beyond 16 will slow.
            // our tests down significantly (tests happen in debug mode)
            // We could limit this to like 18 bits, maybe?
            // cov:excl-start
            bail!("Too many demultiplexed outputs defined - exceeds 64 bits");
            // cov:excl-stop
        }
        Ok(())
    }

    pub(crate) fn into_parts(self) -> (Vec<(usize, OptDemultiplex)>, Vec<DemultiplexStepInfo>) {
        (self.infos, self.step_infos)
    }
}

fn parse_and_send(
    readers: Vec<io::InputFile>,
    raw_tx: &crossbeam::channel::Sender<(FastQChunk, Option<usize>)>,
    buffer_size: usize,
    block_size: NonZero<usize>,
    thread_counts: ParserThreadCounts,
    input_options: InputOptions,
) -> Result<()> {
    let mut parser = ChainedParser::new(
        readers,
        block_size,
        buffer_size,
        thread_counts,
        input_options,
    );
    loop {
        let res = parser.parse()?;
        //at the moment, only bam seems to be sending an empty final block
        //while the others send a filled final block, and no empty otherwise.
        //Let's not filter them.
        //if !res.fastq_block.is_empty() || res.was_final {
        //assert!(!(res.fastq_block.is_empty() && !res.was_final));
        //
        {
            if raw_tx
                .send((res.fastq_block, res.expected_read_count))
                .is_err()
            {
                break;
            }
        }
        if res.was_final {
            break;
        }
    }
    Ok(())
}

fn parse_interleaved_and_send(
    readers: Vec<io::InputFile>,
    combiner_output_tx: &crossbeam::channel::Sender<(io::FastQBlocksCombined, Option<usize>)>,
    segment_count: NonZero<usize>,
    buffer_size: usize,
    thread_counts: ParserThreadCounts,
    block_size: NonZero<usize>,
    input_options: InputOptions,
) -> Result<()> {
    let mut parser = ChainedParser::new(
        readers,
        block_size,
        buffer_size,
        thread_counts,
        input_options,
    );
    let mut block_no = 1; //block numbers are 1 based. Why though?
    let mut expected_read_count = None;
    let mut first_read_in_block_no = 0;
    loop {
        let res = parser.parse()?;
        if let None = expected_read_count
            && let Some(value) = res.expected_read_count
        {
            expected_read_count = Some(value);
        }
        if !res.fastq_block.is_empty() {
            let out_blocks = res.fastq_block.split_interleaved(segment_count)?;
            let block_len = out_blocks[0].len();
            let out = (
                io::FastQBlocksCombined::new(
                    out_blocks,
                    None,
                    IndexMap::default(),
                    false,
                    block_no,
                    first_read_in_block_no,
                ),
                expected_read_count,
            );
            first_read_in_block_no += block_len; //increase *after* the block
            block_no += 1; // the receiver verifies this! pipeline_workpool::process_incoming_block
            if combiner_output_tx.send(out).is_err() {
                break; //cov:ignore might or might not happen, timing dependent.
            }
        } // cov:excl-line

        if res.was_final {
            // Send final empty block
            let final_block = io::FastQBlocksCombined::new(
                vec![FastQChunk::new_empty(); segment_count.into()],
                None,
                IndexMap::default(),
                true,
                block_no,
                first_read_in_block_no,
            );
            let _ = combiner_output_tx.send((final_block, expected_read_count));
            break;
        }
    }
    Ok(())
}

//#[allow(clippy::needless_pass_by_value)]
fn run_combiner_thread(
    raw_rx_readers: &[crossbeam::channel::Receiver<(FastQChunk, Option<usize>)>],
    combiner_output_tx: &crossbeam::channel::Sender<(io::FastQBlocksCombined, Option<usize>)>,
    largest_segment_idx: usize,
    error_collector: &Arc<Mutex<Vec<String>>>,
    target_block_size: usize,
) {
    // Receive the per-segment blocks and align them into lockstep
    // (identical-read-count) combined blocks for the downstream pairing.
    //
    // The per-segment parsers no longer pre-copy reads into fixed `block_size`
    // blocks; each emits its native, arbitrarily-sized blocks (e.g. one decode
    // chunk of reads). R1 and R2 are independent streams whose blocks don't line
    // up, so we align *by slicing*: each round we slice every segment's current
    // block down to the common `min(remaining)` reads — a pure O(1)/metadata
    // operation sharing the decode-chunk `Arc`, no bytes copied — and carry the
    // remainders into the next round. A segment is *done* when its channel is
    // closed and its current block is fully consumed; if some segments run dry
    // while others still have reads, the read counts disagree.
    let segment_count = raw_rx_readers.len();
    let mut block_no = 1; // for the sorting later on.
    let expected_read_count = OnceCell::new();
    let mut first_read_in_block_no = 0;
    // Per-segment current block + reads already consumed from it.
    let mut current: Vec<Option<(FastQChunk, usize)>> = vec![None; segment_count];
    let mut done = vec![false; segment_count];

    loop {
        // Ensure every not-yet-exhausted segment has a current block with reads
        // remaining, pulling the next block when the current one is drained. The
        // parsers may emit empty non-final blocks, so skip those.
        for seg in 0..segment_count {
            if done[seg] {
                continue; //cov:ignore - timing dependent?
            }
            while current[seg]
                .as_ref()
                .is_none_or(|(block, cursor)| *cursor >= block.len())
            {
                match raw_rx_readers[seg].recv() {
                    Ok((block, block_expected_read_count)) => {
                        if expected_read_count.get().is_none() && seg == largest_segment_idx {
                            expected_read_count
                                .set(block_expected_read_count)
                                .expect("Read count already set!?");
                        }
                        if !block.is_empty() {
                            current[seg] = Some((block, 0));
                            break;
                        } // cov:excl-line - not sure this is real, but keep it as defense
                        // empty block: keep pulling
                    }
                    Err(_) => {
                        done[seg] = true;
                        current[seg] = None;
                        break;
                    }
                }
            }
        }

        // All segments exhausted at a block boundary ⇒ clean EOF.
        if done.iter().all(|&d| d) {
            let empty_segments: Vec<FastQChunk> = (0..segment_count)
                .map(|_| FastQChunk::new_empty())
                .collect();
            let final_block = io::FastQBlocksCombined::new(
                empty_segments,
                None,
                Default::default(),
                true,
                block_no,
                first_read_in_block_no,
            );
            let _ = combiner_output_tx.send((
                final_block,
                // Won't have been set if we suffered an early parse error.
                *expected_read_count.get().unwrap_or(&None),
            ));
            return;
        }

        // Some segments are done while others still have reads ⇒ the inputs hold
        // unequal read counts. Direction is keyed off the first segment, matching
        // the historical messages: segment 0 drained first means it had fewer.
        if done.iter().any(|&d| d) {
            if done[0] {
                error_collector.lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .push("Unequal number of reads in the segment inputs (first < later). Check your fastqs for identical read counts".to_string());
                // Send final empty block (matches the historical first-done path).
                let empty_segments: Vec<FastQChunk> = (0..segment_count)
                    .map(|_| FastQChunk::new_empty())
                    .collect();
                let final_block = io::FastQBlocksCombined::new(
                    empty_segments,
                    None,
                    Default::default(),
                    true,
                    block_no,
                    first_read_in_block_no,
                );
                let _ = combiner_output_tx
                    .send((final_block, *expected_read_count.get().unwrap_or(&None)));
            } else {
                error_collector.lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                    .push("Unequal number of reads in the segment inputs (first > later). Check your fastqs for identical read counts".to_string());
            }
            return;
        }

        // Every segment has reads remaining: slice each to the common minimum,
        // advance its cursor, and emit one matched combined block.
        let n = (0..segment_count)
            .map(|seg| {
                let (block, cursor) = current[seg].as_ref().expect("filled above");
                block.len() - cursor
            })
            .min()
            .expect("segment_count >= 1")
            .min(target_block_size);
        let segments: Vec<FastQChunk> = (0..segment_count)
            .map(|seg| {
                let (block, cursor) = current[seg].as_mut().expect("filled above");
                let sliced = block.slice(*cursor..*cursor + n);
                *cursor += n;
                sliced
            })
            .collect();
        let out = (
            io::FastQBlocksCombined::new(
                segments,
                None,
                Default::default(),
                false,
                block_no,
                first_read_in_block_no,
            ),
            *expected_read_count.get().expect("Should have been set"),
        );
        first_read_in_block_no += out.0.len();
        block_no += 1;
        if combiner_output_tx.send(out).is_err() {
            //downstream hung up
            break;
        }
    }
}

//#[expect(clippy::needless_pass_by_value)]
#[mutants::skip]
fn run_benchmark_combiner_thread(
    first_block: &io::FastQBlocksCombined,
    combiner_output_tx: &crossbeam::channel::Sender<(io::FastQBlocksCombined, Option<usize>)>,
    molecule_count: usize,
) {
    let mut block_no = 1;
    let mut molecules_sent = 0;
    let block_molecule_count = first_block
        .segments
        .iter()
        .map(FastQChunk::len)
        .min()
        .unwrap_or(0);

    if block_molecule_count == 0 {
        // cov:excl-start
        unreachable!("Empty first block in benchmark. Should have been validated before?");
        // cov:excl-stop
    }

    while molecules_sent < molecule_count {
        let mut cloned_block = first_block.with_new_block_no(block_no);
        cloned_block.is_final = false;

        // we don't worry about sending more reads than requested

        let current_block_size = cloned_block
            .segments
            .iter()
            .map(FastQChunk::len)
            .min()
            .unwrap_or(0);
        molecules_sent += current_block_size;

        match combiner_output_tx.send((cloned_block, Some(molecule_count))) {
            Ok(()) => {}
            Err(_) => {
                // downstream hung up
                //cov:excl-start
                break;
                //cov:excl-stop
            }
        }

        block_no += 1;

        if molecules_sent >= molecule_count {
            break;
        }
    }

    // Send final empty block
    let final_block = io::FastQBlocksCombined::new(
        first_block
            .segments
            .iter()
            .map(|_| FastQChunk::new_empty())
            .collect(),
        None,
        Default::default(),
        true,
        block_no,
        molecules_sent,
    );
    let _ = combiner_output_tx.send((final_block, Some(molecule_count)));
}

//#[allow(clippy::needless_pass_by_value)]
#[mutants::skip]
fn run_benchmark_interleaved_thread(
    first_block: FastQChunk,
    combiner_output_tx: &crossbeam::channel::Sender<(io::FastQBlocksCombined, Option<usize>)>,
    segment_count: NonZero<usize>,
    molecule_count: usize,
) {
    let mut block_no = 1;
    let mut molecules_sent = 0;
    let block_molecule_count = first_block.len();

    assert!(
        block_molecule_count > 0,
        "Empty first block in benchmark. Should have been validated before?"
    );

    let out_blocks = first_block
        .split_interleaved(segment_count)
        .expect("Interleave splitting failed");

    while molecules_sent < molecule_count {
        //we don't worry about having a few reads too many here.
        let out_blocks = out_blocks.clone();

        let out = (
            io::FastQBlocksCombined::new(
                out_blocks,
                None,
                Default::default(),
                false,
                block_no,
                molecules_sent,
            ),
            Some(molecule_count),
        );

        molecules_sent += block_molecule_count;
        match combiner_output_tx.send(out) {
            Ok(()) => {}
            Err(_) => {
                // downstream hung up
                //cov:excl-start
                break;
                //cov:excl-stop
            }
        }

        block_no += 1;

        if molecules_sent >= molecule_count {
            break;
        }
    }

    // Send final empty block
    let final_block = io::FastQBlocksCombined::new(
        vec![FastQChunk::new_empty()],
        None,
        Default::default(),
        true,
        block_no,
        molecules_sent,
    );
    let _ = combiner_output_tx.send((final_block, Some(molecule_count)));
}

pub struct RunStage0 {}

impl RunStage0 {
    #[expect(clippy::too_many_lines, reason = "needed")]
    pub fn configure_demultiplex_and_init_stages(
        parsed: &mut CheckedConfig,
        output_directory: &Path,
        allow_overwrite: bool,
    ) -> Result<RunStage1> {
        let output_prefix = parsed
            .output
            .as_ref()
            .map_or("mbf_fastq_preprocessor_output", |x| &x.prefix)
            .to_string();
        let output_ix_separator = parsed.get_ix_separator();
        let simulated_failure = parsed.options.debug_failures.simulated_output_failure()?;

        // Fail fast on pre-existing output files, before any step's `init` runs
        // (and in particular before TagOtherFile reads its whole reference file).
        // The per-stage writer construction below re-checks and is authoritative;
        // this only moves the *error* earlier.
        preflight_check_output_files(
            &parsed.stages,
            output_directory,
            &output_prefix,
            &output_ix_separator,
            allow_overwrite,
        )?;

        let report_metadata = std::sync::Arc::new(transformations::ReportMetadata {
            report_labels: parsed.report_labels.clone(),
            input_files: serde_json::to_string(&parsed.input)
                .expect("input config should serialize to JSON")
                .into(),
            raw_config: parsed.raw_config.clone(),
        });
        let input_info = transformations::InputInfo {
            segment_order: parsed.input.get_segment_order().clone(),
            barcodes_data: parsed.barcodes.clone(),
            comment_insert_char: parsed.input.options.read_comment_character,
            initial_filter_capacity: None, // Filled after the first block.
            threading_configuration: parsed.threading_configuration.clone(),
            report_metadata,
            blocks_in_flight: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        };
        // we need to initialize the progress_output first
        // so we can store it on each stage before the stages' init
        let progress_output = {
            let mut res = None;
            for step in &mut parsed.stages {
                if let Transformation::Progress(inner) = &mut step.transformation {
                    let decls = std::mem::take(&mut step.output_declarations);
                    let output_files = build_step_output_files(
                        decls,
                        &OptDemultiplex::No,
                        output_directory,
                        &output_prefix,
                        &output_ix_separator,
                        allow_overwrite,
                        simulated_failure.as_ref(),
                    )?; // cov:excl-line
                    let mut input_files =
                        build_step_input_files(std::mem::take(&mut step.input_declarations))?;
                    inner.init(
                        &input_info,
                        output_files,
                        &OptDemultiplex::No,
                        &mut input_files,
                    )?; // cov:excl-line
                    input_files.assert_consumed();
                    res = Some(inner.clone());
                }
            }
            res
        };

        // we combinatorially combine demultiplex stages
        // and at each stage, it get's to see the tag->output names up to the latest defined
        // demultiplexing step. [`DemultiplexChain`] owns that accumulation (shared
        // with the `output-files` command).
        let mut chain = DemultiplexChain::new();

        for (index, stage) in (parsed.stages).iter_mut().enumerate() {
            if !matches!(stage.transformation, Transformation::Progress(_)) {
                //progress was initialized before hand
                //
                if let Some(progress_output) = &progress_output {
                    stage.transformation.store_progress_output(progress_output);
                }
                let new_demultiplex_barcodes: Option<DemultiplexBarcodes> = {
                    let last_demultiplex_info = chain.current_info();
                    let decls = std::mem::take(&mut stage.output_declarations);
                    let output_files = build_step_output_files(
                        decls,
                        last_demultiplex_info,
                        output_directory,
                        &output_prefix,
                        &output_ix_separator,
                        allow_overwrite,
                        simulated_failure.as_ref(),
                    )
                    .with_context(|| {
                        format!(
                            "Error building output files. Index {index}: {:?}",
                            stage.transformation
                        )
                    })?;
                    let mut input_files =
                        build_step_input_files(std::mem::take(&mut stage.input_declarations))
                            .with_context(|| {
                                format!(
                                    "Error opening declared input files. Index {index}: {:?}",
                                    stage.transformation
                                )
                            })?;
                    let new_demultiplex_barcodes = stage
                        .transformation
                        .init(
                            &input_info,
                            output_files,
                            last_demultiplex_info,
                            &mut input_files,
                        )
                        .with_context(|| {
                            format!(
                                "Error in transform initalization. Index {index}: {:?}",
                                stage.transformation
                            )
                        })?;
                    input_files.assert_consumed();
                    new_demultiplex_barcodes
                };
                if let Some(new_demultiplex_barcodes) = new_demultiplex_barcodes {
                    // Capture the in_label for this step if it is a Demultiplex transformation.
                    let step_in_label = if let Transformation::Demultiplex(ref d) =
                        stage.transformation
                    {
                        d.in_label.to_string()
                    } else {
                        // cov:excl-start
                        unreachable!(
                            "So far, only Demultiplex stages returned barcodes. If this has changed, you need to check and adjust here"
                        );
                        // cov:excl-stop
                    };
                    chain.push(
                        index,
                        new_demultiplex_barcodes,
                        step_in_label,
                        &output_ix_separator,
                    )?; // cov:excl-line
                    // would only fail if you define too many. We're not testing that, I'm not
                    // deleting 2^64  files.
                }
            }
        }

        let (demultiplex_infos, demultiplex_step_infos) = chain.into_parts();

        Ok(RunStage1 {
            input_info,
            output_directory: output_directory.to_owned(),
            demultiplex_infos,
            demultiplex_step_infos,
            allow_overwrite,
        })
    }
}

pub struct RunStage1 {
    input_info: transformations::InputInfo,
    output_directory: PathBuf,
    demultiplex_infos: Vec<(usize, OptDemultiplex)>,
    demultiplex_step_infos: Vec<DemultiplexStepInfo>,
    allow_overwrite: bool,
}

impl RunStage1 {
    #[expect(
        clippy::too_many_lines,
        clippy::similar_names,
        reason = "needed. rx/tx is clear enough"
    )]
    pub fn create_input_threads(self, parsed: &CheckedConfig) -> Result<RunStage2> {
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // invoke the default handler and exit the process
            orig_hook(panic_info);
            eprintln!("A panic occured in a work thread. Aborting run.");
            std::process::exit(1); //abort would be more sensible, but kills coverage.
        }));
        let input_config = &parsed.input;
        // All input-side pool sizes come from the central `calculate_thread_counts`
        // (via `threading_configuration`): decompression and the pod-parser demux
        // pool are tuned separately, so route each from its own field rather than
        // reusing one `threads_per_segment` for both.
        let threads_per_parser = ParserThreadCounts {
            decompression: ThreadCount(parsed.threading_configuration.n_input_per_segment),
            pod_demux: ThreadCount(parsed.threading_configuration.n_pod_demux_per_segment),
        };
        let mut input_files =
            crate::io::open_input_files(input_config, &parsed.input_formats_detected)
                .context("Error opening input files")?;

        let block_size = parsed.options.block_size;
        let buffer_size = parsed.options.buffer_size;
        let channel_size = 5;
        let error_collector = Arc::new(Mutex::new(Vec::<String>::new()));
        let input_options = parsed.input.options.clone();

        let largest_segment_idx = input_files.largest_segment_idx;

        let (input_threads, combiner_thread, combiner_output_rx) = if let Some(benchmark) =
            &parsed.benchmark
        {
            if benchmark.enable {
                // Benchmark mode: read first block and repeat it
                let molecule_count = benchmark.molecule_count;

                match &parsed.input.structured {
                    StructuredInput::Interleaved { segment_order, .. } => {
                        let segment_order_len = NonZero::new(segment_order.len())
                            .expect("Must have at least one segment");
                        let input_threads = Vec::new();
                        let (combiner_output_tx, combiner_output_rx) =
                            bounded::<(io::FastQBlocksCombined, Option<usize>)>(channel_size);

                        // Read the first block
                        let mut parser = ChainedParser::new(
                            input_files.segment_files.segments.pop().expect(
                                "segments must contain at least one element for interleaved input",
                            ),
                            block_size,
                            buffer_size,
                            threads_per_parser,
                            input_options,
                        );

                        let first_block = parser
                            .parse()
                            .context("Failed to read first block for benchmark")?;
                        if first_block.fastq_block.is_empty() {
                            // cov:excl-start
                            bail!(
                                "Benchmark error: Input is empty - cannot benchmark with empty input"
                            );
                            // cov:excl-stop
                        }

                        let combiner_thread = thread::Builder::new()
                            .name("BenchmarkInterleavedReader".into())
                            .spawn(move || {
                                run_benchmark_interleaved_thread(
                                    first_block.fastq_block,
                                    &combiner_output_tx,
                                    segment_order_len,
                                    molecule_count,
                                );
                            })
                            .expect("Thread spawning failed. OS resource exhaustion?");

                        (input_threads, combiner_thread, combiner_output_rx)
                    }
                    StructuredInput::Segmented { .. } => {
                        let input_threads = Vec::new();
                        let (combiner_output_tx, combiner_output_rx) =
                            bounded::<(io::FastQBlocksCombined, Option<usize>)>(channel_size);

                        // Read the first block from each segment
                        let mut first_blocks = Vec::new();
                        //these are already in segment_order, open_input_files does that for us
                        for this_segments_input_files in input_files.segment_files.segments {
                            let mut parser = ChainedParser::new(
                                this_segments_input_files,
                                block_size,
                                buffer_size,
                                threads_per_parser,
                                input_options.clone(),
                            );

                            let first_block = parser
                                .parse()
                                .context("Failed to read first block for benchmark")?;
                            if first_block.fastq_block.is_empty() {
                                // cov:excl-start
                                bail!(
                                    "Benchmark error: Input segment is empty - cannot benchmark with empty input"
                                );
                                // cov:excl-stop
                            }
                            first_blocks.push(first_block.fastq_block);
                        }

                        // Validate that all first blocks have the same size
                        let first_len = first_blocks[0].len();
                        if !first_blocks.iter().all(|b| b.len() == first_len) {
                            // cov:excl-start
                            bail!(
                                "Benchmark error: First blocks of different segments have different sizes. Cannot proceed with benchmark."
                            );
                            // cov:excl-stop
                        }

                        let first_combined = io::FastQBlocksCombined::new(
                            first_blocks,
                            None,
                            Default::default(),
                            false,
                            0,
                            0,
                        );

                        let combiner_thread = thread::Builder::new()
                            .name("BenchmarkCombiner".into())
                            .spawn(move || {
                                run_benchmark_combiner_thread(
                                    &first_combined,
                                    &combiner_output_tx,
                                    molecule_count,
                                );
                            })
                            .expect("Thread spawning failed. OS resource exhaustion?");

                        (input_threads, combiner_thread, combiner_output_rx)
                    }
                }
            } else {
                // cov:excl-start
                bail!("Benchmark is configured but not enabled");
                // cov:excl-stop
            }
        } else {
            // Normal mode
            match &parsed.input.structured {
                StructuredInput::Interleaved { segment_order, .. } => {
                    let error_collector = error_collector.clone();
                    let segment_order_len =
                        NonZero::new(segment_order.len()).expect("Must have at least one segment");
                    let input_threads = Vec::new();
                    let (combiner_output_tx, combiner_output_rx) =
                        bounded::<(io::FastQBlocksCombined, Option<usize>)>(channel_size);
                    let options = input_options.clone();
                    let combiner_thread = thread::Builder::new()
                        .name("InterleavedReader".into())
                        .spawn(move || {
                            if let Err(e) = parse_interleaved_and_send(
                            input_files.segment_files.segments.pop().expect(
                                "segments must contain at least one element for interleaved input",
                            ),
                            &combiner_output_tx,
                            segment_order_len,
                            buffer_size,
                            threads_per_parser,
                            block_size,
                            options,
                        ) {
                            //cov:excl-start
                            error_collector
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner)
                                .push(format!("Error in interleaved parsing thread: {e:?}"));
                            //cov:excl-stop
                        }
                        })
                        .expect("Thread spawning failed. OS resource exhaustion?");

                    /* vec![(
                        "interleaved".to_string(),
                        input_files.segments.pop().unwrap(),
                    )]; */
                    (input_threads, combiner_thread, combiner_output_rx)
                }
                StructuredInput::Segmented { segment_order, .. } => {
                    // we spawn one reading thread per input segment for reading & decompressing.
                    // and another thread that collects the blocks into combined blocks
                    let mut threads = Vec::new();
                    let mut raw_rx_readers = Vec::new();
                    for (segment_name, this_segments_input_files) in segment_order
                        .iter()
                        .zip(input_files.segment_files.segments.into_iter())
                    {
                        let segment_name = segment_name.clone();
                        let error_collector = error_collector.clone();
                        let options = input_options.clone();
                        let (raw_tx_read, raw_rx_read) = bounded(channel_size);
                        let read_thread = thread::Builder::new()
                            .name(format!("Chunker_{segment_name}"))
                            .spawn(move || {
                                if let Err(e) = parse_and_send(
                                    this_segments_input_files,
                                    &raw_tx_read,
                                    buffer_size,
                                    block_size,
                                    threads_per_parser,
                                    options,
                                ) {
                                    error_collector
                                    .lock()
                                    .unwrap_or_else(std::sync::PoisonError::into_inner)
                                    .push(format!(
                                        "Error in reading thread for segment {segment_name}: {e:?}"
                                    ));
                                }
                            })
                            .expect("Thread spawning failed. OS resource exhaustion?");
                        threads.push(read_thread);
                        raw_rx_readers.push(raw_rx_read);
                    }
                    let (combiner_output_tx, combiner_output_rx) =
                        bounded::<(io::FastQBlocksCombined, Option<usize>)>(channel_size);

                    {
                        let error_collector = error_collector.clone();
                        let combiner = thread::Builder::new()
                            .name("Combiner".into())
                            .spawn(move || {
                                run_combiner_thread(
                                    &raw_rx_readers,
                                    &combiner_output_tx,
                                    largest_segment_idx,
                                    &error_collector,
                                    block_size.into(),
                                );
                            })
                            .expect("Thread spawning failed. OS resource exhaustion?");
                        (threads, combiner, combiner_output_rx)
                    }
                }
            }
        }; // Close the if-else statement

        Ok(RunStage2 {
            input_info: self.input_info,
            output_directory: self.output_directory,
            demultiplex_infos: self.demultiplex_infos,
            demultiplex_step_infos: self.demultiplex_step_infos,
            input_threads,
            combiner_thread,
            combiner_output_rx,
            error_collector,
            allow_overwrite: self.allow_overwrite,
        })
    }
}

pub struct RunStage2 {
    input_info: transformations::InputInfo,
    output_directory: PathBuf,
    demultiplex_infos: Vec<(usize, OptDemultiplex)>,
    demultiplex_step_infos: Vec<DemultiplexStepInfo>,

    input_threads: Vec<thread::JoinHandle<()>>,
    combiner_thread: thread::JoinHandle<()>,
    combiner_output_rx: crossbeam::channel::Receiver<(io::FastQBlocksCombined, Option<usize>)>,

    error_collector: Arc<Mutex<Vec<String>>>,
    allow_overwrite: bool,
}

#[derive(Copy, Clone, Debug)]
pub struct BlockFinallyDone {
    // pub block_no: usize,
    pub initial_molecule_count: usize,
}

impl RunStage2 {
    pub fn create_stage_threads(self, parsed: &mut CheckedConfig) -> RunStage3 {
        self.create_workpool_pipeline(parsed)
    }

    pub fn create_workpool_pipeline(self, parsed: &mut CheckedConfig) -> RunStage3 {
        use crate::pipeline_workpool::{WorkpoolCoordinator, worker_thread};

        //take the stages out of parsed now
        let stages = std::mem::take(&mut parsed.stages);
        let worker_count = parsed
            .options
            .threads
            .expect("Thread count should have been set by config parsing");
        let max_molecules_in_flight = parsed.options.max_molecules_in_flight;

        // Create channels
        // unbounded is fine, we later on count reads in flight
        // and prevent having more than max_reads_in_flight
        // across *all* channels (admitting whole blocks, so we may overshoot
        // the budget by up to one block).
        let (todo_tx, todo_rx) = unbounded();
        let (done_tx, done_rx) = unbounded();
        let (output_tx, output_rx) = unbounded();
        let (output_done_tx, output_done_rx) = unbounded();

        let demultiplex_infos = self.demultiplex_infos.clone();
        let input_info = self.input_info.clone();

        // Create coordinator thread
        let report_collector = Arc::new(Mutex::new(
            Vec::<transformations::FinalizeReportResult>::new(),
        ));
        let coordinator_demultiplex_infos = demultiplex_infos.clone();

        let (coordinator, shared_stages) = WorkpoolCoordinator::new(
            stages,
            max_molecules_in_flight.into(),
            self.combiner_output_rx,
            todo_tx,
            done_rx,
            output_tx,
            output_done_rx,
            report_collector,
            self.error_collector.clone(),
            input_info.blocks_in_flight.clone(),
        );

        let coordinator_thread = thread::Builder::new()
            .name("WorkpoolCoordinator".into())
            .spawn(move || {
                coordinator.run(&coordinator_demultiplex_infos);
            })
            .expect("Thread spawning failed. OS resource exhaustion?");

        // Create worker threads
        let mut worker_threads = Vec::new();
        for worker_id in 0..worker_count {
            let todo_rx = todo_rx.clone();
            let done_tx = done_tx.clone();
            let demultiplex_infos = demultiplex_infos.clone();
            let input_info = input_info.clone();

            let stages = shared_stages.clone();

            let worker_thread = thread::Builder::new()
                .name(format!("WorkpoolWorker_{worker_id}"))
                .spawn(move || {
                    worker_thread(
                        worker_id,
                        &todo_rx,
                        &done_tx,
                        &stages,
                        &input_info,
                        &demultiplex_infos,
                    );
                })
                .expect("Thread spawning failed. OS resource exhaustion?");

            worker_threads.push(worker_thread);
        }

        // We need to store coordinator and workers as stage threads
        let mut all_threads = vec![coordinator_thread];
        all_threads.extend(worker_threads);

        RunStage3 {
            output_directory: self.output_directory,
            demultiplex_infos: self.demultiplex_infos,
            demultiplex_step_infos: self.demultiplex_step_infos,
            input_threads: self.input_threads,
            combiner_thread: self.combiner_thread,
            stage_threads: all_threads,
            stage_to_output_channel: output_rx,
            error_collector: self.error_collector,
            allow_overwrite: self.allow_overwrite,
            output_done_tx,
            reads_per_tag: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }
}

pub struct RunStage3 {
    output_directory: PathBuf,
    demultiplex_infos: Vec<(usize, OptDemultiplex)>,
    demultiplex_step_infos: Vec<DemultiplexStepInfo>,
    allow_overwrite: bool,

    input_threads: Vec<thread::JoinHandle<()>>,
    combiner_thread: thread::JoinHandle<()>,
    stage_threads: Vec<thread::JoinHandle<()>>,
    stage_to_output_channel: crossbeam::channel::Receiver<(io::FastQBlocksCombined, Option<usize>)>,
    error_collector: Arc<Mutex<Vec<String>>>,
    output_done_tx: crossbeam::channel::Sender<BlockFinallyDone>,
    reads_per_tag: Arc<Mutex<BTreeMap<crate::demultiplex::Tag, u64>>>,
}

fn collect_thread_failures(
    threads: Vec<thread::JoinHandle<()>>,
    msg: &str,
    error_collector: &Arc<Mutex<Vec<String>>>,
) -> Vec<String> {
    let mut stage_errors = Vec::new();
    for s in error_collector
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .drain(..)
    {
        stage_errors.push(s);
    }
    for p in threads {
        if let Err(e) = p.join() {
            // that's not a controlled 'we detected an error' failure (those are collected
            // above, but something more catastrophic
            // cov:excl-start
            let err_msg = if let Some(e) = e.downcast_ref::<String>() {
                e.clone()
            } else if let Some(e) = e.downcast_ref::<&str>() {
                (*e).to_string()
            } else {
                format!(
                    "Unknown error: {:?} {:?}",
                    e,
                    std::any::type_name_of_val(&e)
                )
            };
            stage_errors.push(format!("{msg}: {err_msg}"));
            // cov:excl-stop
        }
    }
    stage_errors
}

impl RunStage3 {
    pub fn create_output_threads(
        self,
        merge_config: Option<&crate::bam_merge::MergeConfig>,
    ) -> Result<RunStage4> {
        let input_channel = self.stage_to_output_channel;

        let merge_bam_handles = merge_config
            .map(|mc| {
                crate::bam_merge::create_merge_output_handles(
                    &self.output_directory,
                    mc,
                    &self.demultiplex_infos,
                    &self.demultiplex_step_infos,
                    self.allow_overwrite,
                )
            })
            .transpose()?;

        let output_done_tx = self.output_done_tx;
        let reads_per_tag = self.reads_per_tag.clone();
        // Only needed to size the BAI of merged demultiplexed BAMs. Each molecule
        // becomes `records_per_molecule` BAM records (>1 for interleaved output).
        let records_per_molecule: u64 = merge_config
            .filter(|_| merge_bam_handles.is_some())
            .map_or(0, |mc| mc.records_per_molecule as u64);
        let count_reads_per_tag = records_per_molecule > 0; //mutants:skip
        // only performance difference if we do collect them unnecessarily.

        let output = {
            thread::Builder::new()
                .name("output".into())
                .spawn(move || {
                    // Output files and reports are now produced by the Output* steps
                    // inside the work pool. This thread only drains the final channel
                    // (preserving block order so the coordinator's backpressure via
                    // `output_done_tx` stays correct) and joins the stage threads.
                    let mut last_block_outputted = 0;
                    let mut buffer: Vec<(usize, io::FastQBlocksCombined)> = Vec::new();
                    while let Ok((block, _expected_read_count)) = input_channel.recv() {
                        let block_no = block.block_no();
                        // Count reads per demultiplex tag for the BAM merge (the
                        // Output* steps no longer feed this back to the binary).
                        if count_reads_per_tag && let Some(tags) = block.output_tags.as_ref() {
                            let mut counts = reads_per_tag
                                .lock()
                                .unwrap_or_else(std::sync::PoisonError::into_inner);
                            for &tag in tags {
                                *counts.entry(tag).or_insert(0) += records_per_molecule;
                            }
                        }
                        //resort out of order blocks into the right order.
                        buffer.push((block_no, block));
                        loop {
                            let mut send = None;
                            for (ii, (block_no, _block)) in buffer.iter().enumerate() {
                                if block_no - 1 == last_block_outputted {
                                    last_block_outputted += 1;
                                    send = Some(ii);
                                    break;
                                }
                            }
                            if let Some(send_idx) = send {
                                let (_block_no, block) = buffer.remove(send_idx);
                                let _ = output_done_tx.send(BlockFinallyDone {
                                    initial_molecule_count: block.initial_molecule_count(),
                                }); // can fail if the coordinator exited on error
                            } else {
                                break;
                            }
                        }
                    }
                    //all blocks are done, the stage output channel has been closed.
                    //join the stage threads so their finalize / post_finalize
                    //(including OutputReport) has completed.
                    for thread in self.stage_threads {
                        thread.join().expect("thread join failure");
                    }
                })
                .expect("Thread spawning failed. OS resource exhaustion?")
        };

        Ok(RunStage4 {
            input_threads: self.input_threads,
            combiner_thread: self.combiner_thread,
            output_thread: output,
            error_collector: self.error_collector,
            demultiplex_infos: self.demultiplex_infos,
            demultiplex_step_infos: self.demultiplex_step_infos,
            reads_per_demultiplex_tag: self.reads_per_tag,
            merge_bam_handles,
        })
    }
}

pub struct RunStage4 {
    error_collector: Arc<Mutex<Vec<String>>>,
    input_threads: Vec<thread::JoinHandle<()>>,
    combiner_thread: thread::JoinHandle<()>,
    output_thread: thread::JoinHandle<()>,
    pub demultiplex_infos: Vec<(usize, OptDemultiplex)>,
    pub demultiplex_step_infos: Vec<DemultiplexStepInfo>,
    reads_per_demultiplex_tag: Arc<Mutex<BTreeMap<crate::demultiplex::Tag, u64>>>,
    pub merge_bam_handles: Option<crate::bam_merge::MergeBamHandles>,
}

impl RunStage4 {
    pub fn join_threads(self) -> RunStage5 {
        let mut errors = Vec::new();
        for (threads, msg) in [
            (vec![self.output_thread], "Failure in output thread"),
            (
                vec![self.combiner_thread],
                "Failure in read-combination-thread thread",
            ),
            //            (self.stage_threads, "Failure in stage processor thread"),
            (self.input_threads, "Failure in input thread"),
        ] {
            errors.extend(collect_thread_failures(threads, msg, &self.error_collector));
        }

        let reads_per_tag = Arc::into_inner(self.reads_per_demultiplex_tag)
            .expect("reads_per_demultiplex_tag mutex still had multiple references")
            .into_inner()
            .expect("reads_per_tag mutex was poisoned");

        RunStage5 {
            errors,
            demultiplex_infos: self.demultiplex_infos,
            demultiplex_step_infos: self.demultiplex_step_infos,
            reads_per_tag,
            merge_bam_handles: self.merge_bam_handles,
        }
    }
}

pub struct RunStage5 {
    pub errors: Vec<String>,
    pub demultiplex_infos: Vec<(usize, OptDemultiplex)>,
    pub demultiplex_step_infos: Vec<DemultiplexStepInfo>,
    pub reads_per_tag: BTreeMap<crate::demultiplex::Tag, u64>,
    pub merge_bam_handles: Option<crate::bam_merge::MergeBamHandles>,
}

#[cfg(test)]
mod tests {
    use super::bits_needed_to_represent;
    #[test]
    fn test_bits_needed_to_represent() {
        assert_eq!(bits_needed_to_represent(0), 1);
        assert_eq!(bits_needed_to_represent(1), 1);
        assert_eq!(bits_needed_to_represent(7), 3);
        assert_eq!(bits_needed_to_represent(8), 4);
        assert_eq!(bits_needed_to_represent(65535), 16);
        assert_eq!(bits_needed_to_represent(65536), 17);
    }
}
