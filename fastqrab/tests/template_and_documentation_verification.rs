#![expect(clippy::unwrap_used, reason = "it's tests")]
use anyhow::Context;
use indexmap::IndexMap;
use regex::Regex;
use schemars::schema_for;
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::io::Write as IOWrite;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
use tempfile::{TempDir, tempdir};

use fastqrab::config::config_from_string;

static STRUCT_REGEX: OnceLock<Regex> = OnceLock::new();
static ALIAS_REGEX: OnceLock<Regex> = OnceLock::new();
static TRANSFORMATION_SCHEMA: OnceLock<serde_json::Value> = OnceLock::new();
static CONFIG_SCHEMA: OnceLock<serde_json::Value> = OnceLock::new();

/// Get all transformation names from the JSON schema
fn get_all_transformations() -> Vec<String> {
    let schema = get_transformation_schema();
    let mut transformations = Vec::new();

    // Navigate to the oneOf array in the schema

    let one_ofs = schema
        .as_object()
        .expect("schema_for! always produces an object")
        .get("oneOf")
        .expect("Transformation schema must have oneOf field");
    for entry in one_ofs
        .as_array()
        .expect("oneOf field in schema must be an array")
    {
        // `Transformation` is internally tagged on `action`, so each oneOf
        // branch carries the variant name in `properties.action.const`.
        let action_const = entry
            .get("properties")
            .expect("Could not decode schema")
            .get("action")
            .expect("Could not decode schema - no action discriminator")
            .get("const")
            .expect("Could not decode schema - action is not a const")
            .as_str()
            .expect("action const must be a string");

        if !action_const.starts_with('_') {
            transformations.push(action_const.to_string());
        }
    }

    transformations.sort();
    transformations
}

fn get_transformation_target_patterns() -> IndexMap<String, &'static str> {
    let mut patterns = IndexMap::new();

    // Dynamically discover all Rust files in src/transformations/, including subfolders
    let transformations_dir = Path::new("../fastqrab-steps/src/transformations");
    let mut rs_files = Vec::new();
    collect_rs_files(transformations_dir, &mut rs_files);
    let mut any = false;
    for path in rs_files {
        if let Ok(content) = fs::read_to_string(&path) {
            analyze_transformations_in_file(&content, &mut patterns);
            any = true;
        }
    }
    if !any {
        panic!("No Rust files found in src/transformations/");
    }

    // Handle deprecated transformations that have target fields but are deprecated
    // These should be excluded from pattern checking
    /* patterns.remove("TrimAdapterMismatchTail");
    patterns.remove("TrimPolyTail"); */

    patterns
}

fn analyze_transformations_in_file(content: &str, patterns: &mut IndexMap<String, &'static str>) {
    // Use regex to find all struct definitions with their content
    let struct_regex =
        STRUCT_REGEX.get_or_init(|| Regex::new(r"(?s)pub struct (\w+)\s*\{([^}]+)\}").unwrap());

    for captures in struct_regex.captures_iter(content) {
        let struct_name = captures.get(1).unwrap().as_str();
        let struct_body = captures.get(2).unwrap().as_str();

        // Look for segment field in the struct body (must be "pub segment:" to avoid false matches)
        if struct_body.contains("pub segment:") {
            if struct_body.contains("SegmentIndexOrAll") {
                patterns.insert(
                    struct_name.to_string(),
                    r#"segment = "read1" # Any of your input segments, or 'All'"#,
                );
            } else if struct_body.contains("SegmentIndex") {
                patterns.insert(
                    struct_name.to_string(),
                    r#"segment = "read1" # Any of your input segments"#,
                );
            }
        }
    }
}

fn check_target_pattern_in_text(text: &str, expected_pattern: &str) -> bool {
    // Check for target patterns - simplified version
    if expected_pattern.contains("Any of your input segments, or 'All'") {
        // Should contain "All" in the comment
        return text.contains("Any of your input segments, or 'All'");
    } else if expected_pattern.contains("Any of your input segments") {
        // Should contain the 4 base targets but not "All"
        return text.contains("Any of your input segments")
            && !text.contains("Any of your input segments, or 'All'");
    } else {
        unreachable!("should not be called for steps without target/segment/source");
        // Skip transformations without target fields
    }
}

#[expect(clippy::string_slice, reason = "using find")]
fn extract_section_from_template(template_content: &str, transformation: &str) -> String {
    let action_pattern = format!("# ==== {transformation} ====");
    let start = template_content.find(&action_pattern).unwrap_or_else(|| {
        panic!("Could not find section for transformation {transformation} in template.toml",)
    });
    let after_first_newline = template_content[start..]
        .find('\n')
        .map_or(template_content.len(), |pos| start + pos);
    let stop = template_content[after_first_newline..]
        .find("# =")
        .map_or(template_content.len(), |pos| after_first_newline + pos);
    template_content[after_first_newline..stop].replace("\n#", "\n")
}

#[expect(clippy::string_slice, reason = "using find")]
fn collect_actions(section: &str) -> Vec<String> {
    section
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("action")
                && let Some(first_quote) = rest.find('"')
            {
                let remaining = &rest[first_quote + 1..];
                if let Some(end_quote) = remaining.find('"') {
                    return Some(remaining[..end_quote].to_string());
                } else {
                    unreachable!("Missing end quote");
                }
            }
            None
        })
        .collect()
}

fn get_template_section_names(template_content: &str) -> Vec<(String, usize)> {
    template_content
        .lines()
        .enumerate()
        .filter_map(|(line_no, line)| {
            let trimmed = line.trim();
            let without_prefix = trimmed.strip_prefix("# ==== ")?;
            let without_suffix = without_prefix.strip_suffix(" ====")?;
            Some((without_suffix.trim().to_string(), line_no))
        })
        .collect()
}

const ACTIONS_REQUIRING_GENERIC_TAG: &[&str] = &[
    "FilterByTag",
    "TrimAtTag",
    "StoreTagInComment",
    "StoreTagLocationInComment",
    "StoreTagInFastQ",
    "StoreTagBackInSequence",
    "ReplaceTagWithLetter",
    "QuantifyTag",
    "UppercaseTag",
    "LowercaseTag",
    "StoreTagsInTable",
    "HammingCorrect",
    "ForgetTag",
    "ForgetAllTags",
    "Demultiplex",
    "ConvertRegionsToLength",
    "ConcatTags",
    "OutputBAM",
];

const ACTIONS_REQUIRING_TWO_TAGS: &[&str] = &["ConcatTags", "StoreTagInSequence", "PickTag"];

const TAG_DECLARING_CONVERT_STEPS: &[&str] = &["ConvertToRate", "ConvertRegionsToLength"];

