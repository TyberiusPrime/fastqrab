use anyhow::Result;
use indexmap::IndexMap;
use schemars::JsonSchema;
use std::collections::{BTreeMap, HashSet};
use toml_pretty_deser::{Visitor, prelude::*};

use fastqrab_config::{SegmentLabel, StringOrVecString, default_comment_insert_char};
use fastqrab_io::{
    STDIN_MAGIC_PATH,
    io::input::{InputOptions, PartialInputOptions},
};

fn is_default(opt: &InputOptions) -> bool {
    opt.fasta_fake_quality.is_none()
        && opt.bam_include_mapped.is_none()
        && opt.bam_include_unmapped.is_none()
        && opt.read_comment_character == default_comment_insert_char()
}

/// Input configuration
#[derive(serde::Serialize)]
#[tpd]
#[derive(Debug, Clone, JsonSchema)]
pub struct Input {
    /// whether you have input files with interleaved reads, or one file per segment
    /// If interleaved, define the name of the segments here.
    #[tpd(default)]
    pub interleaved: Option<Vec<String>>,

    /// Your segments. Define just one with any name for interlaveed input.
    #[schemars(with = "BTreeMap<String, StringOrVecString>")]
    #[tpd(absorb_remaining)]
    #[serde(flatten)]
    pub segments: IndexMap<SegmentLabel, Vec<String>>,

    #[tpd(nested)]
    #[serde(skip_serializing_if = "is_default")]
    #[schemars(default)]
    pub options: InputOptions,

    #[tpd(skip)]
    #[schemars(skip)]
    #[serde(skip_serializing)]
    pub structured: StructuredInput,
}

impl PartialInput {
    fn verify_same_number_of_input_segments(&mut self) {
        //first me make sure all segments have the same number of files
        if let Some(segments) = self.segments.as_ref() {
            let no_of_file_per_segment: BTreeMap<_, _> = segments
                .map
                .iter()
                .map(|(k, v)| (k, v.value.as_ref().expect("Parent was ok?").len()))
                .collect();
            let observed_no_of_segments: HashSet<_> = no_of_file_per_segment.values().collect();
            if observed_no_of_segments.len() > 1 {
                let spans: Vec<(std::ops::Range<usize>, String)> = segments
                    .map
                    .iter()
                    .map(|(_k, v)| {
                        (
                            v.span.clone(),
                            format!(
                                "{} segment(s)",
                                v.value.as_ref().expect("parent was ok?").len()
                            ),
                        )
                    })
                    .collect();
                self.segments.state = TomlValueState::Custom { spans };
                self.segments.help =
                    Some("Each segment must have the same number of files.".to_string());
            }
        }
    }

