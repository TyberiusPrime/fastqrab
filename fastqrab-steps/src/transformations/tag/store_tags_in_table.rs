use std::cell::RefCell;
use std::rc::Rc;

use crate::transformations::output::validate_compression_level_u8;
use crate::transformations::prelude::*;
use crate::verify_opt_path_component;

use fastqrab_config::{
    default_include_read_comment, default_include_read_name, default_region_separator,
    tpd_adapt_bstring,
};
use fastqrab_io::CompressionFormat;

type OutputHandles = Arc<Mutex<DemultiplexedData<Option<ChunkedRecordWriter>>>>;
type InLabels = Vec<TagLabel>;

/// Store all currently defined tags in a TSV
#[derive(JsonSchema, Clone)]
#[tpd]
#[derive(Debug)]
pub struct StoreTagsInTable {
    #[tpd(default)]
    pub infix: Option<String>, // pub for verification inspection
    #[tpd(default)]
    #[expect(dead_code, reason = "only used in verification")]
    compression: CompressionFormat,
    #[tpd(default)]
    #[expect(dead_code, reason = "only used in verification")]
    compression_level: Option<u8>,

    #[schemars(with = "String")]
    #[tpd(with = "tpd_adapt_bstring")]
    region_separator: BString,

    #[schemars(skip)]
    #[tpd(skip, default)]
    output_handles: Option<OutputHandles>,

    include_read_name: bool,
    include_read_comment: bool,

    #[expect(dead_code, reason = "only used in deser")]
    #[tpd(alias = "tags")]
    #[tpd(adapt_in_verify(String))]
    in_labels: Option<Vec<TagLabel>>,

    #[tpd(skip)]
    #[schemars(skip)]
    final_in_labels: InLabels,
}

impl PartialStoreTagsInTable {}

impl VerifyIn<PartialConfig> for PartialStoreTagsInTable {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.region_separator.or_with(default_region_separator);
        self.include_read_name.or_with(default_include_read_name);
        self.include_read_comment
            .or_with(default_include_read_comment);
        self.infix.verify(verify_opt_path_component);

        validate_compression_level_u8(&self.compression, &mut self.compression_level);

        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialStoreTagsInTable> {
    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        if let Some(inner) = self.toml_value.as_ref() {
            let infix = inner
                .infix
                .as_ref()
                .and_then(|x| x.as_ref())
                .map_or_else(String::new, ToString::to_string);
            let compression = inner.compression.as_ref().copied().unwrap_or_default();
            let suffix = compression.apply_suffix("tsv");
            Some(vec![OutputDeclaration {
                id: "tsv".to_string(),
                target: WriteTargetConfig::new(vec![infix], None, suffix),
                sink_config: SinkConfig {
                    compression,
                    compression_level: inner
                        .compression_level
                        .as_ref()
                        .and_then(|x| x.as_ref())
                        .copied(),
                    hash_uncompressed: false,
                    hash_compressed: false,
                    simulated_failure: None,
                },
                format: fastqrab_io::FileFormat::Text,
                chunk_policy: ChunkPolicy::default(),
                bam_options: None,
                singleton: false,
                span: inner.infix.span(),
            }])
        } else {
            Some(vec![]) //there should be output files, but we can't name them.
        }
    }

