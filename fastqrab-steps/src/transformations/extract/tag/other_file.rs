use std::cell::Cell;
use std::collections::HashSet;

use super::super::extract_bool_tags;
use super::ApproxOrExactFilter;
use crate::transformations::extract::extract_bool_tags_from_tag;
use crate::transformations::tag::initial_filter_elements;
use crate::transformations::{prelude::*, read_name_canonical_prefix};
use fastqrab_config::{OptionalByte, tpd_adapt_u8_from_byte_char_or_false};
use fastqrab_io::io::{apply_to_read_names, apply_to_read_sequences};

/// `StepInputFiles` id linking `declare_input_files` to the handle taken in
/// `init` for the file we test reads against.
const FILENAME_ID: &str = "filename";

/// Tag whether reads are in another file (by sequence)
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct OtherFile {
    pub filename: String,

    #[schemars(with = "String")]
    #[tpd(adapt_in_verify(String), alias = "segment")]
    source: ResolvedSourceNoAll,

    // Accepts a single character / byte to trim read names at, or `false` to
    // match the whole read name. Required for a name source (see `verify`); a
    // plain `Option<u8>` default would silently match whole names -- a footgun.
    #[schemars(with = "Option<u8>")]
    #[tpd(
        with = "tpd_adapt_u8_from_byte_char_or_false",
        alias = "other_read_name_end_char"
    )]
    pub other_readname_end_character: OptionalByte,

    pub out_label: TagLabel,

    pub seed: Option<u64>,
    pub false_positive_rate: f64,

    pub include_mapped: bool,
    pub include_unmapped: bool,

    #[tpd(skip, default)]
    #[schemars(skip)]
    pub filter: Option<Arc<ApproxOrExactFilter>>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    pub progress_output: Option<crate::transformations::reports::Progress>,
}

impl VerifyIn<PartialConfig> for PartialOtherFile {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.source.validate_segment(parent);

        crate::transformations::tag::validate_seed(&mut self.seed, &mut self.false_positive_rate);
        if let Some(filename) = self.filename.as_ref()
            && Path::new(filename)
                .extension()
                .is_some_and(|x| x.eq_ignore_ascii_case("bam") || x.eq_ignore_ascii_case("sam"))
        {
            if self.include_unmapped.is_missing() {
                self.include_unmapped.or(false); // just so it's always set, but still report the
                // error.
                return Err(ValidationFailure::new(
                    "Missing include_unmapped",
                    Some("When using a BAM file, you must specify `include_unmapped` = true|false"),
                ));
            }

            if self.include_mapped.is_missing() {
                self.include_mapped.or(false); // just so it's always set, but still report the
                // error.
                return Err(ValidationFailure::new(
                    "Missing include_mapped",
                    Some("When using a BAM file, you must specify `include_mapped` = true|false"),
                ));
            }
            if !(self.include_mapped.value.unwrap_or(false)
                || self.include_unmapped.value.unwrap_or(false))
            {
                return Err(ValidationFailure::new(
                    "Invalid include_mapped/include_unmapped combination",
                    Some(
                        "At least one of `include_mapped` or `include_unmapped` must be true when using a BAM/SAM file.",
                    ),
                ));
            }
        } else {
            self.include_mapped.or(false); // just so it's always set.
            self.include_unmapped.or(false);
        }