#[expect(clippy::string_slice, reason = "using find")]
fn prep_config_to_parse(extracted_section: &str) -> String {
    // Check if this is a complete configuration that already has [input] and [output] sections
    let has_input_section = extracted_section.contains("[input]");
    let has_output_section = extracted_section.contains("[output]");

    if has_input_section && has_output_section {
        // This is already a complete configuration, return as-is
        return extracted_section.to_string();
    }

    let has_report_step = extracted_section.contains("action = \"Report\"")
        || extracted_section.contains("action = 'Report'");
    // Output is expressed as Output* steps now; the [output] section only keeps
    // `prefix`. The OutputFASTQ / OutputReport steps are appended at the very end
    // (see below) so they sit after every transformation in the pipeline.
    let request_report = has_report_step;

    // Only synthesize the sections the fragment doesn't already provide. A fragment
    // that shows just `[input]` or just `[output]` would otherwise get a duplicate
    // table when we prepend the scaffold.
    let mut config = String::new();
    if !has_input_section {
        config.push_str(
            r#"
[input]
read1 = "test_r1.fastq"
read2 = "test_r2.fastq"
"#,
        );
    }
    if !has_output_section {
        config.push_str(
            r#"
[output]
prefix = "output"
"#,
        );
    }
    config.push('\n');

    let actions = collect_actions(extracted_section);
    let needs_numeric_tag = actions
        .iter()
        .any(|a| a == "FilterByNumericTag" || a == "EvalExpression" || a == "ConvertToRate")
        | has_report_step;
    let if_tag_present =
        extracted_section.contains("if_tag =") && !extracted_section.contains("#if_tag =");
    let needs_bool_tag = actions.iter().any(|a| {
        a == "FilterByBoolTag" || a == "SwapConditional" || a == "ReverseComplementConditional"
    }) | if_tag_present;
    let needs_generic_tag = actions
        .iter()
        .any(|a| ACTIONS_REQUIRING_GENERIC_TAG.contains(&a.as_str()));
    let needs_two_tags = actions
        .iter()
        .any(|a| ACTIONS_REQUIRING_TWO_TAGS.contains(&a.as_str()));

    let provides_numeric_tag = actions.iter().any(|a| {
        matches!(
            a.as_str(),
            "CalcLength" | "CalcExpectedError" | "ConvertRegionsToLength" | "CalcKmers"
        )
    });
    let provides_bool_tag = actions.iter().any(|a| {
        matches!(
            a.as_str(),
            "TagDuplicates" | "TagOtherFileByName" | "TagOtherFileBySequence"
        )
    });
    /* let provides_any_tag = actions.iter().any(|a| {
        a.starts_with("Extract")
            || a.starts_with("Calc")
            || matches!(
                a.as_str(),
                "TagDuplicates"
                    | "TagOtherFileByName"
                    | "TagOtherFileBySequence"
                    | "HammingCorrect"
            )
    }); */

    // Track which tags we've already created to avoid duplicates
    let mut created_tags = std::collections::HashSet::new();

    if needs_numeric_tag && !provides_numeric_tag {
        config.push_str(
            r#"
                [[step]]
                    action = "CalcLength"
                    segment = "read1"
                    out_label = "mytag"
            "#,
        );
        created_tags.insert("mytag".to_string());
    } else if needs_bool_tag && !provides_bool_tag {
        config.push_str(
            r#"
                [[step]]
                    action = "TagDuplicates"
                    source = "read1"
                    out_label = "mytag"
                    false_positive_rate = 0.0
                    seed = 42
            "#,
        );
        created_tags.insert("mytag".to_string());
    } else if needs_two_tags {
        // Add two tags for transformations that require multiple input tags (e.g., ConcatTags)
        config.push_str(
            r#"
                [[step]]
                    action = "ExtractRegion"
                    segment = "read1"
                    start = 0
                    length = 3
                    out_label = "mytag"
                    anchor = "Start"

                [[step]]
                    action = "ExtractRegion"
                    segment = "read1"
                    start = 3
                    length = 3
                    out_label = "mytag2"
                    anchor = "Start"
            "#,
        );
        created_tags.insert("mytag".to_string());
        created_tags.insert("mytag2".to_string());
    } else if needs_generic_tag {
        // && !provides_any_tag {
        config.push_str(
            r#"
                [[step]]
                    action = "ExtractRegion"
                    segment = "read1"
                    start = 0
                    length = 3
                    out_label = "mytag"
                    anchor = "Start"
            "#,
        );
        created_tags.insert("mytag".to_string());
    }

    // For fragments that use in_label, create a generic tag with that name
    // Skip for actions that don't require pre-existing tags
    let skip_tag_creation = actions.iter().any(|a| {
        matches!(
            a.as_str(),
            "ForgetTag" | "ForgetAllTags" | "StoreTagsInTable"
        )
    });

    if extracted_section.contains("in_label") && !skip_tag_creation {
        // Collect all labels that already exist in the section (from out_label)
        let mut existing_labels = std::collections::HashSet::new();
        for line in extracted_section.lines() {
            if line.contains("out_label")
                && let Some(start) = line.find("out_label")
            {
                let after = &line[start..];
                if let Some(quote_start) = after.find(['\'', '"']) {
                    let quote_char = after.chars().nth(quote_start).unwrap();
                    let after_quote = &after[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find(quote_char) {
                        existing_labels.insert(after_quote[..quote_end].to_string());
                    }
                } // cov:excl-line
            }
        }

        // Extract the label name(s) from in_label fields and create appropriate tags
        for line in extracted_section.lines() {
            if line.contains("in_label") {
                // Try to extract the label value
                if let Some(start) = line.find("in_label") {
                    let after = &line[start..];
                    // Look for quoted string after in_label
                    if let Some(quote_start) = after.find(['\'', '"']) {
                        let quote_char = after.chars().nth(quote_start).unwrap();
                        let after_quote = &after[quote_start + 1..];
                        if let Some(quote_end) = after_quote.find(quote_char) {
                            let label = &after_quote[..quote_end];

                            // Skip if this label is already created in the same block or by us
                            if existing_labels.contains(label) || created_tags.contains(label) {
                                continue;
                            }

                            // Create appropriate tag type based on the action

                            write!(
                                &mut config,
                                r#"
                [[step]]
                    action = "ExtractRegion"
                    segment = "read1"
                    start = 0
                    length = 3
                    out_label = "{label}"
                    anchor = "Start"
            "#
                            )
                            .unwrap();
                            created_tags.insert(label.to_string());
                        } // cov:excl-line
                    }
                } // cov:excl-line
            }
        }
    }

    // For StoreSingleCellMatrix: create tags for cell_tag, gene_tag, umi_tag fields
    if extracted_section.contains("StoreSingleCellMatrix") {
        for field in &["cell_tag", "gene_tag", "umi_tag"] {
            for line in extracted_section.lines() {
                if line.contains(field)
                    && !line.trim_start().starts_with('#')
                    && let Some(start) = line.find(field)
                {
                    let after = &line[start..];
                    if let Some(quote_start) = after.find(['\'', '"']) {
                        let quote_char = after.chars().nth(quote_start).unwrap();
                        let after_quote = &after[quote_start + 1..];
                        if let Some(quote_end) = after_quote.find(quote_char) {
                            let label = &after_quote[..quote_end];
                            if !created_tags.contains(label) {
                                write!(
                                    &mut config,
                                    r#"
[[step]]
    action = "ExtractRegion"
    segment = "read1"
    start = 0
    length = 8
    out_label = "{label}"
    anchor = "Start"
"#
                                )
                                .unwrap();
                                created_tags.insert(label.to_string());
                            }
                        } // cov:excl-line
                    }
                }
            }
        }
        // Add barcodes sections for cell_barcodes and gene_barcodes if not already present
        for field in &["cell_barcodes", "gene_barcodes"] {
            for line in extracted_section.lines() {
                if line.contains(field)
                    && !line.trim_start().starts_with('#')
                    && let Some(start) = line.find(field)
                {
                    let after = &line[start..];
                    if let Some(eq_pos) = after.find('=') {
                        let after_eq = &after[eq_pos + 1..];
                        if let Some(quote_start) = after_eq.find(['\'', '"']) {
                            let quote_char = after_eq.chars().nth(quote_start).unwrap();
                            let after_quote = &after_eq[quote_start + 1..];
                            if let Some(quote_end) = after_quote.find(quote_char) {
                                let name = &after_quote[..quote_end];
                                let section_key = format!("[barcodes.{name}]");
                                if !extracted_section.contains(&section_key)
                                    && !config.contains(&section_key)
                                {
                                    write!(
                                        &mut config,
                                        r"
[barcodes.{name}]
    'AAAAAAAA' = 'entry-1'
    'CCCCCCCC' = 'entry-2'
"
                                    )
                                    .unwrap();
                                }
                            } // cov:excl-line
                        } // cov:excl-line
                    } // cov:excl-line
                }
            }
        }
    }

    // An OutputReport step is only valid when a Report step feeds it. A fragment
    // that only shows the OutputReport step needs one synthesized before it.
    let fragment_has_output_report = extracted_section.contains("action = \"OutputReport\"")
        || extracted_section.contains("action = 'OutputReport'");
    if fragment_has_output_report && !has_report_step {
        config.push_str(
            r#"
[[step]]
    action = "Report"
    name = "report"
    count = true
"#,
        );
    }

    config.push_str(extracted_section);

    let declares_tag = actions.iter().any(|a| {
        a.starts_with("Extract")
            || a.starts_with("Calc")
            || TAG_DECLARING_CONVERT_STEPS.contains(&a.as_str())
            || matches!(
                a.as_str(),
                "TagDuplicates"
                    | "TagOtherFileByName"
                    | "TagOtherFileBySequence"
                    | "HammingCorrect"
                    | "EvalExpression"
            )
    });
    let already_stores_tags = actions.iter().any(|a| a == "StoreTagsInTable");
    // Also check if the section contains out_label (for fragments that declare tags)
    // Only count uncommented out_label lines
    let has_out_label = extracted_section.lines().any(|line| {
        let trimmed = line.trim();
        !trimmed.starts_with('#') && trimmed.contains("out_label")
    });
    if (declares_tag || has_out_label) && !already_stores_tags {
        config.push_str(
            r#"
                [[step]]
                    action = "StoreTagsInTable"
                    infix = "tags"
                    compression = "Raw"
            "#,
        );
    }

    // Output steps go last so they sit after every transformation (mirrors the
    // legacy `[output]` behaviour, expressed via Output* steps). Skip a kind of
    // output the fragment already declares itself, so we don't create a second
    // writer to the same file.
    // Match both quote styles (fragments use single quotes, collect_actions only
    // sees double-quoted actions).
    let declares_action = |name: &str| {
        extracted_section.contains(&format!("action = \"{name}\""))
            || extracted_section.contains(&format!("action = '{name}'"))
    };
    let declares_record_output = declares_action("OutputFASTQ")
        || declares_action("OutputFASTA")
        || declares_action("OutputBAM");
    let declares_report_output = declares_action("OutputReport");
    if !declares_record_output {
        config.push_str(
            r#"
[[step]]
    action = "OutputFASTQ"
    compression = "raw"
"#,
        );
    }
    if request_report && !declares_report_output {
        config.push_str(
            r#"
[[step]]
    action = "OutputReport"
    json = true
    html = false
"#,
        );
    }
    config
}

/// Get or generate the full JSON schema for the Config type, `$defs`/`definitions`
/// and all. Kept around (rather than just the `Transformation` sub-schema) because
/// resolving a variant's `$ref` requires looking siblings up in this map.
fn get_config_schema() -> &'static serde_json::Value {
    CONFIG_SCHEMA.get_or_init(|| {
        let config_schema = schema_for!(fastqrab::config::Config);
        serde_json::to_value(&config_schema).expect("Failed to convert config schema to JSON")
    })
}

