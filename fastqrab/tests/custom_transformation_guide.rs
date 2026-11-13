//! Replays the 'implement your own transformation' guide
//! (`docs/content/docs/development/custom_transformation.md`) in a scratch copy
//! of the workspace, so the guide can not silently drift from the code.
//!
//! The guide is annotated with hidden `<!-- guide-test: <action> [args] -->`
//! comments. Actions that consume the code block directly following the marker:
//!
//! - `write <path>`   - write the block to `<path>` (relative to the workspace root)
//! - `append <path>`  - append the block to `<path>`
//! - `excerpt <path>` - assert the block's lines occur in `<path>`, in order
//!   (`...` lines are skipped)
//! - `insert <path>`  - like excerpt, but lines missing from `<path>` are inserted
//!   after the previously matched line
//! - `run`            - run the block with bash in the workspace root
//!
//! Stand alone actions:
//!
//! - `check` / `build`           - build the fastqrab binary, no warnings in files the guide wrote
//!   (a build, not `cargo check`, which would compile everything a second time)
//! - `verify <dir>`              - `fastqrab verify` must pass in `<dir>`
//! - `verify-fails <dir> <text>` - `fastqrab verify` must fail, with `<text>` in its output
//!
//! Every `rust` code block in the guide must be annotated.
//!
//! When editing the guide, run with `GUIDE_TEST_PRINT=1` (and `--nocapture`) to see the
//! actual output of every `verify-fails` instead of stopping at the first mismatch.
//!
//! This builds fastqrab (repeatedly) in its own target directory, so it's
//! `#[ignore]`d for the regular `cargo test` - CI (`nix build .#test`) runs it
//! explicitly via `cargo test --test custom_transformation_guide -- --ignored`.
#![cfg(unix)]

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const GUIDE: &str = "../docs/content/docs/development/custom_transformation.md";
const MARKER: &str = "<!-- guide-test:";

/// What we copy of the workspace - enough to build fastqrab and run the guide's test cases.
const WORKSPACE_PARTS: &[&str] = &[
    "Cargo.toml",
    "Cargo.lock",
    ".cargo",
    "README.md",
    "LICENSE",
    "fastqrab",
    "fastqrab-config",
    "fastqrab-dna",
    "fastqrab-io",
    "fastqrab-steps",
    "test_cases/sample_data",
];

#[derive(Debug)]
struct Instruction {
    line_no: usize,
    action: String,
    args: Vec<String>,
    block: Option<String>,
}

fn needs_block(action: &str) -> bool {
    matches!(action, "write" | "append" | "excerpt" | "insert" | "run")
}

fn parse_guide(guide: &str) -> Vec<Instruction> {
    let lines: Vec<&str> = guide.lines().collect();
    let mut res = Vec::new();
    let mut unannotated_rust = Vec::new();
    let mut ii = 0;
    let mut in_block = false;
    let mut last_was_marker = false;
    while ii < lines.len() {
        let line = lines[ii];
        let trimmed = line.trim();
        if !in_block && let Some(rest) = trimmed.strip_prefix(MARKER) {
            let rest = rest
                .strip_suffix("-->")
                .unwrap_or_else(|| panic!("guide line {}: marker must end with -->", ii + 1));
            let mut parts = rest.split_whitespace().map(ToString::to_string);
            let action = parts
                .next()
                .unwrap_or_else(|| panic!("guide line {}: marker without action", ii + 1));
            let args: Vec<String> = parts.collect();
            let mut block = None;
            if needs_block(&action) {
                let fence = lines.get(ii + 1).map(|x| x.trim()).unwrap_or_default();
                assert!(
                    fence.starts_with("```"),
                    "guide line {}: action '{action}' must be followed by a code block",
                    ii + 1
                );
                let mut content = String::new();
                let mut jj = ii + 2;
                while lines.get(jj).is_some_and(|x| x.trim() != "```") {
                    content.push_str(lines[jj]);
                    content.push('\n');
                    jj += 1;
                }
                assert!(
                    jj < lines.len(),
                    "guide line {}: unterminated code block",
                    ii + 1
                );
                block = Some(content);
                ii = jj; // skip past the block
            }
            res.push(Instruction {
                line_no: ii + 1,
                action,
                args,
                block,
            });
            last_was_marker = true;
            ii += 1;
            continue;
        }
        if trimmed.starts_with("```") {
            if !in_block && trimmed == "```rust" && !last_was_marker {
                unannotated_rust.push(ii + 1);
            }
            in_block = !in_block;
        }
        if !trimmed.is_empty() {
            last_was_marker = false;
        }
        ii += 1;
    }
    assert!(
        unannotated_rust.is_empty(),
        "rust code blocks in the guide without a <!-- guide-test: ... --> annotation at lines {unannotated_rust:?}"
    );
    res
}