    fn validate_stdin_usage(&mut self) -> Result<(), ()> {
        // let Some(structured) = self.structured.as_ref() else {
        //     return Ok(());
        // };
        match &self.structured {
            Some(StructuredInput::Interleaved { files, .. }) => {
                if files.iter().any(|f| f == STDIN_MAGIC_PATH) {
                    if files.len() != 1 {
                        self.interleaved.state = TomlValueState::ValidationFailed {
                            message: "Invalid use of stdin magic value".to_string(),
                        };
                        self.interleaved.help = Some(format!(
                            "When using '{STDIN_MAGIC_PATH}' as an input file, it must be the only file listed in the interleaved segment's input. Found {} times.",
                            files.len()
                        ));
                    }
                    return Ok(());
                }
            }
            Some(StructuredInput::Segmented {
                segment_files,
                segment_order,
            }) => {
                let segments_with_stdin: Vec<_> = segment_order
                    .iter()
                    .filter(|segment| {
                        segment_files
                            .get(*segment)
                            .is_some_and(|files| files.iter().any(|name| name == STDIN_MAGIC_PATH))
                    })
                    .collect();
                if segments_with_stdin.is_empty() {
                    return Ok(());
                }
                if segments_with_stdin.len() > 1 {
                    let spans: Vec<_> = self
                        .segments
                        .as_ref()
                        .expect("segments must exist")
                        .keys
                        .iter()
                        .filter(|toml_value| {
                            segments_with_stdin
                                .contains(&toml_value.as_ref().expect("parent was ok"))
                        })
                        .map(|x| {
                            (
                                x.span().clone(),
                                "Invalid use of {STDIN_MAGIC_PATH}".to_string(),
                            )
                        })
                        .collect();
                    self.segments.state = TomlValueState::Custom { spans };
                    self.segments.help = Some(format!(
                        "When using '{STDIN_MAGIC_PATH}' as an input file, it must be the only file listed in exactly one segment. Found in segments: {segments_with_stdin:?}"
                    ));
                    return Err(());
                }
                if segment_order.len() != 1 {
                    self.segments.state = TomlValueState::ValidationFailed {
                        message: "Invalid use of stdin magic value".to_string(),
                    };
                    self.segments.help = Some(format!(
                        "Using '{STDIN_MAGIC_PATH}' requires exactly one segment (and possibly interleaved)."
                    ));
                }
                let segment = segments_with_stdin[0];
                let files = segment_files.get(segment).expect("segment must exist");
                if files.len() != 1 {
                    self.segments.state = TomlValueState::ValidationFailed {
                        message: "Invalid use of stdin magic value".to_string(),
                    };
                    self.segments.help = Some(format!(
                        "'{STDIN_MAGIC_PATH}' requires exactly one input file."
                    ));
                }
                // if files[0] != STDIN_MAGIC_PATH {
                //     self.segments.state = TomlValueState::ValidationFailed {
                //         message: "Invalid use of stdin magic value".to_string(),
                //     };
                //     self.segments.help = Some(format!(
                //         "When using '{STDIN_MAGIC_PATH}' as an input file, it must be the only file listed in the segment. Found additional files: {files:?}"
                //     ));
                // }
            }
            None => {
                //no segments, no further checking :)
            }
        }
        Ok(())
    }
    fn build_interleaved_structured(&mut self) -> Result<(), ()> {
        let Some(Some(interleaved)) = self.interleaved.as_mut() else {
            // cov:excl-start
            unreachable!();
            // cov:excl-stop
        };
        let Some(segments) = self.segments.as_ref() else {
            // cov:excl-start
            unreachable!();
            // cov:excl-stop
        };
        if segments.map.len() != 1 {
            let mut spans: Vec<_> = segments
                .map
                .iter()
                .map(|(_k, v)| (v.span.clone(), "More than one segment defined".to_string()))
                .collect();
            spans.push((
                self.interleaved.span.clone(),
                "Interleaved segment definition".to_string(),
            ));
            self.interleaved.state = TomlValueState::Custom { spans };
            self.interleaved.help = Some(
                "Interleaved input can only have exactly one other key defining the segments."
                    .to_string(),
            );
            return Err(());
        }
        if interleaved.len() < 2 {
            self.interleaved.state = TomlValueState::ValidationFailed {
                message: "Must define at least two segments".to_string(),
            };
            self.interleaved.help = Some(
                "If you have single end reads, remove interleaved.
If you have paired end reads, name two 'virtual' segments, e.g. ['read1','read2']"
                    .to_string(),
            );
            return Err(());
        }
        //detect duplicate names in interleaved
        let mut seen: IndexMap<String, Vec<std::ops::Range<usize>>> = IndexMap::new();
        for segment_toml_value in interleaved.iter() {
            let segment_name = segment_toml_value.as_ref().expect("parent was ok").trim();
            match seen.entry(segment_name.to_string()) {
                indexmap::map::Entry::Vacant(e) => {
                    e.insert(vec![segment_toml_value.span.clone()]);
                }
                indexmap::map::Entry::Occupied(mut e) => {
                    e.get_mut().push(segment_toml_value.span.clone());
                }
            }
        }
        let mut reported = HashSet::new();
        for segment_toml_value in interleaved.iter_mut() {
            let segment_name = segment_toml_value.as_ref().expect("parent was ok").clone();
            if reported.insert(segment_name.clone()) {
                let spans = seen.get(&segment_name).expect("We just built this map");
                if spans.len() > 1 {
                    segment_toml_value.state = TomlValueState::Custom {
                        spans: spans
                            .iter()
                            .map(|span| (span.clone(), "Duplicate value".to_string()))
                            .collect(),
                    };
                    segment_toml_value.help = Some(
                        "Use each segment only once in interleaved. If you really want to use the same reads twice, define multiple segments, set input.accept_duplicate_files = true.".to_string()
                    );
                }
            }
        }
        if !interleaved.can_concrete() {
            self.interleaved.state = TomlValueState::Nested;
            return Err(());
        }

        let files: Vec<String> = segments
            .map
            .values()
            .next()
            .expect("We ensured there was at least one segment")
            .as_ref()
            .expect("parent was ok")
            .iter()
            .map(|tv| tv.as_ref().expect("parent was ok?").clone())
            .collect();

        self.structured = Some(StructuredInput::Interleaved {
            files,
            segment_order: interleaved
                .iter()
                .map(|x| x.as_ref().expect("parent was ok").trim().to_string())
                .collect(),
        });
        Ok(())
    }

