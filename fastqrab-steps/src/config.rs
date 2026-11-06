use crate::input_files::InputDeclaration;
use crate::transformations::hamming_correct::{OnTie, Partial_HammingPreMatch, PreMatchData};
use crate::transformations::hamming_exact_counter::PartialHammingExactCounter;
use crate::transformations::{PartialTransformation, TagUser, Transformation};
use anyhow::{Result, anyhow, bail};
use bstr::BString;
use fastqrab_config::{
    RemovedTags, StringTagContent, TagLabel, TagValueType, default_max_molecules_in_flight,
};
use fastqrab_config::{
    default_block_size, default_buffer_size, default_output_buffer_size,
    default_spot_check_read_pairing,
};
use fastqrab_io::CompressionFormat;
use fastqrab_io::io::output::chunked_writer::OutputDeclaration;
use fastqrab_io::io::{self, DetectedInputFormat};
use indexmap::IndexMap;
use schemars::JsonSchema;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt::Write;
use std::num::NonZero;
use std::path::Path;
use std::rc::Rc;
use toml_pretty_deser::prelude::*;

mod barcodes;
mod input;
pub mod options;
mod output;
mod segments;
mod tag_labels;

pub use fastqrab_config::segments::{
    ResolvedSourceAll, ResolvedSourceNoAll, SegmentIndex, SegmentIndexOrAll, SegmentOrNameIndex,
};
pub use fastqrab_config::{offer_alternatives, validate_tag_name};
use fastqrab_io::get_number_of_cores;
pub use input::{Input, PartialInput, StructuredInput};

pub use barcodes::{Barcodes, BarcodesFromFile, PartialBarcodes};
pub use options::{Options, PartialOptions};
pub use output::{Output, PartialOutput};
pub use segments::{DenyName, ValidateSegment};
pub use tag_labels::ValidateTagLabel;

#[derive(Debug)]
pub struct TagMetadata {
    pub used: bool,
    pub tag_type: TagValueType,
    pub contents: StringTagContent,
    pub span: std::ops::Range<usize>,
    /// For `Location` tags, the segment the tag's bytes live on (see
    /// [`DeclaredTag::segment`](fastqrab_config::DeclaredTag::segment)). Lets a
    /// conditional `Swap` identify which location tags it must forget.
    pub segment: Option<SegmentIndex>,
}

pub fn config_from_string(toml: &str) -> Result<Config, DeserError<PartialConfig>> {
    Config::tpd_from_toml(toml, FieldMatchMode::AnyCase, VecMode::SingleOk)
}

/// Validates that a segment label conforms to the pattern [a-zA-Z0-9_]+
/// (one or more alphanumeric characters or underscores)
pub fn validate_segment_label(
    label: &str,
    match_mode: toml_pretty_deser::prelude::FieldMatchMode,
) -> Result<()> {
    if label.is_empty() {
        bail!(
            "Segment name may not be empty or just whitespace. Please provide a segment name containing only letters, numbers, and underscores."
        );
    }

    for (i, ch) in label.chars().enumerate() {
        if i == 0 && !ch.is_ascii_alphabetic() && ch != '_' {
            bail!("Segment label must start with a letter or underscore (^[a-zA-Z_]), got '{ch}'",);
        }
        if !ch.is_ascii_alphanumeric() && ch != '_' {
            bail!(
                "Segment label must contain only letters, numbers, and underscores (^[a-zA-Z0-9_]+$), found '{ch}'.",
            );
        }
    }
    for prohibited in &[
        "fasta_fake_quality",
        "bam_include_mapped",
        "bam_include_unmapped",
        "read_comment_character",
        "threads_per_segment",
        "tpd_field_match_mode",
    ] {
        if match_mode.matches(label, prohibited) {
            bail!(
                "'{prohibited}' is not allowed as a segment label, as it could be confused with an existing option name or an internal. Please choose a different segment name, or prefix in with 'options.' if you meant the option."
            );
        }
    }

    Ok(())
}

#[derive(Debug, Clone, JsonSchema)]
#[tpd(no_verify)]
pub struct Benchmark {
    /// Enable benchmark mode
    #[tpd(default)]
    pub enable: bool,

    /// Number of molecules to process in benchmark mode
    pub molecule_count: usize,
}

#[derive(Debug, Default)]
#[expect(
    dead_code,
    reason = "we currently only use gzip for multi thread considerations, but set them all for consistency"
)]
struct InputFormatsObserved {
    fastq: bool,
    fasta: bool,
    bam: bool,
    gzip: bool,
    per_segment: IndexMap<String, DetectedInputFormat>,
}

#[derive(Debug)]
pub struct Stage {
    pub transformation: Transformation,
    pub allowed_tags: Vec<TagLabel>,
    /// Tags this stage declared (via `RemovedTags`) that it forgets. The
    /// workpool physically drops these from the block *before* calling
    /// `apply`, so a forgetting step (`ForgetTag`, `ForgetAllTags`, conditional
    /// `Swap`, `MergeReads`) never sees them and does not re-add them — the
    /// forgetting is centralized here instead of repeated in each step.
    pub forgotten_tags: Vec<TagLabel>,
    pub output_declarations: Option<Vec<OutputDeclaration>>,
    /// Auxiliary input files this stage declared via
    /// `TagUser::declare_input_files`; opened by the runtime and handed to
    /// `Step::init`.
    pub input_declarations: Option<Vec<InputDeclaration>>,
}

#[derive(Debug, Clone)]
pub struct ThreadingConfiguration {
    /// Decompression threads per input segment (rapidgzip `-P`, BAM bgzf
    /// workers). Wants many threads.
    pub n_input_per_segment: std::num::NonZeroUsize,
    /// Pod-parser demux pool per input segment — the columnar scan+copy. Tuned
    /// independently of decompression: its sweet spot is a small handful (2-4),
    /// more just oversubscribes the allocator.
    pub n_pod_demux_per_segment: std::num::NonZeroUsize,
    pub n_output: std::num::NonZeroUsize,
    pub n_processing: std::num::NonZeroUsize,
}

/// The JSON schema for [`Config`], with fixups the `JsonSchema` derive can't
/// express applied. Always build the schema through this — `schema_for!(Config)`
/// alone is incomplete. (Fields that are merely optional use
/// `#[schemars(with = "Option<...>")]` and need nothing here.)
///
/// # Panics
///
/// Should not panic unless the schema stops being an object
#[must_use]
pub fn config_schema() -> schemars::Schema {
    let mut schema = schemars::schema_for!(Config);
    // The fixups below need the assembled `$defs`, which only exist on the
    // finished root schema — not when a `#[schemars(transform)]` hook would run.
    let root = schema.as_object_mut().expect("root schema is an object");
    inline_transformation_variants(root);
    inject_tpd_aliases(root);
    drop_transform_from_required(root);
    for value in root.values_mut() {
        add_lowercase_enum_aliases(value);
    }
    schema
}

/// `Transformation` is internally tagged on `action`, which schemars emits as
/// `{ properties: { action }, $ref: <step> }`. A strict TOML validator treats
/// the referenced step schema as closed and rejects the sibling `action` key,
/// because `$ref` and the inline `action` aren't merged. Inline each step's
/// schema into its variant so `action` lives alongside the step's own fields in
/// one self-contained object.
fn inline_transformation_variants(root: &mut serde_json::Map<String, serde_json::Value>) {
    let defs = root
        .get("$defs")
        .and_then(|d| d.as_object())
        .cloned()
        .expect("schema had no transformations?");
    let variants = root
        .get_mut("$defs")
        .and_then(|d| d.get_mut("Transformation"))
        .and_then(|t| t.get_mut("oneOf"))
        .and_then(serde_json::Value::as_array_mut)
        .expect("variants not found");

    for variant in variants {
        let obj = variant.as_object_mut().expect("Variant was not mut");
        let name = obj
            .get("$ref")
            .and_then(|r| r.as_str())
            .and_then(|r| r.strip_prefix("#/$defs/"))
            .map(str::to_owned)
            .expect("variant had no name?");
        let target = defs
            .get(&name)
            .and_then(|t| t.as_object())
            .expect("Variant had no target");

        let action = obj.get("properties").and_then(|p| p.get("action")).cloned();

        let mut merged = target.clone();
        let action = action.expect("no action? all transforms have actions");
        merged
            .entry("properties")
            .or_insert_with(|| serde_json::json!({}))
            .as_object_mut()
            .expect("properties is an object")
            .insert("action".to_owned(), action);
        // The step structs use plain (non-`Option`) fields that tpd fills with
        // defaults — or validates — in `verify()`, so schemars over-reports them
        // all as `required`. A strict editor then fails *every* `oneOf` branch
        // when any defaulted field is omitted and floods the user with each
        // branch's discriminator mismatch. The parser is the source of truth for
        // which fields are mandatory, so the schema only requires `action` (the
        // discriminator that selects the branch).
        merged.insert("required".to_owned(), serde_json::json!(["action"]));
        *obj = merged;
    }
}

/// Mirror every `#[tpd(alias = ...)]` declaration into the schema so a strict
/// editor accepts the same spellings tpd does. The aliases are gathered by
/// walking the `Config` type tree via [`TpdAliasTree`], keyed by type name
/// (which matches the schemars `$def` name) — no hand-maintained list, so it
/// can't drift from the parser. Each group is injected by schema shape:
/// - `enum` (string enums): aliases appended to the `enum` array;
/// - `properties` (structs, and the root `Config`): the canonical field's
///   subschema is cloned under each alias name (this is what adds the
///   `transform` field's `step`/`steps`/`transforms` aliases);
/// - `oneOf` (the tagged `Transformation` and unit-enums rendered as `oneOf`):
///   the matching branch's `const` discriminator becomes an `enum` of the
///   canonical value plus its aliases.
fn inject_tpd_aliases(root: &mut serde_json::Map<String, serde_json::Value>) {
    for (def_name, entries) in toml_pretty_deser::collect_alias_tree::<Config>() {
        if entries.is_empty() {
            unreachable!("Did not expect entries to be empty. Might just 'continue'?");
        }
        let target = if def_name == "Config" {
            Some(&mut *root)
        } else {
            root.get_mut("$defs")
                .and_then(serde_json::Value::as_object_mut)
                .and_then(|defs| defs.get_mut(def_name))
                .and_then(serde_json::Value::as_object_mut)
        };
        if let Some(target) = target {
            inject_alias_group(target, entries);
        }
    }
}

fn inject_alias_group(
    target: &mut serde_json::Map<String, serde_json::Value>,
    entries: AliasEntries,
) {
    if let Some(values) = target
        .get_mut("enum")
        .and_then(serde_json::Value::as_array_mut)
    {
        let mut seen: std::collections::HashSet<String> = values
            .iter()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect();
        for &(_canonical, aliases) in entries {
            for alias in aliases {
                if seen.insert((*alias).to_owned()) {
                    values.push((*alias).into());
                }
            }
        }
    } else if let Some(branches) = target
        .get_mut("oneOf")
        .and_then(serde_json::Value::as_array_mut)
    {
        for branch in branches {
            // `Transformation` carries the discriminator at `properties.action`;
            // a plain unit-enum rendered as `oneOf` carries it on the branch itself.
            let has_action = branch
                .get("properties")
                .and_then(|p| p.get("action"))
                .is_some();
            for &(canonical, aliases) in entries {
                let discriminator = if has_action {
                    branch
                        .get_mut("properties")
                        .and_then(|p| p.get_mut("action"))
                        .expect("checked has_action")
                } else {
                    &mut *branch
                };
                widen_const_to_enum(discriminator, canonical, aliases);
            }
        }
    } else if let Some(properties) = target
        .get_mut("properties")
        .and_then(serde_json::Value::as_object_mut)
    {
        for &(canonical, aliases) in entries {
            let canonical_schema = properties
                .get(canonical)
                .cloned()
                .expect("No canonical schema");
            for alias in aliases {
                properties.insert((*alias).to_owned(), canonical_schema.clone());
            }
        }
    } // cov:excl-line
}