fn get_schema_definitions() -> &'static serde_json::Map<String, serde_json::Value> {
    get_config_schema()
        .get("$defs")
        .or_else(|| get_config_schema().get("definitions"))
        .and_then(|d| d.as_object())
        .expect("Config schema does not contain definitions")
}

/// Get or generate the JSON schema for Transformation enum
/// We extract this from the Config schema since Transformation is not publicly exported
fn get_transformation_schema() -> &'static serde_json::Value {
    TRANSFORMATION_SCHEMA.get_or_init(|| {
        get_schema_definitions()
            .get("Transformation")
            .expect("Transformation not found in schema definitions")
            .clone()
    })
}

/// Extract field names from the JSON schema for a given transformation variant
/// Returns a map of field name -> list of aliases (empty if no aliases)
fn extract_schema_fields_with_aliases(transformation: &str) -> IndexMap<String, Vec<String>> {
    let schema = get_transformation_schema();
    let mut field_map = IndexMap::new();

    // Get fields from schema - the oneOf array is at the top level
    let one_ofs = schema
        .get("oneOf")
        .and_then(|o| o.as_array())
        .expect("Schema does not contain oneOf array");

    // Distinguishes "resolved to a struct with zero fields" (e.g. a marker
    // transformation like ValidateReadNamesPrintable, whose only field is
    // `#[schemars(skip)]`'d) from "couldn't resolve the variant's fields at
    // all" (the $ref-resolution assumption below broke) — only the latter is
    // a bug worth panicking over.
    let mut resolved = false;

    for variant in one_ofs {
        if let Some(action_const) = variant
            .get("properties")
            .and_then(|p| p.get("action"))
            .and_then(|a| a.get("const"))
            .and_then(|c| c.as_str())
            && action_const == transformation
        {
            // A variant's own `properties` only ever carries the `action` tag.
            // schemars represents the rest of an internally-tagged variant's
            // fields via a sibling `$ref` into `$defs`/`definitions`, so that's
            // where the actual field list has to come from. A struct with no
            // (non-skipped) fields, e.g. ValidateReadNamesPrintable, resolves to
            // a def with no `properties` key at all — that's still "resolved",
            // just to zero fields.
            if let Some(def) = variant
                .get("$ref")
                .and_then(|r| r.as_str())
                .and_then(|r| r.rsplit('/').next())
                .and_then(|def_name| get_schema_definitions().get(def_name))
            {
                resolved = true;
                if let Some(def_properties) = def.get("properties").and_then(|p| p.as_object()) {
                    for field_name in def_properties.keys() {
                        if field_name != "action" {
                            field_map.insert(field_name.clone(), Vec::new());
                        }
                    }
                }
            } // cov:excl-line
            break;
        }
    }

    assert!(
        resolved,
        "Schema for transformation '{transformation}' has no resolvable field list — \
         extract_schema_fields_with_aliases's schemars $ref-resolution assumption may no \
         longer hold; check get_transformation_schema()'s output shape."
    );

    // Now try to extract aliases from Rust source
    if let Some(aliases_map) = extract_field_aliases_from_source(transformation) {
        for (field, aliases) in aliases_map {
            if let Some(field_aliases) = field_map.get_mut(&field) {
                *field_aliases = aliases;
            }
        }
    } // cov:excl-line

    field_map
}

