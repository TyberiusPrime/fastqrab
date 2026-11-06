use anyhow::{Result, bail};
use ex::Wrapper;
use ex::fs::File;
use std::{
    io::{BufReader, Read},
    num::NonZero,
    path::PathBuf,
};

use crate::CompressionFormat;
use crate::blocks::FastQChunk;
use crate::io::input::{DecompressionOptions, open_decompressed_reader};
use crate::io::parsers::{ParseResult, Parser};
use stringpod::{DualStringPodBuilder, StringPod, StringPodBuilder};

#[cfg(unix)]
use crate::io::{
    input::{DecompressorFormat, shm_eligible_format},
    parsers::shm::{ShmChunkReader, shm_enabled, spawn_shm_chunk_reader},
    pod_parser::Chunk,
};
#[cfg(unix)]
use anyhow::{Context, anyhow};
#[cfg(unix)]
use crossbeam::channel::Receiver;
#[cfg(unix)]
use std::{sync::Arc, thread::JoinHandle};

/// FASTA parser. Every path funnels bytes into the same serial assembler
/// ([`FastaPods`]): a regular gzip/zstd file decoded out-of-process takes the
/// shared-memory fast path ([`FastaInner::Pod`]), parsing borrowed decode chunks
/// in place with no bulk pipe copy; everything else ([`FastaInner::Reader`] —
/// uncompressed, stdin/FIFO, out-of-process-piped gzip/zstd, non-Unix) reads the
/// decompressed byte stream in chunks and feeds the same assembler. The FASTA
/// analogue of [`super::PodFastqParser`].
pub struct FastaParser {
    inner: FastaInner,
    compression_format: CompressionFormat,
}

enum FastaInner {
    /// Generic byte-stream path: read `READER_CHUNK_SIZE` bytes at a time from a
    /// (possibly subprocess-piped) reader and feed the serial FASTA assembler.
    Reader {
        reader: BufReader<Box<dyn Read + Send>>,
        state: Box<FastaPods>,
        buf: Vec<u8>,
        eof: bool,
    },
    /// Shared-memory transport: borrowed decode chunks parsed in place. Boxed —
    /// it is much larger than the `Reader` variant and the shm path is not the
    /// common case.
    #[cfg(unix)]
    Pod(Box<FastaPodInner>),
}

impl FastaParser {
    /// # Panics
    /// when out-of-process decode & stdin are specified together (validation
    /// prevents this).
    pub fn new(
        file: File,
        filename: Option<&PathBuf>,
        target_reads_per_block: NonZero<usize>,
        fake_quality_phred: u8,
        decompression_options: DecompressionOptions,
        buffer_size: usize,
    ) -> Result<FastaParser> {
        // `ex::fs::File` → `std::fs::File` so the shm magic-sniff and
        // `open_decompressed_reader` both take a plain handle (mirrors the FASTQ
        // parser's `file.into_inner()`).
        let file = file.into_inner();

        // Shared-memory fast path: out-of-process-decoded gzip/zstd FASTA on a
        // regular file, unless `FASTQRAB_DECOMP_SHM=0`. Everything else (stdin/
        // FIFO, other codecs, uncompressed, the Default path) uses the bio reader.
        #[cfg(unix)]
        if let Some(path) = filename
            && shm_enabled()
            && let Some(format) = shm_eligible_format(&file)
        {
            return Self::new_shm(
                path,
                format,
                decompression_options.thread_count,
                target_reads_per_block,
                fake_quality_phred,
            );
        }

        // Non-shm path: gzip/zstd decode out-of-process (all FFI in the child),
        // uncompressed read directly — both surfaced as one byte stream.
        let (reader, compression_format) =
            open_decompressed_reader(file, filename, decompression_options)?;

        Ok(FastaParser {
            inner: FastaInner::Reader {
                reader: BufReader::new(reader),
                state: Box::new(FastaPods::new(
                    fake_quality_phred,
                    target_reads_per_block.get(),
                )),
                buf: vec![0u8; buffer_size.max(1)],
                eof: false,
            },
            compression_format,
        })
    }