/// If `node` is `{ "const": canonical }`, rewrite it to
/// `{ "type": "string", "enum": [canonical, ..aliases] }`.
fn widen_const_to_enum(node: &mut serde_json::Value, canonical: &str, aliases: &[&str]) {
    if node.get("const").and_then(serde_json::Value::as_str) != Some(canonical) {
        return;
    }
    let obj = node.as_object_mut().expect("enums are always objects");
    obj.remove("const");
    let mut values = vec![serde_json::Value::from(canonical)];
    values.extend(aliases.iter().map(|a| serde_json::Value::from(*a)));
    obj.insert("type".to_owned(), "string".into());
    obj.insert("enum".to_owned(), serde_json::Value::Array(values));
}

/// `transform` is non-`Option`, so the derive marks it required, but it defaults
/// to an empty list when omitted.
fn drop_transform_from_required(root: &mut serde_json::Map<String, serde_json::Value>) {
    if let Some(required) = root
        .get_mut("required")
        .and_then(serde_json::Value::as_array_mut)
    {
        required.retain(|v| v != "transform");
    } // cov:excl-line
}

/// schemars emits enum values as the canonical (`PascalCase`) variant names, but
/// configs are parsed case-insensitively (`FieldMatchMode::AnyCase`). Append the
/// lowercase spelling of each value next to the canonical one, so the common
/// lowercase form validates while the editor still offers completion for both.
fn add_lowercase_enum_aliases(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(serde_json::Value::Array(variants)) = map.get_mut("enum") {
                let mut seen: std::collections::HashSet<String> = variants
                    .iter()
                    .filter_map(|v| v.as_str().map(str::to_owned))
                    .collect();
                let mut additions = Vec::new();
                for variant in variants.iter() {
                    let s = variant.as_str().expect("Variant was not a string");
                    let lower = s.to_lowercase();
                    if seen.insert(lower.clone()) {
                        additions.push(serde_json::Value::from(lower));
                    }
                }
                variants.extend(additions);
            }
            for child in map.values_mut() {
                add_lowercase_enum_aliases(child);
            }
        }
        serde_json::Value::Array(items) => {
            for item in items {
                add_lowercase_enum_aliases(item);
            }
        }
        _ => {}
    }
}

#[derive(JsonSchema)]
#[tpd(root)]
#[derive(Debug)]
pub struct Config {
    /// The input configuration
    #[tpd(nested)]
    pub input: Input,
    #[tpd(nested)]
    pub output: Option<Output>,

    //barcodes must happen before transforms
    #[schemars(with = "Option<BTreeMap<String, Barcodes>>")]
    #[tpd(nested)]
    pub barcodes: Option<IndexMap<TagLabel, Barcodes>>,

    #[schemars(with = "Option<Options>")]
    #[tpd(nested)]
    pub options: Options,

    // we want the transformations to be able to inspect the options
    #[tpd(alias = "step", alias = "steps", alias = "transforms")]
    #[tpd(nested)]
    pub transform: Vec<Transformation>,

    #[tpd(nested)]
    pub benchmark: Option<Benchmark>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub report_labels: Vec<String>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub allowed_tags_per_transformation: Vec<Vec<TagLabel>>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub forgotten_tags_per_transformation: Vec<Vec<TagLabel>>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub output_declarations_per_transformation: Vec<Option<Vec<OutputDeclaration>>>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub input_declarations_per_transformation: Vec<Option<Vec<InputDeclaration>>>,

}

#[derive(Debug)]
pub struct CheckedConfig {
    pub input: Input,
    pub output: Option<Output>,
    pub stages: Vec<Stage>,
    pub options: Options,
    pub barcodes: IndexMap<TagLabel, Barcodes>,
    pub benchmark: Option<Benchmark>,
    pub report_labels: Vec<String>,
    pub threading_configuration: ThreadingConfiguration,
    /// The raw config TOML, embedded in the run report (`run_info.input_toml`).
    /// Populated by the caller after `check()`; empty when unavailable. Shared
    /// (`Arc`) so the report metadata does not duplicate it in memory.
    pub raw_config: std::sync::Arc<str>,
    pub output_declarations_per_transformation: Vec<Option<Vec<OutputDeclaration>>>,

    pub input_formats_detected: IndexMap<String, DetectedInputFormat>,
}

impl VerifyIn<TPDRoot> for PartialConfig {
    fn verify(
        &mut self,
        _parent: &TPDRoot,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized,
    {
        self.options.or_with(|| PartialOptions {
            threads: TomlValue::new_ok(None, 0..0),
            block_size: TomlValue::new_ok(default_block_size(), 0..0),
            max_molecules_in_flight: TomlValue::new_ok(
                default_max_molecules_in_flight(default_block_size()),
                0..0,
            ),
            buffer_size: TomlValue::new_ok(default_buffer_size(), 0..0),
            output_buffer_size: TomlValue::new_ok(default_output_buffer_size(), 0..0),
            accept_duplicate_files: TomlValue::new_ok(false, 0..0),
            spot_check_read_pairing: TomlValue::new_ok(default_spot_check_read_pairing(), 0..0),
            debug_failures: TomlValue::new_ok(
                options::PartialFailureOptions {
                    fail_output_after_bytes: TomlValue::new_ok(None, 0..0),
                    fail_output_error: TomlValue::new_ok(None, 0..0),
                    fail_output_raw_os_code: TomlValue::new_ok(None, 0..0),
                },
                0..0,
            ),
        });
        if !self.input.is_ok() {
            //we can't check transforms if the input def has failed,
            //they'de all have not set their segments/sources
            self.transform = TomlValue::new_ok(Vec::new(), 0..0);
        }
        self.verify_no_duplicate_files_no_empty_segments();
        self.transform.or_default();
        self.verify_reports();
        self.verify_single_output_report();
        self.verify_barcodes_and_segment_names_disjoint();
        self.verify_benchmark_molecule_count();
        self.disable_output_on_benchmark();
        self.expand_transformations();
        let used_barcode_sections = self.verify_transformation_labels();
        self.resolve_bam_output_references();
        self.verify_output_filenames_unique();
        self.collect_input_file_declarations();
        self.verify_for_any_output();

        self.verify_demultiplex_unique();
        self.verify_barcodes_used(&used_barcode_sections);
        self.verify_merge_demultiplexed();
        //todo: verify labels, barcodes, segment_names disjoint

        Ok(())
    }
}

impl PartialConfig {
    fn verify_no_duplicate_files_no_empty_segments(&mut self) {
        let mut seen_files: IndexMap<String, Vec<std::ops::Range<usize>>> = IndexMap::new();
        if let Some(input) = self.input.as_mut()
            && let Some(segments) = input.segments.as_mut()
            && let Some(false) = self
                .options
                .as_ref()
                .and_then(|options| options.accept_duplicate_files.as_ref())
        {
            //should always be true
            for tv_files in segments.map.values_mut() {
                if let Some(files) = tv_files.as_ref() {
                    if files.is_empty() {
                        tv_files.state = TomlValueState::ValidationFailed {
                            message: "Segment has no files specified".to_string(),
                        };
                    } else {
                        for tv_file in files {
                            if let Some(str_file) = tv_file.as_ref() {
                                match seen_files.entry(str_file.clone()) {
                                    indexmap::map::Entry::Occupied(occupied_entry) => {
                                        occupied_entry.into_mut().push(tv_file.span());
                                    }
                                    indexmap::map::Entry::Vacant(vacant_entry) => {
                                        vacant_entry.insert(vec![tv_file.span()]);
                                    }
                                }
                            } // cov:excl-line
                        }
                    }
                } // cov:excl-line
            }
            for (_filename, spans) in seen_files {
                if spans.len() > 1 {
                    let spans = spans
                        .into_iter()
                        .map(|span| (span, "This file is listed multiple times".to_string()))
                        .collect();
                    self.input.state = TomlValueState::Custom { spans };
                    self.input.help = Some(
                        "The same input file is listed multiple times. If this is intentional, set options.accept_duplicate_files = true.".to_string(),
                    );
                }
            }
        }
    }

    fn verify_reports(&mut self) {
        let is_benchmark = self
            .benchmark
            .as_ref()
            .and_then(|x| x.as_ref())
            .and_then(|x| x.enable.as_ref())
            .is_some_and(|o| *o);
        // An OutputReport step satisfies the "report output" requirement just like
        // output.report_html / report_json do.
        self.transform.sync_nested_state();
        let transforms_ok = self.transform.is_ok();

        let report_output_transform_idx = self.transform.as_mut().and_then(|x| {
            x.iter_mut().position(|t| {
                matches!(t.as_ref(), Some(PartialTransformation::OutputReport { .. }))
            })
        });

        let report_transform_idx = self.transform.as_mut().and_then(|x| {
            x.iter_mut().position(|t| {
                matches!(
                    t.as_ref(),
                    Some(
                        PartialTransformation::Report { .. }
                            | PartialTransformation::_InternalReadCount { .. }
                    )
                )
            })
        });
        let (mut report_output_transform, mut report_transform) =
            if let Some(transform) = self.transform.as_mut() {
                match (report_output_transform_idx, report_transform_idx) {
                    (None, None) => (None, None),
                    (None, Some(rti)) => (None, Some(&mut transform[rti])),
                    (Some(roti), None) => (Some(&mut transform[roti]), None),
                    (Some(roti), Some(rti_index)) => {
                        let [a, b] = transform
                            .get_disjoint_mut([roti, rti_index])
                            .expect("coordinates verified before hand");
                        (Some(a), Some(b))
                    }
                }
            } else {
                (None, None)
            };

        if let Some(report_transform) = report_transform.as_mut()
            && report_output_transform.is_none()
            && !is_benchmark
        {
            report_transform.state = TomlValueState::ValidationFailed {
                message: "Report but no `OutputReport` step present".to_string(),
            };
            report_transform.help =
                Some("Either remove the Report step, or add an OutputReport".to_string());
        } else if let Some(PartialTransformation::OutputReport(tv_rep_output)) =
            report_output_transform.as_mut().and_then(|x| x.as_mut())
            && report_transform.is_none()
            && transforms_ok
        {
            tv_rep_output.toml_value.state = TomlValueState::ValidationFailed {
                message: "Report output defined here".to_string(),
            };
            tv_rep_output.toml_value.help =
                Some("No report step, but report output requested.\nRemove the `OutputReport` step, or add in a `Report` step.".to_string());
        } else {
            let mut report_names_to_spans: IndexMap<
                String,
                Vec<&mut TomlValue<PartialTransformation>>,
            > = IndexMap::new();
            if let Some(transform) = self.transform.as_mut() {
                for tv_transform in transform.iter_mut() {
                    if let Some(transform) = tv_transform.as_ref() {
                        let name = if let PartialTransformation::Report(config) = transform {
                            config
                                .toml_value
                                .as_ref()
                                .and_then(|x| x.name.as_ref())
                                .cloned()
                        } else if let PartialTransformation::_InternalReadCount(config) = transform
                        {
                            config
                                .toml_value
                                .as_ref()
                                .and_then(|x| x.out_label.as_ref())
                                .map(std::string::ToString::to_string)
                        } else {
                            None
                        };
                        if let Some(name) = name {
                            match report_names_to_spans.entry(name) {
                                indexmap::map::Entry::Occupied(occupied_entry) => {
                                    occupied_entry.into_mut().push(tv_transform);
                                }
                                indexmap::map::Entry::Vacant(vacant_entry) => {
                                    vacant_entry.insert(vec![tv_transform]);
                                }
                            }
                        }
                    } // cov:excl-line
                }
            }
            for (report_name, mut transforms) in report_names_to_spans {
                if transforms.len() > 1 {
                    let spans = transforms
                        .iter()
                        .map(|tv_transform| {
                            (tv_transform.span(), "Report name not unique".to_string())
                        })
                        .collect();
                    transforms[0].state = TomlValueState::Custom { spans };
                    transforms[0].help = Some(format!(
                        "Multiple reports with the same name '{report_name}' found. Please ensure all reports have unique names."
                    ));
                }
            }
        }
    }