/// Whether a (trimmed) struct-body line declares a field, e.g. `pub segment: SegmentIndex,`
/// or `target: ResolvedSourceAll,` — schemars serializes fields regardless of Rust
/// visibility, so a private field is just as much a schema field as a `pub` one.
fn is_field_declaration_line(line: &str) -> bool {
    let candidate = line
        .trim_start_matches("pub(crate) ")
        .trim_start_matches("pub ");
    if candidate.starts_with('#') || candidate.starts_with("//") || candidate.starts_with('}') {
        return false;
    }
    let Some((name, _rest)) = candidate.split_once(':') else {
        return false;
    };
    let name = name.trim();
    !name.is_empty()
        && name.starts_with(|c: char| c.is_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Extract field aliases from Rust source code for a given transformation
/// Returns a map of `field_name` -> Vec<alias>
#[expect(clippy::string_slice, reason = "using find")]
fn extract_field_aliases_from_source(
    transformation: &str,
) -> Option<IndexMap<String, Vec<String>>> {
    // Find the struct file
    let struct_file = find_struct_file_for_transformation(transformation)
        .with_context(|| format!("Could not find struct file for {transformation}"))
        .unwrap();
    let content = fs::read_to_string(&struct_file).ok()?;

    let mut aliases_map: IndexMap<String, Vec<String>> = IndexMap::new();
    let lines: Vec<&str> = content.lines().collect();

    // Find the struct definition
    let struct_start = lines
        .iter()
        .position(|line| line.contains("pub struct") && line.contains('{'))?;

    // Look for field definitions and their preceding attributes
    let mut i = struct_start + 1;
    while i < lines.len() {
        let line = lines[i].trim();

        // Stop at closing brace
        if line.starts_with('}') {
            break;
        }

        // Check if this is a field definition
        if is_field_declaration_line(line) {
            let field_name = line
                .trim_start_matches("pub(crate) ")
                .trim_start_matches("pub ")
                .split(':')
                .next()
                .unwrap()
                .trim();

            // Walk back to find the start of this field's attribute block (the
            // previous field, or the struct definition), then regex-search that
            // whole block for `alias = "..."` in one pass. This has to be a block
            // search rather than a per-line one: `#[tpd(...)]` attributes routinely
            // span multiple lines (e.g. `#[tpd(\n  with = "...",\n  alias = "...",\n)]`)
            // and `alias` isn't always the first key inside the parens, so a
            // substring check like `line.contains("#[tpd(alias")` misses both cases.
            let mut block_start = struct_start + 1;
            for j in (struct_start + 1..i).rev() {
                let attr_line = lines[j].trim();
                if is_field_declaration_line(attr_line) || attr_line.contains("pub struct") {
                    block_start = j + 1;
                    break;
                }
            }
            let attr_block = lines[block_start..i].join("\n");
            let alias_regex =
                ALIAS_REGEX.get_or_init(|| Regex::new(r#"alias\s*=\s*"([^"]+)""#).unwrap());
            let aliases: Vec<String> = alias_regex
                .captures_iter(&attr_block)
                .map(|c| c[1].to_string())
                .collect();

            if !aliases.is_empty() {
                aliases_map.insert(field_name.to_string(), aliases);
            }
        }

        i += 1;
    }

    Some(aliases_map)
}

/// Index of struct name -> defining file, built once by scanning every `.rs`
/// file under `fastqrab-steps/src/transformations` for `pub struct Name { ... }`.
/// This replaces a previous approach that *guessed* the file path from the
/// struct name (camelCase -> snake_case, plus half a dozen hardcoded
/// exceptions) — that guessing was also rooted at the wrong directory
/// (`src/transformations` instead of `../fastqrab-steps/src/transformations`),
/// so it silently found nothing for every transformation.
static STRUCT_FILE_INDEX: OnceLock<IndexMap<String, PathBuf>> = OnceLock::new();

fn struct_file_index() -> &'static IndexMap<String, PathBuf> {
    STRUCT_FILE_INDEX.get_or_init(|| {
        let transformations_dir = Path::new("../fastqrab-steps/src/transformations");
        let mut rs_files = Vec::new();
        collect_rs_files(transformations_dir, &mut rs_files);
        let struct_regex =
            STRUCT_REGEX.get_or_init(|| Regex::new(r"(?s)pub struct (\w+)\s*\{([^}]+)\}").unwrap());

        let mut index = IndexMap::new();
        for path in rs_files {
            let content = fs::read_to_string(&path).expect("Could not read file");
            for captures in struct_regex.captures_iter(&content) {
                let name = captures.get(1).unwrap().as_str().to_string();
                index.entry(name).or_insert_with(|| path.clone());
            }
        }
        index
    })
}

/// Find the struct file for a transformation (enum name).
fn find_struct_file_for_transformation(transformation: &str) -> Option<PathBuf> {
    let transformations_content =
        fs::read_to_string("../fastqrab-steps/src/transformations.rs").unwrap();
    let (_, after_enum) = transformations_content.split_once("pub enum Transformation {")?;
    let (enum_content, _) = after_enum.split_once("\n}\n")?;

    let variant_prefix = format!("{transformation}(");
    for line in enum_content.lines() {
        let Some((_, after_variant)) = line.split_once(&variant_prefix) else {
            continue;
        };
        let (module_path, _) = after_variant.split_once(')').expect("missing )");
        // Strip a `Box<...>` wrapper (e.g. `Box<convert::EvalExpression>`) before
        // taking the last `::`-separated segment as the struct name.
        let struct_name = module_path
            .trim_start_matches("Box<")
            .trim_end_matches('>')
            .rsplit("::")
            .next()
            .unwrap_or(module_path);
        if let Some(path) = struct_file_index().get(struct_name) {
            return Some(path.clone());
        } // cov:excl-line
    } // cov:excl-line

    None // cov:excl-line
}

/// Check if a field (or any of its aliases) is documented in the given text
fn is_field_in_text(text: &str, field_name: &str, aliases: &[String]) -> bool {
    // Check if the field name or any alias is documented
    let names_to_check = std::iter::once(field_name).chain(aliases.iter().map(String::as_str));

    for name in names_to_check {
        // Look for the name followed by = or :
        // This handles both TOML (field = value) and markdown (field: description)
        let patterns = [
            format!("{name} ="),
            format!("{name}:"),
            format!("`{name}`"),
            format!("**{name}**"),
        ];

        if patterns.iter().any(|pattern| text.contains(pattern)) {
            return true;
        }
    }

    false // cov:excl-line
}

#[test]
fn test_every_step_has_a_template_section() {
    // Get all transformation names from the enum
    let transformations = get_all_transformations();

    // Read the template file
    let template_content =
        fs::read_to_string("src/template.toml").expect("Failed to read template.toml");

    // Test parsing configuration with each transformation and check target patterns
    let target_patterns = get_transformation_target_patterns();
    let mut errors = Vec::new();
    let transformations_set: HashSet<_> = transformations.iter().cloned().collect();
    let mut documented_sections = HashSet::new();

    let (td, ref_file) = prep_temp_reference_fasta();

    for (section_name, line_no) in get_template_section_names(&template_content) {
        if !documented_sections.insert(section_name.clone()) {
            //cov:excl-start
            errors.push(format!(
                "Duplicate section {section_name} found in template.toml"
            ));
            continue;
            //cov:excl-stop
        }

        let extracted_section = match extract_section_from_template(
            &template_content,
            &section_name,
        ) {
            section if section.is_empty() => {
                //cov:excl-start
                errors.push(format!(
                        "Failed to extract section for {section_name}, line_no {line_no} from template.toml"
                    ));
                continue;
                //cov:excl-stop
            }
            section => section,
        };

        // Check target pattern consistency if transformation has a target field
        if let Some(expected_pattern) = target_patterns.get(&section_name)
            && !check_target_pattern_in_text(&extracted_section, expected_pattern)
        //cov:excl-start
        {
            errors.push(format!(
                    "Template section for {section_name}, line_no {line_no} does not contain the correct target pattern.\nExpected pattern like: {expected_pattern}\nActual section:\n{extracted_section}"
                ));
        }
        //cov:excl-stop

        let extracted_section =
            extracted_section.replace("reference.fa", ref_file.to_str().unwrap());
        let config = prep_config_to_parse(&extracted_section);

        // Verify just the parsing
        //
        //
        match config_from_string(&config) {
            Ok(parsed) => {
                if let Err(e) = parsed.check(false) {
                    //cov:excl-start
                    errors.push(format!(
                        "Error in parsing configuration for {section_name}, line_no {line_no}: {e:?}\n{config}",
                    ));
                    //cov:excl-stop
                }
            }
            //cov:excl-start
            Err(e) => {
                errors.push(format!(
                    "Could not parse section for {section_name}, line_no {line_no}: {}.\n{config}",
                    e.pretty("debug.toml")
                ));
            } //cov:excl-stop
        }

        // Check that all struct fields are documented in the template section
        let fields_with_aliases = extract_schema_fields_with_aliases(&section_name);
        for (field, aliases) in &fields_with_aliases {
            if !is_field_in_text(&extracted_section, field, aliases) {
                //cov:excl-start
                let alias_info = if aliases.is_empty() {
                    String::new()
                } else {
                    format!(" (or alias: {})", aliases.join(", "))
                };
                errors.push(format!(
                    "Template section for {section_name} is missing field '{field}'{alias_info} (from schema)"
                ));
                //cov:excl-stop
            }
        }
    }

    let missing: Vec<_> = transformations
        .iter()
        .filter(|transformation| !documented_sections.contains(*transformation))
        .cloned()
        .collect();
    if !missing.is_empty() {
        //cov:excl-start
        let mut missing_sorted = missing;
        missing_sorted.sort();
        errors.push(format!(
            "The following transformations are missing in template.toml:\n{}",
            missing_sorted.join(", ")
        ));
        //cov:excl-stop
    }

    let extra: Vec<_> = documented_sections
        .iter()
        .filter(|section| !transformations_set.contains(*section))
        .cloned()
        .collect();
    if !extra.is_empty() {
        //cov:excl-start
        let mut extra_sorted = extra;
        extra_sorted.sort();
        errors.push(format!(
            "The following sections document unknown transformations:\n{}",
            extra_sorted.join(", ")
        ));
        //cov:excl-stop
    }

    assert!(
        errors.is_empty(),
        "Template validation failed:\n{}",
        errors.join("\n") // cov:excl-line
    );
    drop(td);
}

fn get_all_doc_files() -> Vec<PathBuf> {
    let mut doc_files = Vec::new();

    // Include reference documentation
    let reference_dir = Path::new("../docs/content/docs/reference");
    if reference_dir.exists() {
        visit_dir_recursive(reference_dir, &mut doc_files);
    }

    // Include concept documentation
    let concepts_dir = Path::new("../docs/content/docs/concepts");
    if concepts_dir.exists() {
        visit_dir_recursive(concepts_dir, &mut doc_files);
    }

    doc_files
}

fn visit_dir_recursive(dir: &Path, doc_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                visit_dir_recursive(&path, doc_files);
            } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
                // Skip index files and general documentation
                if let Some(filename) = path.file_name().and_then(|s| s.to_str())
                    && !filename.starts_with('_')
                    && filename != "toml.md"
                    && filename != "Options.md"
                {
                    doc_files.push(path);
                }
            }
        }
    } // cov:excl-line
}

