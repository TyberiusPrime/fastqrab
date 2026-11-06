#![expect(clippy::unwrap_used, reason = "It's tests")]

use bstr::{BString, ByteSlice};
use indexmap::IndexMap;
use std::collections::HashSet;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Stdio;

#[path = "common/mod.rs"]
mod common;

#[test]
fn test_cookbooks_in_sync() {
    // Verify that the generated cookbooks.rs matches the actual cookbook directories

    // Get cookbooks from generated code
    let generated_cookbooks: HashSet<String> = fastqrab::cookbooks::list_cookbooks()
        .iter()
        .map(|(_, name)| (*name).to_string())
        .collect();

    // Get cookbooks from filesystem
    let cookbooks_dir = Path::new("../cookbooks");
    assert!(cookbooks_dir.exists(), "cookbooks directory should exist");

    //contents always matches since they"re include_str!()ed

    let mut fs_cookbooks = HashSet::new();
    let entries = std::fs::read_dir(cookbooks_dir).unwrap();

    for entry in entries.flatten() {
        if entry.path().is_dir() {
            let input_toml = entry.path().join("input.toml");
            if input_toml.exists()
                && let Some(name) = entry.file_name().to_str()
            {
                fs_cookbooks.insert(name.to_string());
            }
        }
    }

    // Check that they match
    let missing_in_generated: Vec<_> = fs_cookbooks.difference(&generated_cookbooks).collect();
    let extra_in_generated: Vec<_> = generated_cookbooks.difference(&fs_cookbooks).collect();

    if !missing_in_generated.is_empty() || !extra_in_generated.is_empty() {
        //cov:excl-start
        eprintln!("\n❌ Cookbook synchronization mismatch!");
        if !missing_in_generated.is_empty() {
            eprintln!("  Missing in generated code: {missing_in_generated:?}");
        }
        if !extra_in_generated.is_empty() {
            eprintln!("  Extra in generated code: {extra_in_generated:?}");
        }
        eprintln!("\n  Run: python3 dev/updated_generated.sh");
        panic!("Cookbooks out of sync. Run dev/update_generated.sh to regenerate.");
        //cov:excl-stop
    }
}

fn get_bin_path() -> PathBuf {
    let current_exe = std::env::current_exe().unwrap();
    current_exe
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        //.join("debug")
        .join("fastqrab")
}
#[test]
fn test_usage() {
    let cmd = std::process::Command::new(get_bin_path()).output().unwrap();
    //let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(stderr.contains("Usage:"));
    assert!(!cmd.status.success());
}

#[test]
fn test_process_command() {
    // Test process command without config file - should show error
    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(stderr.contains("Please specify a configuration file explicitly"));
    assert!(!cmd.status.success());
}

#[test]
fn test_process_nonexistent_toml() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg("/nonexistent/path/config.toml")
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(
        stderr.contains("Could not read toml file"),
        "stderr was: {stderr}"
    );
    assert!(!cmd.status.success());
}

#[test]
fn test_verify_nonexistent_toml() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg("/nonexistent/path/config.toml")
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(
        stderr.contains("Failed to canonicalize TOML file path"),
        "stderr was: {stderr}"
    );
    assert!(!cmd.status.success());
}

#[test]
fn test_verify_benchmark_config_error() {
    let dir = tempfile::tempdir().unwrap();
    let toml_path = dir.path().join("bench.toml");
    std::fs::write(
        &toml_path,
        "[input]\nread1 = \"input.fq\"\n\n[benchmark]\nenable = true\nmolecule_count = 1000\n",
    )
    .unwrap();
    let cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&toml_path)
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(
        stderr.contains("benchmarking configuration"),
        "stderr was: {stderr}"
    );
    assert!(!cmd.status.success());
}

#[test]
fn test_validate_nonexistent_toml() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg("/nonexistent/path/config.toml")
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(
        stderr.contains("Could not read toml file"),
        "stderr was: {stderr}"
    );
    assert!(!cmd.status.success());
}

#[test]
fn test_template_command() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("template")
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    // Verify template contains key sections
    assert!(stdout.contains("# fastqrab Configuration Template"));
    assert!(stdout.contains("[input]"));
    assert!(stdout.contains("[output]"));
    assert!(stdout.contains("[[step]]"));
    assert!(cmd.status.success());
}

#[test]
fn test_version_command() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("version")
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    // Verify version output contains version number
    assert!(!stdout.trim().is_empty());
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
    assert!(cmd.status.success());
}

#[test]
fn test_cookbook_command() {
    // Test cookbook list
    let cmd = std::process::Command::new(get_bin_path())
        .arg("cookbook")
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(stdout.contains("Available cookbooks:"));
    assert!(stdout.contains("01-basic-quality-report"));
    assert!(cmd.status.success());

    // Test specific cookbook
    let cmd = std::process::Command::new(get_bin_path())
        .arg("cookbook")
        .arg("1")
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    //assert!(stdout.contains("Cookbook 1:"));
    assert!(stdout.contains("## Configuration"));
    assert!(cmd.status.success());
}

#[test]
fn test_list_steps_command() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("list-steps")
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    dbg!(&stdout);
    assert!(stdout.contains("Available transformation steps:"));
    assert!(stdout.contains("Report"));
    assert!(stdout.contains("Head"));
    assert!(cmd.status.success());
}

#[test]
fn test_version_flag() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("--version")
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    // Verify --version flag produces same output as version command
    assert!(!stdout.trim().is_empty());
    assert!(stdout.contains(env!("CARGO_PKG_VERSION")));
    assert!(cmd.status.success());
}

#[test]
fn test_template_with_input_section() {
    let cmd = std::process::Command::new(get_bin_path())
        .args(["template", "Input"])
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(
        cmd.status.success(),
        "stderr: {}",
        std::str::from_utf8(&cmd.stderr).unwrap()
    );
    assert!(stdout.contains("[input]"), "stdout was: {stdout}");
    // Should not include the full template preamble
    assert!(
        !stdout.contains("# fastqrab Configuration Template"),
        "stdout was: {stdout}"
    );
}

#[test]
fn test_template_with_output_section() {
    let cmd = std::process::Command::new(get_bin_path())
        .args(["template", "Output"])
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(
        cmd.status.success(),
        "stderr: {}",
        std::str::from_utf8(&cmd.stderr).unwrap()
    );
    assert!(stdout.contains("[output]"), "stdout was: {stdout}");
    assert!(stdout.contains("prefix"), "stdout was: {stdout}");
}

#[test]
fn test_template_with_step_section() {
    let cmd = std::process::Command::new(get_bin_path())
        .args(["template", "Head"])
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(
        cmd.status.success(),
        "stderr: {}",
        std::str::from_utf8(&cmd.stderr).unwrap()
    );
    assert!(stdout.contains("Head"), "stdout was: {stdout}");
    assert!(!stdout.is_empty(), "stdout was empty");
}

#[test]
fn test_template_with_options_section_last_in_template() {
    // The Options section is the last section in template.toml, so search_query hits the
    // `else` branch where no subsequent "# =" marker exists (the "no next section" case).
    let cmd = std::process::Command::new(get_bin_path())
        .args(["template", "Options"])
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(
        cmd.status.success(),
        "stderr: {}",
        std::str::from_utf8(&cmd.stderr).unwrap()
    );
    assert!(!stdout.is_empty(), "stdout was empty");
    assert!(stdout.contains("options"), "stdout was: {stdout}");
    // Must not contain any section header that would belong to a subsequent section
    assert!(
        !stdout.contains("# == "),
        "should not include next section: {stdout}"
    );
}

#[test]
fn test_template_nonexistent_section() {
    let cmd = std::process::Command::new(get_bin_path())
        .args(["template", "NoSuchSectionExists"])
        .output()
        .unwrap();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(
        cmd.status.success(),
        "stderr: {}",
        std::str::from_utf8(&cmd.stderr).unwrap()
    );
    assert!(
        stdout.contains("No such documentation found"),
        "stdout was: {stdout}"
    );
}

#[test]
fn test_interactive_no_config_in_empty_dir() {
    let dir = tempfile::tempdir().unwrap();
    let cmd = std::process::Command::new(get_bin_path())
        .arg("interactive")
        .current_dir(dir.path())
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(!cmd.status.success(), "expected failure, stderr: {stderr}");
    assert!(
        stderr.contains("No TOML file found") || stderr.contains("Please specify"),
        "stderr was: {stderr}"
    );
}

#[test]
fn test_interactive_nonexistent_file() {
    let cmd = std::process::Command::new(get_bin_path())
        .args(["interactive", "/nonexistent/path/config.toml"])
        .output()
        .unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(!cmd.status.success(), "expected failure, stderr: {stderr}");
    assert!(
        stderr.contains("canonicalize") || stderr.contains("No such file"),
        "stderr was: {stderr}"
    );
}

#[test]
#[cfg(unix)]
fn test_interactive_processes_file_on_first_run() {
    use std::io::BufRead;
    use std::sync::{Arc, Mutex};
    use std::time::{Duration, Instant};

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("input.fq"),
        b"@r1\nACGT\n+\nIIII\n@r2\nTTTT\n+\nIIII\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("config.toml"),
        "[input]\nread1 = \"input.fq\"\n\n[output]\nprefix = \"out\"\n",
    )
    .unwrap();

    // By setting TMPDIR the interactive process will create its temp dir under `dir`,
    // whose name encodes the child PID — giving us a deterministic path to monitor.
    let mut child = std::process::Command::new(get_bin_path())
        .args(["interactive", "config.toml"])
        .current_dir(dir.path())
        .env("TMPDIR", dir.path())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let child_pid = child.id();
    let inspect_file = dir
        .path()
        .join(format!("fastqrab-interactive-{child_pid}"))
        .join("interactive_output_inspect_interleaved.fq");

    // Stream the child's stdout on a background thread into a shared buffer.
    // The "Processing completed successfully" line is printed only *after* the
    // processing pass has fully finished (and the inspect file has been written
    // and read back), so it — not the mere existence of the inspect file — is the
    // true completion signal. Polling the file would race: the file can appear a
    // moment before the success line is emitted, and killing the child in that
    // window drops the line we assert on.
    let stdout_pipe = child.stdout.take().unwrap();
    let stdout_buf = Arc::new(Mutex::new(String::new()));
    let reader_buf = Arc::clone(&stdout_buf);
    let reader = std::thread::spawn(move || {
        let mut reader = std::io::BufReader::new(stdout_pipe);
        let mut line = String::new();
        while reader.read_line(&mut line).map(|n| n > 0).unwrap_or(false) {
            reader_buf.lock().unwrap().push_str(&line);
            line.clear();
        }
    });

    // Poll until the success line shows up, or time out after 30 s.
    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline
        && !stdout_buf
            .lock()
            .unwrap()
            .contains("Processing completed successfully")
    {
        std::thread::sleep(Duration::from_millis(100));
    }

    child.kill().unwrap();
    child.wait().unwrap();
    reader.join().unwrap();

    assert!(
        inspect_file.exists(),
        "interactive never produced inspect output file"
    );
    let stdout = stdout_buf.lock().unwrap();
    assert!(
        stdout.contains("Interactive mode starting"),
        "stdout: {stdout}"
    );
    assert!(
        stdout.contains("Processing completed successfully"),
        "stdout: {stdout}"
    );
}

#[test]
#[cfg(unix)]
fn test_interactive_ctrl_c_removes_temp_dir() {
    use std::time::{Duration, Instant};

    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("input.fq"),
        b"@r1\nACGT\n+\nIIII\n@r2\nTTTT\n+\nIIII\n",
    )
    .unwrap();
    std::fs::write(
        dir.path().join("config.toml"),
        "[input]\nread1 = \"input.fq\"\n\n[output]\nprefix = \"out\"\n",
    )
    .unwrap();

    // Setting TMPDIR places the child's temp dir under `dir`, with a PID-derived name.
    let mut child = std::process::Command::new(get_bin_path())
        .args(["interactive", "config.toml"])
        .current_dir(dir.path())
        .env("TMPDIR", dir.path())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .unwrap();

    let child_pid = child.id();
    let temp_dir = dir.path().join(format!("fastqrab-interactive-{child_pid}"));

    let deadline = Instant::now() + Duration::from_secs(30);
    while Instant::now() < deadline && !temp_dir.exists() {
        std::thread::sleep(Duration::from_millis(50)); // cov:ignore-line
    }
    assert!(
        temp_dir.exists(),
        "interactive never created temp dir at {}",
        temp_dir.display() // cov:excl-line
    );

    // Send SIGINT, mimicking a Ctrl+C from the user's terminal.
    let kill_status = std::process::Command::new("kill")
        .arg("-INT")
        .arg(child_pid.to_string())
        .status()
        .unwrap();
    assert!(kill_status.success(), "kill -INT failed");

    child.wait().unwrap();

    assert!(
        !temp_dir.exists(),
        "temp dir not cleaned up after Ctrl+C: {}",
        temp_dir.display() // cov:excl-line
    );
}