    /// At most one `OutputReport` step is allowed: multiple would share the same
    /// collected report data, so we reject the ambiguity.
    fn verify_single_output_report(&mut self) {
        let Some(transforms) = self.transform.as_mut() else {
            return;
        };
        let report_spans: Vec<std::ops::Range<usize>> = transforms
            .iter()
            .filter(|t| matches!(t.as_ref(), Some(PartialTransformation::OutputReport { .. })))
            .map(toml_pretty_deser::TomlValue::span)
            .collect();
        if report_spans.len() <= 1 {
            return;
        }
        //we now know it's multiple
        for t in transforms.iter_mut() {
            if matches!(t.as_ref(), Some(PartialTransformation::OutputReport { .. })) {
                let spans = report_spans
                    .iter()
                    .map(|s| (s.clone(), "Multiple OutputReport steps".to_string()))
                    .collect();
                t.state = TomlValueState::Custom { spans };
                t.help = Some(
                    "Only one OutputReport step is allowed; they would share the same report data. \
                     Remove all but one."
                        .to_string(),
                );
                break; // we only complain once, but it icludes all spans
            }
        }
    }

    fn verify_benchmark_molecule_count(&mut self) {
        if let Some(Some(benchmark)) = self.benchmark.as_mut() {
            benchmark.molecule_count.verify(|v| {
                if *v == 0 {
                    Err(ValidationFailure::new(
                        "molecule_count must be > 0",
                        Some("Set to a positive integer."),
                    ))
                } else {
                    Ok(())
                }
            });
        }
    }

    fn verify_for_any_output(&mut self) {
        let has_any_output = self
            .output_declarations_per_transformation
            .as_ref()
            .is_some_and(|x| x.iter().any(Option::is_some));

        let is_benchmark = self
            .benchmark
            .as_ref()
            .and_then(|x| x.as_ref())
            .and_then(|x| x.enable.as_ref())
            .copied()
            .unwrap_or(false);
        let output_ok = self.output.is_ok();
        let input_ok = self.input.is_ok();
        if !has_any_output && !is_benchmark && output_ok && input_ok {
            self.output.state = TomlValueState::ValidationFailed {
                message: "No output files and no reports requested. Nothing to do.".to_string(),
            };
            self.output.help = Some(
                "Add an Output* step, such as OutputFASTQ or an OutputReport, or use a benchmark configuration.".to_string(),
            );
        }
    }

    fn disable_output_on_benchmark(&mut self) {
        if let Some(Some(benchmark)) = &self.benchmark.as_ref()
            && let Some(true) = benchmark.enable.as_ref()
            && let Some(transforms) = self.transform.as_mut()
        {
            transforms.retain(|t| {
                !matches!(
                    t.as_ref(),
                    Some(
                        PartialTransformation::StoreTagInFastQ { .. }
                            | PartialTransformation::StoreTagsInTable { .. }
                            | PartialTransformation::Inspect { .. }
                            | PartialTransformation::QuantifyTag { .. }
                            | PartialTransformation::OutputFASTQ { .. }
                            | PartialTransformation::OutputFASTA { .. }
                            | PartialTransformation::OutputBAM { .. }
                            | PartialTransformation::OutputReport { .. }
                    )
                )
            });
        }
    }

    fn verify_barcodes_and_segment_names_disjoint(&mut self) {
        let mut segment_names = IndexMap::new();

        if let Some(input) = self.input.as_mut()
            && let Some(structured) = input.structured.as_mut()
            && let Some(Some(barcodes)) = self.barcodes.as_mut()
        {
            match structured {
                StructuredInput::Interleaved { .. } => {
                    if let Some(tv_interleaved) = input.interleaved.as_mut()
                        && let Some(interleaved) = tv_interleaved.as_mut()
                    {
                        for tv_segment in interleaved.iter_mut() {
                            if let Some(segment) = tv_segment.as_mut() {
                                segment_names.insert(segment.clone(), tv_segment);
                            }
                        }
                    } // cov:excl-line
                }
                StructuredInput::Segmented { .. } => {
                    if let Some(segments) = input.segments.as_mut() {
                        for tv_segment in &mut segments.keys {
                            if let Some(segment) = tv_segment.as_mut() {
                                segment_names.insert(segment.clone(), tv_segment);
                            }
                        }
                    } // cov:excl-line
                }
            }

            for tv_barcode_name in &mut barcodes.keys {
                if let Some(barcode_name) = tv_barcode_name.as_ref()
                    && let Some(tv_segment) = segment_names.get(barcode_name)
                {
                    tv_barcode_name.state = TomlValueState::Custom {
                        spans: vec![
                            (
                                tv_barcode_name.span(),
                                "This barcode name collides with a segment name".to_string(),
                            ),
                            (
                                tv_segment.span(),
                                "Segment with the same name defined here".to_string(),
                            ),
                        ],
                    };
                    tv_barcode_name.help = Some(
                            "Barcode names must not collide with segment names. Please choose a different name for this barcode.".to_string(),
                        );
                }
            }
        }
    }