fn extract_transformation_from_filename(file_path: &Path) -> Option<String> {
    //verify it's a reference file, not a concept
    if !file_path.components().any(|c| c.as_os_str() == "reference") {
        return None;
    }
    file_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_string)
}

fn extract_toml_from_markdown(
    file_path: &Path,
) -> Result<Vec<(String, usize)>, Box<dyn std::error::Error>> {
    // Note: `not-a-transformation: true` only opts out of the transformation-specific
    // checks (action matching, target patterns, schema field presence) in the caller.
    // TOML blocks are still extracted and validated so e.g. `[output]` examples can't
    // drift away from the schema unnoticed.
    let content = fs::read_to_string(file_path).unwrap();

    let mut toml_blocks = Vec::new();
    let mut in_toml_block = false;
    let mut current_block = Vec::new();
    let mut skip_this = false;
    let mut start_line = 0;

    for (line_no, line) in content.lines().enumerate() {
        if line.trim().starts_with("```toml") {
            // Still open the block (rather than `continue`ing past it) so its own
            // closing fence sees `in_toml_block == true` and resets `skip_this`
            // there. Previously an ignored block's open was skipped entirely,
            // which left `skip_this` stuck true past its closing fence and
            // silently swallowed the *next* (non-ignored) block instead.
            skip_this = line.contains("# ignore_in_test");
            start_line = line_no;
            in_toml_block = true;
            current_block.clear();
        } else if line.trim() == "```" && in_toml_block {
            in_toml_block = false;
            if !current_block.is_empty() && !skip_this {
                toml_blocks.push((current_block.join("\n"), start_line));
            }
            skip_this = false;
        } else if in_toml_block {
            current_block.push(line);
        }
    }

    Ok(toml_blocks)
}

#[test]
fn test_every_transformation_has_documentation() {
    let transformations = get_all_transformations();
    let doc_files = get_all_doc_files();

    // Create a set of documented transformations
    let mut documented_transformations = std::collections::HashSet::new();
    for doc_file in &doc_files {
        if let Some(transformation) = extract_transformation_from_filename(doc_file) {
            documented_transformations.insert(transformation);
        }
    }

    // Check for missing documentation
    let mut missing_docs = Vec::new();
    for transformation in &transformations {
        if !documented_transformations.contains(transformation) {
            missing_docs.push(transformation.clone()); // cov:excl-line
        }
    }

    missing_docs.sort();
    assert!(
        missing_docs.is_empty(),
        "The following transformations are missing documentation files:\n{}",
        missing_docs.join(", ") // cov:excl-line
    );
}