    /// Shared-memory constructor (Unix, gzip/zstd FASTA). Stands up the shared
    /// [`ShmChunkReader`] transport; the serial FASTA pod parser then consumes its
    /// borrowed chunks directly in [`FastaPodInner::parse`] (no parser thread —
    /// FASTA assembly is serial, so we drive it pull-style from `parse`).
    #[cfg(unix)]
    fn new_shm(
        path: &std::path::Path,
        format: DecompressorFormat,
        thread_count: crate::io::parsers::ThreadCount,
        target_reads_per_block: NonZero<usize>,
        fake_quality_phred: u8,
    ) -> Result<FastaParser> {
        // `parallel_hint = 1`: the FASTA parser is serial, so the ring only needs
        // to cover the decode pool's depth.
        let ShmChunkReader {
            bytes_rx,
            reader_handle,
            slot_writer_handle,
            child,
            region,
        } = spawn_shm_chunk_reader(path, format, thread_count, 1)?;

        Ok(FastaParser {
            inner: FastaInner::Pod(Box::new(FastaPodInner {
                bytes_rx,
                reader_handle: Some(reader_handle),
                slot_writer_handle: Some(slot_writer_handle),
                child: Some(child),
                _region: Some(region as Arc<dyn std::any::Any + Send + Sync>),
                state: FastaPods::new(fake_quality_phred, target_reads_per_block.get()),
                eof: false,
            })),
            compression_format: format.to_compression(),
        })
    }

    /// One pull of the generic-reader path: read decompressed bytes in
    /// `READER_CHUNK_SIZE` chunks into the serial assembler until a block is ready
    /// (≥ `target` records) or the stream ends. Mirrors [`FastaPodInner::parse`],
    /// reading from a byte stream instead of borrowed shm slots.
    fn parse_reader(
        reader: &mut BufReader<Box<dyn Read + Send>>,
        state: &mut FastaPods,
        buf: &mut [u8],
        eof: &mut bool,
    ) -> Result<ParseResult> {
        if *eof {
            return Ok(ParseResult {
                output: FastQChunk::new_empty(),
                was_final: true,
            });
        }
        loop {
            let n = fill(reader, buf)?;
            if n == 0 {
                state.finish_stream();
                *eof = true;
                return Ok(ParseResult {
                    output: state.take_block(),
                    was_final: true,
                });
            }
            state.feed_chunk(&buf[..n]);
            if state.has_block() {
                return Ok(ParseResult {
                    output: state.take_block(),
                    was_final: false,
                });
            }
        }
    }
}

/// One `read`, retrying past `Interrupted`. `Ok(0)` means end of stream.
fn fill(reader: &mut impl Read, buf: &mut [u8]) -> Result<usize> {
    loop {
        match reader.read(buf) {
            Ok(n) => return Ok(n),
            // Interrupted: retry the read on the next loop iteration.
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {} // cov:excl-line //mutants::skip
            Err(e) => return Err(e.into()),                             // cov:excl-line
        }
    }
}

impl Parser for FastaParser {
    fn bytes_per_base(&self) -> f64 {
        match self.compression_format {
            CompressionFormat::Gzip | CompressionFormat::Zstd => 0.38,
            CompressionFormat::Uncompressed => 1.4,
        }
    }

    fn parse(&mut self) -> Result<ParseResult> {
        match &mut self.inner {
            FastaInner::Reader {
                reader,
                state,
                buf,
                eof,
            } => FastaParser::parse_reader(reader, state, buf, eof),
            #[cfg(unix)]
            FastaInner::Pod(inner) => inner.parse(),
        }
    }
}

/// Trim trailing ASCII whitespace, matching rust-bio's `str::trim_end` on each
/// FASTA line so this parser yields byte-identical names and sequences to the
/// reference reader it replaced.
fn trim_end_ascii_whitespace(mut s: &[u8]) -> &[u8] {
    while let [rest @ .., last] = s {
        if last.is_ascii_whitespace() {
            s = rest;
        } else {
            break;
        }
    }
    s
}

/// Serial FASTA record assembler over an arbitrary stream of decode chunks.
///
/// FASTA records have no fixed line count (sequence wraps across any number of
/// lines), so unlike the FASTQ demux there is no phase-rotation parallelism —
/// this threads a partial-line `carry` across chunk boundaries and accumulates
/// each record's (multi-line) sequence into `cur_seq`, emitting completed records
/// into columnar builders. Name/sequence handling mirrors rust-bio exactly (see
/// [`trim_end_ascii_whitespace`] and the header `splitn(2, whitespace)`) so this
/// parser is byte-for-byte interchangeable with the reference reader it replaced.
struct FastaPods {
    names: StringPodBuilder,
    seq_quals: DualStringPodBuilder,
    count: usize,
    /// Header of the record currently being assembled (after the `>`), `None`
    /// between records (and before the first header).
    cur_name: Option<Vec<u8>>,
    /// Concatenated sequence bytes of the record currently being assembled.
    cur_seq: Vec<u8>,
    /// Partial trailing line carried across a chunk boundary.
    carry: Vec<u8>,
    /// Fake-quality scratch grown to the longest sequence seen.
    qual_scratch: Vec<u8>,
    fake_quality: u8,
    target: usize,
}

