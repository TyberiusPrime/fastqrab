use super::{format_numeric_for_comment, store_tag_in_comment};
use crate::transformations::{output::validate_compression_level_u8, prelude::*};
use fastqrab_config::{
    default_comment_insert_char, default_comment_separator, default_region_separator,
    tpd_adapt_bstring, tpd_adapt_u8_from_byte_or_char,
};
use fastqrab_io::{CompressionFormat, FileFormat};

/// Store tag values into FASTQ files.
///
/// Files are named using the pattern: `{output_prefix}_{infix}.tag.fastq.{suffix}`
/// When demultiplexing: `{output_prefix}_{infix}_{barcode}.tag.fastq.{suffix}`
///
/// Optionally adds comment tags to read names before writing, similar to `StoreTagInComment`.
#[derive(JsonSchema, Clone)]
#[tpd]
#[derive(Debug)]
pub struct StoreTagInFastQ {
    #[tpd(adapt_in_verify(String))]
    in_label: TagLabel,

    // Optional read name comment fields (like StoreTagInComment)
    #[tpd(adapt_in_verify(String))]
    #[tpd(alias = "comment_labels")]
    comment_tags: Option<Vec<TagLabel>>,
    //
    #[tpd(with = "tpd_adapt_u8_from_byte_or_char")]
    comment_separator: u8,

    #[tpd(with = "tpd_adapt_u8_from_byte_or_char")]
    comment_insert_char: u8,

    #[tpd(with = "tpd_adapt_bstring")]
    #[schemars(with = "String")]
    region_separator: BString,

    // Optional format configuration (defaults to Raw)
    #[tpd(default)]
    format: FileFormat,
    #[tpd(default)]
    #[expect(dead_code, reason = "only used in verification")]
    compression: CompressionFormat,
    #[tpd(default)]
    #[expect(dead_code, reason = "only used in verification")]
    compression_level: Option<u8>,

    // Internal state for collecting reads during apply
    #[tpd(skip, default)]
    #[schemars(skip)]
    output_handles: Option<Arc<Mutex<DemultiplexedData<Option<ChunkedRecordWriter>>>>>,
}

impl VerifyIn<PartialConfig> for PartialStoreTagInFastQ {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.comment_separator.or_with(default_comment_separator);

        self.comment_insert_char
            .or_with(default_comment_insert_char);

        self.region_separator.or_with(default_region_separator);

        self.format.verify(|format| {
            if !matches!(format, FileFormat::Fastq | FileFormat::Fasta) {
                return Err(ValidationFailure::new(
                    "StoreTagInFastQ supports only 'fastq' or 'fasta' formats",
                    None,
                ));
            }
            Ok(())
        });

        validate_compression_level_u8(
            &self.compression,
            &mut self.compression_level,
            // purposes
        );