fn prep_temp_reference_fasta() -> (TempDir, PathBuf) {
    let td = tempdir().expect("Failed to create temp directory");
    let ref_file = td.path().join("reference.fa");
    {
        let mut fh = std::fs::File::create(&ref_file).expect("Failed to create reference.fa");
        fh.write_all(b">ref1\nATGTCA")
            .expect("Failed to write test_r1.fastq");
    }
    let ref_file = ref_file
        .canonicalize()
        .expect("Failed to canonicalize reference.fa path");
    (td, ref_file)
}

#[test]
#[expect(clippy::string_slice, reason = "using find")]
fn test_documentation_toml_examples_parse() {
    let doc_files = get_all_doc_files();
    let mut failed_files = Vec::new();

    let (td, ref_file) = prep_temp_reference_fasta();

    for doc_file in &doc_files {
        let transformation = extract_transformation_from_filename(doc_file);
        let ignored = ["CalcMeanQuality.md", "benchmark-section.md", "adapters.md"];
        if ignored.contains(&doc_file.file_name().and_then(|o| o.to_str()).unwrap()) {
            continue;
        }

        // Read the markdown content once for field checking
        let markdown_content = match fs::read_to_string(doc_file) {
            Ok(content) => content,
            //cov:excl-start
            Err(e) => {
                failed_files.push(format!(
                    "{}: Failed to read file: {}",
                    doc_file.display(),
                    e
                ));
                continue;
            } //cov:excl-stop
        };

        // `not-a-transformation: true` marks reference pages that document a config
        // table (e.g. `[output]`) rather than a transformation step. It only opts out
        // of the transformation-specific checks below; TOML blocks are still parsed and
        // validated.
        let is_not_a_transformation = markdown_content.contains("not-a-transformation: true");

        // Check that all struct fields are documented in the markdown
        if let Some(transformation) = &transformation
            && !is_not_a_transformation
        {
            let fields_with_aliases = extract_schema_fields_with_aliases(transformation);
            for (field, aliases) in &fields_with_aliases {
                if !is_field_in_text(&markdown_content, field, aliases) {
                    //cov:excl-start
                    let alias_info = if aliases.is_empty() {
                        String::new()
                    } else {
                        format!(" (or alias: {})", aliases.join(", "))
                    };
                    failed_files.push(format!(
                        "{}: Documentation is missing field '{field}'{alias_info} (from schema)",
                        doc_file.display()
                    ));
                    //cov:excl-stop
                }
            }
        }

        match extract_toml_from_markdown(doc_file) {
            Ok(toml_blocks) => {
                // Transformation reference pages must ship at least one example.
                // Non-transformation pages (prose-only reference like threading.md) need
                // not, but any blocks they do contain are still validated below.
                if toml_blocks.is_empty()
                    && !is_not_a_transformation
                    && !ignored.contains(&doc_file.file_name().and_then(|o| o.to_str()).unwrap())
                    && !doc_file.components().any(|c| c.as_os_str() == "concepts")
                {
                    failed_files.push(format!("{}: No TOML examples found", doc_file.display())); // cov:excl-line
                    continue; // cov:excl-line
                }

                let target_patterns = get_transformation_target_patterns();

                for (i, (toml_block, start_line_no)) in toml_blocks.iter().enumerate() {
                    if toml_block.contains("# ignore_in_test") {
                        continue;
                    }

                    // Skip validation for concept files that contain TOML fragments rather than complete configurations
                    let is_concept_file =
                        doc_file.components().any(|c| c.as_os_str() == "concepts");

                    // For concept files, skip the specific transformation matching since they contain examples
                    // using multiple transformations, but still validate TOML parsing.
                    // Likewise skip it for `not-a-transformation` pages, whose blocks
                    // configure a table rather than a step.
                    if let Some(transformation) = &transformation
                        && !is_not_a_transformation
                    {
                        if !is_concept_file
                            && !toml_block.contains(&format!("action = \"{transformation}\""))
                            //cov:excl-start
                            && !toml_block.contains("[barcodes.")
                        {
                            failed_files.push(format!(
                                "{}: TOML block {}, line: {start_line_no} does not contain action = \"{transformation}\"",
                                doc_file.display(),
                                i + 1,
                            ));
                            continue;
                            //cov:excl-stop
                        }

                        // Check target pattern consistency if transformation has a target field
                        // Skip this check for concept files since they contain examples using multiple transformations
                        if !is_concept_file
                            && let Some(expected_pattern) = target_patterns.get(&transformation[..])
                            && !check_target_pattern_in_text(toml_block, expected_pattern)
                        //cov:excl-start
                        {
                            failed_files.push(format!(
                                    "{}: TOML block {}, line: {start_line_no} does not contain the correct target pattern.\nExpected pattern like: {}\nActual block:\n{}",
                                    doc_file.display(),
                                    i + 1,
                                    expected_pattern,
                                    toml_block
                                ));
                        }
                        //cov:excl-stop
                    }

                    let toml_block = toml_block.replace(
                        "reference.fa",
                        ref_file
                            .to_str()
                            .expect("Failed to convert reference.fa path to string"),
                    );
                    let config = prep_config_to_parse(&toml_block);

                    // Try to parse the configuration
                    match config_from_string(&config) {
                        Ok(parsed_config) => {
                            if let Err(e) = parsed_config.check(false) {
                                //cov:excl-start
                                failed_files.push(format!(
                                    "{}: TOML block {}, line: {start_line_no} failed validation: {:?}\n{}",
                                    doc_file.display(),
                                    i + 1,
                                    e,
                                    config,
                                ));
                                //cov:excl-stop
                            }
                        }
                        //cov:excl-start
                        Err(e) => {
                            failed_files.push(format!(
                                "{}: TOML block {}, line: {start_line_no} failed to parse: {}",
                                doc_file.display(),
                                i + 1,
                                e.pretty("debug.toml")
                            ));
                        } //cov:excl-stop
                    }
                }
            }
            //cov:excl-start
            Err(e) => {
                failed_files.push(format!(
                    "{}: Failed to read file: {}",
                    doc_file.display(),
                    e
                ));
            } //cov:excl-stop
        }
    }

    assert!(
        failed_files.is_empty(),
        "Documentation TOML validation failed:\n{}",
        failed_files.join("\n") // cov:excl-line
    );
    drop(td);
}