fn scan_dir(dir: &Path, files: &mut HashSet<std::path::PathBuf>) {
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            scan_dir(&path, files);
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs")
            && let Ok(content) = fs::read_to_string(&path)
        {
            // Check if file contains DemultiplexedData field declarations
            // but skip if it's only imports/uses
            let has_demux_field = content.lines().any(|line| {
                let trimmed = line.trim();
                trimmed.contains("DemultiplexedData<")
                    && !trimmed.contains("use ")
                    && !trimmed.starts_with("//")
                    && (trimmed.contains("pub ")
                        || trimmed.contains(": ")
                        || trimmed.ends_with("DemultiplexedData,"))
            });

            if has_demux_field {
                files.insert(path);
            }
        } // cov:excl-line
    }
}

fn find_toml_files(dir: &Path, files: &mut Vec<std::path::PathBuf>) {
    let entries = std::fs::read_dir(dir).unwrap();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            find_toml_files(&path, files);
        } else if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            files.push(path);
        }
    }
}
#[test]
#[expect(clippy::too_many_lines, reason = "it's a test")]
fn test_every_demultiplexed_data_transform_has_test() {
    // This test verifies that every transformation that uses DemultiplexedData
    // has at least one test case where it occurs after a Demultiplex step.
    // The list of transforms is automatically discovered by scanning the source code.

    // Step 1: Find all Rust files containing DemultiplexedData field declarations
    let mut files_with_demux = HashSet::new();

    scan_dir(
        Path::new("../fastqrab-steps/src/transformations"),
        &mut files_with_demux,
    );

    // Step 2: Extract public struct names from these files (excluding internal ones)
    let mut struct_names = HashSet::new();
    for file_path in &files_with_demux {
        let content = fs::read_to_string(file_path).unwrap();
        for line in content.lines() {
            if line.contains("pub struct")
                && !line.contains("pub(crate)")
                && let Some(struct_part) = line.split("pub struct").nth(1)
            {
                // Extract the name - it's the first word after "pub struct"
                let name = struct_part
                    .trim()
                    .split(|c: char| c == '{' || c == '<' || c.is_whitespace())
                    .find(|s| !s.is_empty())
                    .unwrap_or("")
                    .to_string();

                // Skip internal structs (starting with _)
                if !name.is_empty() && !name.starts_with('_') {
                    struct_names.insert(name);
                }
            }
        }
    }

    // Step 3: Parse Transformation enum to map struct names to action names
    let transformations_path = Path::new("../fastqrab-steps/src/transformations.rs");
    let transformations_content = fs::read_to_string(transformations_path)
        .expect("Failed to read ../fastqrab-steps/src/transformations.rs");

    let mut struct_to_action: IndexMap<String, String> = IndexMap::new();

    // Find the enum definition and parse it
    let mut in_enum = false;
    for line in transformations_content.lines() {
        if line.contains("pub enum Transformation") {
            in_enum = true;
            continue;
        }

        if in_enum {
            if line.trim() == "}" {
                break;
            }

            // Skip lines with #[tpd(skip)] or comments
            if line.contains("#[tpd(skip)]") || line.trim().starts_with("//") {
                continue;
            }
            if line.trim().is_empty() {
                continue;
            }

            // Parse enum variants: ActionName(module::path::StructName)
            if let Some(variant) = line.trim().strip_suffix(',').or(Some(line.trim()))
                && let Some((action_name, struct_path)) = variant.split_once('(')
            {
                let action_name = action_name.trim();
                let struct_path = struct_path.trim_end_matches(')').trim();

                // Extract just the struct name from the path
                if let Some(struct_name) = struct_path.split("::").last() {
                    // Handle Box<...> wrapper
                    let struct_name = struct_name.trim_start_matches("Box<").trim_end_matches('>');

                    if struct_names.contains(struct_name) {
                        struct_to_action.insert(struct_name.to_string(), action_name.to_string());
                    }
                } else {
                    panic!("no :: in struct name!?");
                }
            } else {
                panic!("enum variant parsing failure variant: line: {line}");
            }
        }
    }

    // Get the set of action names that use DemultiplexedData
    let transforms_with_demultiplexed_data: HashSet<String> =
        struct_to_action.values().cloned().collect();

    assert!(
        !transforms_with_demultiplexed_data.is_empty(),
        "No transforms with DemultiplexedData found - this is likely a bug in the test"
    );

    // Step 4: Find all test TOML files
    let test_cases_dir = Path::new("../test_cases");
    assert!(test_cases_dir.exists(), "test_cases directory should exist");
    let mut toml_files = Vec::new();

    find_toml_files(test_cases_dir, &mut toml_files);

    // Step 5: Track which transforms have tests after Demultiplex
    let mut tested_transforms = HashSet::new();

    // Check each TOML file for Demultiplex followed by our transforms
    for toml_path in &toml_files {
        let content = std::fs::read_to_string(toml_path).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        let mut found_demultiplex = false;

        for line in lines {
            let trimmed = line.trim();

            // Check for Demultiplex action
            if trimmed.contains("action")
                && (trimmed.contains("'Demultiplex'") || trimmed.contains("\"Demultiplex\""))
            {
                found_demultiplex = true;
            }

            // If we've seen a Demultiplex, check for our transforms
            if found_demultiplex && trimmed.contains("action") {
                for transform in &transforms_with_demultiplexed_data {
                    if trimmed.contains(&format!("'{transform}'"))
                        || trimmed.contains(&format!("\"{transform}\""))
                    {
                        tested_transforms.insert(transform.clone());
                        println!(
                            "✓ Found test for transform '{transform}' after Demultiplex in {}",
                            toml_path.display()
                        );
                    }
                }
            }
        }
    }

    // Step 6: Check for missing tests
    let missing_tests: Vec<_> = transforms_with_demultiplexed_data
        .difference(&tested_transforms)
        .collect();

    if !missing_tests.is_empty() {
        //cov:excl-start
        eprintln!("\n❌ The following transforms use DemultiplexedData but have no test cases");
        eprintln!("   where they occur after a Demultiplex step:");
        for transform in &missing_tests {
            eprintln!("   - {transform}");
        }
        eprintln!("\n  Please add test cases in test_cases/demultiplex/ for these transforms.");
        panic!(
            "Missing demultiplex tests for {} transform(s)",
            missing_tests.len()
        );
        //cov:excl-stop
    }

    // Print success message
    println!(
        "\n✓ All {} transforms with DemultiplexedData have tests after Demultiplex:",
        transforms_with_demultiplexed_data.len()
    );
    for transform in &transforms_with_demultiplexed_data {
        println!("  ✓ {transform}");
    }
}

/*
* difficult to test, since it only works in --release build binaries...
We're going to test it in the nix build, I suppose
#[test]
fn test_friendly_panic() {
    let current_exe = std::env::current_exe().unwrap();
    let bin_path = current_exe
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        //.join("debug")
        .join("fastqrab");
    let cmd = std::process::Command::new(get_bin_path()).arg("--test-friendly-panic").output().unwrap();
    //let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(stderr.contains("Usage:"));
    assert!(!cmd.status.success());
} */

#[test]
fn test_validate_command_valid_config_with_existing_files() {
    // Create temp directory and files
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq files
    let mut file1 = fs::File::create(temp_path.join("test1.fq")).unwrap();
    writeln!(file1, "@read1\nACGT\n+\nIIII").unwrap();

    let mut file2 = fs::File::create(temp_path.join("test2.fq")).unwrap();
    writeln!(file2, "@read2\nACGT\n+\nIIII").unwrap();

    // Create valid config
    let config_path = temp_path.join("valid_config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'test1.fq'
read_2 = 'test2.fq'

[[step]]
action = 'Report'
name = 'my_report'
count = true

[output]
prefix = 'output'

[[step]]
    action ='OutputFastQ'
[[step]]
    action = 'OutputReport'
    html = true
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("✓ Configuration is valid"),
        "Expected success message, got: {stdout}"
    );
    assert!(
        !stdout.contains("with warnings"),
        "Should not have warnings with existing files"
    );
    assert!(
        stderr.is_empty(),
        "Should have no warnings in stderr. Was: {stderr}"
    );
    assert!(cmd.status.success(), "Exit code should be 0");
}

#[test]
fn test_validate_command_valid_config_missing_files() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config referencing non-existent files
    let config_path = temp_path.join("missing_files_config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'nonexistent1.fq'
read_2 = 'nonexistent2.fq'

[[step]]
    action = 'Report'
    name = 'my_report'
    count = true

[output]
    prefix = 'output'
[[step]]
    action = 'outputfastq'
[[step]]
    action = 'outputreport'
    html = true
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("✓ Configuration is valid (with warnings)"),
        "Expected success with warnings, got: {stdout}"
    );
    assert!(
        stderr.contains("Warning: Input file not found"),
        "Expected file not found warning in stderr: {stderr}"
    );
    assert!(
        stderr.contains("nonexistent1.fq") || stderr.contains("nonexistent2.fq"),
        "Expected missing file names in warnings"
    );
    assert!(
        cmd.status.success(),
        "Exit code should be 0 even with missing files"
    );
}

#[test]
fn test_validate_command_missing_interleaved_files() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let config_path = temp_path.join("interleaved_missing.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'nonexistent_interleaved.fq'
interleaved = ['read1', 'read2']

[[step]]
action = 'Head'
n = 2

[output]
prefix = 'output'

[[step]]
    action = 'outputfastq'
"
    )
    .unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("✓ Configuration is valid (with warnings)"),
        "Expected success with warnings, got: {stdout} stderr: {stderr}"
    );
    assert!(
        stderr.contains("Warning: Input file not found: nonexistent_interleaved.fq"),
        "Expected interleaved file not found warning, got: {stderr}"
    );
    assert!(
        cmd.status.success(),
        "Exit code should be 0 even with missing files"
    );
}

#[test]
fn test_validate_command_missing_segmented_files_with_segment_name() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let config_path = temp_path.join("segmented_missing.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'nonexistent_r1.fq'
read_2 = 'nonexistent_r2.fq'

[[step]]
action = 'Head'
n = 2

[output]
prefix = 'output'

[[step]]
    action = 'output-fastq'
"
    )
    .unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("✓ Configuration is valid (with warnings)"),
        "Expected success with warnings, got: {stdout}"
    );
    assert!(
        stderr.contains("Warning: Input file not found in segment"),
        "Expected segment file not found warning, got: {stderr}"
    );
    assert!(
        stderr.contains("nonexistent_r1.fq") || stderr.contains("nonexistent_r2.fq"),
        "Expected missing file name in warning, got: {stderr}"
    );
    assert!(
        cmd.status.success(),
        "Exit code should be 0 even with missing files"
    );
}

#[test]
fn test_validate_command_interleaved_stdin_no_warning() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let config_path = temp_path.join("interleaved_stdin.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = '--stdin--'
interleaved = ['read1', 'read2']

[[step]]
action = 'Head'
n = 2

[output]
prefix = 'output'

[[step]]
    action = 'outputFASTQ'
"
    )
    .unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("✓ Configuration is valid"),
        "Expected success message, got: {stdout}"
    );
    assert!(
        !stdout.contains("with warnings"),
        "Should not have warnings for stdin input"
    );
    assert!(
        stderr.is_empty(),
        "Should have no warnings in stderr for stdin input, got: {stderr}"
    );
    assert!(cmd.status.success(), "Exit code should be 0");
}

