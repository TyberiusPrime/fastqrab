use std::cell::RefCell;
use std::rc::Rc;

use crate::transformations::{output::validate_compression_level_u8, prelude::*};
use crate::verify_path_component;
use fastqrab_io::{CompressionFormat, FileFormat};

/// Inspect reads within the workflow
#[derive(JsonSchema)]
#[tpd]
pub struct Inspect {
    pub n: usize,

    #[schemars(with = "String")]
    #[tpd(adapt_in_verify(String))]
    segment: SegmentIndexOrAll,

    pub infix: String,
    #[tpd(skip, default)]
    #[schemars(skip)]
    #[expect(dead_code, reason = "only accessed in PartialInspect")]
    resolved_segment_name: String,
    #[tpd(default)]
    pub suffix: Option<String>,
    #[tpd(default)]
    pub format: FileFormat,
    #[tpd(default)]
    pub compression: CompressionFormat,
    #[tpd(default)]
    pub compression_level: Option<u8>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub collector: Arc<Mutex<Vec<(OwnedMolecule, DemultiplexTag)>>>,
    #[tpd(skip)]
    #[schemars(skip)]
    collected: std::sync::atomic::AtomicUsize,

    #[tpd(skip, default)]
    #[schemars(skip)]
    writer: Arc<Mutex<Option<ChunkedRecordWriter>>>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    demultiplex_names: Option<DemultiplexedData<String>>,
}

impl VerifyIn<PartialConfig> for PartialInspect {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.segment.validate_segment(parent);
        self.infix.verify(verify_path_component);
        self.format.verify(|format| {
            if !matches!(format, FileFormat::Fastq | FileFormat::Fasta) {
                return Err(ValidationFailure::new(
                    "Inspect step supports only 'fastq' or 'fasta' formats",
                    None,
                ));
            }
            Ok(())
        });
        validate_compression_level_u8(&self.compression, &mut self.compression_level);
        if let Some(MustAdapt::PostVerify(segment)) = self.segment.as_ref()
            && let Some(segment_order) = parent
                .input
                .as_ref()
                .map(crate::config::PartialInput::get_segment_order)
        {
            let target_name = match segment {
                SegmentIndexOrAll::All => "interleaved".to_string(),
                SegmentIndexOrAll::Indexed(idx) => segment_order
                    .get(idx.as_index())
                    .cloned()
                    .unwrap_or_default(),
            };
            self.resolved_segment_name = Some(target_name);
            self.collector = Some(Arc::new(Mutex::new(Vec::new())));
        } // cov:excl-line
        self.collected = Some(std::sync::atomic::AtomicUsize::new(0));
        Ok(())
    }
}

// cov:excl-start
#[expect(
    clippy::missing_fields_in_debug,
    reason = "that's why we have a manual Debug impl"
)]
impl std::fmt::Debug for Inspect {
    #[mutants::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Inspect")
            .field("n", &self.n)
            .field("segment", &self.segment)
            //        .field("segment_index", &self.segment_index)
            .field("infix", &self.infix)
            .field("suffix", &self.suffix)
            .field("format", &self.format)
            .field("compression", &self.compression)
            //       .field("compression_level", &self.compression_level)
            //.field("collected", &self.collected)
            .finish()
    }
}
// cov:excl-stop

impl TagUser for PartialTaggedVariant<PartialInspect> {
    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        if let Some(inner) = self.toml_value.as_ref() {
            let infix = inner.infix.as_ref().cloned().unwrap_or_default();
            let segment_name = inner
                .resolved_segment_name
                .as_deref()
                .unwrap_or_default()
                .to_string();
            let mut infix_parts = vec![infix];
            if !segment_name.is_empty() {
                infix_parts.push(segment_name);
            }
            let compression = inner.compression.as_ref().copied().unwrap_or_default();
            let compression_level = inner
                .compression_level
                .as_ref()
                .and_then(|x| x.as_ref())
                .copied();
            let format = inner.format.as_ref().copied().unwrap_or_default();
            let custom_suffix = inner.suffix.as_ref().and_then(|opt| opt.as_ref());
            let suffix = format.get_suffix(compression, custom_suffix);
            Some(vec![OutputDeclaration {
                id: "inspect".to_string(),
                target: WriteTargetConfig::new(infix_parts, None, suffix),
                sink_config: SinkConfig {
                    compression,
                    compression_level,
                    hash_uncompressed: false,
                    hash_compressed: false,
                    simulated_failure: None,
                },
                format,
                chunk_policy: ChunkPolicy::default(),
                bam_options: None,
                singleton: true,
                span: inner.infix.span(),
            }])
        } else {
            Some(vec![]) //there should be output files, but we can't name them.
        }
    }

    fn get_tag_usage(
        &mut self,
        tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        let final_in_labels: Vec<_> = tags_available.keys().cloned().collect();
        let toml_source = Rc::new(RefCell::new((
            &mut self.toml_value.state,
            &mut self.toml_value.help,
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
            used_tags,
            ..Default::default()
        })
    }
}