#[test]
fn test_llm_guide_covers_all_transformations() {
    let transformations = get_all_transformations();
    let llm_guide_path = Path::new("../docs/content/docs/reference/llm-guide.md");

    // Check if the file exists
    assert!(
        llm_guide_path.exists(),
        "LLM guide not found at {}",
        llm_guide_path.display() // cov:excl-line
    );

    // Read the LLM guide
    let llm_guide_content =
        fs::read_to_string(llm_guide_path).expect("Failed to read llm-guide.md");

    let mut errors = Vec::new();
    let mut documented_transformations = HashSet::new();

    // Check for each transformation in the LLM guide
    for transformation in &transformations {
        // Look for the transformation name in various contexts:
        // 1. As a heading: "### TransformationName"
        // 2. In action field: action = 'TransformationName'
        // 3. As a step reference
        let heading_pattern = format!("### {transformation}");
        let action_pattern_single = format!("action = '{transformation}'");

        if llm_guide_content.contains(&heading_pattern)
            || llm_guide_content.contains(&action_pattern_single)
        // intentionally ' only
        {
            documented_transformations.insert(transformation.clone());
        } else {
            //cov:excl-start
            errors.push(format!(
                "Transformation '{transformation}' is not documented in llm-guide.md"
            ));
        }
        //cov:excl-stop
    }

    // Report missing transformations
    if !errors.is_empty() {
        //cov:excl-start
        let mut missing_sorted = errors;
        missing_sorted.sort();
        panic!(
            "LLM guide validation failed:\n{}",
            missing_sorted.join("\n")
        );
        //cov:excl-stop
    }

    // Verify we found a reasonable number of transformations
    assert!(
        documented_transformations.len() == transformations.len(),
        "LLM guide coverage is too low: (documented {}/{} transformations)",
        documented_transformations.len(), // cov:excl-line
        transformations.len()             // cov:excl-line
    );
}

fn extract_toml_blocks_from_llm_guide(content: &str) -> Vec<(String, usize)> {
    let mut toml_blocks = Vec::new();
    let mut in_toml_block = false;
    let mut current_block = Vec::new();
    let mut start_line_no = 0;

    for (lineno, line) in content.lines().enumerate() {
        if line.trim() == "```toml" {
            in_toml_block = true;
            current_block.clear();
            start_line_no = lineno;
        } else if line.trim() == "```" && in_toml_block {
            in_toml_block = false;
            if !current_block.is_empty() {
                toml_blocks.push((current_block.join("\n"), start_line_no));
            }
        } else if in_toml_block {
            current_block.push(line);
        }
    }

    toml_blocks
}

#[test]
fn test_llm_guide_toml_examples_parse() {
    let llm_guide_path = Path::new("../docs/content/docs/reference/llm-guide.md");

    let (td, ref_file) = prep_temp_reference_fasta();

    assert!(
        llm_guide_path.exists(),
        "LLM guide not found at {}",
        llm_guide_path.display() // cov:excl-line
    );

    let llm_guide_content =
        fs::read_to_string(llm_guide_path).expect("Failed to read llm-guide.md");

    let toml_blocks = extract_toml_blocks_from_llm_guide(&llm_guide_content);
    let mut failed_examples = Vec::new();

    for (i, (toml_block, line_no)) in toml_blocks.iter().enumerate() {
        // Skip blocks that are marked as fragments or incomplete
        if toml_block.contains("# fragment")
            || toml_block.contains("# incomplete")
            || toml_block.contains("# example-only")
        {
            continue;
        }

        // Check if this is a complete configuration (has [input] and [output])
        let has_input = toml_block.contains("[input]");
        let has_output = toml_block.contains("[output]");
        let toml_block = toml_block.replace("reference.fa", ref_file.to_str().unwrap());

        if !has_input || !has_output {
            // This is a partial example, wrap it with minimal config
            let config = prep_config_to_parse(&toml_block);

            match config_from_string(&config) {
                Ok(parsed_config) => {
                    if let Err(e) = parsed_config.check(false) {
                        //cov:excl-start
                        failed_examples.push(format!(
                            "LLM guide TOML block {} , line_no {line_no}failed validation: {:?}\nBlock:\n{}",
                            i + 1,
                            e,
                            toml_block
                        ));
                        //cov:excl-stop
                    }
                }
                //cov:excl-start
                Err(e) => {
                    failed_examples.push(format!(
                        "LLM guide TOML block {}, line_no {line_no} failed to parse: {}\nBlock:\n{}",
                        i + 1,
                        e.pretty("template.toml"),
                        toml_block
                    ));
                } //cov:excl-stop
            }
        } else {
            // This is a complete configuration, parse directly
            match config_from_string(&toml_block) {
                Ok(parsed_config) => {
                    if let Err(e) = parsed_config.check(false) {
                        //cov:excl-start
                        failed_examples.push(format!(
                            "LLM guide complete config block {}, line_no {line_no} failed validation: {:?}\nBlock:\n{}",
                            i + 1,
                            e,
                            toml_block
                        ));
                        //cov:excl-stop
                    }
                }
                //cov:excl-start
                Err(e) => {
                    failed_examples.push(format!(
                        "LLM guide complete config block {}, line_no {line_no} failed to parse: {}\nBlock:\n{}",
                        i + 1,
                        e.pretty("template.toml"),
                        toml_block
                    ));
                } //cov:excl-stop
            }
        }
    }

    assert!(
        failed_examples.is_empty(),
        "LLM guide TOML examples validation failed:\n{}",
        failed_examples.join("\n\n") // cov:excl-line
    );
    drop(td);
}

#[test]
fn test_hugo_builds_documentation_site() {
    let temp_destination =
        tempdir().expect("Failed to allocate temporary directory for Hugo output");
    let mut command = Command::new("hugo");
    command
        .current_dir(Path::new(env!("CARGO_MANIFEST_DIR")))
        .arg("--source")
        .arg("../docs")
        .arg("--destination")
        .arg(temp_destination.path())
        .arg("--panicOnWarning")
        //.arg("--quiet")
        .env("HUGO_ENVIRONMENT", "production")
        .env("HUGO_ENV", "production");

    let output = match command.output() {
        Ok(output) => output,
        // complain loudly over missing hugo!
        // Err(error) if error.kind() == ErrorKind::NotFound => {
        //     eprintln!("Skipping Hugo documentation build test: `hugo` binary not found in PATH.");
        //     return;
        // }
        Err(error) => panic!("Failed to execute `hugo`: {error}"),
    };

    assert!(
        output.status.success(),
        "Hugo failed to build documentation (status {}).\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout), // cov:excl-line
        String::from_utf8_lossy(&output.stderr)  // cov:excl-line
    );
}

#[test]
fn test_every_transformation_has_benchmark() {
    let transformations = get_all_transformations();
    let benchmark_file =
        fs::read_to_string("benches/simple_benchmarks.rs").expect("Failed to read benchmark file");

    let mut missing_benchmarks = Vec::new();
    let mut found_benchmarks = Vec::new();
    let ignored = ["ForgetTag", "Inspect"];

    for transformation in &transformations {
        if ignored.contains(&transformation.as_str()) {
            continue;
        }
        // Look for the transformation in benchmark configurations
        // Check for both quoted and unquoted versions in the benchmark configs
        let quoted_pattern = format!("\"{transformation}\"",);
        let action_pattern = format!("action = \"{transformation}\"");

        if benchmark_file.contains(&quoted_pattern) || benchmark_file.contains(&action_pattern) {
            found_benchmarks.push(transformation.clone());
        } else {
            missing_benchmarks.push(transformation.clone()); // cov:excl-line
        } // cov:excl-line
    }

    if !missing_benchmarks.is_empty() {
        //cov:excl-start
        missing_benchmarks.sort();
        panic!(
            "The following transformations are missing benchmarks in simple_benchmarks.rs:\n{}",
            missing_benchmarks.join(", ")
        );
        //cov:excl-stop
    }

    println!(
        "✓ All {} transformations have benchmarks",
        transformations.len()
    );
}