#[test]
fn test_validate_command_segmented_stdin_no_warning() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let config_path = temp_path.join("segmented_stdin.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = '--stdin--'

[[step]]
action = 'Head'
n = 2

[output]
prefix = 'output'

[[step]]
    action = 'output-fastq'
"
    )
    .unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("✓ Configuration is valid"),
        "Expected success message, got: {stdout}"
    );
    assert!(
        !stdout.contains("with warnings"),
        "Should not have warnings for stdin input"
    );
    assert!(
        stderr.is_empty(),
        "Should have no warnings in stderr for stdin input, got: {stderr}"
    );
    assert!(cmd.status.success(), "Exit code should be 0");
}

#[test]
fn test_validate_command_mixing_stdin() {
    // let temp_dir = tempfile::tempdir().unwrap();
    // let temp_path = temp_dir.path();

    //let config_path = temp_path.join("segmented_stdin.toml");
    let config = r"[input]
read1 = '--stdin--'

[[step]]
action = 'Head'
n = 2

[output]
prefix = 'output'
[[step]]
    action = 'output-fastq'
";

    let mut child = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg("-")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(config.as_bytes()).unwrap();
    }
    let mut stdout = BString::new("".into());
    let mut stderr = BString::new("".into());

    if let Some(mut out) = child.stdout.take() {
        out.read_to_end(&mut stdout).unwrap();
    }
    if let Some(mut err) = child.stderr.take() {
        err.read_to_end(&mut stderr).unwrap();
    }

    let status = child.wait().unwrap(); // optionally inspect
    assert!(
        stderr.contains_str(b"Cannot read configuration from stdin ('-') when the configuration also uses stdin ('--stdin--') for FASTQ input. "),
        "Should have error in stderr"
    );
    assert!(!status.success(), "Exit code should be != 0");
}

#[test]
fn test_commands_mixing_stdin() {
    // let temp_dir = tempfile::tempdir().unwrap();
    // let temp_path = temp_dir.path();

    //let config_path = temp_path.join("segmented_stdin.toml");
    let config = r"[input]
read1 = '--stdin--'

[[step]]
action = 'Head'
n = 2

[output]
prefix = 'output'
[[step]]
    action = 'output-fastq'
";

    for command in &["validate", "verify", "process", "output-files"] {
        let mut child = std::process::Command::new(get_bin_path())
            .arg(command)
            .arg("-")
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(config.as_bytes()).unwrap();
        }
        let mut stdout = BString::new("".into());
        let mut stderr = BString::new("".into());

        if let Some(mut out) = child.stdout.take() {
            out.read_to_end(&mut stdout).unwrap();
        }
        if let Some(mut err) = child.stderr.take() {
            err.read_to_end(&mut stderr).unwrap();
        }

        let status = child.wait().unwrap(); // optionally inspect
        assert!(
        stderr.contains_str(b"Cannot read configuration from stdin ('-') when the configuration also uses stdin ('--stdin--') for FASTQ input. "),
        "Should have error in stderr (command was: {command})"
    );
        assert!(!status.success(), "Exit code should be != 0");
    }
}
#[test]
fn test_validate_command_invalid_action() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with invalid action
    let config_path = temp_path.join("invalid_action.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'test.fq'

[[step]]
action = 'InvalidAction'
name = 'test'

[output]
prefix = 'output'
[[step]]
    action = 'outputfastq'
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stderr.contains("Configuration validation failed"),
        "Expected validation failure message"
    );
    assert!(
        stderr.contains("InvalidAction") || stderr.contains("Unknown variant"),
        "Expected error about invalid action: {stderr}"
    );
    assert!(
        !cmd.status.success(),
        "Exit code should be non-zero for invalid config"
    );
}

#[test]
fn test_validate_command_bad_blocksize() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with invalid action
    let config_path = temp_path.join("input.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
seq = 'test.fq'
interleaved = ['read1','read2']

[options]
block_size = 3


[output]
prefix = 'output'
[[step]]
    action = 'outputfastq'
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        //.arg(&config_path) // to test the auto detection
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    dbg!(&stderr);
    assert!(
        stderr.contains("Configuration validation failed"),
        "Expected validation failure message"
    );
    assert!(
        stderr.contains("block_size must be a multiple of 2"),
        "Expected error about invalid action: {stderr}"
    );
    assert!(
        !cmd.status.success(),
        "Exit code should be non-zero for invalid config"
    );
}

#[test]
fn test_validate_command_bad_autodetect_toml() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with invalid action
    let config_path = temp_path.join("input.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
seq = 'test.fq'
interleaved = ['read1','read2']
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(stderr.contains(
        "TOML file(s) found in current directory, but none were valid TOML configuration files."
    ));
    assert!(
        stderr.contains("A valid configuration must contain both [input] and [output] sections")
    );
}

#[test]
fn test_validate_command_two_autodetect_toml() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with invalid action
    let config_path = temp_path.join("input.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
seq = 'test.fq'
[output]
    prefix = 'out'
    [[step]]
        action = 'output_fastq'
"
    )
    .unwrap();
    let config_path2 = temp_path.join("input2.toml");
    let mut config2 = fs::File::create(&config_path2).unwrap();
    writeln!(
        config2,
        r"[input]
seq = 'test.fq'
[output]
    prefix = 'out'
[[step]]
    action ='outputfastq'
"
    )
    .unwrap();
    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(stderr.contains(
        "Found 2 valid TOML files in current directory. Please specify which one to use"
    ));
    assert!(stderr.contains("input.toml"));
    assert!(stderr.contains("input2.toml"));
}

#[test]
fn test_validate_command_bad_autodetect_toml_missing_input() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with invalid action
    let config_path = temp_path.join("input.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[output]
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(stderr.contains(
        "TOML file(s) found in current directory, but none were valid TOML configuration files."
    ));
    assert!(
        stderr.contains("A valid configuration must contain both [input] and [output] sections")
    );
}

#[test]
fn test_validate_command_no_autodetect_toml() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(stderr.contains("No TOML file found in current directory by auto-detection."));
    assert!(
        stderr.contains(
            "Add one to the current directory or specify a configuration file explicitly."
        )
    );
}
#[test]
fn test_validate_command_not_followyng_symlink_to_good_toml() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with invalid action
    let config_path = temp_path.join("input.not-a-toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
seq = 'test.fq'
interleaved = ['read1','read2']
"
    )
    .unwrap();
    fastqrab::cli::verify::create_symlink(&config_path, &temp_path.join("input.toml")).unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(stderr.contains(
        "TOML file(s) found in current directory, but none were valid TOML configuration files."
    ));
}

#[test]
fn test_validate_command_ignoring_nontoml_even_with_right_content() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with invalid action
    let config_path = temp_path.join("input.not-a-toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
seq = 'test.fq'
interleaved = ['read1','read2']
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stderr.contains("No TOML file found in current directory by auto-detection."),
        "missing message was '{stderr}'"
    );
}

#[test]
fn test_validate_command_nonexistent_toml() {
    // Try to validate a non-existent file
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg("/nonexistent/path/to/config.toml")
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stderr.contains("Configuration validation failed") || stderr.contains("Could not read"),
        "Expected error about missing TOML file: {stderr}"
    );
    assert!(
        !cmd.status.success(),
        "Exit code should be non-zero for missing file"
    );
}

#[test]
fn test_validate_command_malformed_toml() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create malformed TOML
    let config_path = temp_path.join("malformed.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input
read_1 = 'test.fq'
this is not valid toml
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stderr.contains("Configuration validation failed") || stderr.contains("Could not parse"),
        "Expected error about malformed TOML: {stderr}"
    );
    assert!(
        !cmd.status.success(),
        "Exit code should be non-zero for malformed TOML"
    );
}

#[test]
fn test_validate_command_invalid_block_size() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create malformed TOML
    let config_path = temp_path.join("malformed.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
reads = 'test.fq'
interleaved  = ['read1','read2']

[options]
    block_size= 5

[output]
    prefix = 'output'
[[step]] 
    action = 'outputfastq'
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    dbg!(&stderr);
    assert!(
        stderr.contains("Configuration validation failed") || stderr.contains("Could not parse"),
        "Expected error about malformed TOML: {stderr}"
    );
    assert!(stderr.contains("block_size must be a multiple of 2"));
    assert!(
        !cmd.status.success(),
        "Exit code should be non-zero for this error"
    );
}

#[test]
fn test_validate_command_missing_required_fields() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config missing required fields (no output)
    let config_path = temp_path.join("missing_fields.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'test.fq'

[[step]]
action = 'Report'
name = 'my_report'
count = true
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(&config_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    // This should fail because output is required when there's a Report step
    assert!(
        stderr.contains("Configuration validation failed")
            || stderr.contains("Report") // cov:excl-line
            || stderr.contains("No output"), // cov:excl-line
        "Expected error about missing output configuration: {stderr}"
    );
    assert!(!cmd.status.success(), "Exit code should be non-zero");
}

#[test]
fn test_validate_command_no_arguments() {
    // Run validate without config file
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stderr.contains(
            "TOML file(s) found in current directory, but none were valid TOML configuration files."
        ),
        "Expected error about missing config argument: {stderr}"
    );
    assert!(!cmd.status.success(), "Exit code should be non-zero");
}

#[test]
fn test_verify_command_matching_outputs() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();
    writeln!(input_file, "@read2\nTGCA\n+\nIIII").unwrap();

    // Create config with JSON and HTML reports
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[[step]]
action = 'Report'
name = 'test_report'
count = true

[output]
    prefix = 'output'
    # report_timing = true

[[step]]
    action = 'outputreport'
    json = true
    html = true

[[step]]
    action ='OutputFastQ'
"
    )
    .unwrap();

    // First, run process to generate expected outputs
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    assert!(
        process_cmd.status.success(),
        "Process command should succeed: {}",
        std::str::from_utf8(&process_cmd.stderr).unwrap()
    );

    // Verify that output files were created
    assert!(
        temp_path.join("output_read1.fq").exists(),
        "Output fastq file should exist"
    );
    assert!(
        temp_path.join("output.json").exists(),
        "Output JSON report should exist"
    );
    assert!(
        temp_path.join("output.html").exists(),
        "Output HTML report should exist"
    );

    // Now run verify command - should pass since outputs match
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        verify_cmd.status.success(),
        "Verify should succeed with matching outputs. Stderr: {stderr}",
    );
    assert!(
        stdout.contains("✓ Verification passed"),
        "Expected success message, got: {stdout}",
    );
}

#[test]
fn test_verify_command_mismatched_outputs() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    // Create config
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create a fake output file with wrong content
    let mut output_file = fs::File::create(temp_path.join("output_read1.fq")).unwrap();
    writeln!(output_file, "@wrong\nTTTT\n+\nIIII").unwrap();

    // Run verify command - should fail
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail with mismatched outputs"
    );
    assert!(
        stderr.contains("Verification failed") || stderr.contains("mismatch"),
        "Expected error about mismatch, got: {stderr}",
    );
}

#[test]
fn test_verify_command_missing_outputs() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    // Create config
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Don't create any output files - verify should fail

    // Run verify command - should fail due to missing expected outputs
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail with missing outputs"
    );
    assert!(
        stderr.contains("No expected output files found") || stderr.contains("Verification failed"),
        "Expected error about missing files, got: {stderr}",
    );
}
#[test]
fn test_verify_command_missing_outputs_subdir() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    // Create config
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'another_output/output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Don't create any output files - verify should fail

    // Run verify command - should fail due to missing expected outputs
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail with missing outputs"
    );
    assert!(
        stderr.contains("No expected output files found") || stderr.contains("Verification failed"),
        "Expected error about missing files, got: {stderr}",
    );
}

#[test]
fn test_verify_command_auto_detection() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    // Create config (single TOML file in directory)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // First, generate expected outputs
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .current_dir(temp_path)
        .output()
        .unwrap();

    assert!(
        process_cmd.status.success(),
        "Process should succeed with auto-detection"
    );

    // Now verify without specifying config file - should auto-detect
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap();

    assert!(
        verify_cmd.status.success(),
        "Verify should succeed with auto-detection. Stderr: {stderr}",
    );
    assert!(
        stderr.contains("Auto-detected configuration file"),
        "Should show auto-detection message, got: {stdout}",
    );
    assert!(
        stdout.contains("✓ Verification passed"),
        "Should verify successfully, got: {stdout}",
    );
}

