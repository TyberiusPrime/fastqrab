use anyhow::{Result, bail};
use bstr::BString;
use regex::bytes::Regex;
use schemars::JsonSchema;
use std::fmt;
use std::rc::Rc;
use std::{cell::RefCell, num::NonZero};
use toml_pretty_deser::{MustAdapt, TomlValue, TomlValueState, ValidationFailure, tpd_alias_leaf};
use typed_floats::tf64::NonNaN;

pub use fastqrab_dna::dna;
pub mod fileformats;
pub mod segments;

use toml_pretty_deser::prelude::tpd;

use crate::segments::SegmentIndexOrAll;

pub const STDIN_MAGIC_PATH: &str = "--stdin--";

#[must_use]
#[mutants::skip]
pub fn get_number_of_cores() -> usize {
    std::env::var("FASTQRAB_PROCESSOR_NUM_CPUS")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or_else(num_cpus::get)
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, JsonSchema)]
#[tpd]
pub enum CompressionFormat {
    #[tpd(alias = "uncompressed")]
    #[tpd(alias = "raw")]
    #[default]
    Uncompressed,
    #[tpd(alias = "gzip")]
    #[tpd(alias = "gz")]
    Gzip,
    #[tpd(alias = "zstd")]
    #[tpd(alias = "zst")]
    Zstd,
}

impl CompressionFormat {
    #[must_use]
    pub fn apply_suffix(self, base: &str) -> String {
        match self {
            CompressionFormat::Uncompressed => base.to_string(),
            CompressionFormat::Gzip => format!("{base}.gz"),
            CompressionFormat::Zstd => format!("{base}.zst"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, JsonSchema)]
#[tpd]
pub enum FileFormat {
    #[default]
    Fastq,
    Fasta,
    Bam,
    /// Generic text output (TSV, JSON, progress files, etc.). Uses the same
    /// write path as Fastq/Fasta but without implying a specific record format.
    #[tpd(skip)]
    #[schemars(skip)]
    Text,
    None,
}

impl FileFormat {
    #[must_use]
    pub fn default_suffix(&self) -> &'static str {
        match self {
            FileFormat::Fastq => "fq",
            FileFormat::Fasta => "fasta",
            FileFormat::Bam => "bam",
            // cov:excl-start
            FileFormat::Text => unreachable!(
                "Text format has no default suffix — use WriteTargetConfig::File suffix instead"
            ),
            FileFormat::None => unreachable!("No output has no suffix either"), // cov:excl-stop
        }
    }

    #[must_use]
    pub fn get_suffix(
        &self,
        compression: CompressionFormat,
        custom_suffix: Option<&String>,
    ) -> String {
        if let Some(custom) = custom_suffix {
            return custom.clone();
        }

        match self {
            FileFormat::Fastq | FileFormat::Fasta => {
                let base = self.default_suffix();
                compression.apply_suffix(base)
            }
            FileFormat::Bam => self.default_suffix().to_string(), // cov:exl-line
            FileFormat::Text | FileFormat::None => unreachable!("No default suffix for text files"), // cov:exl-line
        }
    }
}
//
// Default functions for common values
#[must_use]
pub fn default_region_separator() -> bstr::BString {
    b"_".into()
}

#[must_use]
pub fn default_comment_separator() -> u8 {
    b'|'
}

#[must_use]
pub fn default_comment_insert_char() -> u8 {
    b' '
}

//input defaults
#[must_use]
#[mutants::skip]
pub const fn default_buffer_size() -> usize {
    100 * 1024 // bytes, per fastq input file
}

#[mutants::skip]
#[must_use]
pub const fn default_output_buffer_size() -> usize {
    1024 * 1024 // bytes, per fastq input file
}

#[must_use]
#[mutants::skip]
/// # Panics
/// Never.
pub const fn default_block_size() -> NonZero<usize> {
    NonZero::new(10000) // in 'molecules', ie. read1, read2, index1, index2 tuples.
        .expect("Can not fail")
}

#[must_use]
#[mutants::skip]
/// # Panics
/// Can't static value into non-zero
pub fn default_max_molecules_in_flight(block_size: NonZero<usize>) -> NonZero<usize> {
    // in 'molecules', ie. read1, read2, index1, index2 tuples.
    // Historically this was 100 *blocks*; at the default block_size of 10000
    // that is 1_000_000 molecules, which we keep as the read-based default.
    let block_size: usize = block_size.into();
    NonZero::new(block_size * 100).expect("Block_size == 0 should have been caught in validation")
}

#[must_use]
pub const fn default_spot_check_read_pairing() -> bool {
    true
}

#[must_use]
pub const fn default_include_read_name() -> bool {
    true
}

#[must_use]
pub const fn default_include_read_comment() -> bool {
    false
}
// Schema helper for string or list of strings
#[derive(JsonSchema)]
#[serde(untagged)]
pub enum StringOrVecString {
    String(String),
    Vec(Vec<String>),
}

#[derive(Debug, Clone, Copy)]
pub enum TagValueType {
    Location, // string + in-sequence-location
    String,   // just a piece of text
    Numeric((Option<NonNaN>, Option<NonNaN>)),
    Bool,
}

#[derive(Debug)]
pub struct UsedTag<'a> {
    pub name: TagLabel,
    pub accepted_tag_types: &'a [TagValueType],
    pub toml_source: Rc<RefCell<(&'a mut TomlValueState, &'a mut Option<String>)>>,
    pub further_help: Option<String>,
}