        // `other_read_name_end_char` is tied to a name source: it is *mandatory*
        // there (leaving it out would silently match the *whole* read name,
        // comment included, against the other file -- almost never intended) and
        // *forbidden* otherwise (it trims read names, which is meaningless for a
        // sequence/tag source). The user must choose a character to trim at, or
        // `false` to opt into whole-name matching. This runs after the
        // include_mapped/unmapped defaulting above so a missing end char does
        // not cascade into spurious "missing include_*" errors.
        match self.source.as_ref().and_then(|x| x.as_ref_post()) {
            Some(ResolvedSourceNoAll::Name { .. }) => {
                if self.other_readname_end_character.is_missing() {
                    self.other_readname_end_character.or(OptionalByte(None)); // set it so it still concretizes
                    return Err(ValidationFailure::new(
                        "Missing other_read_name_end_char",
                        Some(
                            "When the source is a read name (`source = \"name:...\"`), set `other_read_name_end_char` to the character that ends the read name in the other file (e.g. ' '), or to `false` to match the whole read name.",
                        ),
                    ));
                }
            }
            Some(_) => {
                // Sequence or tag source: reject the end char rather than
                // silently ignoring it.
                if !self.other_readname_end_character.is_missing() {
                    return Err(ValidationFailure::new(
                        "other_read_name_end_char is only valid for a name source",
                        Some(
                            "`other_read_name_end_char` trims read names and only applies when `source = \"name:...\"`. Remove it, or change the source to a read name.",
                        ),
                    ));
                }
                self.other_readname_end_character.or(OptionalByte(None));
            }
            None => {
                // Source did not resolve; it reports its own error. Just default
                // this so the field concretizes and we don't pile on.
                self.other_readname_end_character.or(OptionalByte(None));
            }
        }