    fn get_tag_usage(
        &mut self,
        tags_available: &IndexMap<TagLabel, TagMetadata>,
        segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            #[expect(clippy::single_match_else, reason = "clearer")]
            match inner.in_labels.value.as_mut() {
                Some(Some(in_labels)) => {
                    //they're not ok yet...
                    for tag in in_labels.iter_mut() {
                        tag.validate_incoming_tag_label(tags_available, segment_order);
                    }
                    if in_labels.is_empty() {
                        inner.in_labels.state = TomlValueState::ValidationFailed {
                            message: "May not be an empty list".to_string(),
                        };
                        inner.in_labels.help = Some("Set to a non-empty list of tag labels. Or leave off to store all (regular) tags.".to_string());
                        inner.final_in_labels = Some(Vec::new());
                        return None;
                    }
                    inner.final_in_labels = Some(
                        in_labels
                            .iter()
                            .filter_map(|tag| tag.as_ref())
                            .filter_map(|v| v.as_ref_post())
                            .cloned()
                            .collect(),
                    );

                    // Each tag keeps its own toml-value state/span here, so a
                    // "no such tag" error (set further up the pipeline once
                    // tags_available is known) points at that tag's own
                    // location in the list, not at the whole step.
                    let used_tags = in_labels
                        .iter_mut()
                        .map(|tag| tag.to_used_tag(ANY_TAG_TYPE))
                        .collect();

                    Some(TagUsageInfo {
                        used_tags,
                        ..Default::default()
                    })
                }
                Some(None) | None => {
                    if tags_available.is_empty() {
                        self.toml_value.state = TomlValueState::ValidationFailed {
                    message: "StoreTagsInTable needs at least one tag to be set before it in the transformation chain.".to_string(),
                    };
                        inner.final_in_labels = Some(Vec::new());
                        return None;
                    }
                    let mut final_in_labels: Vec<_> = tags_available.keys().cloned().collect();
                    final_in_labels.sort_unstable();
                    inner.final_in_labels = Some(final_in_labels.clone());

                    // No explicit tag list was given: every currently
                    // available tag is used, so there's no per-tag span to
                    // point at on failure. These tags always exist (they came
                    // straight from tags_available), so the step-level span
                    // is only ever a fallback, never actually shown.
                    let toml_source = Rc::new(RefCell::new((
                        &mut inner.in_labels.state,
                        &mut inner.in_labels.help,
                    )));
                    let used_tags = final_in_labels
                        .into_iter()
                        .map(|tag| {
                            Some(UsedTag {
                                name: tag,
                                accepted_tag_types: ANY_TAG_TYPE,
                                toml_source: toml_source.clone(),
                                further_help: None,
                            })
                        })
                        .collect();

                    Some(TagUsageInfo {
                        must_see_all_tags: true,
                        used_tags,
                        ..Default::default()
                    })
                }
            }
        } else {
            None // cov:excl-line
        }
    }
}

fn format_tsv_row(fields: &[Cow<BStr>]) -> Vec<u8> {
    let mut row = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        if i > 0 {
            row.push(b'\t');
        }
        row.extend_from_slice(field);
    }
    row.push(b'\n');
    row
}

impl Step for StoreTagsInTable {
    fn init(
        &mut self,
        _input_info: &InputInfo,
        mut output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        _input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        let per_tag = output_files.take("tsv");

        // Build header bytes and call set_header on each writer
        let tag_list = &self.final_in_labels;
        let mut header_fields: Vec<Cow<BStr>> = Vec::new();
        if self.include_read_name {
            header_fields.push(Cow::Borrowed(b"ReadName".into()));
        }
        if self.include_read_comment {
            header_fields.push(Cow::Borrowed(b"ReadComment".into()));
        }
        for tag in tag_list {
            header_fields.push(Cow::Borrowed(BStr::new(tag.as_ref())));
        }
        let header_bytes = format_tsv_row(&header_fields);

        let mut handles = DemultiplexedData::new();
        for (tag, mut writer) in per_tag {
            writer.set_header(header_bytes.clone())?;
            handles.insert(tag, Some(writer));
        }
        self.output_handles = Some(Arc::new(Mutex::new(handles)));

        Ok(None)
    }

    // needed to ensure output order
    fn needs_serial(&self) -> bool {
        true
    }

    fn transmits_premature_termination(&self) -> bool {
        false // since we want to dump all the reads even if later on there's a Head
    }

    fn apply(
        &self,
        block: FastQBlocksCombined,
        input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let output_tags = block.output_tags.as_ref();
        let mut ii = 0;
        let mut output_handles = self
            .output_handles
            .as_ref()
            .expect("was set in init?")
            .lock()
            .expect("lock poisoned");
        for name in &block.segments[0].names {
            let output_tag = output_tags.map_or(0, |x| x[ii]);
            if let Some(Some(writer)) = output_handles.get_mut(&output_tag) {
                let mut record: Vec<Cow<BStr>> = Vec::new();
                if self.include_read_name | self.include_read_comment {
                    let (name, comment) =
                        split_name_and_comment(name, input_info.comment_insert_char);
                    if self.include_read_name {
                        record.push(Cow::Borrowed(name));
                    }
                    if self.include_read_comment {
                        record.push(Cow::Borrowed(comment));
                    }
                }
                for tag in &self.final_in_labels {
                    let col = block.tags.get(tag).expect("tag must exist in block.tags");
                    record.push(col.to_bstr(
                        ii,
                        |float| float.to_string(),
                        Some(self.region_separator.as_ref()),
                    ));
                }
                ii += 1;
                let row = format_tsv_row(&record);
                writer.write_text_record(&row)?;
            } // cov:excl-line
        }

        Ok((block, true))
    }

    fn finalize(&self, _demultiplex_info: &OptDemultiplex) -> Result<Option<FinalizeReportResult>> {
        for (_tag, writer) in self
            .output_handles
            .as_ref()
            .expect("was set in init")
            .lock()
            .expect("lock poisoned")
            .iter_mut()
        {
            let () = writer.take().expect("Should have had writer?!").finish()?;
        }
        Ok(None)
    }
}