impl UsedTag<'_> {
    #[must_use]
    pub fn add_help(mut self, line: impl AsRef<str>) -> Self {
        self.further_help = match self.further_help.take() {
            //cov:excl-start
            Some(existing) => Some(format!("{}\n{}", existing, line.as_ref())),
            // cov:excl-stop
            None => Some(line.as_ref().to_string()),
        };
        self
    }
}

pub trait ToUsedTag {
    fn to_used_tag<'a>(&'a mut self, accepted_tag_types: &'a [TagValueType])
    -> Option<UsedTag<'a>>;
}

pub trait ToUsedTags {
    fn to_used_tags(&mut self) -> Vec<Option<UsedTag<'_>>>;
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum StringTagContent {
    Undefined,
    Barcodes,
    Labels,
}

#[derive(Debug)]
pub struct DeclaredTag<'a> {
    pub name: TagLabel,
    pub tag_type: TagValueType,
    pub toml_source_state: &'a mut TomlValueState,
    pub toml_source_help: &'a mut Option<String>,
    pub toml_source_span: std::ops::Range<usize>,
    pub contains: StringTagContent,
    /// For `Location` tags, the single segment this tag's bytes live on (its
    /// runtime `source_id`). Recorded so a step that moves reads between segments
    /// per-read (conditional `Swap`) can tell, at config-verify, which location
    /// tags it would split across segments and must forget. `None` for tags with
    /// no single segment of origin (most non-location tags).
    pub segment: Option<crate::segments::SegmentIndex>,
}

impl DeclaredTag<'_> {
    #[must_use]
    pub fn with_contents(self, contains: StringTagContent) -> Self {
        Self { contains, ..self }
    }

    /// Record the segment a `Location` tag's bytes originate on. See
    /// [`segment`](Self::segment).
    #[must_use]
    pub fn with_segment(self, segment: crate::segments::SegmentIndex) -> Self {
        Self {
            segment: Some(segment),
            ..self
        }
    }
}

pub trait ToDeclaredTag {
    fn to_declared_tag(&mut self, tag_type: TagValueType) -> Option<DeclaredTag<'_>>;
}
//see deser for impl