fn copy_workspace(src: &Path, dst: &Path) {
    for part in WORKSPACE_PARTS {
        let target = dst.join(part);
        fs::create_dir_all(target.parent().expect("no parent dir")).expect("failed to create dir");
        let status = Command::new("cp")
            .arg("-a")
            .arg(src.join(part))
            .arg(&target)
            .status()
            .expect("failed to run cp");
        assert!(status.success(), "failed to copy {part}");
    }
}

/// Match the (trimmed, non '...') lines of `excerpt` in order against `path`.
/// If `insert`, lines not found are inserted after the last matched line,
/// otherwise a missing line is an error.
fn apply_excerpt(path: &Path, excerpt: &str, insert: bool, line_no: usize) {
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "guide line {line_no}: failed to read {}: {e}",
            path.display()
        )
    });
    let mut file_lines: Vec<String> = content.lines().map(ToString::to_string).collect();
    let mut cursor = 0;
    for wanted in excerpt.lines() {
        let wanted_trimmed = wanted.trim();
        if wanted_trimmed.is_empty() || wanted_trimmed == "..." {
            continue;
        }
        if let Some(pos) = file_lines[cursor..]
            .iter()
            .position(|x| x.trim() == wanted_trimmed)
        {
            cursor += pos + 1;
        } else if insert {
            file_lines.insert(cursor, wanted.to_string());
            cursor += 1;
        } else {
            panic!(
                "guide line {line_no}: excerpt line '{wanted_trimmed}' not found (in order) in {}",
                path.display()
            );
        }
    }
    if insert {
        fs::write(path, file_lines.join("\n") + "\n").expect("failed to write excerpt target");
    }
}

struct Env {
    workspace: PathBuf,
    target_dir: PathBuf,
    release: bool,
    /// files the guide wrote - warnings in those are errors
    written: Vec<String>,
}

impl Env {
    fn cargo(&self, args: &[&str]) -> Output {
        let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
        let mut cmd = Command::new(cargo);
        cmd.args(args)
            .arg("--offline")
            .arg("--target-dir")
            .arg(&self.target_dir)
            .current_dir(&self.workspace)
            .env_remove("CARGO_TARGET_DIR");
        if self.release {
            cmd.arg("--release");
        }
        cmd.output().expect("failed to run cargo")
    }

    fn cargo_ok(&self, args: &[&str], line_no: usize) {
        let out = self.cargo(args);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(
            out.status.success(),
            "guide line {line_no}: cargo {args:?} failed:\n{stderr}"
        );
        for written in &self.written {
            if Path::new(written)
                .extension()
                .is_some_and(|ext| ext == "rs")
            {
                assert!(
                    !stderr.contains(&format!("--> {written}")),
                    "guide line {line_no}: cargo {args:?} produced warnings in {written}:\n{stderr}"
                );
            }
        }
    }

    fn fastqrab(&self) -> PathBuf {
        self.target_dir
            .join(if self.release { "release" } else { "debug" })
            .join("fastqrab")
    }

