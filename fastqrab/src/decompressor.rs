//! fastqrab's gzip/zstd decompressor, run as a subprocess via `fastqrab __decompressor`.
//!
//! Parses args, decodes the input (gzip via `rusty_rapidgzip::read_gz`, zstd via
//! [`read_zstd`]) into a channel of decompressed byte chunks, and ships that
//! channel out — either as raw bytes on stdout (pipe mode) or memcpy'd into a
//! shared-memory ring (shm mode). fastqrab spawns itself with `__decompressor` so
//! the decoders' `unsafe` (rapidgzip, libzstd) runs in an isolated process.

use std::io::{Read as _, Write};
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use crossbeam::channel::{Receiver, Sender, bounded};
use rusty_rapidgzip::{Config, Verbosity, elapsed_since_start, read_gz};

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
enum Format {
    Gzip,
    Zstd,
}

#[derive(Parser, Debug)]
#[command(name = "fastqrab-decompressor", version)]
struct Args {
    /// Input file, or `-` for stdin.
    input: PathBuf,
    /// Input compression format.
    #[arg(long, value_enum, default_value_t = Format::Gzip)]
    format: Format,
    /// Number of worker threads (0 = auto). Gzip only; zstd decode is serial.
    #[arg(short = 'P', long, default_value_t = 0)]
    threads: usize,
    /// Approximate chunk size in bytes.
    #[arg(long, default_value_t = 4 * 1024 * 1024)]
    chunk_size: usize,
    /// Print per-member / per-chunk diagnostics to stderr.
    #[arg(short = 'v', long)]
    verbose: bool,

    /// Peek mode: emit at most this many decoded bytes on stdout, then exit
    /// without decoding the rest of the input. Used by format detection, which
    /// only needs the first few decoded bytes to tell BAM/FASTA/FASTQ apart.
    #[arg(long)]
    peek_bytes: Option<usize>,

    /// File descriptor of the inherited shared-memory region (`memfd`).
    #[arg(long)]
    shm_fd: Option<i32>,
    /// Number of slots in the shared-memory region.
    #[arg(long)]
    shm_slots: Option<usize>,
    /// Size of each shared-memory slot in bytes.
    #[arg(long)]
    shm_slot_size: Option<usize>,

    /// Test-only hook: after the landlock sandbox is applied, try to open this
    /// path for reading and report the outcome on stdout (`PROBE_READ_OK` /
    /// `PROBE_READ_DENIED`), then exit without decoding anything. The normal
    /// decode path only ever opens the input file itself, so it can't exercise
    /// whether the sandbox denies reads elsewhere; this lets integration tests
    /// check that directly.
    #[arg(long, hide = true)]
    landlock_probe_read: Option<PathBuf>,
    /// Test-only hook: after the landlock sandbox is applied, try to create
    /// this path and report the outcome on stdout (`PROBE_WRITE_OK` /
    /// `PROBE_WRITE_DENIED`), then exit without decoding anything.
    #[arg(long, hide = true)]
    landlock_probe_write: Option<PathBuf>,
}