    /// expansion of transforms into their final form
    /// # Panics
    /// When `used_tags declares` one tag multiple times
    pub fn expand_transformations(&mut self) {
        self.transform.sync_nested_state(); // since we normally would only update this once
        // verify is done, but we need accurate info here now.
        if self.transform.value.is_some() {
            let transform_span = self.transform.span();
            if let Some(mut transforms) = self.transform.value.take() {
                //childs may or may not be ok - if they need to lookup up tags, segment_order
                //in get_used_tags they may not be ok yet.

                let expanded_transforms: RefCell<Vec<TomlValue<PartialTransformation>>> =
                    RefCell::new(Vec::new());
                let mut res_report_labels = Vec::new();
                let mut report_no = 0;
                let mut push_existing =
                    |t: TomlValue<PartialTransformation>| expanded_transforms.borrow_mut().push(t);
                let mut push_new = |t: PartialTransformation| {
                    expanded_transforms
                        .borrow_mut()
                        .push(TomlValue::new_ok(t, 0..0));
                };

                self.expand_spot_checks(&mut push_new, &transforms);

                for mut t in transforms.drain(..) {
                    match t.as_mut() {
                        Some(PartialTransformation::ExtractRegion(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            let step_config = step_config
                                .toml_value
                                .take()
                                .into_inner()
                                .expect("Parent was ok?");
                            let source_span = step_config.source.span.clone();
                            let regions = TomlValue::new_ok(
                                vec![TomlValue::new_ok(
                                    crate::transformations::PartialRegionDefinition {
                                        start: step_config.start,
                                        length: step_config.len,
                                        anchor: step_config.anchor,
                                    },
                                    source_span,
                                )],
                                tag_span.clone(),
                            );
                            push_new(PartialTransformation::ExtractRegions(
                                PartialTaggedVariant {
                                    toml_value: TomlValue::new_ok_unplaced(
                                        crate::transformations::extract::PartialRegions {
                                            out_label: step_config.out_label,
                                            regions,
                                            // region_separator: None,
                                            output_tag_type: None,
                                            source: step_config.source.clone(),
                                        },
                                    ),
                                    tag_span: tag_span.clone(),
                                },
                            ));
                        }
                        Some(PartialTransformation::Report(report_config)) => {
                            Self::expand_reports(
                                &mut push_new,
                                &mut push_existing,
                                &mut res_report_labels,
                                &mut report_no,
                                &mut report_config.toml_value,
                                report_config.tag_span.clone(),
                            );
                        }

                        Some(PartialTransformation::_InternalReadCount(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.take().into_inner() {
                                res_report_labels.push(
                                    step_config
                                        .out_label
                                        .as_ref()
                                        .expect("parent was ok")
                                        .to_string(),
                                );
                                let step_config: Box<_> =
                                    Box::new(crate::transformations::Partial_InternalReadCount {
                                        out_label: step_config.out_label,
                                        report_no: Some(report_no),
                                        count: Some(Default::default()),
                                    });
                                report_no += 1;
                                push_new(PartialTransformation::_InternalReadCount(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(step_config),
                                        tag_span,
                                    },
                                ));
                            }
                        }
                        Some(PartialTransformation::CalcGCContent(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.take().into_inner() {
                                push_new(PartialTransformation::CalcBaseContent(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(
                                            crate::transformations::calc::PartialBaseContent::new(
                                                step_config.out_label,
                                                step_config.segment,
                                                *step_config
                                                    .relative
                                                    .as_ref()
                                                    .expect("was required"),
                                                BString::from("GC"),
                                                BString::from("N"),
                                            ),
                                        ),
                                        tag_span,
                                    },
                                ));
                            }
                        }
                        Some(PartialTransformation::CalcNContent(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.take().into_inner() {
                                push_new(PartialTransformation::CalcBaseContent(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(
                                            crate::transformations::calc::PartialBaseContent::new(
                                                step_config.out_label,
                                                step_config.segment,
                                                *step_config
                                                    .relative
                                                    .as_ref()
                                                    .expect("was required"),
                                                BString::from("N"),
                                                BString::default(),
                                            ),
                                        ),
                                        tag_span,
                                    },
                                ));
                            }
                        }
                        Some(PartialTransformation::FilterEmpty(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.take().into_inner() {
                                // Replace FilterEmpty with CalcLength + FilterByNumericTag
                                let segment_index = *step_config
                                    .segment
                                    .as_ref()
                                    .expect("parent was ok")
                                    .as_ref_post()
                                    .expect("parent was ok");
                                push_new(PartialTransformation::FilterByNumericTag(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(
                                            crate::transformations::filters::PartialByNumericTag {
                                                in_label: TomlValue::new_ok(
                                                    MustAdapt::PostVerify(TagLabel::Length(
                                                        segment_index,
                                                        "len_not_visible_to_user".to_string(),
                                                    )),
                                                    step_config.segment.span(),
                                                ),
                                                min_value: TomlValue::new_ok_unplaced(Some(1.0)), // Non-empty means length >= 1
                                                max_value: TomlValue::new_ok_unplaced(None),
                                                keep_or_remove: TomlValue::new_ok_unplaced(
                                                    crate::transformations::KeepOrRemove::Keep,
                                                ),
                                            },
                                        ),
                                        tag_span,
                                    },
                                ));
                            }
                        }
                        Some(PartialTransformation::ConvertQuality(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.as_ref() {
                                //implies a check beforehand
                                push_new(PartialTransformation::ValidateQuality(
                                    PartialTaggedVariant {
                                        toml_value:
                                TomlValue::new_ok_unplaced(
                                    crate::transformations::validation::PartialValidateQuality {
                                        encoding: TomlValue::new_ok(
                                            *step_config.from.as_ref().expect("parent was ok"),
                                            step_config.from.span(),
                                        ),
                                        segment: TomlValue::new_ok_unplaced(MustAdapt::PostVerify(
                                            SegmentIndexOrAll::All,
                                        )),
                                    },
                                ),
                                tag_span,
                                    }
                            ));
                                push_existing(t);
                            }
                        }
                        Some(PartialTransformation::Lowercase(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.take().into_inner() {
                                push_new(PartialTransformation::_ChangeCase(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(
                                            crate::transformations::edits::Partial_ChangeCase::new(
                                                step_config.target,
                                                crate::transformations::edits::CaseType::Lower,
                                                step_config.if_tag,
                                            ),
                                        ),
                                        tag_span,
                                    },
                                ));
                            }
                        }
                        Some(PartialTransformation::Uppercase(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.take().into_inner() {
                                push_new(PartialTransformation::_ChangeCase(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(
                                            crate::transformations::edits::Partial_ChangeCase::new(
                                                step_config.target,
                                                crate::transformations::edits::CaseType::Upper,
                                                step_config.if_tag,
                                            ),
                                        ),
                                        tag_span,
                                    },
                                ));
                            }
                        }

                        Some(PartialTransformation::HammingCorrect(step_config)) => {
                            let tag_span = step_config.tag_span.clone();
                            if let Some(step_config) = step_config.toml_value.as_mut()
                                && matches!(
                                    step_config.on_tie.as_ref(),
                                    Some(OnTie::ByMajority | OnTie::ByEditProbability)
                                )
                                && let Some(barcodes_to_use) = step_config.barcodes.as_ref()
                                && let Some(Some(barcodes_data)) = self.barcodes.as_ref()
                                && let Some(barcodes_section) =
                                    barcodes_data.map.get(barcodes_to_use)
                                && let Some(barcodes_section) = barcodes_section.as_ref()
                                && let Some(seq_to_name) = &barcodes_section.seq_to_name
                            {
                                let on_tie_min_molecules_to_start = *step_config.on_tie_min_molecules_to_start.as_ref().expect("parent was ok, VerifyIn<HammingCorrect> must have set this");
                                let pt = PartialHammingExactCounter::new(
                                    step_config
                                        .in_label
                                        .as_ref()
                                        .expect("parent was ok")
                                        .clone(),
                                    seq_to_name.clone(),
                                    on_tie_min_molecules_to_start,
                                );
                                step_config.majority_data = Some(pt.majority_data.clone());
                                push_new(PartialTransformation::_HammingExactCounter(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(pt),
                                        tag_span: tag_span.clone(),
                                    },
                                ));

                                // Build the shared PreMatchData and insert the
                                // parallel matcher right before HammingCorrect.
                                let resonator = step_config
                                    .resonator
                                    .as_ref()
                                    .expect("verify must have built the resonator")
                                    .clone();
                                let seq_to_idx = step_config
                                    .seq_to_idx
                                    .as_ref()
                                    .expect("verify must have built seq_to_idx")
                                    .clone();
                                let needs_qualities = matches!(
                                    step_config.on_tie.as_ref(),
                                    Some(OnTie::ByEditProbability)
                                );
                                let shared = std::sync::Arc::new(PreMatchData {
                                    seq_to_name: seq_to_name.clone(),
                                    seq_to_idx,
                                    resonator,
                                    needs_qualities,
                                    pending: std::sync::Mutex::new(Default::default()),
                                });
                                step_config.pre_match = Some(Some(shared.clone()));
                                let in_label = step_config
                                    .in_label
                                    .as_ref()
                                    .expect("parent was ok")
                                    .clone();
                                push_new(PartialTransformation::_HammingPreMatch(
                                    PartialTaggedVariant {
                                        toml_value: TomlValue::new_ok_unplaced(
                                            Partial_HammingPreMatch::new(in_label, shared),
                                        ),
                                        tag_span,
                                    },
                                ));
                            }

                            push_existing(t);
                        }

                        _ => {
                            push_existing(t);
                        }
                    }
                }
                self.transform =
                    TomlValue::new_ok(expanded_transforms.into_inner(), transform_span);
                self.report_labels = Some(res_report_labels);
            } else {
                // cov:excl-start
                unreachable!()
                // cov:excl-stop
            }
        } // cov:excl-line
    }

    fn expand_spot_checks<F: FnMut(PartialTransformation)>(
        &self,
        mut push_new: F,
        transforms: &[TomlValue<PartialTransformation>],
    ) {
        if let Some(options) = self.options.as_ref()
            && let Some(spot_check_read_pairing) = options.spot_check_read_pairing.as_ref()
            && !spot_check_read_pairing
        {
            return;
        }
        if let Some(input) = self.input.as_ref()
            && input.segment_count() <= 1
        {
            return;
        }
        let has_validate_name = transforms
            .iter()
            .any(|step| matches!(step.as_ref(), Some(PartialTransformation::ValidateName(_))));
        let has_spot_check = transforms.iter().any(|step| {
            matches!(
                step.as_ref(),
                Some(PartialTransformation::ValidateReadPairing(_))
            )
        });
        let is_benchmark = self
            .benchmark
            .as_ref()
            .and_then(|x| x.as_ref())
            .and_then(|x| x.enable.as_ref())
            .copied()
            .unwrap_or(false);

        if !has_validate_name && !has_spot_check && !is_benchmark {
            push_new(PartialTransformation::ValidateReadPairing(
                PartialTaggedVariant {
                    toml_value: TomlValue::new_ok_unplaced(
                        crate::transformations::validation::PartialValidateReadPairing::new(None),
                    ),
                    tag_span: 0..0,
                },
            ));
        }
    }

    fn expand_reports<
        F: FnMut(PartialTransformation),
        G: FnMut(TomlValue<PartialTransformation>),
    >(
        mut push_new: F,
        _push_existing: G,
        res_report_labels: &mut Vec<String>,
        report_no: &mut usize,
        tv_config: &mut TomlValue<crate::transformations::reports::PartialReport>,
        _enum_tag_span: std::ops::Range<usize>,
    ) {
        use crate::transformations::reports;
        if let Some(config) = tv_config.as_ref() {
            // it already has been validated...
            // push_existing(PartialTransformation::Report(
            //     tv_config.clone(),
            //     enum_tag_span,
            // )); // for validation.
            // // We remove it again later on. Transformation::Expand
            res_report_labels.push(config.name.as_ref().expect("parent was ok").clone());
        }
        if let Some(config) = tv_config.as_mut() {
            if let Some(true) = config.count.as_ref() {
                push_new(PartialTransformation::_ReportCount(PartialTaggedVariant {
                    toml_value: TomlValue::new_ok_unplaced(Box::new(
                        reports::Partial_ReportCount::new(*report_no),
                    )),
                    tag_span: 0..0,
                }));
            }
            if let Some(true) = config.length_distribution.as_ref() {
                push_new(PartialTransformation::_ReportLengthDistribution(
                    PartialTaggedVariant {
                        toml_value: TomlValue::new_ok_unplaced(Box::new(
                            reports::Partial_ReportLengthDistribution::new(*report_no),
                        )),
                        tag_span: 0..0,
                    },
                ));
            }
            if let Some(true) = config.duplicate_count_per_read.as_ref() {
                push_new(PartialTransformation::_ReportDuplicateCount(
                    PartialTaggedVariant {
                        toml_value: TomlValue::new_ok_unplaced(Box::new(
                            reports::Partial_ReportDuplicateCount::new(
                                *report_no,
                                config.debug_reproducibility.clone(),
                            ),
                        )),
                        tag_span: 0..0,
                    },
                ));
            }

            if let Some(true) = config.duplicate_count_per_fragment.as_ref() {
                push_new(PartialTransformation::_ReportDuplicateFragmentCount(
                    PartialTaggedVariant {
                        toml_value: TomlValue::new_ok_unplaced(Box::new(
                            reports::Partial_ReportDuplicateFragmentCount::new(
                                *report_no,
                                config.debug_reproducibility.clone(),
                            ),
                        )),
                        tag_span: 0..0,
                    },
                ));
            }
            if let Some(true) = config.base_statistics.as_ref() {
                push_new(PartialTransformation::_ReportBaseStatisticsPart1(
                    PartialTaggedVariant {
                        toml_value: TomlValue::new_ok_unplaced(Box::new(
                            reports::Partial_ReportBaseStatisticsPart1::new(*report_no),
                        )),
                        tag_span: 0..0,
                    },
                ));
                push_new(PartialTransformation::_ReportBaseStatisticsPart2(
                    PartialTaggedVariant {
                        toml_value: TomlValue::new_ok_unplaced(Box::new(
                            reports::Partial_ReportBaseStatisticsPart2::new(*report_no),
                        )),
                        tag_span: 0..0,
                    },
                ));
            }
            if let Some(Some(count_oligos)) = config.count_oligos.take().into_inner() {
                let oligos_map: IndexMap<String, bstr::BString> = count_oligos
                    .map
                    .into_iter()
                    .filter_map(|(name, tv_seq)| {
                        tv_seq.into_inner().map(|seq| (name, seq.0)) //already verified to be uppercase by NonAmbigousDNA
                    })
                    .collect();
                push_new(PartialTransformation::_ReportCountOligos(
                    PartialTaggedVariant {
                        toml_value: TomlValue::new_ok_unplaced(Box::new(
                            reports::Partial_ReportCountOligos::new(
                                *report_no,
                                oligos_map,
                                config.count_oligos_segment.clone(),
                            ),
                        )),
                        tag_span: 0..0,
                    },
                ));
            }
            if let Some(Some(tag_histograms)) = config.tag_histograms.as_ref() {
                for tag_name in tag_histograms {
                    push_new(PartialTransformation::_ReportTagHistogram(
                        PartialTaggedVariant {
                            toml_value: TomlValue::new_ok_unplaced(Box::new(
                                reports::Partial_ReportTagHistogram::new(
                                    *report_no,
                                    tag_name.clone(),
                                ),
                            )),
                            tag_span: 0..0,
                        },
                    ));
                }
            }
            *report_no += 1;
        } // cov:excl-line
    }

    fn _set_type_error(
        toml_source: &Rc<RefCell<(&mut TomlValueState, &mut Option<String>)>>,
        tag_name: &str,
        tag_types: &[TagValueType],
        actual_tag_type: &TagValueType,
        further_help: Option<&String>,
    ) {
        *toml_source.borrow_mut().0 =
            TomlValueState::new_validation_failed("Incompatible tag type");
        let str_supposed_tag_types = tag_types
            .iter()
            .map(|t| format!("'{t}'"))
            .collect::<Vec<_>>()
            .join(", ");
        let mut help_str = format!(
            "This transform requires tag '{tag_name}' to be one of the following types: [{str_supposed_tag_types}],\n\
                                        but it is actually of type '{actual_tag_type}'.",
        );
        if let Some(further_help) = further_help.as_ref() {
            let _ = write!(help_str, "\n{further_help}");
        }

        *toml_source.borrow_mut().1 = Some(help_str);
    }

    /// # Panics
    /// Tag declared twice
    pub fn verify_transformation_labels(&mut self) -> std::collections::HashSet<TagLabel> {
        use crate::transformations::TagUser;
        let mut allowed_tags_per_stage: Vec<Vec<TagLabel>> = Vec::new();
        let mut forgotten_tags_per_stage: Vec<Vec<TagLabel>> = Vec::new();
        let mut tags_available: IndexMap<TagLabel, TagMetadata> = IndexMap::new();
        let mut used_barcode_sections: std::collections::HashSet<TagLabel> =
            std::collections::HashSet::new();
        if let Some(transformations) = self.transform.value.as_mut() //the trafos have no final
        //looked up tags yet.
            && let Some(input) = self.input.as_ref()
        {
            let mut just_trafos = transformations
                .iter_mut()
                .filter_map(|t| t.value.as_mut())
                .collect::<Vec<_>>();
            let mut all_tags_ever: IndexMap<String, std::ops::Range<usize>> = IndexMap::new();
            let segment_order = input.get_segment_order();
            let mut any_tag_errors = false;
            for trafo in &mut just_trafos {
                //     if let err(e) =
                //         t.validate_others(&self.input, self.output.as_ref(), &self.transform, step_no)
                //     {
                //         errors.push(e.context(format!("[step {step_no} ({t})]:")));
                //         continue; // skip further processing of this transform if validation failed
                //     }
                let Some(tag_info) = trafo.get_tag_usage(&tags_available, segment_order) else {
                    any_tag_errors = true;
                    //break;
                    continue;
                };
                used_barcode_sections.extend(tag_info.used_barcodes.iter().cloned());
                let mut tags_used_here: Vec<TagLabel> = Vec::new();
                // Tags this stage forgets. They are NOT added to `tags_used_here`
                // (the stage's allowed_tags) — instead the workpool drops them
                // before `apply`, so the step never sees them and cannot re-add
                // them. Centralizes what ForgetTag/ForgetAllTags/Swap/MergeReads
                // each used to do by hand in their `apply`.
                let mut tags_forgotten_here: Vec<TagLabel> = Vec::new();
                match tag_info.removed_tags {
                    RemovedTags::None => {}
                    RemovedTags::All => {
                        for metadata in tags_available.values_mut() {
                            metadata.used = true;
                        }
                        tags_forgotten_here.extend(tags_available.keys().cloned());
                        tags_available.clear();
                    }
                    RemovedTags::Some(tags) => {
                        for (tag_name, toml_source) in tags {
                            //no need to check if empty, empty will never be present
                            if let Some(metadata) = tags_available.get_mut(&tag_name) {
                                metadata.used = true;
                                tags_forgotten_here.push(tag_name.clone());
                            } else {
                                any_tag_errors = true;
                                toml_source.state = TomlValueState::new_validation_failed(format!(
                                    "No such tag to remove: '{tag_name}'",
                                ));
                                toml_source.help = Some(offer_alternatives(
                                    tag_name.as_ref(),
                                    &tags_available.keys().map(AsRef::as_ref).collect::<Vec<_>>(),
                                ));
                                continue; //no point on doing anything else with this tag
                            }
                            tags_available.shift_remove(&tag_name);
                        }
                    }
                    RemovedTags::SomeOwned(tags) => {
                        // Computed set (e.g. conditional Swap / MergeReads
                        // forgetting location tags on the affected segments).
                        // These were read straight out of `tags_available`, so
                        // they always exist.
                        for tag_name in tags {
                            if let Some(metadata) = tags_available.get_mut(&tag_name) {
                                metadata.used = true;
                                tags_forgotten_here.push(tag_name.clone());
                            }
                            tags_available.shift_remove(&tag_name);
                        }
                    }
                }

                if tag_info.must_see_all_tags {
                    tags_used_here.extend(tags_available.keys().cloned());
                }
                for used_tag_info in tag_info.used_tags.iter().filter_map(|x| x.as_ref()) {
                    let tag_name = &used_tag_info.name;
                    if !tag_name.is_virtual() {
                        let tag_types = &used_tag_info.accepted_tag_types;
                        let toml_source = &used_tag_info.toml_source;
                        //no need to check if empty, empty will never be present
                        let entry = tags_available.get_mut(tag_name);
                        if let Some(metadata) = entry {
                            metadata.used = true;
                            if tag_types
                                .iter()
                                .any(|tag_type| tag_type.compatible(metadata.tag_type))
                            {
                                if !tag_info.must_see_all_tags {
                                    //otherwise, we already have the tag in the list.
                                    if tags_used_here.contains(tag_name) {
                                        // cov:excl-start
                                        panic!(
                                            "tag declared twice in used_tags, fix that! {tag_name}"
                                        );
                                        // cov:excl-stop
                                    } else {
                                        tags_used_here.push(tag_name.clone());
                                    }
                                }
                            } else {
                                any_tag_errors = true;
                                Self::_set_type_error(
                                    toml_source,
                                    tag_name.as_ref(),
                                    tag_types,
                                    &metadata.tag_type,
                                    used_tag_info.further_help.as_ref(),
                                );
                            }
                        } else {
                            any_tag_errors = true;
                            *toml_source.borrow_mut().0 = TomlValueState::new_validation_failed(
                                format!("No such tag: '{tag_name}'"),
                            );
                            if all_tags_ever.contains_key(tag_name.as_ref()) {
                                *toml_source.borrow_mut().1 = Some(format!(
                                    "Tag '{tag_name}' was generated by a previous step, but it is not available at this point.\n\
                                        This likely means that it was removed (forgotten) by an intermediate step.\n{}",
                                    offer_alternatives(
                                        tag_name.as_ref(),
                                        &tags_available
                                            .keys()
                                            .map(AsRef::as_ref)
                                            .collect::<Vec<_>>()
                                    )
                                ));
                            } else {
                                *toml_source.borrow_mut().1 = Some(offer_alternatives(
                                    tag_name.as_ref(),
                                    &tags_available.keys().map(AsRef::as_ref).collect::<Vec<_>>(),
                                ));
                            }
                        }
                    } else {
                        if let Some(source_tag) = tag_name.source_tag() {
                            //todo get rid of the alloc?
                            if let Some(entry) =
                                tags_available.get_mut(&TagLabel::Normal(source_tag.clone()))
                            {
                                entry.used = true;
                                match tag_name {
                                    TagLabel::Normal(_)
                                    | TagLabel::ReadNo
                                    | TagLabel::Length(_, _) => {
                                        unreachable!() // cov:excl-line
                                    } // cov:excl-line should not have a source_tag
                                    TagLabel::TagLength(_source_tag, _) => {
                                        if !entry.tag_type.compatible(TagValueType::Location)
                                            && !entry.tag_type.compatible(TagValueType::String)
                                        {
                                            let toml_source = &used_tag_info.toml_source;
                                            Self::_set_type_error(
                                                toml_source,
                                                source_tag,
                                                &[TagValueType::String, TagValueType::Location],
                                                &entry.tag_type,
                                                used_tag_info.further_help.as_ref(),
                                            );
                                        }
                                    }
                                    TagLabel::TagInitialLocation { .. }
                                    | TagLabel::TagLocation { .. } => {
                                        if !entry.tag_type.compatible(TagValueType::Location) {
                                            let toml_source = &used_tag_info.toml_source;
                                            Self::_set_type_error(
                                                toml_source,
                                                source_tag,
                                                &[TagValueType::Location],
                                                &entry.tag_type,
                                                used_tag_info.further_help.as_ref(),
                                            );
                                        }
                                    }
                                }
                            } else {
                                unreachable!(
                                    "Would only have been reachable by a step that uses it's own forgotten tag? Code is still here if you ever reach this unreachable"
                                );
                                // // The virtual tag's source is gone — typically
                                // // forgotten by an intermediate step (a conditional
                                // // `Swap` forgets location tags on the swapped
                                // // segments). Without this the missing source would
                                // // only surface as a runtime panic when the virtual
                                // // tag is materialized.
                                // any_tag_errors = true;
                                // let toml_source = &used_tag_info.toml_source;
                                // *toml_source.borrow_mut().0 = TomlValueState::new_validation_failed(
                                //     format!("No such tag: '{source_tag}'"),
                                // );
                                // let alternatives = offer_alternatives(
                                //     source_tag.as_str(),
                                //     &tags_available.keys().map(AsRef::as_ref).collect::<Vec<_>>(),
                                // );
                                // let help = if all_tags_ever.contains_key(source_tag.as_str()) {
                                //     format!(
                                //         "Tag '{source_tag}' was generated by a previous step, but it is not available at this point.\n\
                                //         This likely means that it was removed (forgotten) by an intermediate step.\n{alternatives}",
                                //     )
                                // } else {
                                //     alternatives
                                // };
                                // *toml_source.borrow_mut().1 = Some(help);
                            }
                        }

                        tags_used_here.push(tag_name.clone());
                    }
                }
                allowed_tags_per_stage.push(tags_used_here);
                forgotten_tags_per_stage.push(tags_forgotten_here);

                if let Some(dt) = tag_info.declared_tag {
                    if let Some(meta) = tags_available.get(&dt.name) {
                        any_tag_errors = true;
                        let spans = vec![
                            (
                                dt.toml_source_span.clone(),
                                "Tag also declared here".to_string(),
                            ),
                            (meta.span.clone(), "Tag declared here ".to_string()),
                        ];
                        *dt.toml_source_state = TomlValueState::Custom { spans };
                        *dt.toml_source_help = Some(
                            "Rename either tag, or add a ForgetTag step inbetween.".to_string(),
                        );
                    } else {
                        all_tags_ever
                            .insert(dt.name.as_ref().to_string(), dt.toml_source_span.clone());
                        tags_available.insert(
                            dt.name.clone(),
                            TagMetadata {
                                used: false,
                                tag_type: dt.tag_type,
                                contents: dt.contains,
                                span: dt.toml_source_span,
                                segment: dt.segment,
                            },
                        );
                    }
                }
            }
            self.allowed_tags_per_transformation = Some(allowed_tags_per_stage);
            self.forgotten_tags_per_transformation = Some(forgotten_tags_per_stage);

            //now verify the tags don't overlap barcodes or segments - they must be disjonit
            if let Some(Some(barcodes)) = self.barcodes.as_mut() {
                for tv_barcode in &mut barcodes.keys {
                    if let Some(barcode_name) = tv_barcode.as_ref() {
                        any_tag_errors = true;
                        if all_tags_ever.contains_key(barcode_name) {
                            let spans = vec![
                                (
                                    tv_barcode.span(),
                                    "Barcode with the same name as a tag defined here".to_string(),
                                ),
                                (
                                    all_tags_ever
                                        .get(barcode_name)
                                        .expect("just checked contains_key")
                                        .clone(),
                                    "Tag with the same name as a barcode defined here".to_string(),
                                ),
                            ];
                            tv_barcode.state = TomlValueState::Custom { spans };
                            tv_barcode.help = Some(
                            "Barcode names must not collide with tag names. Please choose a different name for this barcode.".to_string(),
                        );
                        }
                    }
                }
            }
            if let Some(input) = self.input.as_mut()
                && let Some(structured) = input.structured.as_ref()
            {
                let segment_iter: Vec<&mut TomlValue<String>> = match structured {
                    StructuredInput::Interleaved { .. } => input
                        .interleaved
                        .as_mut()
                        .and_then(|x| x.as_mut())
                        .map_or_else(std::vec::Vec::new, |x| x.iter_mut().collect::<Vec<_>>()),
                    StructuredInput::Segmented { .. } => input
                        .segments
                        .as_mut()
                        .map_or_else(std::vec::Vec::new, |x| x.keys.iter_mut().collect()),
                };
                for tv_segment in segment_iter {
                    if let Some(segment_name) = tv_segment.as_ref()
                        && all_tags_ever.contains_key(segment_name)
                    {
                        any_tag_errors = true;
                        let spans = vec![
                            (
                                tv_segment.span(),
                                "Segment with the same name as a tag defined here".to_string(),
                            ),
                            (
                                all_tags_ever
                                    .get(segment_name)
                                    .expect("just checked contains_key")
                                    .clone(),
                                "Tag with the same name as a segment defined here".to_string(),
                            ),
                        ];
                        tv_segment.state = TomlValueState::Custom { spans };
                        tv_segment.help = Some(
                                    "Segment names must not collide with tag names. Please choose a different name for this tag (or segment).".to_string(),
                                );
                    }
                }
            } // cov:excl-line

            //complain about unused tags if there were no tag errors
            //otherwise, mistyping a tag will give you two errors
            //one for 'no such tag' and one for 'you did not use the real one'
            if !any_tag_errors {
                let mut spans = vec![];
                for (_tag_label, meta) in &tags_available {
                    if !meta.used {
                        spans.push((meta.span.clone(), "Unused tag".to_string()));
                    }
                }
                if !spans.is_empty() {
                    self.transform.state = TomlValueState::Custom { spans };
                    self.transform.help = Some(
                        "Make sure to either use, or forget (using ForgetTag) all your tags."
                            .to_string(),
                    );
                }

                //now let's go and run the inter-transformation checks
                if let Some(trafos) = self.transform.as_mut() {
                    for idx in 0..trafos.len() {
                        let (before, rest) = trafos.split_at_mut(idx);
                        if let Some(trafo) = rest[0].as_mut() {
                            trafo.verify_others(
                                self.input.as_ref(),
                                self.output.as_ref().and_then(|o| o.as_ref()),
                                before,
                            );
                        }
                    }
                }
            }
        }
        used_barcode_sections
    }

    /// Resolve the BAM header / reference sequences for every `OutputBAM` step.
    ///
    /// Runs after tag validation (so referenced tags/barcodes are known) and
    /// before [`Self::verify_output_filenames_unique`] (which builds the output
    /// declarations from the resolved options). Reference sequences come either
    /// from a `[barcodes.*]` section (resolved here from the verified barcode
    /// data) or from a reference BAM file (read by the step). Validation errors
    /// (unknown barcode section, unreadable BAM) are reported on the step.
    fn resolve_bam_output_references(&mut self) {
        // Build per-section reference (name, length) lists from the verified
        // barcode sections. Each unique barcode *name* is one reference; its
        // length is the barcode sequence length (mirrors the legacy resolver).
        let mut barcode_section_refs: IndexMap<String, Vec<(String, usize)>> = IndexMap::new();
        if let Some(Some(barcodes)) = self.barcodes.as_ref() {
            for (label, tv_section) in &barcodes.map {
                if let Some(section) = tv_section.as_ref()
                    && let Some(seq_to_name) = &section.seq_to_name
                {
                    let mut seen: std::collections::HashSet<&String> =
                        std::collections::HashSet::new();
                    let mut refs: Vec<(String, usize)> = Vec::new();
                    for (seq, name) in seq_to_name.iter() {
                        if seen.insert(name) {
                            refs.push((name.clone(), seq.len()));
                        }
                    }
                    barcode_section_refs.insert(label.to_string(), refs);
                } // cov:excl-line
            }
        }

        if let Some(trafos) = self.transform.value.as_mut() {
            for tv in trafos.iter_mut() {
                if let Some(PartialTransformation::OutputBAM(pt)) = tv.value.as_mut()
                    && let Some(partial) = pt.toml_value.value.as_mut()
                {
                    crate::transformations::output::resolve_output_bam(
                        partial,
                        &barcode_section_refs,
                    );
                }
            }
        } // cov:excl-line
    }

    fn collect_input_file_declarations(&mut self) {
        let mut all_decls: Vec<Option<Vec<InputDeclaration>>> = Vec::new();
        self.transform.sync_nested_state();
        if let Some(transforms) = self.transform.value.as_ref() {
            for tv_transform in transforms {
                let decls = tv_transform
                    .value
                    .as_ref()
                    .map(TagUser::declare_input_files)
                    .unwrap_or_default();
                all_decls.push(decls);
            }
        } //cov:excl-line
        self.input_declarations_per_transformation = Some(all_decls);
    }

    /// Detect output filename conflicts across all steps.
    ///
    /// Each step's `TagUser::declare_output_files()` carries a `span` pointing
    /// at the config field most responsible for the filename (e.g. `infix`).
    /// Collect each step's declared auxiliary input files (via
    /// `TagUser::declare_input_files`) so the runtime can open them and hand the
    /// handles to `Step::init`. Mirrors the declaration collection in
    /// [`Self::verify_output_filenames_unique`]; kept separate as inputs need no
    /// cross-step conflict detection.
    pub fn verify_output_filenames_unique(&mut self) {
        use fastqrab_io::io::output::chunked_writer::WriteTargetConfig;

        // First pass (immutable): collect declarations and detect conflicts.
        type ConflictEntry = (usize, std::ops::Range<usize>);
        let mut key_to_entries: IndexMap<
            (Vec<String>, Option<String>, String),
            Vec<ConflictEntry>,
        > = IndexMap::new();
        let mut all_decls: Vec<Option<Vec<OutputDeclaration>>> = Vec::new();

        self.transform.sync_nested_state();
        if let Some(transforms) = self.transform.value.as_mut() {
            for (idx, tv_transform) in transforms.iter_mut().enumerate() {
                if let Some(decls) = tv_transform
                    .value
                    .as_ref()
                    .map(TagUser::declare_output_files)
                    .unwrap_or_default()
                {
                    for decl in &decls {
                        if let WriteTargetConfig::File(ft) = &decl.target {
                            let key = (
                                ft.infix_parts().to_vec(),
                                ft.second_infix().map(ToOwned::to_owned),
                                ft.suffix().to_string(),
                            );
                            let part = format!("{key:?}");

                            if part.contains('/') || part.contains('\\') || part.contains(':') {
                                //cov:excl-start
                                tv_transform.state = TomlValueState::new_validation_failed(
                                        "Output filename components must not contain path separators or colons"
                                            .to_string(),
                                    );
                                tv_transform.help = Some(
                                        "All output files must be below the current directory. \
                                            This is the last ditch check and you seeing this means that a step\n
                                            has neglected to verify this earlier in verification".to_string(),
                                    );
                                return;
                                //cov:excl-stop
                            }

                            key_to_entries
                                .entry(key)
                                .or_default()
                                .push((idx, decl.span.clone()));
                        }
                    }
                    all_decls.push(Some(decls));
                } else {
                    all_decls.push(None);
                }
            }
        } //cov:excl-line
        // Collect stdout-related errors before moving all_decls.
        // stdout_entries: (transform_idx, span) for each stdout declaration.
        // stdout_has_demux_before: parallel vec; true if a Demultiplex step preceded that transform.
        let mut stdout_entries: Vec<(usize, std::ops::Range<usize>)> = Vec::new();

        let mut stdout_has_demux_before: Vec<bool> = Vec::new();
        {
            let mut seen_demux = false;
            if let Some(transforms) = self.transform.value.as_ref() {
                for (idx, tv_transform) in transforms.iter().enumerate() {
                    let has_demux_before = seen_demux;
                    if matches!(
                        tv_transform.value.as_ref(),
                        Some(PartialTransformation::Demultiplex(_))
                    ) {
                        seen_demux = true;
                    }
                    for decl in all_decls.get(idx).into_iter().flatten().flatten() {
                        if matches!(decl.target, WriteTargetConfig::Stdout) {
                            stdout_entries.push((idx, decl.span.clone()));
                            stdout_has_demux_before.push(has_demux_before);
                        }
                    }
                }
            } //cov:excl-line
        }

        self.output_declarations_per_transformation = Some(all_decls);

        // Second pass (mutable): report conflicts.
        for ((infix_parts, _second_suffix, suffix), entries) in key_to_entries {
            if entries.len() > 1
                && let Some(transforms) = self.transform.value.as_mut()
            {
                let spans: Vec<_> = entries
                    .iter()
                    .enumerate()
                    .map(|(n, (_, span))| {
                        (
                            span.clone(),
                            format!(
                                "{}step writing to this file",
                                if n == 0 { "1st " } else { "2nd " }
                            ),
                        )
                    })
                    .collect();
                let file_hint = if infix_parts.is_empty() {
                    format!("suffix .{suffix}")
                } else {
                    format!("infix '{}', suffix .{suffix}", infix_parts.join("_"))
                };
                transforms[entries[0].0].state = TomlValueState::Custom { spans };
                transforms[entries[0].0].help = Some(format!(
                    "Two steps would write to the same output file ({file_hint}).\n\
                        Change the infix in one of them to avoid the conflict."
                ));
            }
        }

        // Third pass (mutable): report stdout errors.
        if let Some(transforms) = self.transform.value.as_mut() {
            // Stdout after demultiplex (non-singleton) is invalid.
            for (entry_idx, (idx, span)) in stdout_entries.iter().enumerate() {
                if stdout_has_demux_before[entry_idx] {
                    transforms[*idx].state = TomlValueState::Custom {
                        spans: vec![(span.clone(), "this step writes to stdout".to_string())],
                    };
                    transforms[*idx].help = Some(
                        "Cannot write to stdout after a Demultiplex step — output from multiple \
                         barcodes would be interleaved. Use an infix to write to a file instead."
                            .to_string(),
                    );
                }
            }
            // Multiple stdout outputs are invalid (independent of demultiplex).
            if stdout_entries.len() > 1 {
                let spans: Vec<_> = stdout_entries
                    .iter()
                    .enumerate()
                    .map(|(n, (_, span))| {
                        (
                            span.clone(),
                            format!(
                                "{}step writing to stdout",
                                if n == 0 { "1st " } else { "2nd " }
                            ),
                        )
                    })
                    .collect();
                transforms[stdout_entries[0].0].state = TomlValueState::Custom { spans };
                transforms[stdout_entries[0].0].help = Some(
                    "Multiple steps write to stdout. Only one step may write to stdout at a time."
                        .to_string(),
                );
            }
        } // cov:excl-line
    }

    pub fn verify_demultiplex_unique(&mut self) {
        let mut seen: IndexMap<String, std::ops::Range<usize>> = IndexMap::new();
        if let Some(transforms) = self.transform.as_mut() {
            for trafo in transforms.iter_mut() {
                if let Some(PartialTransformation::Demultiplex(demultiplex_config)) =
                    trafo.value.as_mut()
                    && let Some(demultiplex_config_value) =
                        demultiplex_config.toml_value.value.as_ref()
                    && let Some(in_label) = demultiplex_config_value.in_label.as_ref()
                {
                    let in_label: String = in_label.as_ref().to_string();
                    if let Some(old) =
                        seen.insert(in_label.clone(), demultiplex_config_value.in_label.span())
                    {
                        let spans = vec![
                            (
                                demultiplex_config_value.in_label.span(),
                                "2nd use of this label for demultiplexing".to_string(),
                            ),
                            (
                                old.clone(),
                                "first use for this label for demultiplexing".to_string(),
                            ),
                        ];
                        demultiplex_config.toml_value.help = Some(
                                    "Demultiplexing twice on the same label is nonsentical and unsupported.".to_string());
                        demultiplex_config.toml_value.state = TomlValueState::Custom { spans }
                    }
                }
            }
        }
    }

    fn verify_barcodes_used(
        &mut self,
        used_barcode_sections: &std::collections::HashSet<TagLabel>,
    ) {
        // Check that barcode names are unique across all barcodes sections
        if self.input.is_ok() {
            // we clear transforms otherwise

            if let Some(Some(barcodes)) = self.barcodes.as_mut() {
                for (barcode_section_name, tv_barcodes) in &mut barcodes.map {
                    let found_barcode_using_step =
                        used_barcode_sections.contains(barcode_section_name);
                    // Also check wether the  output's tag_to_reference uses this barcode section
                    if !found_barcode_using_step {
                        tv_barcodes.state = TomlValueState::new_validation_failed(
                            "Barcode section not referenced by any step",
                        );
                        tv_barcodes.help = Some(
                            "Add a step that uses this barcode section, use it in `output.bam.references_from_barcodes`, or remove it.".to_string(),
                        );
                    }
                }
            }
        }
    }

    fn verify_merge_demultiplexed(&mut self) {
        // Collect the in_labels of the (lookup-mode) Demultiplex steps; the
        // legacy behaviour only considered the first such step.
        let mut available_demultiplex_labels: Vec<String> = Vec::new();
        if let Some(trafos) = self.transform.value.as_ref() {
            for trafo in trafos {
                if let Some(PartialTransformation::Demultiplex(demultiplex_config_toml)) =
                    trafo.value.as_ref()
                    && let Some(demultiplex_config) =
                        demultiplex_config_toml.toml_value.value.as_ref()
                    && let Some(TagLabel::Normal(in_label)) = demultiplex_config.in_label.as_ref()
                    && matches!(
                        demultiplex_config.lookup_mode,
                        Some(crate::transformations::demultiplex::LookupMode::Label) | None
                    )
                {
                    available_demultiplex_labels.push(in_label.clone());
                    break;
                }
            }
        } // cov:excl-line

        if let Some(trafos) = self.transform.value.as_mut() {
            for trafo in trafos.iter_mut() {
                if let Some(PartialTransformation::OutputBAM(pt)) = trafo.value.as_mut()
                    && let Some(partial) = pt.toml_value.value.as_mut()
                {
                    crate::transformations::output::verify_output_bam_merge(
                        partial,
                        &available_demultiplex_labels,
                    );
                }
            }
        } // cov:excl-line
    }
}

impl Config {
    /// There are transformations that we need to expand right away,
    /// so we can accurately check the names
    pub fn check(self, config_input_is_stdin: bool) -> Result<CheckedConfig> {
        self.inner_check(true, config_input_is_stdin)
    }