#[derive(Default, Debug)]
pub enum RemovedTags<'a> {
    #[default]
    None,
    All,
    Some(Vec<(TagLabel, &'a mut TomlValue<TagLabel>)>),
    /// Like [`Some`](RemovedTags::Some) but for a *computed* set the step doesn't
    /// have a config field to point at (e.g. conditional `Swap` forgetting every
    /// location tag on the swapped segments). These tags are known to exist, so
    /// no per-tag toml source is needed for an error span.
    SomeOwned(Vec<TagLabel>),
}
impl TagValueType {
    #[must_use]
    pub fn compatible(self, other: TagValueType) -> bool {
        matches!(
            (self, other),
            (TagValueType::Location, TagValueType::Location)
                | (TagValueType::String, TagValueType::String)
                | (TagValueType::Bool, TagValueType::Bool)
                | (TagValueType::Numeric(_), TagValueType::Numeric(_))
        )
    }
}

impl std::fmt::Display for TagValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TagValueType::Location => write!(f, "Location"),
            TagValueType::String => write!(f, "String"),
            TagValueType::Numeric((lower, upper)) => match (lower, upper) {
                (None, None) => write!(f, "Numeric"),
                (None, Some(upper)) => write!(f, "Numeric (..={upper})"), // cov:excl-line
                (Some(lower), None) => write!(f, "Numeric ({lower}..)"),
                (Some(lower), Some(upper)) => write!(f, "Numeric ({lower}..={upper})"),
            },
            TagValueType::Bool => write!(f, "Boolean"),
        }
    }
}

#[must_use]
pub fn tpd_adapt_bstring(input: TomlValue<String>) -> TomlValue<BString> {
    input.map(|s| BString::from(s.as_bytes()))
}

#[must_use]
pub fn tpd_adapt_bstring_uppercase(input: TomlValue<String>) -> TomlValue<BString> {
    input.map(|s| BString::from(s.as_bytes().to_ascii_uppercase()))
}

// #[must_use]
// pub fn tpd_adapt_dna_bstring(mut input: TomlValue<String>) -> TomlValue<BString> {
//     input.try_map(|s| {
//         let res = BString::from(s.as_bytes());
//         for org_c in res.iter() {
//             let c = org_c.to_ascii_uppercase();
//             if !matches!(c, b'A' | b'C' | b'G' | b'T') {
//                 return Err(ValidationFailure::new(
//                     format!(
//                         "Invalid DNA base: '{}' (ascii: {org_c}).",
//                         std::char::from_u32(*org_c as u32)
//                             .unwrap_or(std::char::REPLACEMENT_CHARACTER)
//                     ),
//                     None,
//                 ));
//             }
//         }
//         Ok(res)
//     })
// }
//
//
fn err_invalid_base(org_c: u8) -> String {
    format!(
        "Invalid DNA base: '{}' (ascii: {org_c}). Allowed letters are A, C, G, T and N.",
        std::char::from_u32(org_c.into()).unwrap_or(std::char::REPLACEMENT_CHARACTER)
    )
}

#[must_use]
pub fn tpd_adapt_dna_bstring_plus_n(mut input: TomlValue<String>) -> TomlValue<BString> {
    input.try_map(|s| {
        let res = BString::from(s.as_bytes());
        for org_c in res.iter() {
            let c = org_c.to_ascii_uppercase();
            if !matches!(c, b'A' | b'C' | b'G' | b'T' | b'N') {
                return Err(ValidationFailure::new(err_invalid_base(*org_c), None));
            }
        }
        Ok(res)
    })
}

#[must_use]
pub fn tpd_adapt_iupac_bstring(mut input: TomlValue<String>) -> TomlValue<BString> {
    input.try_map(|s| {
        let res = BString::from(s.as_bytes().to_ascii_uppercase());
        if let Some(org_c) = dna::first_non_iupac(res.as_ref()) {
            return Err(ValidationFailure::new(
                err_invalid_base(org_c),
                Some("Allowed letters are A G T C I R Y S W K M B D H V N ".to_string()),
            ));
        }
        Ok(res)
    })
}