/// Entry point when invoked as `fastqrab __decompressor <args...>`.
///
/// # Errors
/// Propagates decode, IO, or spawn errors.
///
/// # Panics
/// If the producer thread panics.
pub fn run() -> Result<()> {
    // Invoked as: fastqrab __decompressor <decompressor-args...>
    // Drop argv[1] ("__decompressor") so clap sees only the decompressor's args.
    let mut raw = std::env::args_os();
    let argv0 = raw.next().unwrap_or_default();
    let _ = raw.next(); // drop "__decompressor"
    let args = Args::parse_from(std::iter::once(argv0).chain(raw));

    let recycle_cap = std::thread::available_parallelism().map_or(4, std::num::NonZero::get) * 2; //mutants::skip
    let (recycle_tx, recycle_rx) = bounded::<Vec<u8>>(recycle_cap);
    let (tx, rx) = bounded::<Arc<Vec<u8>>>(16);

    let input = if args.input.as_os_str() == "-" {
        PathBuf::from("/dev/stdin")
    } else {
        args.input.clone()
    };

    // Restrict filesystem access before spawning any threads: reads to just
    // the input path, and writes/creates/deletes/renames everywhere, denied
    // outright. Stdin isn't special-cased here — `input` is already
    // `/dev/stdin` at this point, and reading it still goes through a
    // path-based `open()` (see below) — but `apply_landlock` degrades to
    // write-only enforcement when that open can't be scoped to a pipe.
    #[cfg(target_os = "linux")]
    apply_landlock(&input).unwrap_or_else(|e| {
        eprintln!("[decompressor] warning: landlock sandbox not applied: {e}"); // cov:excl-line
    }); // cov:excl-line

    if let Some(probe) = args.landlock_probe_read.as_deref() {
        //mutants::skip - test-only hook, exercised directly by its own tests
        let ok = std::fs::File::open(probe).is_ok();
        println!(
            "{}",
            if ok {
                "PROBE_READ_OK"
            } else {
                "PROBE_READ_DENIED"
            }
        );
        return Ok(());
    }
    if let Some(probe) = args.landlock_probe_write.as_deref() {
        //mutants::skip - test-only hook, exercised directly by its own tests
        let ok = std::fs::File::create(probe).is_ok();
        println!(
            "{}",
            if ok {
                "PROBE_WRITE_OK"
            } else {
                "PROBE_WRITE_DENIED"
            }
        );
        return Ok(());
    }

    // Peek mode only wants the first few decoded bytes, so cap the chunk size
    // (don't inflate a multi-MiB chunk for a 4-byte answer) and decode serially.
    let peek_bytes = args.peek_bytes;
    let chunk_size = match peek_bytes {
        Some(_) => args.chunk_size.min(PEEK_CHUNK_SIZE),
        None => args.chunk_size,
    };
    let threads = if peek_bytes.is_some() {
        1
    } else {
        args.threads
    };
    let verbose = args.verbose;
    let producer = match args.format {
        Format::Gzip => {
            let cfg = Config {
                num_threads: threads,
                chunk_size_bytes: chunk_size,
                verbose: if verbose {
                    Verbosity::On // cov:excl-line
                } else {
                    Verbosity::Off
                },
                recycle_rx: Some(recycle_rx),
                recycle_tx: Some(recycle_tx.clone()),
            };
            std::thread::spawn(move || read_gz(&input, tx, cfg).map(|_| ()).map_err(Into::into))
        }
        Format::Zstd => std::thread::spawn(move || read_zstd(&input, &tx, &recycle_rx, chunk_size)),
    };

    match args.shm_fd {
        Some(fd) => {
            let slots = args
                .shm_slots
                .context("--shm-slots is required together with --shm-fd")?;
            let slot_size = args
                .shm_slot_size
                .context("--shm-slot-size is required together with --shm-fd")?;
            run_shm(fd, slots, slot_size, &rx, &recycle_tx, producer)?;
            drop(recycle_tx);
        }
        None if peek_bytes.is_some() => {
            //mutants::skip - running the full pipe is slow, but correct
            // Emits the head then returns, leaving the producer unjoined if it is
            // still decoding the rest (which we don't need).
            run_peek(&rx, peek_bytes.unwrap_or(0), producer)?;
        }
        None => {
            run_pipe(&rx, &recycle_tx, args.verbose)?;
            drop(recycle_tx);
            producer.join().expect("producer thread panicked")?;
        }
    }
    Ok(())
}