impl Step for Inspect {
    #[mutants::skip] // doesn't always trigger if replaced by false, but often enough
    fn needs_serial(&self) -> bool {
        true
    }

    fn init(
        &mut self,
        _input_info: &InputInfo,
        mut output_files: StepOutputFiles,
        demultiplex_info: &OptDemultiplex,
        _input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        let mut per_tag = output_files.take("inspect");
        let writer = per_tag
            .remove(&0)
            .expect("tag 0 writer must exist for inspect output");
        *self.writer.lock().expect("poisoned") = Some(writer);

        if let OptDemultiplex::Yes(info) = demultiplex_info {
            self.demultiplex_names = Some(
                info.tag_to_name
                    .iter()
                    .filter_map(|(tag, name)| name.as_ref().map(|name| (*tag, name.clone())))
                    .collect(),
            );
        }
        Ok(None)
    }

    fn apply(
        &self,
        block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let mut collected = self.collected.load(std::sync::atomic::Ordering::Relaxed);
        if collected >= self.n {
            return Ok((block, true));
        }

        let mut collector = self.collector.lock().expect("collector mutex poisoned");
        let iter: Box<dyn Iterator<Item = (Molecule, DemultiplexTag)>> =
            if let Some(output_tags) = block.output_tags.as_ref() {
                Box::new(block.molecules().zip(output_tags.iter().copied()))
            } else {
                Box::new(block.molecules().zip(std::iter::repeat(0))) // if no output tags, treat all as tag 0
            };
        let name_read = |name: &mut BString, read_idx: usize| {
            for (key, values) in &block.tags {
                let str_key: &str = key.as_ref();
                name.push(b' ');
                name.extend_from_slice(str_key.as_bytes());
                name.push(b'=');
                name.extend_from_slice(&values.to_bstr(read_idx, |float| float.to_string(), None));
            }
        };
        for (read_idx, (molecule, tag)) in iter.enumerate() {
            if collected >= self.n {
                break;
            }

            let mut molecule: OwnedMolecule = match self.segment {
                SegmentIndexOrAll::All => molecule.into(),
                SegmentIndexOrAll::Indexed(idx) => {
                    let single_segment_molecule: OwnedMolecule = (&molecule[idx.as_index()]).into();
                    single_segment_molecule
                }
            };
            name_read(&mut molecule.reads[0].name, read_idx);
            collector.push((molecule, tag));

            collected += 1; //count per molecule, not per segment
        }
        self.collected
            .store(collected, std::sync::atomic::Ordering::Relaxed);
        Ok((block, true))
    }
    fn finalize(&self, _demultiplex_info: &OptDemultiplex) -> Result<Option<FinalizeReportResult>> {
        let collector = self.collector.lock().expect("collector mutex poisoned");
        let mut writer = self
            .writer
            .lock()
            .expect("writer mutex poisoned")
            .take()
            .expect("writer must be set during initialization");

        if !collector.is_empty() {
            let mut buf = Vec::with_capacity(256);
            match self.format {
                FileFormat::None | FileFormat::Fastq | FileFormat::Text => {
                    for (molecule, tag) in collector.iter() {
                        for read in &molecule.reads {
                            buf.clear();
                            buf.push(b'@');
                            buf.extend_from_slice(&read.name);
                            if let Some(demux_names) = &self.demultiplex_names
                                && let Some(demux_name) = demux_names.get(tag)
                            {
                                buf.extend_from_slice(b" _Demultiplex=");
                                buf.extend_from_slice(demux_name.as_bytes());
                            }
                            buf.push(b'\n');
                            buf.extend_from_slice(&read.seq);
                            buf.extend_from_slice(b"\n+\n");
                            buf.extend_from_slice(&read.qual);
                            buf.push(b'\n');
                            writer.write_text_record(&buf)?;
                        }
                    }
                }
                FileFormat::Fasta => {
                    for (molecule, tag) in collector.iter() {
                        for read in &molecule.reads {
                            buf.clear();
                            buf.push(b'>');
                            buf.extend_from_slice(&read.name);
                            if let Some(demux_names) = &self.demultiplex_names
                                && let Some(demux_name) = demux_names.get(tag)
                            {
                                buf.extend_from_slice(b" Demultiplex=");
                                buf.extend_from_slice(demux_name.as_bytes());
                            }
                            buf.push(b'\n');
                            buf.extend_from_slice(&read.seq);
                            buf.push(b'\n');
                            writer.write_text_record(&buf)?;
                        } // cov:excl-line
                    }
                }
                // cov:excl-start
                FileFormat::Bam => {
                    panic!("Bam not valid - should have been caught in verify");
                } // cov:excl-stop
            }
        }

        let () = writer.finish()?;
        Ok(None)
    }
}