#[must_use]
pub fn tpd_adapt_regex(mut input: TomlValue<String>) -> TomlValue<regex::bytes::Regex> {
    input.try_map(|s| match Regex::new(s) {
        Ok(r) => Ok(r),
        Err(e) => Err(ValidationFailure::new(
            "Invalid regex".to_string(),
            Some(format!("Regex engine error: {e}")),
        )),
    })
}

#[must_use]
pub fn tpd_adapt_u8_from_byte_or_char(mut input: TomlValue<toml_edit::Item>) -> TomlValue<u8> {
    let help =
        "Provide either a number (0..255), or a single letter string (with an ascii character)";
    input.try_map(|input| match input {
        toml_edit::Item::Value(toml_edit::Value::Integer(v)) => {
            if let Ok(b) = TryInto::<u8>::try_into(*v.value()) {
                Ok(b)
            } else {
                Err(ValidationFailure::new(
                    format!("Integer value {v} is out of range for a byte (0..255)"),
                    Some(help.to_string()),
                ))
            }
        }
        toml_edit::Item::Value(toml_edit::Value::String(v)) => {
            let mut chars = v.value().chars();
            let first = chars.next();
            let second = chars.next();
            if let Some(first) = first
                && let None = second
                && let Ok(char_first) = TryInto::<u8>::try_into(first)
            {
                Ok(char_first)
            } else {
                Err(ValidationFailure::new("Invalid value", Some(help)))
            }
        }
        _ => Err(ValidationFailure::new(
            "Wrong type, expected a byte",
            Some(help),
        )),
    })
}

/// A `u8` separator/terminator character that can also be explicitly disabled.
///
/// In TOML the field accepts a single-character string, an integer byte
/// (`0..=255`), or the literal `false` — the latter meaning "none"
/// (`Option::None`). Deserialize with [`tpd_adapt_u8_from_byte_char_or_false`].
///
/// Unlike a plain `Option<u8>` field (where tpd folds a *missing* value into
/// `None` and `is_missing()` always returns false), a value of this type carried
/// through `#[tpd(with = ...)]` keeps its missing state: `false` is a *present*
/// value, distinct from the field being absent. That lets `verify` require the
/// field to be set while still accepting `false` as "disabled".
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OptionalByte(pub Option<u8>);

impl fmt::Debug for OptionalByte {
    // Render transparently as the inner `Option<u8>`, so debug/error output
    // (e.g. transform-init error messages) is unaffected by the wrapper.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

/// tpd adapter for [`OptionalByte`]: accepts a single-character string, an
/// integer byte (`0..=255`), or `false` (→ `OptionalByte(None)`). `true` is
/// rejected. A missing value passes through as missing, so a `#[tpd(with =
/// "tpd_adapt_u8_from_byte_char_or_false")]` field can be required in `verify`
/// via [`toml_pretty_deser::TomlValue::is_missing`].
#[must_use]
pub fn tpd_adapt_u8_from_byte_char_or_false(
    mut input: TomlValue<toml_edit::Item>,
) -> TomlValue<OptionalByte> {
    let help = "Provide either a single letter string, an integer byte (0..255), or `false` to disable (match without trimming)";
    input.try_map(|input| match input {
        toml_edit::Item::Value(toml_edit::Value::Boolean(v)) => {
            if *v.value() {
                Err(ValidationFailure::new(
                    "`true` is not a valid value here; use `false` to disable, or a single character",
                    Some(help),
                ))
            } else {
                Ok(OptionalByte(None))
            }
        }
        toml_edit::Item::Value(toml_edit::Value::Integer(v)) => {
            if let Ok(b) = TryInto::<u8>::try_into(*v.value()) {
                Ok(OptionalByte(Some(b)))
            } else {
                Err(ValidationFailure::new(
                    format!("Integer value {v} is out of range for a byte (0..255)"),
                    Some(help.to_string()),
                ))
            }
        }
        toml_edit::Item::Value(toml_edit::Value::String(v)) => {
            let mut chars = v.value().chars();
            let first = chars.next();
            let second = chars.next();
            if let Some(first) = first
                && let None = second
                && let Ok(char_first) = TryInto::<u8>::try_into(first)
            {
                Ok(OptionalByte(Some(char_first)))
            } else {
                Err(ValidationFailure::new("Invalid value", Some(help)))
            }
        }
        _ => Err(ValidationFailure::new(
            "Wrong type, expected a byte, a single character, or `false`",
            Some(help),
        )),
    })
}

#[must_use]
pub fn tpd_adapt_extract_base_or_dot(mut input: TomlValue<String>) -> TomlValue<u8> {
    fn err() -> Result<u8, ValidationFailure> {
        Err(ValidationFailure::new(
            "Invalid DNA base",
            Some("Must be a single character: A, C, G, T, N or '.'"),
        ))
    }
    input.try_map(|input| {
        if input.len() == 1 {
            let s = input.as_bytes()[0].to_ascii_uppercase();
            if matches!(s, b'A' | b'C' | b'G' | b'T' | b'N' | b'.') {
                Ok(s)
            } else {
                err()
            }
        } else {
            err()
        }
    })
}

#[derive(Clone, Debug, JsonSchema, PartialEq, Eq)]
#[schemars(with = "String")]
pub enum TagLabel {
    Normal(String),
    Length(SegmentIndexOrAll, String),
    TagLength(String, String),
    TagLocation { source: String, definition: String },
    TagInitialLocation { source: String, definition: String },
    ReadNo,
}

impl TagLabel {
    #[must_use]
    pub fn is_virtual(&self) -> bool {
        matches!(
            self,
            TagLabel::Length(_, _)
                | TagLabel::ReadNo
                | TagLabel::TagLength(_, _)
                | TagLabel::TagLocation { .. }
                | TagLabel::TagInitialLocation { .. }
        )
    }