        if parent.output.is_ok()
            && let Some(None) = parent.output.value.as_ref()
        {
            return Err(ValidationFailure::new(
                "Missing output configuration",
                Some(
                    "StoreTagInFastQ requires output configuration to determine file paths and formats",
                ),
            ));
        }
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialStoreTagInFastQ> {
    fn get_tag_usage(
        &mut self,
        tags_available: &IndexMap<TagLabel, TagMetadata>,
        segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            inner
                .in_label
                .validate_incoming_tag_label(tags_available, segment_order);
            let in_label = inner
                .in_label
                .to_used_tag(&[TagValueType::Location, TagValueType::String]);

            let mut used_tags = vec![in_label];
            if let Some(Some(comment_tags)) = inner.comment_tags.value.as_mut() {
                for tag in comment_tags.iter_mut() {
                    tag.validate_incoming_tag_label(tags_available, segment_order);
                    if let Some(this_tag) = tag.as_ref().and_then(|x| x.as_ref_post())
                        && used_tags[0].as_ref().is_some_and(|x| x.name == *this_tag)
                    {
                        tag.state = TomlValueState::new_validation_failed(
                            "InLabel repeated in comment_tags",
                        );
                        tag.help = Some("The same tag cannot be used as both the in_label and a comment tag. Remove from comment_tags?".to_string());
                    } else {
                        used_tags.push(tag.to_used_tag(&[
                            TagValueType::Bool,
                            TagValueType::String,
                            TagValueType::Location,
                            TagValueType::Numeric((None, None)),
                        ]));
                    }
                }
            }

            Some(TagUsageInfo {
                used_tags,
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }

    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        if let Some(inner) = self.toml_value.as_ref() {
            let in_label = inner
                .in_label
                .as_ref()
                .and_then(|v| v.as_ref_post())
                .expect("declare_output_files called without successsful verification");

            let format = inner.format.as_ref().copied().unwrap_or_default();
            let compression = inner.compression.as_ref().copied().unwrap_or_default();
            Some(vec![OutputDeclaration {
                id: "tag_fastq".to_string(),
                target: WriteTargetConfig::new(
                    vec![format!("tag.{in_label}")],
                    None,
                    format.get_suffix(compression, None),
                ),
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
                format,
                chunk_policy: ChunkPolicy::default(),
                bam_options: None,
                singleton: false,
                span: inner.in_label.span(),
            }])
        } else {
            Some(vec![]) //there should be output files, but we can't name them.
        }
    }
}

impl Step for StoreTagInFastQ {
    fn needs_serial(&self) -> bool {
        true
    }
    fn transmits_premature_termination(&self) -> bool {
        false // since we want to dump all the reads even if later on there's a Head
    }

    fn init(
        &mut self,
        _input_info: &InputInfo,
        mut output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        _input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        let per_tag = output_files.take("tag_fastq");
        let mut handles = DemultiplexedData::new();
        for (tag, writer) in per_tag {
            handles.insert(tag, Some(writer));
        }
        self.output_handles = Some(Arc::new(Mutex::new(handles)));
        Ok(None)
    }

    fn apply(
        &self,
        block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> Result<(FastQBlocksCombined, bool)> {
        let in_tag_col = block
            .tags
            .get(&self.in_label)
            .expect("in_label tag must exist in block");
        let col = in_tag_col
            .as_locations()
            .expect("tag was verified to be column!)");

        for (idx, ((name, tag_seq), tag_qual)) in block.segments[col.source_id() as usize]
            .names
            .iter()
            .zip(col.iter_seq_joined(Some(self.region_separator.as_ref())))
            .zip(col.iter_qual_joined(Some(self.region_separator.as_ref())))
            .enumerate()
        //todo: do we want itertools izip for this?
        {
            if !tag_seq.is_empty() {
                // Determine which output stream to use based on demultiplexing
                let output_idx = block.output_tags.as_ref().map_or(0, |x| x[idx]);

                let mut output_handles = self
                    .output_handles
                    .as_ref()
                    .expect("Should have been set in init")
                    .lock()
                    .expect("lock poisoned");
                if let Some(Some(writer)) = output_handles.get_mut(&output_idx) {
                    //if we have demultiplex & no-unmatched-output, this happens
                    let name = if let Some(comment_tags) = self.comment_tags.as_ref() {
                        let mut name: BString = BString::from(name);
                        for tag in comment_tags {
                            let tag_col = block.tags.get(tag).expect("tag must exist in block");
                            let tag_bytes = tag_col.to_bstr(
                                idx,
                                format_numeric_for_comment,
                                Some(self.region_separator.as_ref()),
                            );
                            name = store_tag_in_comment(
                                &name,
                                tag.as_ref().as_bytes(),
                                &tag_bytes,
                                self.comment_separator,
                                self.comment_insert_char,
                            )?
                            .into();
                        }
                        Cow::Owned(name)
                    } else {
                        Cow::Borrowed(name)
                    };

                    let mut buf = Vec::new();
                    match self.format {
                        FileFormat::Fastq | FileFormat::None | FileFormat::Text => {
                            buf.push(b'@');
                            buf.extend_from_slice(&name);
                            buf.push(b'\n');
                            buf.extend_from_slice(&tag_seq);
                            buf.extend_from_slice(b"\n+\n");
                            buf.extend_from_slice(&tag_qual);
                            buf.push(b'\n');
                        }
                        FileFormat::Fasta => {
                            buf.push(b'>');
                            buf.extend_from_slice(&name);
                            buf.push(b'\n');
                            buf.extend_from_slice(&tag_seq);
                            buf.push(b'\n');
                        }
                        // cov:excl-start
                        FileFormat::Bam => {
                            unreachable!("Unsupported format encountered after validation")
                        } // cov:excl-stop
                    }
                    writer.write_text_record(&buf)?;
                }
            }
        } // cov:excl-line // end of as_location

        Ok((block, true))
    }

    fn finalize(
        &self,
        _demultiplex_info: &OptDemultiplex,
    ) -> Result<Option<crate::transformations::FinalizeReportResult>> {
        for (_tag, writer) in self
            .output_handles
            .as_ref()
            .expect("output_handles should have been set in init")
            .lock()
            .expect("lock poisoned")
            .iter_mut()
        {
            let () = writer.take().expect("Should have had writer?!").finish()?;
        }
        Ok(None)
    }
}