    fn inner_check(
        mut self,
        check_input_files_exist: bool,
        config_input_is_stdin: bool,
    ) -> Result<CheckedConfig> {
        use fastqrab_io::STDIN_MAGIC_PATH;
        let mut errors = Vec::new();

        if config_input_is_stdin && self.input.structured.any_input_is_stdin() {
            anyhow::bail!(
                "Cannot read configuration from stdin ('-') when the configuration also uses stdin \
             ('{STDIN_MAGIC_PATH}') for FASTQ input. Use a config file on disk instead."
            );
        }
        //no point in checking them if segment definition is broken
        let mut input_formats_per_segment = None;

        let threading_configuration = if check_input_files_exist {
            //todo: if we figure out a way to have VerifyIn do this only
            // when requested, we could have better error messages.
            let input_formats_observed = self.check_input_format(&mut errors);
            let r = self.configure_multithreading(&input_formats_observed);
            input_formats_per_segment = Some(input_formats_observed.per_segment);
            r
        } else {
            ThreadingConfiguration {
                n_input_per_segment: std::num::NonZeroUsize::MIN,
                n_pod_demux_per_segment: std::num::NonZeroUsize::MIN,
                n_output: std::num::NonZeroUsize::MIN,
                n_processing: std::num::NonZeroUsize::MIN,
            }
        };
        let stages = self.transforms_to_stages();
        //self.transform is now empty, the trafos have been expanded into steps.
        assert!(self.transform.is_empty());
        // Return collected errors if any
        if !errors.is_empty() {
            // For multiple errors, format them cleanly
            let combined_error = errors
                .into_iter()
                .map(|e| format!("{e:?}"))
                .collect::<Vec<_>>()
                .join("\n\n---------\n\n");
            bail!("Multiple errors occurred:\n\n{combined_error}");
        }

        Ok(CheckedConfig {
            input: self.input,
            output: self.output,
            stages,
            options: self.options,
            barcodes: self.barcodes.unwrap_or_default(),
            benchmark: self.benchmark,
            report_labels: self.report_labels,
            threading_configuration,
            raw_config: std::sync::Arc::from(""),
            output_declarations_per_transformation: self.output_declarations_per_transformation,
            input_formats_detected: input_formats_per_segment.unwrap_or_else(Default::default),
        })
    }