#[test]
fn test_verify_command_multiple_toml_files() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    // Create two config files
    let config1_path = temp_path.join("config1.toml");
    let mut config1 = fs::File::create(&config1_path).unwrap();
    writeln!(
        config1,
        r"[input]
read1 = 'input.fq'

[output]
prefix = 'output1'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    let config2_path = temp_path.join("config2.toml");
    let mut config2 = fs::File::create(&config2_path).unwrap();
    writeln!(
        config2,
        r"[input]
read1 = 'input.fq'

[output]
prefix = 'output2'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Try to verify without specifying config file - should fail with multiple files
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when multiple TOML files exist"
    );
    assert!(
        stderr.contains("Found 2 valid TOML files") || stderr.contains("multiple"),
        "Expected error about multiple files, got: {stderr}",
    );
    assert!(
        stderr.contains("Please specify"),
        "Should ask user to specify which file, got: {stderr}",
    );
}

#[test]
fn test_json_schema_command() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("json-schema")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(cmd.status.success(), "stderr: {stderr}");
    assert!(stderr.is_empty(), "stderr should be empty, got: {stderr}");

    // stdout must be exactly a JSON schema document, nothing else
    let value: serde_json::Value = serde_json::from_str(stdout.trim())
        .unwrap_or_else(|e| panic!("stdout was not valid JSON: {e}\nstdout was: {stdout}"));
    assert!(value.is_object(), "schema should be a JSON object");
    assert!(
        value.get("$schema").is_some(),
        "expected a $schema marker identifying this as a JSON schema, got: {stdout}"
    );
}

#[test]
fn test_completions_command_bash() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("completions")
        .arg("bash")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(cmd.status.success(), "Completions command should succeed");
    assert!(
        stdout.contains("_fastqrab"),
        "Should contain bash completion function name"
    );
    assert!(
        stdout.contains("complete"),
        "Should contain bash completion directives"
    );
    assert!(
        stdout.contains("process"),
        "Should include process subcommand"
    );
    assert!(
        stdout.contains("cookbook"),
        "Should include cookbook subcommand"
    );
    assert!(
        stdout.contains("template"),
        "Should include template subcommand"
    );
}

#[test]
fn test_completions_command_fish() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("completions")
        .arg("fish")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(cmd.status.success(), "Completions command should succeed");
    assert!(
        stdout.contains("complete -c fastqrab"),
        "Should contain fish completion commands"
    );
    assert!(
        stdout.contains("process"),
        "Should include process subcommand"
    );
    assert!(
        stdout.contains("cookbook"),
        "Should include cookbook subcommand"
    );
    assert!(
        stdout.contains("template"),
        "Should include template subcommand"
    );
}

#[test]
fn test_completions_command_zsh() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("completions")
        .arg("zsh")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(cmd.status.success(), "Completions command should succeed");
    assert!(
        stdout.contains("#compdef fastqrab"),
        "Should contain zsh completion directive"
    );
    assert!(
        stdout.contains("_fastqrab"),
        "Should contain zsh completion function name"
    );
    assert!(
        stdout.contains("process"),
        "Should include process subcommand"
    );
    assert!(
        stdout.contains("cookbook"),
        "Should include cookbook subcommand"
    );
    assert!(
        stdout.contains("template"),
        "Should include template subcommand"
    );
}

#[test]
fn test_completions_command_powershell() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("completions")
        .arg("powershell")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(cmd.status.success(), "Completions command should succeed");
    assert!(
        stdout.contains("Register-ArgumentCompleter"),
        "Should contain PowerShell completion registration"
    );
    assert!(
        stdout.contains("fastqrab"),
        "Should reference the command name"
    );
}

#[test]
fn test_completions_command_elvish() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("completions")
        .arg("elvish")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(cmd.status.success(), "Completions command should succeed");
    assert!(
        stdout.contains("edit:completion:arg-completer"),
        "Should contain elvish completion setup"
    );
    assert!(
        stdout.contains("fastqrab"),
        "Should reference the command name"
    );
}

#[test]
fn test_completions_command_invalid_shell() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("completions")
        .arg("invalid-shell")
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(!cmd.status.success(), "Should fail with invalid shell");
    assert!(
        stderr.contains("invalid value") || stderr.contains("error"),
        "Should show error about invalid shell"
    );
}

#[test]
fn test_completions_command_missing_shell() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("completions")
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(!cmd.status.success(), "Should fail without shell argument");
    assert!(
        stderr.contains("required") || stderr.contains("<SHELL>"),
        "Should show error about missing shell argument"
    );
}

#[test]
fn test_environment_completion_bash() {
    let cmd = std::process::Command::new(get_bin_path())
        .env("COMPLETE", "bash")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(
        cmd.status.success(),
        "Environment completion should succeed"
    );
    assert!(
        stdout.contains("_fastqrab"),
        "Should contain bash completion function name"
    );
    assert!(
        stdout.contains("complete"),
        "Should contain bash completion directives"
    );
}

#[test]
fn test_environment_completion_fish() {
    let cmd = std::process::Command::new(get_bin_path())
        .env("COMPLETE", "fish")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(
        cmd.status.success(),
        "Environment completion should succeed"
    );
    assert!(
        stdout.contains("complete -c fastqrab"),
        "Should contain fish completion commands"
    );
}

#[test]
fn test_environment_completion_zsh() {
    let cmd = std::process::Command::new(get_bin_path())
        .env("COMPLETE", "zsh")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    assert!(
        cmd.status.success(),
        "Environment completion should succeed"
    );
    assert!(
        stdout.contains("#compdef fastqrab"),
        "Should contain zsh completion directive"
    );
    assert!(
        stdout.contains("_fastqrab"),
        "Should contain zsh completion function name"
    );
}

#[test]
fn test_environment_completion_invalid_shell() {
    // With invalid shell in environment variable, should fall through to normal CLI parsing
    let cmd = std::process::Command::new(get_bin_path())
        .env("COMPLETE", "invalid-shell")
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    // Should fail due to arg_required_else_help, not completion error
    assert!(
        !cmd.status.success(),
        "Should fail due to missing arguments"
    );
    assert!(stderr.contains("Usage:"), "Should show usage help");
}

#[test]
fn test_help_flag() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("--help")
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();

    // Verify --help flag outputs usage information to stdout
    assert!(
        stdout.contains("Usage"),
        "Help output should contain 'Usage'"
    );
    assert!(cmd.status.success(), "Help command should succeed");
}

#[test]
fn test_benchmark_command_no_output() {
    // Create temp directory and files
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq files
    let mut file1 = fs::File::create(temp_path.join("test1.fq")).unwrap();
    writeln!(file1, "@read1\nACGT\n+\nIIII").unwrap();

    // Create valid config
    let config_path = temp_path.join("valid_config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'test1.fq'

[[step]]
action = 'Report'
name = 'my_report'
count = true

[benchmark]
enable = true
molecule_count = 20
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg("valid_config.toml")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("Benchmark completed in "),
        "Expected success message, got: {stdout}\n:stderr: {stderr}"
    );
    assert!(
        !stdout.contains("with warnings"),
        "Should not have warnings with existing files"
    );
    assert!(stderr.is_empty(), "Should have no warnings in stderr");
    assert!(cmd.status.success(), "Exit code should be 0");
}

#[test]
fn test_benchmark_zero_molecules() {
    // Create temp directory and files
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq files
    let mut file1 = fs::File::create(temp_path.join("test1.fq")).unwrap();
    writeln!(file1, "@read1\nACGT\n+\nIIII").unwrap();

    // Create valid config
    let config_path = temp_path.join("valid_config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'test1.fq'

[[step]]
action = 'Report'
name = 'my_report'
count = true

[benchmark]
enable = true
molecule_count = 0
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg("valid_config.toml")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        !stdout.contains("Benchmark completed in "),
        "Did not expect success message, got: {stdout}\n:stderr: {stderr}"
    );
    assert!(
        stderr.contains("molecule_count must be > 0"),
        "Expected error message, got {stderr}"
    );

    assert!(
        stderr.contains("Set to a positive integer."),
        "Expected error message, got {stderr}"
    );
    assert!(!cmd.status.success(), "Exit code should not be 0");
}

#[test]
fn test_benchmark_command_no_output_interleaved() {
    // Create temp directory and files
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq files
    let mut file1 = fs::File::create(temp_path.join("test1.fq")).unwrap();
    writeln!(file1, "@read1/1\nACGT\n+\nIIII\n@read1/2\nACGT\n+\nIIII\n").unwrap();

    // Create valid config
    let config_path = temp_path.join("valid_config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read_1 = 'test1.fq'
interleaved = ['read_1','read_2']

[options]
accept_duplicate_files  = true

[[step]]
action = 'Report'
name = 'my_report'
count = true

[benchmark]
enable = true
molecule_count = 20
"
    )
    .unwrap();

    // Run validate command
    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg("valid_config.toml")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();

    assert!(
        stdout.contains("Benchmark completed in "),
        "Expected success message, got: {stdout}\n:stderr: {stderr}"
    );
    assert!(
        !stdout.contains("with warnings"),
        "Should not have warnings with existing files"
    );
    dbg!(&stderr);
    assert!(stderr.is_empty(), "Should have no warnings in stderr");
    assert!(cmd.status.success(), "Exit code should be 0");
}

#[test]
fn test_verify_command_expected_error_exact() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a validation error
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 23

[output]
prefix = 'output'"
    )
    .unwrap();

    // Create expected panic file
    fs::write(temp_path.join("expected_error.txt"), "expected string").unwrap();

    // Run verify command - should pass since panic matches expected
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        verify_cmd.status.success(),
        "Verify should pass with matching _error. stdout: {stdout}\nstderr: {stderr}",
    );
}

#[test]
fn test_verify_command_expected_error_regex() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a panic (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 23

[output]
prefix = 'output'

[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create expected panic regex file
    fs::write(temp_path.join("expected_error.regex"), r"expected [a-z]{6}").unwrap();

    // Run verify command - should pass since panic matches regex
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        verify_cmd.status.success(),
        "Verify should pass with matching error regex. stdout: {stdout}\nstderr: {stderr}",
    );
}

#[test]
fn test_verify_command_unexpected_error_success() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create valid config and input
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'

[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create expected panic file (but command will succeed)
    fs::write(temp_path.join("expected_error.txt"), "Some error message").unwrap();

    // Run verify command - should fail since panic was expected but didn't occur
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when error expected but command succeeds"
    );
    assert!(
        stderr.contains("Expected validation failure but 'validate' command succeeded"),
        "Should report unexpected success, got: {stderr}",
    );
}

#[test]
fn test_verify_command_wrong_error_message() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a panic (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 23

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
    "
    )
    .unwrap();

    // Create expected panic file with wrong message
    fs::write(temp_path.join("expected_error.txt"), "Wrong error message").unwrap();

    // Run verify command - should fail since panic message doesn't match
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when error message doesn't match expected"
    );
    assert!(
        stderr.contains("did not fail in the way that was expected"),
        "Should report failure mismatch, got: {stderr}",
    );
}

#[test]
fn test_verify_command_runtime_failure_but_validation_expected() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a panic (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'missing.txt'

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
    "
    )
    .unwrap();

    // Create expected panic file with wrong message
    fs::write(temp_path.join("expected_error.txt"), "expected string").unwrap();

    // Run verify command - should fail since panic message doesn't match
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when validation error did not occure"
    );
    assert!(
        stderr.contains("Expected validation failure but 'validate' command succeeded."),
        "Should report failure mismatch, got: {stderr}",
    );
}

#[test]
fn test_verify_command_validation_failure_but_runtime_expected() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a panic (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 23

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create expected panic file with wrong message
    fs::write(temp_path.join("expected_runtime_error.txt"), "No such file").unwrap();

    // Run verify command - should fail since panic message doesn't match
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when validation error did occur"
    );
    assert!(
        stderr.contains("Configuration validation failed, but a runtime error was expected."),
        "Should report failure mismatch, got: {stderr}",
    );
}