        self.source.verify(|v| {
            if let Some(output_tag) = self.out_label.as_ref()
                && let MustAdapt::PostVerify(ResolvedSourceNoAll::Tag(input_tag)) = v
                && output_tag == input_tag
            {
                return Err(ValidationFailure::new(
                    "Source cannot be the same as output tag",
                    Some("The source (segment or tag) cannot be the same as the output tag"),
                ));
            }
            Ok(())
        });
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialOtherFile> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            let used_tags = inner.source.to_used_tags();
            Some(TagUsageInfo {
                declared_tag: inner.out_label.to_declared_tag(TagValueType::Bool),
                used_tags,
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
    fn verify_others(
        &mut self,
        input_def: Option<&crate::config::PartialInput>,
        _output_def: Option<&crate::config::PartialOutput>,
        transformations_before_this_one: &[TomlValue<PartialTransformation>],
    ) {
        let inner = self
            .toml_value
            .as_mut()
            .expect("get_tag_usage should only be called after successful verification");
        if let Some(ResolvedSourceNoAll::Name { .. }) =
            &inner.source.as_ref().and_then(|x| x.as_ref_post())
            && let Some(input_def) = input_def.as_ref()
        {
            //if there's a StoreTagInComment before us
            //and our fastq_readname_end_char is != their comment_insert_char
            for trafo in transformations_before_this_one.iter().rev() {
                if let Some(PartialTransformation::StoreTagInComment(info)) = trafo.value.as_ref()
                    && let Some(info) = info.toml_value.value.as_ref()
                    && let Some(info_comment_char) = info.comment_separator.as_ref()
                    && let Some(read_comment_character) = input_def
                        .options
                        .as_ref()
                        .and_then(|x| x.read_comment_character.as_ref())
                    && *info_comment_char != *read_comment_character
                {
                    let spans = vec![
                        (
                            info.comment_separator.span(),
                            "Must match to options.read_comment_character".to_string(),
                        ),
                        (
                            input_def
                                .options
                                .as_ref()
                                .map_or(0..0, |x| x.read_comment_character.span()),
                            "Must match with StoreTagInComment step's comment_separator"
                                .to_string(),
                        ),
                    ];
                    self.toml_value.state = TomlValueState::Custom { spans };
                    self.toml_value.help = Some("Adjust them to be identical.".to_string());
                    return;
                }
            }
        }
    }

    fn declare_input_files(&self) -> Option<Vec<InputDeclaration>> {
        let inner = self.toml_value.as_ref()?;
        let filename = inner.filename.as_ref()?;
        Some(vec![InputDeclaration {
            id: FILENAME_ID.to_string(),
            path: filename.clone().into(),
        }])
    }
}

impl Step for OtherFile {
    fn store_progress_output(&mut self, progress: &crate::transformations::reports::Progress) {
        self.progress_output = Some(progress.clone());
    }

    fn init(
        &mut self,
        _input_info: &InputInfo,
        _output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        // The runtime opened the file we declared in `declare_input_files`; read
        // through that handle rather than re-opening `self.filename` ourselves.
        let file = input_files.take(FILENAME_ID);
        let mut filter: ApproxOrExactFilter = if self.false_positive_rate == 0.0 {
            ApproxOrExactFilter::Exact(HashSet::new())
        } else {
            let seed = self
                .seed
                .expect("seed should be validated to exist when false_positive_rate > 0.0");
            ApproxOrExactFilter::Approximate(Box::new(reproducible_cuckoofilter(
                seed,
                initial_filter_elements(&self.filename, self.include_mapped, self.include_unmapped),
                self.false_positive_rate,
            )))
        };
        // read them all.
        if let Some(pg) = self.progress_output.as_mut() {
            pg.output(&format!("Reading all reads from {}", self.filename))?;
        }
        let count: Cell<usize> = Cell::new(0);

        match self.source {
            ResolvedSourceNoAll::Segment(_) | ResolvedSourceNoAll::Tag(_) => {
                apply_to_read_sequences(
                    file,
                    &mut |read_seq| {
                        if !filter.contains(&FragmentEntry(&[read_seq])) {
                            filter.insert(&FragmentEntry(&[read_seq]));
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    },
                    self.include_mapped,
                    self.include_unmapped,
                )?; // cov:excl-line
            }
            ResolvedSourceNoAll::Name { .. } => {
                apply_to_read_names(
                    file,
                    &mut |read_name| {
                        let trimmed = read_name_canonical_prefix(
                            read_name,
                            self.other_readname_end_character.0,
                        );

                        if !filter.contains(&FragmentEntry(&[trimmed])) {
                            filter.insert(&FragmentEntry(&[trimmed]));
                        }
                        count.set(count.get() + 1);
                        Ok(())
                    },
                    self.include_mapped,
                    self.include_unmapped,
                )?; // cov:excl-line
            }
        }

        if count.get() == 0 {
            bail!(
                "No reads were read from {}. Check that the file is not empty and (for BAM) that the include_mapped/include_unmapped options are set correctly.",
                self.filename
            );
        }
        if let Some(pg) = self.progress_output.as_mut() {
            pg.output(&format!(
                "Finished reading all ({}) read sequences from {}",
                count.get(),
                self.filename
            ))?;
        }

        self.filter = Some(Arc::new(filter));
        Ok(None)
    }

    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        match &self.source {
            ResolvedSourceNoAll::Segment(segment_index) => {
                extract_bool_tags(
                    &mut block,
                    *segment_index,
                    &self.out_label,
                    |read, _ignored_demultiplex_tag| {
                        let filter = self
                            .filter
                            .as_ref()
                            .expect("filter must be set during initialization");
                        let query = read.seq;
                        filter.contains(&FragmentEntry(&[query]))
                    },
                );
            }
            ResolvedSourceNoAll::Tag(tag) => {
                extract_bool_tags_from_tag(
                    &mut block,
                    &self.out_label,
                    tag,
                    |query, _ignored_demultiplex_tag| {
                        let filter = self
                            .filter
                            .as_ref()
                            .expect("filter must be set during initialization");
                        if let Some(query) = query
                            && !query.is_empty()
                        {
                            filter.contains(&FragmentEntry(&[&query]))
                        } else {
                            false
                        }
                    },
                );
            }
            ResolvedSourceNoAll::Name {
                segment_index,
                split_character,
            } => {
                extract_bool_tags(
                    &mut block,
                    *segment_index,
                    &self.out_label,
                    |read, _ignored_demultiplex_tag| {
                        let query = read_name_canonical_prefix(read.name, Some(*split_character));

                        self.filter
                            .as_ref()
                            .expect("filter must be set during initialization")
                            .contains(&FragmentEntry(&[query]))
                    },
                );
            }
        }

        Ok((block, true))
    }
}
