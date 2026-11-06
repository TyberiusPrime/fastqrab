//! Guards that every `#[tpd(alias = "...")]` in the workspace is also accepted
//! by the exported JSON schema. The aliases are mirrored into the schema by
//! `config_schema()` (driven by `TpdAliasTree`); this test scrapes the source
//! independently so a missed type, a `$def`-name mismatch, or an injection bug
//! is caught rather than silently shipping a schema stricter than the parser.

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Every string the schema will accept as a key or value: property names,
/// `enum` members and `const` values, gathered recursively.
fn collect_accepted_tokens(value: &serde_json::Value, out: &mut HashSet<String>) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Array(values)) = map.get("enum") {
                out.extend(values.iter().filter_map(|v| v.as_str().map(str::to_owned)));
            }
            if let Some(c) = map.get("const").and_then(|c| c.as_str()) {
                out.insert(c.to_owned());
            }
            if let Some(serde_json::Value::Object(props)) = map.get("properties") {
                out.extend(props.keys().cloned());
            }
            for child in map.values() {
                collect_accepted_tokens(child, out);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                collect_accepted_tokens(item, out);
            }
        }
        _ => {}
    }
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = std::fs::read_dir(dir).expect("Failed to read source dir");
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            // if path.file_name().is_some_and(|n| n == "target") {
            //     continue;
            // }
            rust_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// `(alias, file)` for every `#[tpd(alias = "...")]` in the workspace's crates.
fn scan_tpd_aliases() -> Vec<(String, String)> {
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("crate dir has a parent (the workspace root)")
        .to_path_buf();
    let alias = regex::Regex::new(r#"alias\s*=\s*"([^"]+)""#).expect("regex compilation failed");
    let tpd_attr = regex::Regex::new(r"tpd\s*\(").expect("regex compilation failed");

    let mut files = Vec::new();
    for entry in std::fs::read_dir(&workspace)
        .into_iter()
        .flatten()
        .flatten()
    {
        let p = entry.path();
        if p.is_dir()
            && p.file_name()
                .is_some_and(|n| n.to_string_lossy().starts_with("fastqrab"))
        {
            rust_files(&p.join("src"), &mut files);
        }
    }

    let mut found = Vec::new();
    for file in files {
        let text = std::fs::read_to_string(&file).expect("Failed to read source file");
        for line in text.lines() {
            // only attribute lines, to avoid matching `alias = "..."` in prose/code
            if !tpd_attr.is_match(line) {
                continue;
            }
            for cap in alias.captures_iter(line) {
                found.push((cap[1].to_owned(), file.display().to_string()));
            }
        }
    }
    found
}

#[test]
fn every_tpd_alias_is_accepted_by_the_schema() {
    let schema = fastqrab_steps::config::config_schema();
    let value = serde_json::to_value(&schema).expect("schema serializes");
    let mut tokens = HashSet::new();
    collect_accepted_tokens(&value, &mut tokens);

    let aliases = scan_tpd_aliases();
    assert!(
        aliases.len() > 50,
        "expected to scan many aliases, found {} — scanning likely broke",
        aliases.len() // cov:excl-line
    );

    let mut missing: Vec<String> = aliases
        .iter()
        .filter(|(alias, _)| !tokens.contains(alias))
        .map(|(alias, file)| format!("{alias}  (declared in {file})"))
        .collect();
    missing.sort();
    missing.dedup();

    assert!(
        missing.is_empty(),
        "these `#[tpd(alias)]` values are not accepted by the schema \
         (config_schema must inject them — check the type is reachable from \
         `Config` and its $def name matches):\n  {}",
        missing.join("\n  ") // cov:excl-line
    );
}
