//! Verifies the `__decompressor` subprocess's landlock sandbox
//! (`fastqrab::decompressor::apply_landlock`) actually confines it: reads of
//! any path other than the input are denied, and *all* writes (anywhere,
//! including beside the input) are denied. The normal decode path never
//! attempts either of those operations, so it can't exercise the sandbox on
//! its own — these tests drive the hidden `--landlock-probe-read` /
//! `--landlock-probe-write` flags, which exist solely for this purpose.
//!
//! Linux-only: landlock is a Linux-specific syscall API, and `apply_landlock`
//! is `#[cfg(target_os = "linux")]`.

#![cfg(target_os = "linux")]

use std::io::Write as _;
use std::process::{Command, Stdio};

#[path = "common/mod.rs"]
mod common;

/// Runs `__decompressor <input> --landlock-probe-{read,write} <target>` and
/// returns the trimmed `PROBE_*` line it printed. `stdin` feeds the child's
/// stdin when `input` is `-`; pass `None` when `input` is a real file.
fn probe(
    flag: &str,
    input: &std::path::Path,
    target: &std::path::Path,
    stdin: Option<&[u8]>,
) -> String {
    let mut child = Command::new(common::decompressor())
        .arg("__decompressor")
        .arg(input)
        .arg(flag)
        .arg(target)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(data) = stdin {
        child.stdin.take().unwrap().write_all(data).unwrap();
    }

    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "probe process exited non-zero: {:?}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stderr), // cov:excl-line
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

#[test]
fn landlock_allows_reading_the_input_file_itself() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("f.gz");
    std::fs::write(&input, b"whatever, never actually decoded in probe mode").unwrap();

    assert_eq!(
        probe("--landlock-probe-read", &input, &input, None),
        "PROBE_READ_OK"
    );
}

#[test]
fn landlock_denies_reading_a_sibling_file() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("f.gz");
    let secret = dir.path().join("secret.txt");
    std::fs::write(&input, b"whatever, never actually decoded in probe mode").unwrap();
    std::fs::write(&secret, b"top secret").unwrap();

    assert_eq!(
        probe("--landlock-probe-read", &input, &secret, None),
        "PROBE_READ_DENIED"
    );
}

#[test]
fn landlock_denies_all_writes_including_beside_the_input_file() {
    let dir = tempfile::tempdir().unwrap();
    let input = dir.path().join("f.gz");
    std::fs::write(&input, b"whatever, never actually decoded in probe mode").unwrap();

    // A brand-new file next to the input, and the input file itself: both are
    // writes, so both must be denied (the sandbox only ever grants read).
    let new_file = dir.path().join("new.txt");
    assert_eq!(
        probe("--landlock-probe-write", &input, &new_file, None),
        "PROBE_WRITE_DENIED"
    );
    assert_eq!(
        probe("--landlock-probe-write", &input, &input, None),
        "PROBE_WRITE_DENIED"
    );
}

/// When stdin is a regular file-like descriptor (here, `/dev/null` via
/// `Stdio::null()`), `/dev/stdin` resolves to something Landlock *can* bind a
/// rule to, so the same strict, read-scoped-to-input enforcement applies as
/// for a real file argument.
#[test]
fn landlock_denies_reads_beyond_dev_stdin_when_stdin_is_not_a_pipe() {
    let dir = tempfile::tempdir().unwrap();
    let secret = dir.path().join("secret.txt");
    std::fs::write(&secret, b"top secret").unwrap();

    assert_eq!(
        probe(
            "--landlock-probe-read",
            std::path::Path::new("-"),
            &secret,
            None
        ),
        "PROBE_READ_DENIED"
    );
}

/// When stdin is an actual anonymous pipe (the common `cat f.gz | fastqrab`
/// case), `/dev/stdin` resolves through `pipefs`, which Landlock can't bind a
/// `PathBeneath` rule to (the kernel rejects it with EBADFD). `apply_landlock`
/// detects that and falls back to enforcing write-only containment: reads are
/// left unrestricted (unavoidable — the decoder must still be able to reopen
/// `/dev/stdin` to read the pipe), but writes anywhere are still denied.
#[test]
fn landlock_falls_back_to_write_only_enforcement_when_stdin_is_a_pipe() {
    let dir = tempfile::tempdir().unwrap();
    let secret = dir.path().join("secret.txt");
    std::fs::write(&secret, b"top secret").unwrap();

    assert_eq!(
        probe(
            "--landlock-probe-read",
            std::path::Path::new("-"),
            &secret,
            Some(b"unused, probe mode exits before decoding stdin")
        ),
        "PROBE_READ_OK",
        "reads can't be scoped down for a piped stdin, so they stay open"
    );

    let new_file = dir.path().join("new.txt");
    assert_eq!(
        probe(
            "--landlock-probe-write",
            std::path::Path::new("-"),
            &new_file,
            Some(b"unused, probe mode exits before decoding stdin")
        ),
        "PROBE_WRITE_DENIED",
        "writes must still be denied even when stdin is a pipe"
    );
}
