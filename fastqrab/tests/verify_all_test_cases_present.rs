use indexmap::IndexMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

#[test]
fn all_test_cases_are_generated() {
    let generated = fs::read_to_string("tests/generated.rs").expect("Failed to read generated.rs");

    let mut expected_tests = HashSet::new();
    for search_dir in &[
        PathBuf::from("../test_cases"),
        PathBuf::from("../cookbooks"),
    ] {
        assert!(
            search_dir.exists(),
            "{} directory does not exist",
            search_dir.display() // cov:excl-line
        );

        let mut counters = IndexMap::new();

        for entry in WalkDir::new(search_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_name().to_string_lossy().starts_with("input")
                    && e.file_name().to_string_lossy().ends_with(".toml")
            })
        {
            let case_dir = entry.path().parent().unwrap().to_owned();
            if case_dir
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("actual")
            {
                continue; // cov:excl-line
            }
            {
                *counters.entry(case_dir.clone()).or_insert(0) += 1;
            }
            let count = counters.get(&case_dir).expect("just set");

            let name = case_dir
                .strip_prefix(search_dir)
                .unwrap()
                .to_string_lossy()
                .replace(['/', '\\'], "_x_")
                .replace(|c: char| !c.is_ascii_alphanumeric() && c != '_', "_x_")
                .to_lowercase();

            let test_sh = case_dir.join("test.sh");

            if *count > 1 && !test_sh.exists() {
                expected_tests.insert(format!("fn test_cases_x_{name}_{count}()"));
            } else {
                expected_tests.insert(format!("fn test_cases_x_{name}()"));
            }
        }
    }
    for test_fn in expected_tests {
        if !generated.contains(&test_fn) {
            //cov:ignore-start
            std::process::Command::new("bash")
                .arg("./dev/ci/update_generated.sh")
                .current_dir("..")
                .status()
                .expect("Failed to run update_generated.sh when test cases were missing");
            panic!(
                "Missing test function: {test_fn}. Reran ./dev/ci/update_generated.sh, so it should just work if you try again",
            );
            //cov:ignore-stop
        }
    }
}

#[test]
#[cfg(target_os = "linux")]
fn verify_all_shell_scripts_pass_shellcheck() {
    let shellcheck = std::process::Command::new("shellcheck")
        .arg("--version")
        .output();

    assert!(
        shellcheck.is_ok() && shellcheck.expect("shellcheck failure").status.success(),
        "shellcheck not available"
    );

    for search_dir in &[
        PathBuf::from("../test_cases"),
        PathBuf::from("../cookbooks"),
        PathBuf::from("tests"),
    ] {
        assert!(search_dir.exists(), "Search dir not present {search_dir:?}");

        for entry in WalkDir::new(search_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_name().to_string_lossy().ends_with(".sh"))
        {
            let content = std::fs::read_to_string(entry.path()).expect("Failed to read file");

            if content.starts_with("#!/usr/bin/env python3") {
                continue;
            }

            let output = std::process::Command::new("shellcheck")
                .arg(entry.path())
                .output()
                .expect("Failed to run shellcheck");

            assert!(
                output.status.success(),
                "shellcheck failed for {} .\nstdout: {}\nstderr: {}",
                entry.path().display(),                  // cov:excl-line
                String::from_utf8_lossy(&output.stdout), // cov:excl-line
                String::from_utf8_lossy(&output.stderr)  // cov:excl-line
            );
        }
    }
}

#[test]
fn verify_coobooks_censored() {
    let homes_re = regex::Regex::new("/home/([^/]+)").expect("regex wrong");
    for search_dir in &[
        PathBuf::from("../test_cases"),
        PathBuf::from("../cookbooks"),
    ] {
        assert!(
            search_dir.exists(),
            "{} directory does not exist",
            search_dir.display() // cov:excl-line
        );

        for entry in WalkDir::new(search_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                let name = e.file_name().to_string_lossy();
                name.ends_with(".json") || name.ends_with(".html") || name.ends_with(".txt")
            })
        {
            // Some fixtures carry a text extension but hold binary data (e.g.
            // gzipped output saved as .txt); there's nothing to censor there.
            let Ok(content) = std::fs::read_to_string(entry.path()) else {
                continue;
            };
            if let Some(hit) = homes_re.captures(&content)
                && hit
                    .get(1)
                    .expect("regex can't hit without group 1")
                    .as_str()
                    != "user"
            {
                std::process::Command::new("./dev/scripts/censor_cookbooks.py")
                    .current_dir("..")
                    .status()
                    .expect("Failed to run censor_cookbooks.py ");
                panic!(
                    "found not /home/user home path in {}: {}. Automatically rerunning  ./dev/scripts/censor_cookbooks.py, test again",
                    entry.path().display(), // cov:excl-line
                    hit.get(0).expect("Regex hit = group 0 present").as_str()
                )
            }
        }
    }
}