fn read_zstd(
    input: &std::path::Path,
    tx: &Sender<Arc<Vec<u8>>>,
    recycle_rx: &Receiver<Vec<u8>>,
    chunk_size: usize,
) -> Result<()> {
    let file = std::fs::File::open(input)
        .with_context(|| format!("opening {} for zstd decode", input.display()))?;
    let mut decoder =
        zstd::stream::read::Decoder::new(file).context("initializing zstd decoder")?;
    loop {
        let mut buf = recycle_rx.try_recv().unwrap_or_default();
        buf.clear();
        buf.resize(chunk_size, 0);
        let mut filled = 0usize;
        while filled < chunk_size {
            //mutants::skip - not observable but in ram usage.
            let n = decoder.read(&mut buf[filled..])?;
            if n == 0 {
                break;
            }
            filled += n;
        }
        if filled == 0 {
            break;
        }
        buf.truncate(filled);
        if tx.send(Arc::new(buf)).is_err() {
            break;
        }
        if filled < chunk_size {
            //mutants::skip - early exit, otherwise the next read would get 0.
            break;
        }
    }
    Ok(())
}

//cov:excl-start
#[expect(
    clippy::cast_precision_loss,
    reason = "MB/s is a human-facing diagnostic; f64 precision loss on byte counts is irrelevant"
)]
#[mutants::skip]
fn report_throughput(bytes_since_last: u64, total_bytes: u64, elapsed_secs: f64, since_start: f64) {
    let mbps = bytes_since_last as f64 / elapsed_secs / (1024.0 * 1024.0);
    let avg = total_bytes as f64 / since_start / (1024.0 * 1024.0);
    eprintln!(
        "[decompressor +{:.2}s] {:.1} MB/s (avg {:.1} MB/s, {:.1} MB total)",
        elapsed_since_start(),
        mbps,
        avg,
        total_bytes as f64 / (1024.0 * 1024.0),
    );
}
//cov:excl-stop

/// Chunk size used in peek mode: large enough that the first decoded chunk always
/// covers the format-discriminating head, small enough to be cheap to inflate.
// changing this constant doesn't affect correctness, only performance
const PEEK_CHUNK_SIZE: usize = 64 * 1024; //mutants:skip

/// Peek mode: write at most `n` decoded bytes to stdout, then stop without
/// decoding the rest of the input. Format detection only needs the first few
/// decoded bytes; returning as soon as we have them (rather than running to EOF or
/// waiting to be killed) lets the parent reap us cleanly — which also lets a
/// coverage build's profiling runtime flush its `.profraw` on the way out.
///
/// If the stream ends before we fill the head, the producer has finished, so we
/// join it and surface any decode error (e.g. a truncated gzip) just as the
/// full-read path does — otherwise detection would silently treat a broken input
/// as empty and the error would only surface later, mid-read.
fn run_peek(
    rx: &Receiver<Arc<Vec<u8>>>,
    n: usize,
    producer: std::thread::JoinHandle<Result<()>>,
) -> Result<()> {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut written = 0usize;
    while written <= n {
        //makes no difference whether we output exactly n,
        //or n -1 bytes, since both are above what the consumer needs
        let Ok(chunk) = rx.recv() else {
            // Channel disconnected: every sender dropped, so the producer is done
            // and joining it won't block. Surface its error, if any.
            let _ = out.flush();
            return producer.join().expect("producer thread panicked");
        };
        let take = (n.checked_sub(written).expect("written > n?")).min(chunk.len());
        if out.write_all(&chunk[..take]).is_err() {
            // consumer already has what it needs
            break; // cov:excl-line
        }
        written = written.saturating_add(take);
    }
    // Head delivered. The producer may still be mid-decode; we deliberately leave
    // it unjoined (detached at process exit) rather than wait out the rest.
    let _ = out.flush();
    Ok(())
}