    /// Check configuration for validation mode (allows missing input files)
    pub fn check_for_validation(self, config_input_is_stdin: bool) -> Result<CheckedConfig> {
        self.inner_check(false, config_input_is_stdin)
    }

    #[expect(clippy::similar_names, reason = "domain names are that way")]
    #[expect(clippy::too_many_lines, reason = "validation takes lines")]
    #[mutants::skip] // saw_gzip is only necessary for multi threading, and that's not being
    // observed
    fn check_input_format(&mut self, errors: &mut Vec<anyhow::Error>) -> InputFormatsObserved {
        let mut saw_fasta = false;
        let mut saw_bam = false;
        let mut saw_fastq = false;
        let mut saw_gzip = false;
        let mut per_segment = IndexMap::new();

        match &self.input.structured {
            StructuredInput::Interleaved { files, .. } => {
                let mut interleaved_format: Option<DetectedInputFormat> = None;
                for filename in files {
                    if let Ok((format, compression_format)) =
                        io::input::detect_input_format(Path::new(filename))
                    {
                        if let Some(existing) = interleaved_format {
                            if existing != format {
                                errors.push(anyhow!(
                                        "(input): Interleaved inputs must all have the same format. Found both {existing:?} and {format:?}."
                                    ));
                            }
                        } else {
                            interleaved_format = Some(format);
                        }

                        per_segment.insert("--interleaved--".to_string(), format);
                        match format {
                            DetectedInputFormat::Fastq => {
                                saw_fastq = true;
                                if compression_format == CompressionFormat::Gzip {
                                    saw_gzip = true;
                                }
                            }
                            DetectedInputFormat::Fasta => {
                                saw_fasta = true;
                                if compression_format == CompressionFormat::Gzip {
                                    saw_gzip = true;
                                }
                            }
                            DetectedInputFormat::Bam => saw_bam = true,
                        }
                    } else {
                        //ignore for now. We'll complain again later,
                        //but here we're only checking the consistency within the configuration
                        /* errors.push(
                        e.context(format!(
                            "(input): Failed to detect input format for interleaved file '{filename}'."
                        )),) */
                    }
                }
            }
            StructuredInput::Segmented {
                segment_order,
                segment_files,
            } => {
                for segment_name in segment_order {
                    let mut segment_format: Option<DetectedInputFormat> = None;
                    if let Some(files) = segment_files.get(segment_name) {
                        for filename in files {
                            if let Ok((format, compression_format)) =
                                io::input::detect_input_format(Path::new(filename))
                            {
                                if let Some(existing) = segment_format {
                                    if existing != format {
                                        errors.push(anyhow!(
                                                "(input): Segment '{segment_name}' mixes input formats {existing:?} and {format:?}. Mixing formats like this is not supported."
                                            ));
                                    }
                                } else {
                                    segment_format = Some(format);
                                }
                                per_segment.insert(segment_name.clone(), format);
                                match format {
                                    DetectedInputFormat::Fastq => {
                                        saw_fastq = true;
                                        if compression_format == CompressionFormat::Gzip {
                                            saw_gzip = true;
                                        }
                                    }
                                    DetectedInputFormat::Fasta => {
                                        saw_fasta = true;
                                        if compression_format == CompressionFormat::Gzip {
                                            saw_gzip = true;
                                        }
                                    }
                                    DetectedInputFormat::Bam => saw_bam = true,
                                }
                            } else {
                                //ignore for now. We'll complain again later,
                                //but here we're only checking the consistency within the configuration
                                /* errors.push(
                                    e.context(format!(
                                        "(input): Failed to detect input format for file '{filename}' in segment '{segment_name}'."
                                    )),
                                ), */
                            }
                        }
                    } // cov:excl-line
                }
            }
        }

        if saw_fasta && self.input.options.fasta_fake_quality.is_none() {
            errors.push(anyhow!(
                "[input.options]: 'fasta_fake_quality' must be set when reading FASTA inputs."
            ));
        }

        if saw_bam {
            let include_mapped = self.input.options.bam_include_mapped;
            let include_unmapped = self.input.options.bam_include_unmapped;
            if include_mapped.is_none() {
                errors.push(anyhow!(
                    "[input.options]: 'bam_include_mapped' must be set (true or false) when reading BAM inputs."
                ));
            }
            if include_unmapped.is_none() {
                errors.push(anyhow!(
                    "[input.options]: 'bam_include_unmapped' must be set (true or false) when reading BAM inputs."
                ));
            } else if include_mapped == Some(false) && include_unmapped == Some(false) {
                errors.push(anyhow!(
                    "[input.options]: At least one of 'bam_include_mapped' or 'bam_include_unmapped' must be true when reading BAM inputs."
                ));
            }
        }

        InputFormatsObserved {
            fastq: saw_fastq,
            fasta: saw_fasta,
            bam: saw_bam,
            gzip: saw_gzip,
            per_segment,
        }
    }