    fn verify(&self, dir: &str) -> (bool, String) {
        let out = Command::new(self.fastqrab())
            .arg("verify")
            .current_dir(self.workspace.join(dir))
            .output()
            .expect("failed to run fastqrab verify");
        let mut text = String::from_utf8_lossy(&out.stdout).to_string();
        text.push_str(&String::from_utf8_lossy(&out.stderr));
        (out.status.success(), text)
    }
}

#[test]
#[ignore = "builds fastqrab several times - run explicitly (CI does)"]
fn custom_transformation_guide_works() {
    let guide = fs::read_to_string(GUIDE).expect("failed to read the guide");
    let instructions = parse_guide(&guide);
    assert!(
        instructions.iter().any(|x| x.action == "verify"),
        "the guide verifies nothing?"
    );

    let src = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let td = tempfile::tempdir().unwrap();
    let workspace = td.path().join("workspace");
    copy_workspace(src, &workspace);

    let mut env = Env {
        workspace,
        // persistent, so external dependencies are only built once
        target_dir: Path::new(env!("CARGO_TARGET_TMPDIR")).join("custom_transformation_guide"),
        release: !cfg!(debug_assertions),
        written: Vec::new(),
    };

    let mut log = String::new();
    for inst in &instructions {
        let line_no = inst.line_no;
        let arg = |ii: usize| -> &str {
            inst.args.get(ii).unwrap_or_else(|| {
                panic!(
                    "guide line {line_no}: action '{}' needs an argument",
                    inst.action
                )
            })
        };
        writeln!(
            log,
            "line {line_no}: {} {}",
            inst.action,
            inst.args.join(" ")
        )
        .unwrap();
        match inst.action.as_str() {
            "write" | "append" => {
                let path = env.workspace.join(arg(0));
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                let block = inst.block.as_ref().unwrap();
                if inst.action == "write" {
                    fs::write(&path, block).unwrap();
                } else {
                    let mut content = fs::read_to_string(&path).unwrap_or_default();
                    content.push('\n');
                    content.push_str(block);
                    fs::write(&path, content).unwrap();
                }
                env.written.push(arg(0).to_string());
            }
            "excerpt" | "insert" => apply_excerpt(
                &env.workspace.join(arg(0)),
                inst.block.as_ref().unwrap(),
                inst.action == "insert",
                line_no,
            ),
            "run" => {
                let out = Command::new("bash")
                    .arg("-euo")
                    .arg("pipefail")
                    .arg("-c")
                    .arg(inst.block.as_ref().unwrap())
                    .current_dir(&env.workspace)
                    .output()
                    .expect("failed to run bash");
                assert!(
                    out.status.success(),
                    "guide line {line_no}: shell block failed:\n{}{}",
                    String::from_utf8_lossy(&out.stdout),
                    String::from_utf8_lossy(&out.stderr)
                );
            }
            "check" | "build" => {
                env.cargo_ok(&["build", "-p", "fastqrab", "--bin", "fastqrab"], line_no);
            }
            "verify" => {
                let (ok, text) = env.verify(arg(0));
                assert!(
                    ok,
                    "guide line {line_no}: verify {} failed:\n{text}\n\nsteps so far:\n{log}",
                    arg(0)
                );
            }
            "verify-fails" => {
                let expected = inst.args[1..].join(" ");
                assert!(
                    !expected.is_empty(),
                    "guide line {line_no}: verify-fails needs <dir> <expected text>"
                );
                let (ok, text) = env.verify(arg(0));
                if std::env::var("GUIDE_TEST_PRINT").is_ok() {
                    // when editing the guide: show the actual output instead of failing
                    println!(
                        "=== guide line {line_no}: verify {} (ok={ok})\n{text}",
                        arg(0)
                    );
                    continue;
                }
                assert!(
                    !ok,
                    "guide line {line_no}: verify {} should have failed, but passed\n\nsteps so far:\n{log}",
                    arg(0)
                );
                assert!(
                    text.contains(&expected),
                    "guide line {line_no}: verify {} failed, but output did not contain '{expected}':\n{text}",
                    arg(0)
                );
            }
            other => panic!("guide line {line_no}: unknown guide-test action '{other}'"),
        }
    }
}