impl FastaPods {
    fn new(fake_quality: u8, target: usize) -> Self {
        FastaPods {
            names: StringPodBuilder::with_capacity(0, target),
            seq_quals: DualStringPodBuilder::with_capacity(0, target),
            count: 0,
            cur_name: None,
            cur_seq: Vec::new(),
            carry: Vec::new(),
            qual_scratch: Vec::new(),
            fake_quality,
            target,
        }
    }

    /// Feed one decode chunk: complete the carried line, then split the rest on
    /// newlines, carrying the final partial line for the next chunk.
    fn feed_chunk(&mut self, data: &[u8]) {
        let mut body = data;
        if !self.carry.is_empty() {
            match memchr::memchr(b'\n', data) {
                Some(nl) => {
                    let mut line = std::mem::take(&mut self.carry);
                    line.extend_from_slice(&data[..nl]);
                    self.feed_line(&line);
                    line.clear();
                    self.carry = line; // reuse the allocation
                    body = &data[nl + 1..];
                }
                None => {
                    // No newline in the whole chunk: still mid-line, keep carrying.
                    self.carry.extend_from_slice(data);
                    return;
                }
            }
        }
        let mut line_start = 0usize;
        for nl in memchr::memchr_iter(b'\n', body) {
            self.feed_line(&body[line_start..nl]);
            line_start = nl + 1;
        }
        self.carry.extend_from_slice(&body[line_start..]);
    }

    /// Process one complete line (newline already removed). A `>` line closes the
    /// previous record and opens a new one; any other line appends to the current
    /// record's sequence.
    fn feed_line(&mut self, raw: &[u8]) {
        let line = trim_end_ascii_whitespace(raw);
        if line.first() == Some(&b'>') {
            self.emit_current();
            // Mirror bio: `line[1..].splitn(2, whitespace)` → id + optional desc,
            // rejoined as `id` or `id ' ' desc`.
            let header = &line[1..];
            let name = match header.iter().position(u8::is_ascii_whitespace) {
                Some(pos) => {
                    let id = &header[..pos];
                    let desc = &header[pos + 1..];
                    let mut name = Vec::with_capacity(id.len() + 1 + desc.len());
                    name.extend_from_slice(id);
                    name.push(b' ');
                    name.extend_from_slice(desc);
                    name
                }
                None => header.to_vec(),
            };
            self.cur_name = Some(name);
            self.cur_seq.clear();
        } else {
            if self.cur_name.is_none() {
                // Tolerate leading blank lines; real sequence before a header is
                // not FASTA.
                unreachable!(
                    "Fasta with leading lines? We detect on the first char, can not happen"
                );
                // if line.is_empty() {
                //     return Ok(());
                // }
                // bail!(
                //     "FASTA parse error: expected a '>' header line. Check that your input is valid FASTA."
                // );
            }
            self.cur_seq.extend_from_slice(line);
        }
    }

    /// Push the record currently being assembled (if any) into the columns.
    fn emit_current(&mut self) {
        if let Some(name) = self.cur_name.take() {
            self.names.push(&name);
            let n = self.cur_seq.len();
            if self.qual_scratch.len() != n {
                self.qual_scratch.resize(n, self.fake_quality);
            }
            self.seq_quals.push(&self.cur_seq, &self.qual_scratch[..n]);
            self.count += 1;
        }
    }

    /// End of stream: fold the final unterminated line (if any) and flush the last
    /// open record.
    fn finish_stream(&mut self) {
        if !self.carry.is_empty() {
            let carry = std::mem::take(&mut self.carry);
            self.feed_line(&carry);
        }
        self.emit_current();
    }

    /// A full block (≥ `target` completed records) is ready to emit.
    fn has_block(&self) -> bool {
        self.count >= self.target
    }

    /// Take the accumulated whole records as a columnar block, resetting builders.
    fn take_block(&mut self) -> FastQChunk {
        let names = std::mem::replace(
            &mut self.names,
            StringPodBuilder::with_capacity(0, self.target),
        )
        .finish();
        let seq_quals = std::mem::replace(
            &mut self.seq_quals,
            DualStringPodBuilder::with_capacity(0, self.target),
        )
        .finish();
        let count = self.count;
        self.count = 0;
        FastQChunk {
            names,
            seq_quals,
            pluses: StringPod::new_all_empty(
                u32::try_from(count).expect("too many reads in a block for u32"),
            ),
        }
    }
}