    #[must_use]
    pub fn source_tag(&self) -> Option<&String> {
        match self {
            TagLabel::Normal(_) | TagLabel::Length(_, _) | TagLabel::ReadNo => None,
            TagLabel::TagLength(source, _)
            | TagLabel::TagLocation { source, .. }
            | TagLabel::TagInitialLocation { source, .. } => Some(source),
        }
    }
}

impl PartialOrd for TagLabel {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TagLabel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl std::hash::Hash for TagLabel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_ref().hash(state);
    }
}
impl fmt::Display for TagLabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_ref().fmt(f)
    }
}

impl std::borrow::Borrow<str> for TagLabel {
    fn borrow(&self) -> &str {
        self.as_ref()
        // match self {
        //     TagLabel::Normal(s) | TagLabel::Length(_, s) | TagLabel::TagLength(_, s) => s.as_str(),
        //     TagLabel::ReadNo => "ReadNo",
        // }
    }
}

impl AsRef<str> for TagLabel {
    fn as_ref(&self) -> &str {
        match self {
            TagLabel::Normal(s)
            | TagLabel::Length(_, s)
            | TagLabel::TagLength(_, s)
            | TagLabel::TagInitialLocation { definition: s, .. }
            | TagLabel::TagLocation { definition: s, .. } => s.as_str(),
            TagLabel::ReadNo => "ReadNo",
        }
    }
}

impl ToDeclaredTag for TomlValue<TagLabel> {
    fn to_declared_tag(&mut self, tag_type: TagValueType) -> Option<DeclaredTag<'_>> {
        if self.as_ref().is_some() {
            let name = self.as_ref().expect("just checked").clone();
            let span = self.span();
            Some(DeclaredTag {
                name,
                tag_type,
                toml_source_state: &mut self.state,
                toml_source_help: &mut self.help,
                toml_source_span: span,
                contains: StringTagContent::Undefined,
                segment: None,
            })
        } else {
            // Since the virtual tag introduction, we do reach this on invalid TagLabes.
            None
            // cov:excl-stop
        }
    }
}
impl ToDeclaredTag for TomlValue<Option<TagLabel>> {
    fn to_declared_tag(&mut self, tag_type: TagValueType) -> Option<DeclaredTag<'_>> {
        if let Some(name) = self.as_ref().and_then(|x| x.as_ref()) {
            let span = self.span();
            Some(DeclaredTag {
                name: name.clone(),
                tag_type,
                toml_source_state: &mut self.state,
                toml_source_help: &mut self.help,
                toml_source_span: span,
                contains: StringTagContent::Undefined,
                segment: None,
            })
        } else {
            None
        }
    }
}