fn run_pipe(
    rx: &Receiver<Arc<Vec<u8>>>,
    recycle_tx: &Sender<Vec<u8>>,
    verbose: bool,
) -> Result<()> {
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let start = std::time::Instant::now();
    let mut last_report = start;
    let mut bytes_since_last: u64 = 0;
    let mut total_bytes: u64 = 0;
    for chunk in rx {
        if let Err(e) = out.write_all(&chunk) {
            if is_consumer_gone(&e) {
                return Ok(());
            } // cov:excl-line
            return Err(e.into()); // cov:excl-line
        }
        bytes_since_last += chunk.len() as u64; //mutants::skip
        total_bytes += chunk.len() as u64; //mutants::skip
        if verbose {
            //cov:excl-start
            let now = std::time::Instant::now();
            let elapsed = now.duration_since(last_report);
            if elapsed.as_secs_f64() >= 1.0 {
                //mutants::skip
                report_throughput(
                    bytes_since_last,
                    total_bytes,
                    elapsed.as_secs_f64(),
                    now.duration_since(start).as_secs_f64(),
                );
                last_report = now;
                bytes_since_last = 0;
            }
            //cov:excl-stop
        }
        if let Ok(v) = Arc::try_unwrap(chunk) {
            let _ = recycle_tx.try_send(v);
        }
    }
    Ok(())
}

#[cfg(unix)]
fn run_shm(
    fd: i32,
    slots: usize,
    slot_size: usize,
    rx: &Receiver<Arc<Vec<u8>>>,
    recycle_tx: &Sender<Vec<u8>>,
    producer: std::thread::JoinHandle<Result<()>>,
) -> Result<()> {
    use std::io::Read as _;

    let total = slots
        .checked_mul(slot_size)
        .context("shared-memory region size overflow")?;
    // SAFETY: `fd` is the memfd the parent created and we inherited; mapping it
    // `MAP_SHARED` makes our writes visible to the parent. The mapping lives for
    // the rest of the process, longer than any descriptor we emit for it.
    let base = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            total,
            libc::PROT_READ | libc::PROT_WRITE, //mutants::skip
            libc::MAP_SHARED,
            fd,
            0,
        )
    };
    if base == libc::MAP_FAILED {
        //cov:excl-start
        bail!(
            "mmap of shared-memory region failed: {}",
            std::io::Error::last_os_error()
        );
        //cov:excl-stop
    }
    let base = base.cast::<u8>();

    let (free_tx, free_rx) = crossbeam::channel::unbounded::<u32>();
    for s in 0..slots {
        free_tx
            .send(u32::try_from(s).expect("slot count fits in u32"))
            .expect("receiver is live");
    }

    let stdin_reader = std::thread::spawn(move || {
        let mut stdin = std::io::stdin().lock();
        let mut buf = [0u8; 4];
        while stdin.read_exact(&mut buf).is_ok() {
            if free_tx.send(u32::from_le_bytes(buf)).is_err() {
                break;
            }
        }
    });

    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    for chunk in rx {
        let mut off = 0usize;
        while off < chunk.len() {
            let n = (chunk.len() - off).min(slot_size);
            let slot = free_rx.recv().expect("free pool stays open until EOF");
            // SAFETY: `slot < slots` (only valid ids ever enter the pool) and
            // `n <= slot_size`, so the destination range is inside the mapping.
            // We own this slot exclusively until its descriptor is sent below.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    chunk.as_ptr().add(off),
                    base.add(slot as usize * slot_size),
                    n,
                );
            }
            if let Err(e) = write_descriptor(
                &mut out,
                slot,
                u32::try_from(n).expect("n <= slot_size, a usize that fit a slot"),
            ) {
                if is_consumer_gone(&e) {
                    return Ok(());
                } // cov:excl-line
                return Err(e.into()); // cov:excl-line
            }
            off += n;
        }
        if let Ok(v) = Arc::try_unwrap(chunk) {
            let _ = recycle_tx.try_send(v);
        }
    }
    producer.join().expect("producer thread panicked")?;

    if let Err(e) = write_descriptor(&mut out, u32::MAX, 0) {
        //cov:ignore-start
        if is_consumer_gone(&e) {
            return Ok(());
        }
        return Err(e.into());
        //cov:ignore-stop
    }
    out.flush()?;
    drop(stdin_reader);
    Ok(())
}