#[test]
fn test_readme_toml_examples_validate() {
    // This test extracts TOML code blocks from README.md and validates them
    use std::fs;
    use std::path::Path;

    let readme_path = Path::new("../README.md");
    assert!(readme_path.exists(), "README.md not found");

    let readme_content = fs::read_to_string(readme_path).expect("Failed to read README.md");

    // Extract TOML code blocks (between ```toml and ```)
    let mut toml_blocks = Vec::new();
    let mut in_toml_block = false;
    let mut current_block = String::new();
    let mut block_start_line = 0;
    let mut line_num = 0;

    for line in readme_content.lines() {
        line_num += 1;
        if line.trim().starts_with("```toml") {
            in_toml_block = true;
            current_block.clear();
            block_start_line = line_num;
        } else if line.trim().starts_with("```") && in_toml_block {
            in_toml_block = false;
            if !current_block.trim().is_empty() {
                toml_blocks.push((block_start_line, current_block.clone()));
            }
        } else if in_toml_block {
            current_block.push_str(line);
            current_block.push('\n');
        }
    }

    assert!(
        !toml_blocks.is_empty(),
        "No TOML code blocks found in README.md"
    );

    println!("\n✓ Found {} TOML block(s) in README.md", toml_blocks.len());

    // Validate each TOML block using the same approach as the run() function
    for (line_no, toml_content) in &toml_blocks {
        println!("  Validating TOML block starting at line {line_no}...");

        let parsed = match config_from_string(toml_content) {
            Ok(config) => config,
            //cov:excl-start
            Err(e) => {
                panic!("README.md TOML block at line {line_no} failed to parse:\n{e:?}",); // cov:excl-line
            } //cov:excl-stop
        };

        // Validate the config using check() (same as in run())
        // Note: This will fail on input file validation since files don't exist,
        // but it will catch TOML syntax errors and structural issues
        match parsed.check(false) {
            Ok(_) => {
                println!("    ✓ TOML block at line {line_no} validated successfully",);
            }
            //cov:excl-start
            Err(e) => {
                let error_msg = format!("{e:?}");
                // Allow errors about missing input files, but catch everything else
                if error_msg.contains("Could not read")
                    || error_msg.contains("No such file")
                    || error_msg.contains("does not exist")
                {
                    println!(
                        "    ✓ TOML block at line {line_no} validated (structure valid, expected file errors ignored)",
                    );
                } else {
                    panic!(
                        "README.md TOML block at line {line_no} failed validation:\n{error_msg}",
                    );
                }
            } //cov:excl-stop
        }
    }

    println!("\n✓ All README.md TOML examples are valid!");
}

// #[test]
// fn test_all_transformations_are_deny_unknown_fields() {
//     // toml-pretty-deser is unknown fields by default
//
// }

fn collect_rs_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap();

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            out.push(path);
        }
    }
}

#[test]
fn test_every_link_docs_target_has_a_redirect_page() {
    // link_docs() is called two ways:
    //   1. Dynamically with step.tpd_get_tag() — covers every transformation name
    //   2. With a literal string for non-step targets (e.g. "barcodes")
    // We check both.

    let redirects_dir = Path::new("../docs/content/docs/redirects");

    // --- 1. All transformation names (dynamic call sites) ---
    let transformations = get_all_transformations();

    // --- 2. Literal link_docs("...") call sites ---
    let link_docs_re = Regex::new(r#"link_docs\(\s*"([^"]+)"\s*\)"#).unwrap();

    let src_roots = [
        Path::new("src"),
        Path::new("../fastqrab-steps/src"),
        Path::new("../fastqrab/src/cli/"),
    ];
    let mut literal_targets: Vec<(String, PathBuf)> = Vec::new();
    for root in &src_roots {
        if !root.exists() {
            panic!("Missing root {root:?}");
        }
        let mut rs_files = Vec::new();
        collect_rs_files(root, &mut rs_files);
        for file in rs_files {
            let content = fs::read_to_string(&file).unwrap();
            for cap in link_docs_re.captures_iter(&content) {
                literal_targets.push((cap[1].to_string(), file.clone())); // cov:excl-line
            } // cov:excl-line 
        } // cov:excl-line
    }

    // --- Check everything ---
    let mut missing: Vec<String> = Vec::new();

    for name in &transformations {
        if !redirects_dir.join(format!("{name}.md")).exists() {
            //cov:excl-start
            missing.push(format!(
                "  {name}  (transformation — run dev/update_generated.sh)"
            ));
            //cov:excl-stop
        }
    }
    for (target, source_file) in &literal_targets {
        //currently empty
        //cov:excl-start
        if !redirects_dir.join(format!("{target}.md")).exists() {
            missing.push(format!(
                "  {target}  (literal call in {})",
                source_file.display()
            ));
        }
        //cov:excl-stop
        unreachable!(
            "Currently empty - no longer true, go and verify the link doc test actually works"
        );
    }

    if !missing.is_empty() {
        //cov:excl-start
        missing.sort();
        missing.dedup();
        panic!(
            "The following link_docs() targets have no redirect page in docs/reference/redirects/.\n\
             Run dev/update_generated.sh to regenerate, or add a doc page for missing entries:\n{}",
            missing.join("\n")
        );
        //cov:excl-stop
    }

    println!(
        "✓ All {} transformation + {} literal link_docs() targets have redirect pages",
        transformations.len(),
        literal_targets.len()
    );
}

#[test]
#[expect(clippy::string_slice, reason = "using find")]
fn test_flake_rust_version_matches_msrv() {
    // Verify that the Rust version used in flake.nix exactly matches the MSRV declared in Cargo.toml.
    // This ensures we actually build and test on the minimum supported version.

    // Read Cargo.toml and extract rust-version
    let cargo_toml_path = Path::new("../Cargo.toml");
    let cargo_content = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");

    let msrv = cargo_content
        .lines()
        .find(|line| line.trim().starts_with("rust-version"))
        .and_then(|line| {
            let after_eq = line.split('=').nth(1)?;
            let trimmed = after_eq.trim().trim_matches('"');
            Some(trimmed.to_string())
        })
        .expect("Could not find rust-version in Cargo.toml");

    // Read flake.nix and extract Rust version
    let flake_path = Path::new("../flake.nix");
    let flake_content = fs::read_to_string(flake_path).expect("Failed to read flake.nix");

    // Look for pattern like: rust = pkgs.rust-bin.stable."1.90.0".default
    let flake_rust_version = flake_content
        .lines()
        .find(|line| line.contains("rust-bin.stable.") && line.contains("default"))
        .and_then(|line| {
            // Extract version between quotes after "stable."
            let after_stable = line.split("stable.").nth(1)?;
            let version_start = after_stable.find('"')? + 1;
            let after_first_quote = &after_stable[version_start..];
            let version_end = after_first_quote.find('"')?;
            Some(after_first_quote[..version_end].to_string())
        })
        .expect("Could not find rust-bin.stable version in flake.nix");

    assert_eq!(
        flake_rust_version, msrv,
        "flake.nix uses Rust {flake_rust_version} but Cargo.toml declares rust-version = \"{msrv}\". \
         These must match to ensure we build and test on the declared MSRV."
    );

    println!("✓ flake.nix Rust version and MSRV both set to {msrv}");
}