impl ToUsedTag for TomlValue<TagLabel> {
    fn to_used_tag<'a>(
        &'a mut self,
        accepted_tag_types: &'a [TagValueType],
    ) -> Option<UsedTag<'a>> {
        if let Some(name) = self.as_ref() {
            Some(UsedTag {
                name: name.clone(),
                accepted_tag_types,
                toml_source: Rc::new(RefCell::new((&mut self.state, &mut self.help))),
                further_help: None,
            })
        } else {
            None // cov:excl-line
        }
    }
}

impl ToUsedTag for TomlValue<MustAdapt<String, TagLabel>> {
    fn to_used_tag<'a>(
        &'a mut self,
        accepted_tag_types: &'a [TagValueType],
    ) -> Option<UsedTag<'a>> {
        if let Some(name) = self.as_ref().and_then(|x| x.as_ref_post()) {
            Some(UsedTag {
                name: name.clone(),
                accepted_tag_types,
                toml_source: Rc::new(RefCell::new((&mut self.state, &mut self.help))),
                further_help: None,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, JsonSchema)]
#[schemars(with = "String")]
pub struct ConditionalTagLabel {
    pub tag: TagLabel,
    pub invert: bool,
}
impl TryFrom<&str> for ConditionalTagLabel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(tag) = value.strip_prefix('!') {
            Ok(ConditionalTagLabel {
                tag: TagLabel::try_from(tag)?,
                invert: true,
            })
        } else {
            Ok(ConditionalTagLabel {
                tag: TagLabel::try_from(value)?,
                invert: false,
            })
        }
    }
}
impl ToUsedTag for TomlValue<Option<ConditionalTagLabel>> {
    #[track_caller]
    fn to_used_tag<'a>(
        &'a mut self,
        accepted_tag_types: &'a [TagValueType],
    ) -> Option<UsedTag<'a>> {
        assert!(
            accepted_tag_types.is_empty(),
            "accepted_tag_types not used for ConditionalTagLabel"
        );
        let ct = self.as_ref().expect("parent was ok?").as_ref();
        if let Some(ct) = ct {
            Some(UsedTag {
                name: ct.tag.clone(),
                accepted_tag_types: &[
                    TagValueType::Bool,
                    TagValueType::Location,
                    TagValueType::String,
                ][..],
                toml_source: Rc::new(RefCell::new((&mut self.state, &mut self.help))),
                further_help: None,
            })
        } else {
            None
        }
    }
}

pub fn offer_alternatives<T: AsRef<str>>(current: &str, available: &[T]) -> String {
    let available: Vec<_> = available
        .iter()
        .map(|s| format!("'{}'", s.as_ref()))
        .collect();
    toml_pretty_deser::suggest_alternatives(current, &available)
}

