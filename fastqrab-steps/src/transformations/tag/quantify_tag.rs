use bstr::ByteSlice;
use std::collections::BTreeMap;

use crate::transformations::prelude::*;
use crate::verify_path_component;
use fastqrab_config::{default_region_separator, tpd_adapt_bstring};

type QuantifyTagCollector = Arc<Mutex<DemultiplexedData<BTreeMap<BString, usize>>>>;
type OutputHandles = Arc<Mutex<DemultiplexedData<Option<ChunkedRecordWriter>>>>;

/// Write a histogram of tag values to a JSON file.
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct QuantifyTag {
    pub infix: String,
    pub in_label: TagLabel,

    #[schemars(with = "String")]
    #[tpd(with = "tpd_adapt_bstring")]
    region_separator: BString,

    #[tpd(skip, default)]
    #[schemars(skip)]
    pub collector: Option<QuantifyTagCollector>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    output_handles: Option<OutputHandles>,
}

impl VerifyIn<PartialConfig> for PartialQuantifyTag {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.infix.verify(verify_path_component);
        self.region_separator.or_with(default_region_separator);
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialQuantifyTag> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            Some(TagUsageInfo {
                used_tags: vec![
                    inner
                        .in_label
                        .to_used_tag(&[TagValueType::Location, TagValueType::String]),
                ],
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }

    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        if let Some(inner) = self.toml_value.as_ref() {
            let infix = inner.infix.as_ref().expect("Verification had passed");
            Some(vec![OutputDeclaration {
                id: "qr".to_string(),
                target: WriteTargetConfig::new(vec![infix.clone()], None, "qr.json".to_string()),
                sink_config: SinkConfig::default(),
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
}

impl Step for QuantifyTag {
    fn transmits_premature_termination(&self) -> bool {
        false
    }
    #[mutants::skip] //better to have the pipeline know about it than blocking in our
    //internal lock
    fn needs_serial(&self) -> bool {
        true
    }

    fn init(
        &mut self,
        _input_info: &InputInfo,
        mut output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        _input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        let per_tag = output_files.take("qr");

        let mut collector = DemultiplexedData::new();
        let mut handles = DemultiplexedData::new();
        for (tag, writer) in per_tag {
            collector.insert(tag, BTreeMap::new());
            handles.insert(tag, Some(writer));
        }
        self.collector = Some(Arc::new(Mutex::new(collector)));
        self.output_handles = Some(Arc::new(Mutex::new(handles)));

        Ok(None)
    }

    fn apply(
        &self,
        block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let mut collector = self
            .collector
            .as_ref()
            .expect("collector should have been set in init")
            .lock()
            .expect("Lock poisoned");
        let hits = block
            .tags
            .get(&self.in_label)
            .expect("Tag not found. Should have been caught in validation");
        let iter: Box<dyn Iterator<Item = Cow<BStr>>> = match hits {
            TagColumn::Location(col) => {
                Box::new(col.iter_seq_joined(Some(self.region_separator.as_ref())))
            }
            TagColumn::String(col) => Box::new(col.iter().map(|s| {
                Cow::Borrowed(s.as_ref().map_or_else(|| BStr::new(b""), |x| x.as_bstr()))
            })),
            _ => unreachable!("Tag validation must prevent numeric/bool tags from reaching here"),
        };

        if let Some(demultiplex_tags) = &block.output_tags {
            for (seq, demultiplex_tag) in iter.zip(demultiplex_tags) {
                if !seq.is_empty()
                    && let Some(inner) = collector.get_mut(demultiplex_tag)
                {
                    *inner
                        .entry(BString::new(seq.to_vec())) //todo: avoid this allocation?
                        .or_insert(0) += 1;
                }
            }
        } else {
            for seq in iter {
                if !seq.is_empty()
                    && let Some(inner) = collector.get_mut(&0)
                {
                    *inner.entry(BString::new(seq.to_vec())).or_insert(0) += 1;
                }
            }
        }

        Ok((block, true))
    }

    fn finalize(&self, _demultiplex_info: &OptDemultiplex) -> Result<Option<FinalizeReportResult>> {
        let collector = self
            .collector
            .as_ref()
            .expect("collector should have been set in init")
            .lock()
            .expect("Lock poisoned");
        let mut handles = self
            .output_handles
            .as_ref()
            .expect("output_handles should have been set in init")
            .lock()
            .expect("Lock poisoned");
        for (tag, writer_opt) in handles.iter_mut() {
            let mut writer = writer_opt
                .take()
                .expect("Writer should have been set in init");
            let mut str_collector: Vec<(String, usize)> = collector
                .get(&tag)
                .expect("value must exist in histogram_values")
                .iter()
                .map(|(k, v)| (String::from_utf8_lossy(k).to_string(), *v))
                .collect();
            str_collector.sort_by(|a, b| {
                b.1.cmp(&a.1)
                    .then_with(|| a.0.to_lowercase().cmp(&b.0.to_lowercase()))
            });
            let str_collector: indexmap::IndexMap<String, usize> =
                str_collector.into_iter().collect();
            let json = serde_json::to_string_pretty(&str_collector)?;
            writer.write_text_record(json.as_bytes())?;
            let () = writer.finish()?;
        }
        Ok(None)
    }
}