    fn transforms_to_stages(&mut self) -> Vec<Stage> {
        let allowed_tags_per_stage = self.allowed_tags_per_transformation.clone();
        let forgotten_tags_per_stage = self.forgotten_tags_per_transformation.clone();
        let output_declarations_per_stage = self.output_declarations_per_transformation.clone();
        let input_declarations_per_stage = self.input_declarations_per_transformation.clone();

        let stages: Vec<Stage> = self
            .transform
            .drain(..)
            .zip(allowed_tags_per_stage)
            .zip(forgotten_tags_per_stage)
            .zip(output_declarations_per_stage)
            .zip(input_declarations_per_stage)
            .filter(|((((t, _), _), _), _)| !matches!(t, Transformation::Report { .. }))
            .map(|((((t, tags), forgotten), decls), input_decls)| Stage {
                transformation: t,
                allowed_tags: tags.into_iter().collect(),
                forgotten_tags: forgotten.into_iter().collect(),
                output_declarations: decls,
                input_declarations: input_decls,
            })
            .collect();

        stages
    }

    #[mutants::skip] // only used for threading decisions
    fn any_bam_or_gzip_output(&self) -> bool {
        for transform in &self.transform {
            if let Transformation::OutputBAM(_) = transform {
                return true;
            }
            if let Transformation::OutputFASTA(crate::transformations::output::OutputFASTA {
                compression,
                ..
            }) = transform
                && matches!(compression, CompressionFormat::Gzip)
            {
                return true;
            }

            if let Transformation::OutputFASTQ(crate::transformations::output::OutputFASTQ {
                compression,
                ..
            }) = transform
                && matches!(compression, CompressionFormat::Gzip)
            {
                return true;
            }
        }
        false
    }