#[test]
fn test_verify_command_expected_runtime_error_exact() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a runtime error (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'missing_file.fq'

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
    "
    )
    .unwrap();

    // Create expected panic file
    fs::write(
        temp_path.join("expected_runtime_error.txt"),
        "No such file or directory",
    )
    .unwrap();

    // Run verify command - should pass since panic matches expected
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        verify_cmd.status.success(),
        "Verify should pass with matching runtime_error. stdout: {stdout}\nstderr: {stderr}",
    );
}

#[test]
fn test_verify_command_expected_runtime_error_regex() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a panic (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'nonexistent_file.fq'

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create expected panic regex file
    fs::write(
        temp_path.join("expected_runtime_error.regex"),
        r"No such file or directory",
    )
    .unwrap();

    // Run verify command - should pass since panic matches regex
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        verify_cmd.status.success(),
        "Verify should pass with matching runtime_error regex. stdout: {stdout}\nstderr: {stderr}",
    );
}

#[test]
fn test_verify_command_unexpected_runtime_error_success() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create valid config and input
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create expected panic file (but command will succeed)
    fs::write(
        temp_path.join("expected_runtime_error.txt"),
        "Some error message",
    )
    .unwrap();

    // Run verify command - should fail since panic was expected but didn't occur
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when runtime_error expected but command succeeds"
    );
    assert!(
        stderr.contains("Expected runtime failure but 'process' command succeeded"),
        "Should report unexpected success, got: {stderr}",
    );
}

#[test]
fn test_verify_command_wrong_runtime_error_message() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a panic (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'missing_file.fq'

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create expected panic file with wrong message
    fs::write(
        temp_path.join("expected_runtime_error.txt"),
        "Wrong error message",
    )
    .unwrap();

    // Run verify command - should fail since panic message doesn't match
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when runtime_error message doesn't match expected"
    );
    assert!(
        stderr.contains("did not fail in the way that was expected"),
        "Should report panic mismatch, got: {stderr}",
    );
}

#[test]
fn test_verify_command_both_error_and_runtime_error() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test config that will cause a panic (missing input file)
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'missing_file.fq'

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
"
    )
    .unwrap();

    // Create expected panic file
    fs::write(
        temp_path.join("expected_error.txt"),
        "No such file or directory",
    )
    .unwrap();
    fs::write(
        temp_path.join("expected_runtime_error.txt"),
        "No such file or directory",
    )
    .unwrap();

    // Run verify command - should pass since panic matches expected
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    //let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(!verify_cmd.status.success(),);
    assert!(stderr.contains(
            "Both expected_error(.txt|.regex) and expected_runtime_error(.txt|.regex) files exist. Please provide only one, depending on wether it's a validation or a processing error."
    ));
}

#[test]
fn test_verify_command_output_dir() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();
    writeln!(input_file, "@read2\nTGCA\n+\nIIII").unwrap();

    // Create config with JSON and HTML reports
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[[step]]
    action = 'Progress'
    output_infix = 'progress' 

[[step]]
    action = 'Report'
    name = 'test_report'
    count = true

[output]
    prefix = 'output'
[[step]]
    action = 'OutputFASTQ'

[[step]]
    action = 'OutputReport'
    json = true
    html = true
"
    )
    .unwrap();

    // First, run process to generate expected outputs
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    assert!(
        process_cmd.status.success(),
        "Process command should have succeeded: {}",
        std::str::from_utf8(&process_cmd.stderr).unwrap()
    );

    // Verify that output files were created
    assert!(
        temp_path.join("output_read1.fq").exists(),
        "Output fastq file should exist"
    );
    assert!(
        temp_path.join("output.json").exists(),
        "Output JSON report should exist"
    );
    assert!(
        temp_path.join("output.html").exists(),
        "Output HTML report should exist"
    );
    // // //list dir for debug
    // println!(
    //     "Temp dir contents: {:?}",
    //     fs::read_dir(temp_path)
    //         .unwrap()
    //         .map(|res| res.map(|e| e.path()))
    //         .collect::<Result<Vec<_>, std::io::Error>>()
    //         .unwrap()
    // );
    assert!(
        temp_path.join("output_progress.progress").exists(),
        "Output progress logshould exist"
    );

    let mut output_file = fs::File::create(temp_path.join("output_read1.fq")).unwrap();
    writeln!(output_file, "make it fail").unwrap();
    // Now run verify command - should pass since outputs match
    let mut verify_cmd = std::process::Command::new(get_bin_path());
    verify_cmd
        .arg("verify")
        .arg(&config_path)
        .arg("--output-dir")
        .arg(temp_path.canonicalize().unwrap().join("actual_output"))
        .current_dir(temp_path);
    println!("{verify_cmd:?}");

    let verify_cmd = verify_cmd.output().unwrap();
    //
    // let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    // let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    // //list dir for debug
    // println!(
    //     "Temp dir contents: {:?}",
    //     fs::read_dir(temp_path)
    //         .unwrap()
    //         .map(|res| res.map(|e| e.path()))
    //         .collect::<Result<Vec<_>, std::io::Error>>()
    //         .unwrap()
    // );
    // println!("stderr: {}", stderr);
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail (because of expected_error.txt",
    );

    // println!(
    //     "{}",
    //     std::fs::read_to_string(temp_path.join("actual_output/output.json")).unwrap()
    // );
    assert!(
        std::fs::read_to_string(temp_path.join("actual_output/output.json"))
            .expect("failed to read actual_output/output.json")
            .contains("_IGNORED_")
    );

    assert!(
        std::fs::read_to_string(temp_path.join("actual_output/output.html"))
            .expect("failed to read actual_output/output.json")
            .contains("_IGNORED")
    );

    assert!(
        std::fs::read_to_string(temp_path.join("actual_output/output_progress.progress"))
            .expect("failed to read actual_output/output.json")
            .contains("_IGNORED")
    );
}

#[test]
fn test_cookbook_list() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("cookbook")
        .output()
        .unwrap();
    //let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(stdout.contains("Available cookbooks:"));
    assert!(cmd.status.success());
}

#[test]
fn test_cookbook_01() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("cookbook")
        .arg("1")
        .output()
        .unwrap();
    //let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(stdout.contains("# # Cookbook 01: Basic Quality Report"));
    assert!(cmd.status.success());
}

#[test]
fn test_cookbook_01_by_name() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("cookbook")
        .arg("01-basic-quality-report")
        .output()
        .unwrap();
    //let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    assert!(stdout.contains("# # Cookbook 01: Basic Quality Report"));
    assert!(cmd.status.success());
}

#[test]
fn test_cookbook_not_found() {
    let cmd = std::process::Command::new(get_bin_path())
        .arg("cookbook")
        .arg("99-basic-quality-report")
        .output()
        .unwrap();
    //let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(stderr.contains("Use 'cookbook' without argument to list all available cookbooks",));
    assert!(!cmd.status.success());
}

#[test]
fn test_only_list_one_case_variant_on_error() {
    //we only list one casing, i.e. 'Worse', but not 'worse'
    //since that's the canonical spelling.
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create config with JSON and HTML reports
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
    read1 = 'input.fq'

[[step]]
    action= 'CalcQualifiedBases'
    out_label = 'qb'
    op = 'wrse'
    threshold = 5

[output]
    prefix = 'output'
[[step]]
    action = 'OutputFastQ'
"
    )
    .unwrap();
    let cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg(config_path)
        .output()
        .unwrap();
    //let stdout = std::str::from_utf8(&cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    //assert!(stderr.contains("# # Cookbook 01: Basic Quality Report"));
    assert!(stderr.contains("Worse"));
    assert!(!stderr.contains("worse"));
    assert!(!cmd.status.success());
}

#[test]
fn test_output_already_exists() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    // Create test fastq files
    let mut file1 = fs::File::create(temp_path.join("test1.fq")).unwrap();
    writeln!(file1, "@read1\nACGT\n+\nIIII").unwrap();

    // Create config with JSON and HTML reports
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
    read1 = 'test1.fq'

    [output]
        prefix = 'output'

    [[step]]
        action ='OutputFastQ'
"
    )
    .unwrap();

    {
        let mut output_file = fs::File::create(temp_path.join("output_read1.fq")).unwrap();
        writeln!(output_file, "@read1_already\nACGT\n+\nIIII").unwrap();
    }

    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(!cmd.status.success());
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    dbg!(&stderr);
    assert!(stderr.contains("output_read1.fq\" already exists, refusing to overwrite."));

    let written_output = std::fs::read_to_string(temp_path.join("output_read1.fq")).unwrap();
    assert!(written_output.contains("read1_already"));

    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .arg("--allow-overwrite")
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(cmd.status.success());

    let written_output = std::fs::read_to_string(temp_path.join("output_read1.fq")).unwrap();
    assert!(!written_output.contains("read1_already"));

    {
        let mut output_file = fs::File::create(temp_path.join("output_read1.fq")).unwrap();
        writeln!(output_file, "@read1_already\nACGT\n+\nIIII").unwrap();
    }

    let marker_file = temp_path.join("output.incomplete");
    {
        let mut marker = fs::File::create(&marker_file).unwrap();
        writeln!(marker, "incomplete").unwrap();
    }
    let written_output = std::fs::read_to_string(temp_path.join("output_read1.fq")).unwrap();

    assert!(written_output.contains("read1_already"));

    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .arg("--allow-overwrite")
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(cmd.status.success());

    let written_output = std::fs::read_to_string(temp_path.join("output_read1.fq")).unwrap();
    assert!(!written_output.contains("read1_already"));
}

#[test]
fn test_verify_command_missing_output_file() {
    // Expected file exists in test dir but process doesn't produce it
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'

[[step]]
    action ='OutputFastQ'

",
    )
    .unwrap();

    // Run process to generate the correct expected output
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(process_cmd.status.success(), "Process should succeed");

    // Add a phantom expected file that process will never produce
    fs::write(
        temp_path.join("output_read2.fq"),
        "@phantom\nACGT\n+\nIIII\n",
    )
    .unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Missing output file"),
        "Should report missing output file, got: {stderr}"
    );
}

#[test]
fn test_verify_command_unexpected_output_file() {
    // Process produces a file that has no corresponding expected file
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_r1 = fs::File::create(temp_path.join("input_r1.fq")).unwrap();
    writeln!(input_r1, "@read1\nACGT\n+\nIIII").unwrap();
    let mut input_r2 = fs::File::create(temp_path.join("input_r2.fq")).unwrap();
    writeln!(input_r2, "@read1\nTGCA\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input_r1.fq'
read2 = 'input_r2.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'

[[step]]
    action ='OutputFastQ'
",
    )
    .unwrap();

    // Run process to generate both output_read1.fq and output_read2.fq as expected files
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(process_cmd.status.success(), "Process should succeed");
    assert!(
        temp_path.join("output_read2.fq").exists(),
        "output_read2.fq should have been produced"
    );

    // Remove read2 expected file so it becomes "unexpected" when verify runs
    fs::remove_file(temp_path.join("output_read2.fq")).unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Unexpected output file"),
        "Should report unexpected output file, got: {stderr}"
    );
}

#[test]
fn test_verify_command_error_message_found_multiple_times() {
    // expected_error.txt contains a string that appears more than once in stderr
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let config_path = temp_path.join("config.toml");
    // read1 = 23 triggers a validation error; the word "error" appears multiple times in the output
    fs::write(
        &config_path,
        r"[input]
read1 = 23

[output]
prefix = 'output'
",
    )
    .unwrap();

    // Use a short string guaranteed to appear multiple times in any error output
    fs::write(temp_path.join("expected_error.txt"), "e").unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("expected message was found multiple times"),
        "Should report duplicate match, got: {stderr}"
    );
}

#[test]
fn test_verify_command_expected_warning_but_none_produced() {
    // expected_validation_warning.regex present but config produces no warnings
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    // Config with an existing input file -> no "file not found" warning
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
",
    )
    .unwrap();

    // Generate expected output so verify doesn't fail for other reasons
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(process_cmd.status.success());

    fs::write(
        temp_path.join("expected_validation_warning.regex"),
        "Input file not found",
    )
    .unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Expected validation warning, but none were produced"),
        "Should report missing warning, got: {stderr}"
    );
}

