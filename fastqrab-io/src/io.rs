use anyhow::{Context, Result};
use std::path::Path;

use crate::get_number_of_cores;
use crate::io::input::InputOptions;
use crate::io::parsers::{ParserThreadCounts, ThreadCount};
use fastqrab_config::{default_block_size, default_buffer_size};
pub use input::{
    DetectedInputFormat, InputFile, InputFiles, open_file, open_input_file, total_file_size,
};
pub use reads::{FastQBlocksCombined, Position, SegmentsCombined, Tags};

pub use output::simulated_failure;
pub use parsers::bam_read_count_from_index;
pub use pod_parser::{FastqChunk, parse_pods_from_channel};

pub mod input;
pub mod output;
pub mod parsers;
pub mod pod_parser;
pub mod reads;

/// Drive a parser over an already-resolved [`InputFile`], invoking `func` on
/// every read. The shared core behind the `apply_to_read_*` helpers.
fn drive_reads(
    input_file: InputFile,
    func: &mut impl FnMut(&[u8], &[u8], &[u8]) -> Result<()>,
    include_mapped: bool,
    include_unmapped: bool,
) -> Result<()> {
    use bstr::ByteSlice;

    let options = InputOptions {
        fasta_fake_quality: Some(33),
        bam_include_mapped: Some(include_mapped),
        bam_include_unmapped: Some(include_unmapped),
        read_comment_character: b' ', // ignored here.
        threads_per_segment: Some(get_number_of_cores()), // at this point, we're ready to multicore this
                                                          // hard.
    };
    let one = ThreadCount(std::num::NonZero::<usize>::MIN);
    let mut parser = input_file
        .get_parser(
            default_block_size(),
            default_buffer_size(),
            ParserThreadCounts {
                decompression: one,
                pod_demux: one,
            },
            &options,
        )
        .context("Getting parser")?; // cov:excl-line
    loop {
        let res = parser.parse()?;
        // The parser may hand us either the legacy row-oriented block (FASTA /
        // BAM) or columnar chunks (FASTQ); iterate reads out of whichever.
        for i in 0..res.output.len() {
            let (seq, qual) = res.output.seq_quals.pair(i);
            func(
                res.output.names.get(i).as_bytes(),
                seq.as_bytes(),
                qual.as_bytes(),
            )?;
        }
        if res.was_final {
            break;
        } // cov:excl-line
    }

    Ok(())
}

/// Run `func` over the name of every read in an already-open input handle
/// (FASTQ/FASTA/BAM, transparently decompressed). The whole handle is read;
/// declared step inputs hand the runtime-opened handle straight in. rapidgzip
/// needs a path and so is unavailable here.
pub fn apply_to_read_names(
    file: ex::fs::File,
    func: &mut impl FnMut(&[u8]) -> Result<()>,
    include_mapped: bool,
    include_unmapped: bool,
) -> Result<()> {
    drive_reads(
        InputFile::from_handle(file)?,
        &mut |name: &[u8], _seq: &[u8], _qual: &[u8]| func(name),
        include_mapped,
        include_unmapped,
    )
}

/// As [`apply_to_read_names`], but `func` receives each read's sequence.
pub fn apply_to_read_sequences(
    file: ex::fs::File,
    func: &mut impl FnMut(&[u8]) -> Result<()>,
    include_mapped: bool,
    include_unmapped: bool,
) -> Result<()> {
    drive_reads(
        InputFile::from_handle(file)?,
        &mut |_name: &[u8], seq: &[u8], _qual: &[u8]| func(seq),
        include_mapped,
        include_unmapped,
    )
}

/// Given a FASTA or FASTQ or BAM file *by path*, run a callback on each read's
/// (name, sequence) pair. For FASTA, the name is the record id (and description
/// if present), without the leading `>`. Still path-based because the
/// `[barcodes]` loader uses it and is not yet on the declared-input path.
pub fn apply_to_read_names_and_sequences(
    filename: impl AsRef<Path>,
    func: &mut impl FnMut(&[u8], &[u8]) -> Result<()>,
) -> Result<()> {
    let input_file = open_input_file(filename.as_ref(), None).context("open_input_file")?;
    drive_reads(
        input_file,
        &mut |name: &[u8], seq: &[u8], _qual: &[u8]| func(name, seq),
        true,
        true,
    )
}