    #[mutants::skip] // yeah, no rapidgzip doesn't change the result
    fn configure_multithreading(
        &mut self,
        input_formats_observed: &InputFormatsObserved,
    ) -> ThreadingConfiguration {
        let segment_count = self.input.parser_count();
        let can_multicore_input = input_formats_observed.gzip;
        // self.input_formats_observed.saw_bam as of 2025-12-16, multi core bam isn't faster. I
        // mean the user can enable it by setting threads_per_segment > 1, but by default we
        // choose one core

        let can_multicore_compression = self.any_bam_or_gzip_output();
        let counts = calculate_thread_counts(
            self.options.threads,
            self.input.options.threads_per_segment,
            self.output.as_ref().map(|x| x.compression_threads.into()),
            segment_count,
            get_number_of_cores(),
            can_multicore_input,
            can_multicore_compression,
        );
        self.options.threads = Some(counts.processing);
        self.input.options.threads_per_segment = Some(counts.input_per_segment);
        if let Some(output) = &mut self.output {
            output.compression_threads = NonZero::new(counts.compression)
                .expect("Calculated output threads should never be zero");
        }

        ThreadingConfiguration {
            n_input_per_segment: std::num::NonZeroUsize::new(counts.input_per_segment)
                .expect("Thread count must be > 0"),
            n_pod_demux_per_segment: std::num::NonZeroUsize::new(counts.pod_demux_per_segment)
                .expect("Thread count must be > 0"),
            n_output: std::num::NonZeroUsize::new(counts.compression)
                .expect("Thread count must be > 0"),
            n_processing: std::num::NonZeroUsize::new(counts.processing)
                .expect("Thread count must be > 0"),
        }
    }
}

impl CheckedConfig {
    pub fn get_ix_separator(&self) -> String {
        self.output
            .as_ref()
            .map_or_else(output::default_ix_separator, |x| x.ix_separator.clone())
    }
}

/// Every thread-pool size the pipeline spawns, computed in one place so they can
/// be tuned together. `input_per_segment` sizes decompression (rapidgzip `-P`,
/// BAM bgzf), `pod_demux_per_segment` the columnar pod-parser's demux pool,
/// `processing` the worker pool, and `compression` the output writers.
#[derive(Debug, PartialEq, Eq)]
struct ThreadCounts {
    processing: usize,
    input_per_segment: usize,
    pod_demux_per_segment: usize,
    compression: usize,
}

fn calculate_thread_counts(
    step_thread_count: Option<usize>,
    threads_per_segment: Option<usize>,
    compression_threads: Option<usize>,
    segment_count: usize,
    cpu_count: usize,
    can_multicore_decompression: bool,
    can_multicore_compression: bool,
) -> ThreadCounts {
    let threads_per_segment = if can_multicore_decompression {
        threads_per_segment
    } else {
        Some(1)
    };
    let compression_threads = compression_threads.unwrap_or_else(|| {
        if can_multicore_compression {
            let half = cpu_count / 2;
            half.min(5)
        } else {
            1
        }
    });

    let (processing, input_per_segment) = match (step_thread_count, threads_per_segment) {
        (Some(step_thread_count), Some(threads_per_segment)) => {
            (step_thread_count, threads_per_segment)
            //keep whatever the user set.
        }
        (None, Some(threads_per_segment)) => (
            //all remaining cores into steps
            cpu_count
                .saturating_sub(threads_per_segment * segment_count)
                .max(1),
            threads_per_segment,
        ),
        (Some(thread_count), None) => {
            //all remaining cores into parsing
            let per_segment = (cpu_count.saturating_sub(thread_count) / segment_count).max(1);
            (thread_count, per_segment)
        }
        (None, None) => {
            let half = cpu_count / 2;
            //our benchmarks says the sweet spot is somewhere around 5 threads per segment
            let threads_per_segment = (half / segment_count).clamp(1, 5);
            (
                //if we rounded down, or had way more cores, we will use more threads per steps
                cpu_count
                    .saturating_sub(threads_per_segment * segment_count)
                    .max(1),
                threads_per_segment,
            )
        }
    };

    // Pod-parser demux pool, per segment. Independent of decompression: it's the
    // columnar scan+copy, useful even for uncompressed input. Its sweet spot is
    // 2-4 workers — more just oversubscribes the allocator — so we size it off
    // spare cores per segment and clamp it small.
    let pod_demux_per_segment = (cpu_count / segment_count.max(1) / 2).clamp(1, 4);

    ThreadCounts {
        processing,
        input_per_segment,
        pod_demux_per_segment,
        compression: compression_threads,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_tag_name_valid() {
        // Valid tag names
        validate_tag_name("a").unwrap();
        validate_tag_name("A").unwrap();
        validate_tag_name("_").unwrap();
        validate_tag_name("abc").unwrap();
        validate_tag_name("ABC").unwrap();
        validate_tag_name("a123").unwrap();
        validate_tag_name("A123").unwrap();
        validate_tag_name("_123").unwrap();
        validate_tag_name("tag_name").unwrap();
        validate_tag_name("TagName").unwrap();
        validate_tag_name("tag123_name").unwrap();
        validate_tag_name("_private_tag").unwrap();
    }

    #[test]
    fn test_validate_tag_name_invalid() {
        // Invalid tag names
        validate_tag_name("").unwrap_err();
        validate_tag_name("123").unwrap_err();
        validate_tag_name("123abc").unwrap_err();
        validate_tag_name("tag-name").unwrap_err();
        validate_tag_name("tag.name").unwrap_err();
        validate_tag_name("tag name").unwrap_err();
        validate_tag_name("tag@name").unwrap_err();
        validate_tag_name("tag/name").unwrap_err();
        validate_tag_name("tag\\name").unwrap_err();
        validate_tag_name("tag:name").unwrap_err();
        validate_tag_name("len_123").unwrap_err();
        validate_tag_name("len_shu").unwrap_err();
        validate_tag_name("ReadName").unwrap_err();
        validate_tag_name("read_no").unwrap_err();
    }

    #[test]
    fn test_validate_segment_label_valid() {
        // Valid segment labels
        let f = toml_pretty_deser::FieldMatchMode::Exact;
        validate_segment_label("a", f).unwrap();
        validate_segment_label("A", f).unwrap();
        validate_segment_label("_", f).unwrap();
        validate_segment_label("abc", f).unwrap();
        validate_segment_label("ABC", f).unwrap();
        validate_segment_label("123", f).unwrap_err();
        validate_segment_label("a123", f).unwrap();
        validate_segment_label("A123", f).unwrap();
        validate_segment_label("123abc", f).unwrap_err();
        validate_segment_label("read1", f).unwrap();
        validate_segment_label("READ1", f).unwrap();
        validate_segment_label("segment_name", f).unwrap();
        validate_segment_label("segment123", f).unwrap();
        validate_segment_label("_internal", f).unwrap();
    }

    #[test]
    fn test_validate_segment_label_invalid() {
        // Invalid segment labels
        let f = toml_pretty_deser::FieldMatchMode::Exact;
        validate_segment_label("", f).unwrap_err();
        validate_segment_label("1", f).unwrap_err();
        validate_segment_label("segment-name", f).unwrap_err();
        validate_segment_label("segment.name", f).unwrap_err();
        validate_segment_label("segment name", f).unwrap_err();
        validate_segment_label("segment@name", f).unwrap_err();
        validate_segment_label("segment/name", f).unwrap_err();
        validate_segment_label("segment\\name", f).unwrap_err();
        validate_segment_label("segment:name", f).unwrap_err();
        validate_segment_label("fasta_fake_quality", f).unwrap_err();
        validate_segment_label("bam_include_mapped", f).unwrap_err();
        validate_segment_label("bam_include_unmapped", f).unwrap_err();
        validate_segment_label("read_comment_character", f).unwrap_err();
        validate_segment_label("threads_per_segment", f).unwrap_err();
        validate_segment_label("tpd_field_match_mode", f).unwrap_err();

        let f = toml_pretty_deser::FieldMatchMode::AnyCase;
        validate_segment_label("FaSTA___FAKE-QUALITY", f).unwrap_err();
    }

    #[test]
    fn test_calculate_thread_counts() {
        let tc = |processing, input_per_segment, pod_demux_per_segment, compression| ThreadCounts {
            processing,
            input_per_segment,
            pod_demux_per_segment,
            compression,
        };
        // Test various combinations of inputs
        assert_eq!(
            calculate_thread_counts(Some(8), Some(2), None, 4, 16, true, false),
            tc(8, 2, 2, 1)
        );
        assert_eq!(
            calculate_thread_counts(Some(8), Some(2), None, 40, 1, true, false),
            tc(8, 2, 1, 1)
        );
        assert_eq!(
            calculate_thread_counts(None, Some(2), None, 4, 16, true, false),
            tc(8, 2, 2, 1)
        );
        assert_eq!(
            calculate_thread_counts(Some(8), None, None, 4, 16, true, false),
            tc(8, 2, 2, 1)
        );
        assert_eq!(
            calculate_thread_counts(Some(9), None, None, 4, 16, true, false),
            tc(9, 1, 2, 1)
        );
        assert_eq!(
            calculate_thread_counts(None, None, None, 4, 16, true, false),
            tc(8, 2, 2, 1)
        );
        assert_eq!(
            calculate_thread_counts(None, None, None, 2, 16, true, false),
            tc(8, 4, 4, 1)
        );
        assert_eq!(
            calculate_thread_counts(None, None, None, 1, 16, true, false),
            tc(11, 5, 4, 1)
        );
        assert_eq!(
            calculate_thread_counts(None, None, None, 1, 16, false, false),
            tc(15, 1, 4, 1)
        );
        assert_eq!(
            calculate_thread_counts(None, None, None, 1, 16, false, true),
            tc(15, 1, 4, 5)
        );

        assert_eq!(
            calculate_thread_counts(None, None, None, 1, 8, false, true),
            tc(7, 1, 4, 4)
        );
    }
}