/// Shared-memory FASTA transport: the [`ShmChunkReader`] plus the serial
/// assembler state. `parse` pulls borrowed chunks straight off the ring (no
/// parser thread — FASTA assembly is serial) and emits one record-aligned block
/// per call.
#[cfg(unix)]
struct FastaPodInner {
    bytes_rx: Receiver<Chunk>,
    reader_handle: Option<JoinHandle<Result<()>>>,
    slot_writer_handle: Option<JoinHandle<()>>,
    child: Option<std::process::Child>,
    _region: Option<Arc<dyn std::any::Any + Send + Sync>>,
    state: FastaPods,
    eof: bool,
}

#[cfg(unix)]
impl FastaPodInner {
    fn parse(&mut self) -> Result<ParseResult> {
        if self.eof {
            unreachable!("Should not be called after was_final");
            // return Ok(ParseResult {
            //     output: FastQChunk::new_empty(),
            //     was_final: true,
            // });
        }
        loop {
            match self.bytes_rx.recv() {
                Ok(chunk) => {
                    self.state.feed_chunk(&chunk);
                    drop(chunk); // copied out; release the slot
                    if self.state.has_block() {
                        return Ok(ParseResult {
                            output: self.state.take_block(),
                            was_final: false,
                        });
                    }
                }
                Err(_) => {
                    // Ring closed: every chunk decoded. Finalize and join.
                    self.state.finish_stream();
                    self.eof = true;
                    self.finish_threads()?;
                    return Ok(ParseResult {
                        output: self.state.take_block(),
                        was_final: true,
                    });
                }
            }
        }
    }

    /// Join the transport threads and surface a non-zero decompressor exit, once
    /// the ring has closed and every borrowed chunk has dropped (returning its
    /// slot). Mirrors `PodFastqParser::finish_threads`.
    #[mutants::skip] //it's all worst case error handling / defensive code
    fn finish_threads(&mut self) -> Result<()> {
        if let Some(handle) = self.reader_handle.take() {
            handle
                .join()
                .map_err(|e| anyhow!("fasta shm reader thread panicked {e:?}"))??;
        } // cov:excl-line
        if let Some(handle) = self.slot_writer_handle.take() {
            handle
                .join()
                .map_err(|e| anyhow!("fasta shm slot-return thread panicked: {e:?}"))?;
        } // cov:excl-line
        if let Some(mut child) = self.child.take() {
            let status = child
                .wait()
                .context("waiting on fastqrab-decompressor (shm mode)")?;
            if !status.success() {
                bail!("fastqrab-decompressor exited unsuccessfully: {status}"); // cov:excl-line
            }
        } // cov:excl-line
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use bstr::ByteSlice;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parses_fasta_records_into_fastq_reads() -> Result<()> {
        let mut temp = NamedTempFile::new()?;
        writeln!(temp, ">read1\nACGT\n>read2 description\nTGCA\n")?;
        temp.flush()?;

        let file = File::open(temp.path())?;
        let mut parser = FastaParser::new(
            file,
            Some(temp.path().to_owned()).as_ref(),
            NonZero::new(10).unwrap(),
            30,
            DecompressionOptions {
                thread_count: crate::io::parsers::ThreadCount(std::num::NonZero::<usize>::MIN),
            },
            256 * 1024,
        )?; // cov:excl-line

        let ParseResult { output, was_final } = parser.parse()?;
        let chunk = output;
        assert!(was_final);
        assert_eq!(chunk.len(), 2);

        assert_eq!(chunk.names.get(0).as_bytes(), b"read1");
        let (seq0, qual0) = chunk.seq_quals.pair(0);
        assert_eq!(seq0.as_bytes(), b"ACGT");
        assert_eq!(qual0.as_bytes(), [30u8; 4].as_slice());
        // FASTA has no '+' line — the plus column is empty for every read.
        assert_eq!(chunk.pluses.get(0).as_bytes(), b"");

        assert_eq!(chunk.names.get(1).as_bytes(), b"read2 description");
        let (seq1, qual1) = chunk.seq_quals.pair(1);
        assert_eq!(seq1.as_bytes(), b"TGCA");
        assert_eq!(qual1.as_bytes(), [30u8; 4].as_slice());

        let ParseResult {
            output: second_output,
            was_final: is_final,
        } = parser.parse()?;
        let second_chunk = second_output;

        assert!(is_final);
        assert!(second_chunk.is_empty());

        Ok(())
    }
}