    fn build_segmented_structured(&mut self) -> Result<(), ()> {
        let Some(segments) = self.segments.as_mut() else {
            return Ok(());
        };
        let mut segment_order: Vec<String> = segments.map.keys().map(|x| x.0.clone()).collect();
        segment_order.sort(); //always alphabetical...
        if segment_order.is_empty() {
            self.segments.state = TomlValueState::ValidationFailed {
                message: "No segments defined in input.".to_string(),
            };
            self.segments.help = Some(
                "At least one segment must be defined. Example: read1 = ['filename.fq']"
                    .to_string(),
            );
            return Err(());
        }
        if let Some(all_segment) = segments.keys.iter_mut().find(|tv| {
            tv.as_ref()
                .expect("Parent was ok")
                .eq_ignore_ascii_case("all")
        }) {
            all_segment.state = TomlValueState::ValidationFailed {
                message: "Reserved segment name".to_string(),
            };
            all_segment.help = Some(
                "Segment name 'all' is reserved and cannot be used as a segment name.".to_string(),
            );
            self.segments.state = TomlValueState::Nested;
            return Err(());
        }
        if let Some(all_segment) = segments.keys.iter_mut().find(|tv| {
            tv.as_ref()
                .expect("Parent was ok")
                .to_ascii_lowercase()
                .starts_with("_internal_")
        }) {
            all_segment.state = TomlValueState::ValidationFailed {
                message: "Reserved segment name".to_string(),
            };
            all_segment.help = Some(
                "Segment names starting with '_internal_' are reserved and cannot be used as a segment name. Choose something else."
                    .to_string(),
            );
            self.segments.state = TomlValueState::Nested;
            return Err(());
        }

        assert!(
            !segment_order
                .iter()
                .any(|x| x.eq_ignore_ascii_case("options")),
            "Options should have been filtered by toml-pretty-deser"
        );
        let segment_files: IndexMap<String, Vec<String>> = segments
            .map
            .iter()
            .map(|(k, v)| {
                let files = v
                    .as_ref()
                    .expect("Parent was ok")
                    .iter()
                    .map(|tv| tv.as_ref().expect("parent was ok?").clone())
                    .collect();
                (k.0.clone(), files)
            })
            .collect();
        self.structured = Some(StructuredInput::Segmented {
            segment_files,
            segment_order,
        });
        Ok(())
    }

    fn build_structured(&mut self) -> Result<(), ()> {
        if self
            .interleaved
            .as_ref()
            .is_some_and(std::option::Option::is_some)
            && self.segments.as_ref().is_some()
        {
            self.build_interleaved_structured()?;
        } else {
            self.build_segmented_structured()?;
        }

        self.validate_stdin_usage()?;

        Ok(())
    }
}

impl VerifyIn<super::PartialConfig> for PartialInput {
    fn verify(
        &mut self,
        _parent: &super::PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.options.or_with(|| {
            //this could be prettier...
            let default = InputOptions::default();
            PartialInputOptions {
                fasta_fake_quality: TomlValue::new_ok(default.fasta_fake_quality, 0..0),
                bam_include_mapped: TomlValue::new_ok(default.bam_include_mapped, 0..0),
                bam_include_unmapped: TomlValue::new_ok(default.bam_include_unmapped, 0..0),
                read_comment_character: TomlValue::new_ok(default.read_comment_character, 0..0),
                threads_per_segment: TomlValue::new_ok(default.threads_per_segment, 0..0),
            }
        });

        if let Some(segments) = self.segments.as_ref()
            && segments.keys.len() > 256
        {
            return Err(ValidationFailure::new(
                "Too many segments defined",
                Some("The maximum number of segments is 256."),
            ));
        }

        self.verify_same_number_of_input_segments();

        if let Err(()) = self.build_structured()
        //errors go into the fields
        {}

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum StructuredInput {
    Interleaved {
        files: Vec<String>,
        segment_order: Vec<String>,
    },
    Segmented {
        segment_files: IndexMap<String, Vec<String>>,
        segment_order: Vec<String>,
    },
}

impl StructuredInput {
    #[must_use]
    pub fn is_interleaved(&self) -> bool {
        matches!(self, StructuredInput::Interleaved { .. })
    }

    #[must_use]
    pub fn any_input_is_stdin(&self) -> bool {
        use fastqrab_io::STDIN_MAGIC_PATH;
        match self {
            crate::config::StructuredInput::Interleaved { files, .. } => {
                files.iter().any(|f| f == STDIN_MAGIC_PATH)
            }
            crate::config::StructuredInput::Segmented { segment_files, .. } => segment_files
                .values()
                .flatten()
                .any(|f| f == STDIN_MAGIC_PATH),
        }
    }
}

impl Input {
    #[must_use]
    #[mutants::skip] // only used to figure out thread count.
    pub fn parser_count(&self) -> usize {
        match &self.structured {
            StructuredInput::Interleaved { .. } => 1,
            StructuredInput::Segmented { segment_order, .. } => segment_order.len(),
        }
    }

    #[must_use]
    pub fn get_segment_order(&self) -> &Vec<String> {
        match &self.structured {
            StructuredInput::Interleaved { segment_order, .. }
            | StructuredInput::Segmented { segment_order, .. } => segment_order,
        }
    }
}

impl PartialInput {
    #[must_use]
    /// # Panics
    /// if config parsing did not set structured input
    pub fn segment_count(&self) -> usize {
        match self
            .structured
            .as_ref()
            .expect("structured input must be set after config parsing")
        {
            StructuredInput::Interleaved { segment_order, .. }
            | StructuredInput::Segmented { segment_order, .. } => segment_order.len(),
        }
    }

    /// # Panics
    /// if config parsing did not set structured input
    #[must_use]
    pub fn get_segment_order(&self) -> &Vec<String> {
        match self
            .structured
            .as_ref()
            .expect("structured input must be set after config parsing")
        {
            StructuredInput::Interleaved { segment_order, .. }
            | StructuredInput::Segmented { segment_order, .. } => segment_order,
        }
    }
}