#[test]
fn test_verify_command_validation_warning_wrong_pattern() {
    // expected_validation_warning.regex present but actual warning doesn't match
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let config_path = temp_path.join("config.toml");
    // Missing input file -> warning "Input file not found: missing_input.fq"
    fs::write(
        &config_path,
        r"[input]
read1 = 'missing_input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'outputfastq'
",
    )
    .unwrap();

    // Regex that does NOT match the actual warning
    fs::write(
        temp_path.join("expected_validation_warning.regex"),
        "This pattern will never match",
    )
    .unwrap();

    // expected_runtime_error so verify doesn't fail because process fails
    fs::write(
        temp_path.join("expected_runtime_error.txt"),
        "No such file or directory",
    )
    .unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Validation warnings did not match expected pattern"),
        "Should report warning mismatch, got: {stderr}"
    );
    assert!(
        stderr.contains("This pattern will never match"),
        "Should report pattern ('This pattern will never match'), got {stderr}"
    );
}

#[test]
#[cfg(unix)]
fn test_verify_broken_prep_sh_fails() {
    // Verify that when prep.sh exits with a non-zero status, the verify command fails
    // with the expected error message (covers verify.rs line 303).
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
",
    )
    .unwrap();

    // Write a prep.sh that always fails
    let prep_sh = temp_path.join("prep.sh");
    fs::write(&prep_sh, "#!/usr/bin/env bash\nexit 42\n").unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&prep_sh, fs::Permissions::from_mode(0o755)).unwrap();
    }

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .arg("--unsafe-call-prep-sh")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when prep.sh exits non-zero"
    );
    assert!(
        stderr.contains("prep.sh failed with exit code"),
        "Should report prep.sh failure, got: {stderr}"
    );
}

fn minimal_config_and_input(temp_path: &std::path::Path) -> PathBuf {
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();
    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'

[[step]]
    action ='OutputFastQ'
",
    )
    .unwrap();
    config_path
}

#[cfg(unix)]
fn make_executable(path: &std::path::Path) {
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

#[cfg(not(unix))]
fn make_executable(_path: &std::path::Path) {}

#[test]
fn test_verify_prep_sh_without_unsafe_flag_fails() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = minimal_config_and_input(temp_path);

    let prep_sh = temp_path.join("prep.sh");
    fs::write(&prep_sh, "#!/usr/bin/env bash\nexit 0\n").unwrap();
    make_executable(&prep_sh);

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when prep.sh exists but --unsafe-call-prep-sh is absent"
    );
    assert!(
        stderr.contains("prep.sh script found in") && stderr.contains("--unsafe-call-prep-sh"),
        "Should explain how to enable prep.sh, got: {stderr}"
    );
}

#[test]
fn test_verify_post_sh_without_unsafe_flag_fails() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = minimal_config_and_input(temp_path);

    let post_sh = temp_path.join("post.sh");
    fs::write(&post_sh, "#!/usr/bin/env bash\nexit 0\n").unwrap();
    make_executable(&post_sh);

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when post.sh exists but --unsafe-call-prep-sh is absent"
    );
    assert!(
        stderr.contains("post.sh script found in") && stderr.contains("--unsafe-call-prep-sh"),
        "Should explain how to enable post.sh, got: {stderr}"
    );
}

#[test]
fn test_verify_test_sh_without_unsafe_flag_fails() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = minimal_config_and_input(temp_path);

    let test_sh = temp_path.join("test.sh");
    fs::write(&test_sh, "#!/usr/bin/env bash\nexit 0\n").unwrap();
    make_executable(&test_sh);

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when test.sh exists but --unsafe-call-prep-sh is absent"
    );
    assert!(
        stderr.contains("test.sh script found in") && stderr.contains("--unsafe-call-prep-sh"),
        "Should explain how to enable test.sh, got: {stderr}"
    );
}

#[test]
fn test_verify_config_validation_failed_unexpectedly() {
    // Config has a validation error, no expected_error file → triggers
    // "Configuration validation failed unexpectedly." context message.
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'NonExistentAction'

[output]
prefix = 'output'
",
    )
    .unwrap();

    // No expected_error.txt — validation failure is unexpected
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail on unexpected validation error"
    );
    assert!(
        stderr.contains("Configuration validation failed unexpectedly."),
        "Should report unexpected validation failure, got: {stderr}"
    );
}

#[test]
#[cfg(unix)]
fn test_verify_failing_post_sh_is_detected() {
    // When post.sh exits non-zero, verify should report "post.sh failed with exit code"
    // as part of the "Output verification failed:" mismatch list.
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = minimal_config_and_input(temp_path);

    // Pre-generate expected output so find_output_files doesn't bail before post.sh is checked.
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(process_cmd.status.success(), "process should succeed");

    let post_sh = temp_path.join("post.sh");
    fs::write(
        &post_sh,
        "#!/usr/bin/env bash\necho 'post step broke' >&2\nexit 2\n",
    )
    .unwrap();
    make_executable(&post_sh);

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .arg("--unsafe-call-prep-sh")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when post.sh exits non-zero"
    );
    assert!(
        stderr.contains("post.sh failed with exit code"),
        "Should report post.sh failure, got: {stderr}"
    );
    assert!(
        stderr.contains("post step broke"),
        "Should include post.sh stderr output, got: {stderr}"
    );
}

#[test]
#[cfg(unix)]
fn test_verify_failing_test_sh_is_detected() {
    // When test.sh exits non-zero, verify should fail with "Test script failed:"
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = minimal_config_and_input(temp_path);

    let test_sh = temp_path.join("test.sh");
    fs::write(
        &test_sh,
        "#!/usr/bin/env bash\necho 'something went wrong' >&2\nexit 1\n",
    )
    .unwrap();
    make_executable(&test_sh);

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .arg("--unsafe-call-prep-sh")
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(
        !verify_cmd.status.success(),
        "Verify should fail when test.sh exits non-zero"
    );
    assert!(
        stderr.contains("Test script failed:"),
        "Should report test script failure, got: {stderr}"
    );
    assert!(
        stderr.contains("something went wrong"),
        "Should include test.sh stderr output, got: {stderr}"
    );
}

// ── stdout / stderr stream-file branch coverage ─────────────────────────────

/// Run process first so expected `output_read1.fq` exists, then add an extra
/// expected `stdout` (or `stderr`) file; verify must then report it missing.
fn setup_with_expected_output(temp_path: &std::path::Path) -> PathBuf {
    let config_path = minimal_config_and_input(temp_path);
    let process_cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(process_cmd.status.success(), "process should succeed");
    config_path
}

#[test]
fn test_verify_missing_stdout_file() {
    // Expected `stdout` file present but processor produces no stdout → "Missing stdout file"
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = setup_with_expected_output(temp_path);

    fs::write(temp_path.join("stdout"), b"anything\n").unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Missing stdout file"),
        "Should report missing stdout, got: {stderr}"
    );
}

#[test]
fn test_verify_missing_stderr_file() {
    // Expected `stderr` file present but processor produces no stderr → "Missing stderr file"
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = setup_with_expected_output(temp_path);

    fs::write(temp_path.join("stderr"), b"anything\n").unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Missing stderr file"),
        "Should report missing stderr, got: {stderr}"
    );
}

#[test]
fn test_verify_unexpected_stdout_file() {
    // Config uses stdout=true, no expected `stdout` file → "Unexpected stdout file"
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'OutputFastQ'
    stdout = true
",
    )
    .unwrap();

    // No expected `stdout` file — processor will write to stdout
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Unexpected stdout file"),
        "Should report unexpected stdout, got: {stderr}"
    );
}

#[test]
fn test_verify_wrong_content_stdout_file() {
    // Config uses stdout=true, expected `stdout` has wrong content → "stdout: …"
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'

[[step]]
    action ='OutputFastQ'
    stdout = true
",
    )
    .unwrap();

    fs::write(temp_path.join("stdout"), b"this is not the correct fastq\n").unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("stdout:"),
        "Should report stdout content mismatch, got: {stderr}"
    );
}

#[test]
fn test_process_failure_captured() {
    // Config uses stdout=true, expected `stdout` has wrong content → "stdout: …"
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nAXGT\n+\nIIII").unwrap();

    let config_path = temp_path.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'input.fq'
[[step]]
    action='ValidateSequence'
    allowed = 'AGTC'

[output]
prefix = 'output'
[[step]]
    action = 'outputfastq'
",
    )
    .unwrap();

    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains(" Invalid base found in read named 'read1'"),
        "Should report invalid base, got: {stderr}"
    );
}

#[test]
fn test_verify_compressed_size_difference_too_large() {
    // Generate expected outputs at compression_level=9 (small), then verify with
    // compression_level=1 (large). The size difference on read1 (~27%) exceeds the
    // 5% tolerance and triggers "Compressed file size difference too large".
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Use the existing compressed-output test data: two FASTQ files, 5 reads each.
    let src = Path::new("../test_cases/output/output_compression_gzip_level");
    fs::copy(src.join("input_read1.fq"), temp_path.join("input_read1.fq")).unwrap();
    fs::copy(src.join("input_read2.fq"), temp_path.join("input_read2.fq")).unwrap();

    // Step 1: produce the expected .gz files at level 9.
    let config_level9 = temp_path.join("config.toml");
    fs::write(
        &config_level9,
        r"[input]
    read1 = 'input_read1.fq'
    read2 = 'input_read2.fq'
[[step]]
    action = 'Head'
    n = 5
[output]
    prefix = 'output'

[[step]]
    action = 'outputfastq'
    compression = 'gzip'
    compression_level = 9
",
    )
    .unwrap();

    let proc = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_level9)
        .current_dir(temp_path)
        .output()
        .unwrap();
    assert!(proc.status.success(), "process (level 9) should succeed");

    // Step 2: replace config with level=1; the expected .gz files (level 9) stay in place.
    fs::write(
        &config_level9,
        r"[input]
    read1 = 'input_read1.fq'
    read2 = 'input_read2.fq'
[[step]]
    action = 'Head'
    n = 5
[output]
    prefix = 'output'

[[step]]
    action = 'outputfastq'
    compression = 'gzip'
    compression_level = 1
",
    )
    .unwrap();

    // Step 3: verify — processor produces level-1 output (~27% larger for read1)
    // which exceeds the 5% size-difference tolerance.
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_level9)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();
    assert!(!verify_cmd.status.success(), "Verify should fail");
    assert!(
        stderr.contains("Compressed file size difference too large"),
        "Should report compressed size mismatch, got: {stderr}"
    );
}