/// Validates that a tag name conforms to the pattern [a-zA-Z_][a-zA-Z0-9_]*
/// (starts with a letter or underscore, followed by zero or more alphanumeric characters or underscores)
///
/// # Panics
/// when `tag_name.is_empty()` lies
///
/// # Errors
/// Empty tag, invalid characters, etc
pub fn validate_tag_name(tag_name: &str) -> Result<()> {
    if tag_name.is_empty() {
        bail!(
            "Cannot be empty. Please provide a non-empty tag name that starts with a letter or underscore."
        );
    }

    let mut chars = tag_name.chars();
    let first_char = chars
        .next()
        .expect("tag_name is not empty so must have at least one char");

    if !first_char.is_ascii_alphabetic() && first_char != '_' {
        bail!("Must start with a letter or underscore (a-zA-Z_), got '{first_char}'",);
    }

    for ch in chars {
        if !ch.is_ascii_alphanumeric() && ch != '_' {
            bail!(
                "Must contain only letters, numbers, and underscores (a-zA-Z0-9_), found '{ch}'.",
            );
        }
    }

    for (forbidden, reason) in &[
        ("ReadName", "the index column in StoreTagsInTable"),
        ("read_no", "read numbering in EvalExpression"),
        (
            "all",
            "it may be confused with the segment-identifier 'all'",
        ),
    ] {
        if tag_name == *forbidden {
            // because that's what we store in the output tables as
            // column 0
            bail!(
                "Reserved value '{forbidden}' cannot be used here. This name is reserved for {reason}. Please choose a different value."
            );
        }
    }
    for (reserved_prefix, reason) in &[
        ("len_", "virtual tags computing the length of tags/segments"),
        (
            "location_",
            "virtual tags deriving the location of other tags",
        ),
    ] {
        if tag_name.starts_with(reserved_prefix) {
            bail!(
                "Cannot start with reserved prefix '{reserved_prefix}'. This prefix is reserved for {reason}. Please choose a different value that doesn't start with '{reserved_prefix}'."
            );
        }
    }
    Ok(())
}
impl TryFrom<&str> for TagLabel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match validate_tag_name(value) {
            Ok(()) => Ok(TagLabel::Normal(value.to_string())),
            Err(e) => Err(e.to_string()),
        }
    }
}

toml_pretty_deser::impl_visitor_for_try_from_str!(TagLabel, "Invalid label");
toml_pretty_deser::impl_visitor_for_try_from_str!(
    ConditionalTagLabel,
    "Invalid (conditional) label"
);

#[derive(Debug, Clone)]
pub struct NonAmbigousDNA(pub BString);

impl TryFrom<&str> for NonAmbigousDNA {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("DNA sequence cannot be empty".to_string());
        }
        let s = value.to_uppercase();
        for c in s.chars() {
            if !matches!(c, 'A' | 'C' | 'G' | 'T') {
                return Err(format!("Invalid DNA base: {c}"));
            }
        }
        Ok(NonAmbigousDNA(s.as_bytes().into()))
    }
}

toml_pretty_deser::impl_visitor_for_try_from_str!(NonAmbigousDNA, "Invalid DNA sequence");

// Custom scalar types (hand-written visitors) carry no `#[tpd(alias)]`s; give
// them no-op alias-tree impls so `TpdAliasTree` recursion can pass through them.

#[derive(Debug, Clone, Hash, Eq, PartialEq, serde::Serialize, PartialOrd, Ord)]
pub struct SegmentLabel(pub String);

impl TryFrom<&str> for SegmentLabel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match validate_segment_label(value, toml_pretty_deser::FieldMatchMode::AnyCase) {
            Ok(()) => Ok(SegmentLabel(value.to_string())),
            Err(e) => Err(e.to_string()),
        }
    }
}

toml_pretty_deser::impl_visitor_for_try_from_str!(SegmentLabel, "Invalid segment label");
/// Validates that a segment label conforms to the pattern [a-zA-Z0-9_]+
/// (one or more alphanumeric characters or underscores)
///
/// # Errors
///
/// On invalid segment labels
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
        "use_rapidgzip",
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

#[cfg(test)]
mod test {

    use crate::validate_segment_label;

    use super::validate_tag_name;
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
        validate_tag_name("location_shu").unwrap_err();
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
        validate_segment_label("use_rapidgzip", f).unwrap_err();
        validate_segment_label("threads_per_segment", f).unwrap_err();
        validate_segment_label("tpd_field_match_mode", f).unwrap_err();

        let f = toml_pretty_deser::FieldMatchMode::AnyCase;
        validate_segment_label("FaSTA___FAKE-QUALITY", f).unwrap_err();
    }
}