#[cfg(not(unix))]
#[mutants::skip] // shared-memory output is only supported on Unix
fn run_shm(
    _fd: i32,
    _slots: usize,
    _slot_size: usize,
    _rx: &Receiver<Arc<Vec<u8>>>,
    _recycle_tx: &Sender<Vec<u8>>,
    _producer: std::thread::JoinHandle<Result<()>>,
) -> Result<()> {
    bail!("shared-memory output is only supported on Unix");
}

fn write_descriptor(out: &mut impl Write, slot: u32, len: u32) -> std::io::Result<()> {
    let mut desc = [0u8; 8];
    desc[0..4].copy_from_slice(&slot.to_le_bytes());
    desc[4..8].copy_from_slice(&len.to_le_bytes());
    out.write_all(&desc)?;
    out.flush()
}

#[cfg(target_os = "linux")]
#[mutants::skip] // landlock is best effort.
fn apply_landlock(input: &std::path::Path) -> Result<()> {
    use landlock::{
        ABI, Access, AccessFs, BitFlags, PathBeneath, PathFd, Ruleset, RulesetAttr, RulesetCreated,
        RulesetCreatedAttr,
    };

    let abi = ABI::V1;

    // Under coverage instrumentation (cargo-llvm-cov sets LLVM_PROFILE_FILE), the
    // LLVM profiling runtime reads and writes a .profraw on exit (the default
    // `%m` online-merge pattern opens the existing file to merge counters
    // across processes, then rewrites it) — both outside the input path, which
    // this sandbox would otherwise deny ("Permission denied", leaking onto
    // stderr and breaking tests). Allow access beneath the profile directory
    // when present, under whichever rights `ruleset` handles.
    fn allow_profile_dir(
        mut ruleset: RulesetCreated,
        access: BitFlags<AccessFs>,
    ) -> Result<RulesetCreated> {
        if let Some(profile) = std::env::var_os("LLVM_PROFILE_FILE")
            && let Some(dir) = std::path::Path::new(&profile).parent()
            && let Ok(fd) = PathFd::new(dir)
        {
            ruleset = ruleset.add_rule(PathBeneath::new(fd, access))?;
        } // cov:excl-line
        Ok(ruleset)
    }

    // Strict form: deny every read except `input`, and every write anywhere.
    // Building the `input` rule fails (EBADFD) when `input` doesn't resolve to
    // a real filesystem path Landlock can bind a rule to — notably
    // `/dev/stdin` when stdin is a pipe rather than a regular file, since pipes
    // live on `pipefs`, not on any mount hierarchy Landlock can restrict.
    let strict = (|| -> Result<()> {
        let access = AccessFs::from_all(abi);
        let ruleset = Ruleset::default()
            .handle_access(access)?
            .create()?
            .add_rule(PathBeneath::new(
                PathFd::new(input)?,
                AccessFs::from_read(abi),
            ))?; // cov:excl-line
        allow_profile_dir(ruleset, access)?.restrict_self()?;
        Ok(())
    })();
    if strict.is_ok() {
        return strict;
    }

    // Fall back to denying every write, leaving reads unrestricted. Still real
    // containment — a compromised decoder can no longer write, create,
    // delete, or rename anything on disk — for inputs (like a piped stdin)
    // that can't be scoped down for reads.
    let access = AccessFs::from_write(abi);
    let ruleset = Ruleset::default().handle_access(access)?.create()?;
    allow_profile_dir(ruleset, access)?.restrict_self()?;
    Ok(())
}

#[mutants::skip]
fn is_consumer_gone(e: &std::io::Error) -> bool {
    e.kind() == std::io::ErrorKind::BrokenPipe
}