/// Wait until `stdout_path` contains at least `run_number` run-completion markers.
/// Uses the timestamp-bracket suffix " [" to distinguish the header lines from error bodies.
/// "Processing completed successfully [time]" and "Processing failed [time]" each appear exactly
/// once per run (the error body "Processing failed:\n..." uses ":" not "[").
fn wait_for_interactive_run(stdout_path: &std::path::Path, run_number: usize) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(15);
    loop {
        assert!(
            std::time::Instant::now() < deadline,
            "Timeout waiting for interactive run {run_number}"
        );
        let content = std::fs::read_to_string(stdout_path).unwrap_or_default();
        let completed = content
            .matches("Processing completed successfully [")
            .count()
            + content.matches("Processing failed [").count();
        if completed >= run_number {
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

#[test]
fn test_interactive() {
    let dir = tempfile::tempdir().unwrap();
    let temp_path = dir.path();
    let config = temp_path.join("config.toml");
    fs::write(
        &config,
        r"[input]
            read1 = 'input_read1.fq'
        [output]
            prefix = 'output'

        [[step]]
            action = 'outputfastq'
",
    )
    .unwrap();
    let mut input_file = fs::File::create(temp_path.join("input_read1.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();
    writeln!(input_file, "@read2\nTGCA\n+\nIIII").unwrap();

    let stdout_path = temp_path.join("stdout");
    let stderr_path = temp_path.join("stderr");
    let stdout_file = std::fs::File::create(&stdout_path).unwrap();
    let stderr_file = std::fs::File::create(stderr_path).unwrap();

    let mut cmd = std::process::Command::new(get_bin_path())
        .arg("interactive")
        .arg("--poll-interval")
        .arg("50")
        .arg("--max-runs")
        .arg("6")
        .current_dir(temp_path)
        .stderr(Stdio::from(stderr_file))
        .stdout(Stdio::from(stdout_file))
        .spawn()
        .unwrap();

    wait_for_interactive_run(&stdout_path, 1); // wait for run 1 to complete

    fs::write(
        &config,
        r"[input]
            read1 = 'input_read1.fq'
          [[step]]
                action = 'prefix'
                seq = 'nn'
                qual = 'BB'
        [output]
            prefix = 'output'

        [[step]]
            action = 'outputfastq'
",
    )
    .unwrap();
    wait_for_interactive_run(&stdout_path, 2); // wait for run 2 to complete

    fs::write(
        &config,
        r"[input]
            read1 = ['input_read1.fq']
            options = {}
          [[step]]
                action = 'prefix'
                seq = 'nn'
                # qual = 'BB'
        [output]
            prefix = 'output'

        [[step]]
            action = 'outputfastq'
",
    )
    .unwrap();
    wait_for_interactive_run(&stdout_path, 3); // wait for run 3 to complete

    // Run 4: absolute path + interleaved key + Report step — covers has_report_step = true,
    // key == "interleaved" skip, and !path.is_absolute() == false, all in one run
    let abs_input = temp_path
        .join("input_read1.fq")
        .to_string_lossy()
        .into_owned();
    fs::write(
        &config,
        format!(
            "[input]
     read1 = '{abs_input}'
     interleaved = ['read1', 'read2']

     [[step]]
        action = 'Report'
        name = 'my_report'
        count = true
     [output]
        prefix = 'output'
     [[step]]
           action = 'outputfastq'
"
        ),
    )
    .unwrap();
    wait_for_interactive_run(&stdout_path, 4); // wait for run 4 to complete

    // Run 5: integer-valued segment — covers `_ => bail!` (line 275) in make_paths_absolute
    fs::write(
        &config,
        r"[input]
            read1 = 23
        [output]
            prefix = 'output'
",
    )
    .unwrap();
    wait_for_interactive_run(&stdout_path, 5); // wait for run 5 to complete

    // Run 6: sub-table segment — covers `_ => bail!` (line 277) in make_paths_absolute
    fs::write(
        &config,
        r"[input]
            read1.path = 'input_read1.fq'
        [output]
            prefix = 'output'
",
    )
    .unwrap();
    wait_for_interactive_run(&stdout_path, 6); // wait for run 6 to complete
    cmd.wait().unwrap(); // wait for clean exit (flushes coverage data)
    let stdout = std::fs::read_to_string(temp_path.join("stdout")).unwrap();

    // println!("stdout:\n{stdout}");
    // println!("stderr:\n{stderr}");
    let pos_round_1 = stdout.find(
        "Inspect Output:
@read1
ACGT
+
IIII
@read2
TGCA
+
IIII",
    );
    assert!(
        pos_round_1.is_some(),
        "Output should contain the full contents of input_read1.fq, got: {stdout}"
    );
    let pos_round_2 = stdout.find(
        "Inspect Output:
@read1
nnACGT
+
BBIIII
@read2
nnTGCA
+
BBIIII",
    );
    assert!(
        pos_round_2.is_some(),
        "Output should contain the modified sequences and qualities, got: {stdout}"
    );

    let pos_round3 = stdout.find(
        "Error 1/1
   ╭─config.toml
   ┆
13 │           [[step]]
14 │                 action = 'prefix'
   ┆                          ────┬───
   ┆                              │
   ┆                              ╰──── In this step
15 │                 seq = 'nn'
   ┆                           ┬
   ┆                           │
   ┆                           ╰─ Missing required key: 'qual'.
───╯",
    );
    assert!(
        pos_round3.is_some(),
        "Output should contain the error message about missing 'qual' key, got: {stdout}"
    );
    assert!(
        pos_round_1.unwrap() < pos_round_2.unwrap() && pos_round_2.unwrap() < pos_round3.unwrap()
    );
}

#[test]
fn test_interactive_no_output() {
    // Covers the display_success "No output" branch — triggered when processing succeeds
    // but produces no inspect output. We filter all reads out with FilterByNumericTag
    // (reads are 4 bases long, filter keeps only reads >= 100 bases → 0 reads reach Inspect).
    let dir = tempfile::tempdir().unwrap();
    let temp_path = dir.path();
    let config = temp_path.join("config.toml");
    let mut input_file = fs::File::create(temp_path.join("input_read1.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();
    fs::write(
        &config,
        r"[input]
            read1 = 'input_read1.fq'
        [[step]]
            action = 'CalcLength'
            out_label = 'len'
        [[step]]
            action = 'FilterByNumericTag'
            in_label = 'len'
            min_value = 100
            keep_or_remove = 'keep'
        [output]
            prefix = 'output'
",
    )
    .unwrap();

    let stdout_path = temp_path.join("stdout");
    let stdout_file = std::fs::File::create(&stdout_path).unwrap();
    let stderr_file = std::fs::File::create(temp_path.join("stderr")).unwrap();

    let mut cmd = std::process::Command::new(get_bin_path())
        .arg("interactive")
        .arg("--poll-interval")
        .arg("50")
        .arg("--max-runs")
        .arg("1")
        .current_dir(temp_path)
        .stdout(Stdio::from(stdout_file))
        .stderr(Stdio::from(stderr_file))
        .spawn()
        .unwrap();

    cmd.wait().unwrap();
    let stdout = std::fs::read_to_string(&stdout_path).unwrap();
    assert!(
        stdout.contains("No output (processing completed without messages)"),
        "Expected 'No output' message, got: {stdout}"
    );
}

const MINIMAL_CONFIG: &str = r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'

[[step]]
    action = 'outputfastq'
";

const MINIMAL_FASTQ: &str = "@read1\nACGT\n+\nIIII\n@read2\nTTTT\n+\nHHHH\n";

#[test]
fn test_process_config_from_stdin() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    fs::write(temp_path.join("input.fq"), MINIMAL_FASTQ).unwrap();

    let mut cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg("-")
        .current_dir(temp_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    cmd.stdin
        .take()
        .unwrap()
        .write_all(MINIMAL_CONFIG.as_bytes())
        .unwrap();
    let output = cmd.wait_with_output().unwrap();

    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(output.status.success(), "process - failed: {stderr}");

    let result = fs::read_to_string(temp_path.join("output_read1.fq")).unwrap();
    assert!(
        result.contains("@read1"),
        "expected read1 in output, got: {result}"
    );
    assert!(
        !result.contains("@read2"),
        "Head n=1 should not include read2"
    );
}

#[test]
fn test_validate_config_from_stdin_valid() {
    let mut cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    cmd.stdin
        .take()
        .unwrap()
        .write_all(MINIMAL_CONFIG.as_bytes())
        .unwrap();
    let output = cmd.wait_with_output().unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(output.status.success(), "validate - failed: {stderr}");
    assert!(stdout.contains("✓ Configuration is valid"), "got: {stdout}");
}

#[test]
fn test_validate_config_from_stdin_invalid() {
    let bad_config = "[input]\nread1 = 'input.fq'\n\n[[step]]\naction = 'Heaad'\n\n[output]\nprefix = 'output'\n";

    let mut cmd = std::process::Command::new(get_bin_path())
        .arg("validate")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    cmd.stdin
        .take()
        .unwrap()
        .write_all(bad_config.as_bytes())
        .unwrap();
    let output = cmd.wait_with_output().unwrap();

    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(
        !output.status.success(),
        "expected failure for invalid config"
    );
    assert!(
        stderr.contains("Heaad"),
        "expected error about 'Heaad', got: {stderr}"
    );
}

#[test]
fn test_process_stdin_config_incompatible_with_stdin_fastq() {
    let stdin_config = "[input]\nread1 = '--stdin--'\n\n[[step]]\naction = 'Head'\nn = 1\n\n[output]\nprefix = 'output'\n[[step]]\naction=\"outputfastq\"\n";

    let mut cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    cmd.stdin
        .take()
        .unwrap()
        .write_all(stdin_config.as_bytes())
        .unwrap();
    let output = cmd.wait_with_output().unwrap();

    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(!output.status.success(), "expected failure");
    assert!(
        stderr.contains("Cannot read configuration from stdin"),
        "expected incompatibility error, got: {stderr}"
    );
}

#[test]
fn test_interactive_rejects_stdin_config() {
    let output = std::process::Command::new(get_bin_path())
        .arg("interactive")
        .arg("-")
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(!output.status.success(), "expected failure");
    assert!(
        stderr.contains("interactive mode cannot read configuration from stdin"),
        "got: {stderr}"
    );
}

// ---------------------------------------------------------------------------
// `output-files` command
// ---------------------------------------------------------------------------

/// Run `fastqrab output-files <config>` against `config` (written to a temp
/// file) and return the listed output files as a sorted Vec. Asserts success.
/// Input files need not exist — the command validates with missing-file
/// tolerance, like `validate`.
fn output_files_lines(config: &str) -> Vec<String> {
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    fs::write(&config_path, config).unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("output-files")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap();
    assert!(
        cmd.status.success(),
        "output-files failed.\nstdout: {stdout}\nstderr: {stderr}"
    );

    let mut files: Vec<String> = stdout
        .lines()
        .filter_map(|line| line.strip_prefix("  "))
        .map(str::to_string)
        .collect();
    files.sort();
    files
}

#[test]
fn test_output_files_simple() {
    let files = output_files_lines(
        r"[input]
read1 = 'in.fq'
read2 = 'in2.fq'

[output]
prefix = 'output'

[[step]]
action = 'OutputFASTQ'
",
    );
    assert_eq!(files, vec!["output_read1.fq", "output_read2.fq"]);
}

#[test]
fn test_output_files_no_output_producing_step_fails() {
    // An [output] section with no Output*/Report step behind it produces
    // nothing, so `output-files` (like `process`) must reject it up front
    // instead of silently reporting an empty file list.
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'in.fq'
read2 = 'in2.fq'

[output]
prefix = 'output'
",
    )
    .unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("output-files")
        .arg(&config_path)
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap();
    assert!(
        !cmd.status.success(),
        "expected failure, stdout: {}",
        std::str::from_utf8(&cmd.stdout).unwrap()
    );
    assert!(
        stderr.contains("No output files and no reports requested"),
        "got: {stderr}"
    );
}

#[test]
fn test_output_files_stdin_config_incompatible_with_stdin_fastq() {
    // Reading the config itself from stdin while the config *also* points a
    // FASTQ input at stdin is ambiguous (both would try to read the same
    // stream), so `list_config_output_files` rejects it explicitly
    // (output_files.rs:47) rather than deadlocking or silently misreading.
    let stdin_config = r"[input]
read1 = '--stdin--'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
action = 'OutputFASTQ'
";

    let mut cmd = std::process::Command::new(get_bin_path())
        .arg("output-files")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    cmd.stdin
        .take()
        .unwrap()
        .write_all(stdin_config.as_bytes())
        .unwrap();
    let output = cmd.wait_with_output().unwrap();

    let stderr = std::str::from_utf8(&output.stderr).unwrap();
    assert!(!output.status.success(), "expected failure");
    assert!(
        stderr.contains(
            "Cannot read configuration from stdin ('-') when the configuration also uses stdin ('--stdin--') for FASTQ input"
        ),
        "expected incompatibility error, got: {stderr}"
    );
}

#[test]
fn test_output_files_benchmark_config_has_no_output_files() {
    // A benchmark config has no [output] section at all (it measures
    // throughput rather than writing files), so `output-files` must report an
    // empty listing rather than treating "no output section" as an error.
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'in.fq'

[benchmark]
enable = true
molecule_count = 1000
",
    )
    .unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("output-files")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap();
    assert!(cmd.status.success(), "stderr: {stderr}");
    assert!(
        stdout.contains("No output files would be produced"),
        "got: {stdout}"
    );
}

#[test]
fn test_output_files_stdout_output() {
    // `OutputFASTQ` with `stdout = true` writes to stdout instead of a file;
    // the listing must report that as the `--stdout--` sentinel rather than a
    // real filename.
    let files = output_files_lines(
        r"[input]
read1 = 'in.fq'
read2 = 'in2.fq'

[output]
prefix = 'output'

[[step]]
action = 'OutputFASTQ'
stdout = true
",
    );
    assert_eq!(files, vec!["--stdout--"]);
}

#[test]
fn test_output_files_demultiplex() {
    // OutputFASTQ after a Demultiplex must list one file per barcode tag
    // (plus the no-barcode bucket because output_unmatched = true), not a
    // single un-demultiplexed file.
    let files = output_files_lines(
        r"[input]
read1 = 'in.fq'

[output]
prefix = 'output'

[[step]]
action = 'ExtractRegion'
anchor = 'Left'
segment = 'read1'
start = 0
length = 2
out_label = 'bc'

[[step]]
action = 'Demultiplex'
in_label = 'bc'
output_unmatched = true
barcodes = 'd'
tag_contains_barcode = true

[barcodes.d]
AA = 'samplea'
CC = 'samplec'

[[step]]
action = 'OutputFASTQ'
",
    );
    assert_eq!(
        files,
        vec![
            "output_nobarcode_read1.fq",
            "output_samplea_read1.fq",
            "output_samplec_read1.fq",
        ]
    );
}

#[test]
fn test_output_files_multiple_output_steps() {
    // Several output-producing steps in one pipeline; all their files appear.
    let files = output_files_lines(
        r"[input]
read1 = 'in.fq'

[output]
prefix = 'output'

[[step]]
action = 'Report'
name = 'r'
count = true

[[step]]
action = 'OutputFASTQ'

[[step]]
action = 'OutputReport'
json = true
html = true
",
    );
    assert_eq!(files, vec!["output.html", "output.json", "output_read1.fq"]);
}

#[test]
fn test_output_files_store_tags_in_table_demultiplexed() {
    // StoreTagsInTable uses DemultiplexedData, so after a Demultiplex it must
    // produce one table per tag. output_unmatched = false -> no nobarcode file.
    let files = output_files_lines(
        r"[input]
read1 = 'in.fq'

[output]
prefix = 'output'

[[step]]
action = 'ExtractRegion'
anchor = 'Left'
segment = 'read1'
start = 0
length = 2
out_label = 'bc'

[[step]]
action = 'Demultiplex'
in_label = 'bc'
output_unmatched = false
barcodes = 'd'
tag_contains_barcode = true

[barcodes.d]
AA = 'samplea'
CC = 'samplec'

[[step]]
action = 'StoreTagsInTable'

[[step]]
action = 'OutputFASTQ'
",
    );
    assert_eq!(
        files,
        vec![
            "output_samplea.tsv",
            "output_samplea_read1.fq",
            "output_samplec.tsv",
            "output_samplec_read1.fq",
        ]
    );
}

#[test]
fn test_output_files_progress_is_singleton_after_demultiplex() {
    // Progress is a singleton: exactly one .progress file even downstream of a
    // Demultiplex (it does not split per tag, unlike OutputFASTQ).
    let files = output_files_lines(
        r"[input]
read1 = 'in.fq'

[output]
prefix = 'output'

[[step]]
action = 'ExtractRegion'
anchor = 'Left'
segment = 'read1'
start = 0
length = 2
out_label = 'bc'

[[step]]
action = 'Demultiplex'
in_label = 'bc'
output_unmatched = true
barcodes = 'd'
tag_contains_barcode = true

[barcodes.d]
AA = 'samplea'

[[step]]
action = 'Progress'
output_infix = 'pp'

[[step]]
action = 'OutputFASTQ'
",
    );
    assert_eq!(
        files,
        vec![
            "output_nobarcode_read1.fq",
            "output_pp.progress",
            "output_samplea_read1.fq",
        ]
    );
}

#[test]
fn test_output_files_inspect_is_singleton_after_demultiplex() {
    // Inspect is also a singleton: one inspect file, no per-tag fan-out.
    let files = output_files_lines(
        r"[input]
read1 = 'in.fq'

[output]
prefix = 'output'

[[step]]
action = 'ExtractRegion'
anchor = 'Left'
segment = 'read1'
start = 0
length = 2
out_label = 'bc'

[[step]]
action = 'Demultiplex'
in_label = 'bc'
output_unmatched = true
barcodes = 'd'
tag_contains_barcode = true

[barcodes.d]
AA = 'samplea'

[[step]]
action = 'Inspect'
n = 5
infix = 'insp'

[[step]]
action = 'OutputFASTQ'
",
    );
    assert_eq!(
        files,
        vec![
            "output_insp_read1.fq",
            "output_nobarcode_read1.fq",
            "output_samplea_read1.fq",
        ]
    );
}

#[test]
fn test_output_files_chunked_lists_first_chunk_and_warns() {
    // Chunked outputs must be listed with their first-chunk ('.0') infix, and
    // the command must warn (on stderr) that further chunks may follow.
    let dir = tempfile::tempdir().unwrap();
    let config_path = dir.path().join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = 'in.fq'

[output]
prefix = 'output'

[[step]]
action = 'OutputFASTQ'
chunk_size = 1000
",
    )
    .unwrap();

    let cmd = std::process::Command::new(get_bin_path())
        .arg("output-files")
        .arg(&config_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&cmd.stdout).unwrap();
    let stderr = std::str::from_utf8(&cmd.stderr).unwrap();
    assert!(cmd.status.success(), "stderr: {stderr}");
    assert!(
        stdout.contains("output_read1.0.fq"),
        "expected first-chunk name, got: {stdout}"
    );
    assert!(
        stderr.contains("first chunk") && stderr.contains("'.0'"),
        "expected chunk warning on stderr, got: {stderr}"
    );
}

/// Locate an executable by name on `PATH`. We deliberately avoid hard-coding
/// `/usr/bin/gzip` etc.: under Nix the system tools live at hashed
/// `/nix/store/...` paths and `/usr/bin` may not exist at all, so the only
/// portable way to find `gzip` is to walk `PATH`.
fn find_on_path(name: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

/// Lay down a gzip-compressed FASTQ input and a minimal identity-pipeline config
/// in `dir`, then return the path to the config. Reuses the checked-in sample
/// gzip so we don't depend on a system gzip just to produce the input.
fn write_gz_process_case(dir: &Path) -> PathBuf {
    let sample = Path::new("../test_cases/sample_data/gzip/home_input_read1.fq.gz");
    assert!(
        sample.exists(),
        "sample gzip input missing at {}",
        sample.display() // cov:excl-line
    );
    fs::copy(sample, dir.join("input_read1.fq.gz")).unwrap();

    let config_path = dir.join("config.toml");
    fs::write(
        &config_path,
        r"[input]
read1 = ['input_read1.fq.gz']

[output]
prefix = 'output'

[[step]]
action = 'OutputFASTQ'
",
    )
    .unwrap();
    config_path
}

/// Pointing `FASTQRAB_DECOMPRESSOR` at a path that does not exist must fail with
/// a clear "could not determine fastqrab binary path" error rather than, say, a
/// silent empty output: `find_decompressor` rejects a non-existent override and
/// the gzip input then has no decoder to run.
#[test]
fn test_decompressor_nonexistent_binary() {
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = write_gz_process_case(temp_path);

    let cmd = std::process::Command::new(get_bin_path())
        .arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .env(
            "FASTQRAB_DECOMPRESSOR",
            temp_path.join("this-binary-does-not-exist"),
        )
        .output()
        .unwrap();

    let stderr = std::str::from_utf8(&cmd.stderr).unwrap().to_string();
    assert!(
        !cmd.status.success(),
        "expected failure for non-existent decompressor, stderr: {stderr}"
    );
    assert!(
        stderr.contains("could not determine fastqrab binary path for decompressor"),
        "expected decompressor-path error, got: {stderr}"
    );
}

/// Run a gzip-input `process` with `FASTQRAB_DECOMPRESSOR` pointed at a real but
/// incompatible binary (`gzip`, which knows nothing of our `__decompressor`
/// subcommand). Returns the failing run's stderr. Skips (returns `None`) if gzip
/// isn't on `PATH`, since it isn't a build dependency. `extra_env` lets a caller
/// pin the transport.
fn run_incompatible_decompressor(extra_env: &[(&str, &str)]) -> Option<String> {
    let gzip = find_on_path("gzip")?;

    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();
    let config_path = write_gz_process_case(temp_path);

    let mut cmd = std::process::Command::new(get_bin_path());
    cmd.arg("process")
        .arg(&config_path)
        .current_dir(temp_path)
        .env("FASTQRAB_DECOMPRESSOR", &gzip);
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    let output = cmd.output().unwrap();

    let stderr = std::str::from_utf8(&output.stderr).unwrap().to_string();
    assert!(
        !output.status.success(),
        "expected failure for incompatible decompressor ({}), stderr: {stderr}",
        gzip.display() // cov:excl-line
    );
    Some(stderr)
}

/// An incompatible decompressor over the *default* transport (shm on unix) must
/// still fail loudly rather than hang or silently truncate. We deliberately do
/// not set `FASTQRAB_DECOMP_SHM` — a user who points the variable at the wrong
/// binary will not have disabled shm either, so this is the realistic path.
#[test]
fn test_decompressor_incompatible_binary_default_transport() {
    let Some(stderr) = run_incompatible_decompressor(&[]) else {
        //cov:excl-start
        eprintln!("skipping: gzip not found on PATH");
        return;
        //cov:excl-stop
    };
    assert!(
        stderr.contains("fastqrab-decompressor exited unsuccessfully")
            || stderr.contains("fastqrab-decompressor closed before sending its EOF sentinel")
            || stderr.contains("decompressor subprocess exited unsuccessfully"),
        "expected a decompressor failure error, got: {stderr}"
    );
}

/// The same incompatible binary over the forced pipe transport
/// (`FASTQRAB_DECOMP_SHM=0`) takes the `DecompressorReader` EOF path, whose
/// non-zero-exit check is the deterministic "subprocess exited unsuccessfully".
#[test]
fn test_decompressor_incompatible_binary_pipe_transport() {
    let Some(stderr) = run_incompatible_decompressor(&[("FASTQRAB_DECOMP_SHM", "0")]) else {
        //cov:excl-start
        eprintln!("skipping: gzip not found on PATH");
        return;
        //cov:excl-stop
    };
    assert!(
        stderr.contains("decompressor subprocess exited unsuccessfully"),
        "expected subprocess-exit error, got: {stderr}"
    );
}

#[test]
fn test_verify_command_missing_outputs_but_stdout() {
    // Create temp directory
    let temp_dir = tempfile::tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create test fastq file
    let mut input_file = fs::File::create(temp_path.join("input.fq")).unwrap();
    writeln!(input_file, "@read1\nACGT\n+\nIIII").unwrap();

    let mut stdout_file = fs::File::create(temp_path.join("stdout")).unwrap();
    writeln!(stdout_file, "@read1\nACGT\n+\nIIII").unwrap();

    // Create config
    let config_path = temp_path.join("config.toml");
    let mut config = fs::File::create(&config_path).unwrap();
    writeln!(
        config,
        r"[input]
read1 = 'input.fq'

[[step]]
action = 'Head'
n = 1

[output]
prefix = 'output'
[[step]]
    action = 'OutputFASTQ'
    stdout = true
"
    )
    .unwrap();

    // Don't create any output files - verify should fail

    // Run verify command - should fail due to missing expected outputs
    let verify_cmd = std::process::Command::new(get_bin_path())
        .arg("verify")
        .arg(&config_path)
        .current_dir(temp_path)
        .output()
        .unwrap();

    let stdout = std::str::from_utf8(&verify_cmd.stdout).unwrap().to_string();
    let stderr = std::str::from_utf8(&verify_cmd.stderr).unwrap().to_string();

    assert!(
        verify_cmd.status.success(),
        "Expected success, got error, stderr: {stderr}"
    );
    assert!(
        stdout.contains("Verification passed"),
        "got wrong stdout?: '{stdout}'",
    );
}
