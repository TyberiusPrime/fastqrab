use crate::transformations::prelude::*;

use super::{build_report_json, render_html_report};

const JSON_ID: &str = "report_json";
const HTML_ID: &str = "report_html";

/// Write the combined run report (JSON and/or HTML) as a pipeline step.
///
/// Replicates the report-writing behaviour of the legacy `[output]` section
/// (`report_json` / `report_html`). The report data is handed to the step in
/// [`Step::post_finalize`], after every other step's `finalize` has populated
/// the report collector.
///
/// At most one `OutputReport` step is allowed per pipeline (enforced in config
/// validation); multiple would share the same collected report data.
///
/// Reports are keyed by their config label (from [`ReportMetadata::report_labels`],
/// indexed by `report_no`); the run/input metadata the legacy `[output]` renderer
/// embedded is taken from the [`ReportMetadata`] handed to the step at `init`.
#[derive(JsonSchema, Clone)]
#[tpd]
#[derive(Debug)]
pub struct OutputReport {
    /// Emit a `{prefix}.json` report.
    #[tpd(default)]
    pub json: bool,
    /// Emit a `{prefix}.html` report.
    #[tpd(default)]
    pub html: bool,

    #[tpd(skip, default)]
    #[schemars(skip)]
    json_writer: Option<Arc<Mutex<Option<ChunkedRecordWriter>>>>,
    #[tpd(skip, default)]
    #[schemars(skip)]
    html_writer: Option<Arc<Mutex<Option<ChunkedRecordWriter>>>>,
    #[tpd(skip, default)]
    #[schemars(skip)]
    report_metadata: Option<Arc<ReportMetadata>>,
}

impl VerifyIn<PartialConfig> for PartialOutputReport {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        if !*self.json.as_ref().unwrap_or(&false) && !*self.html.as_ref().unwrap_or(&false) {
            return Err(ValidationFailure::new(
                "OutputReport writes nothing",
                Some("Set 'json = true' and/or 'html = true'"),
            ));
        }
        Ok(())
    }
}

fn report_declaration(id: &str, suffix: &str, span: std::ops::Range<usize>) -> OutputDeclaration {
    OutputDeclaration {
        id: id.to_string(),
        target: WriteTargetConfig::new(Vec::new(), None, suffix.to_string()),
        sink_config: SinkConfig::new_uncompressed_unhashed(),
        format: FileFormat::Text,
        chunk_policy: ChunkPolicy::default(),
        bam_options: None,
        singleton: true,
        span,
    }
}

impl TagUser for PartialTaggedVariant<PartialOutputReport> {
    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        if let Some(inner) = self.toml_value.as_ref() {
            let mut decls = Vec::new();
            if *inner.json.unwrap_ref() {
                decls.push(report_declaration(JSON_ID, "json", inner.json.span()));
            }
            if *inner.html.unwrap_ref() {
                decls.push(report_declaration(HTML_ID, "html", inner.html.span()));
            }
            Some(decls)
        } else {
            Some(vec![]) //there should be output files, but we can't name them.
        }
    }
}

/// Take the single (singleton, tag 0) writer for a declared id, if present.
fn take_singleton(files: &mut StepOutputFiles, id: &str) -> Option<ChunkedRecordWriter> {
    files.take(id).into_iter().next().map(|(_tag, w)| w)
}

impl Step for OutputReport {
    // does not need serial
    // fn needs_serial(&self) -> bool {
    //     true
    // }
    // that's relevant for the count steps, but this one
    // can safely 'hang up',
    // fn transmits_premature_termination(&self) -> bool {
    //     true
    // }

    fn init(
        &mut self,
        input_info: &InputInfo,
        mut output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        _input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        self.report_metadata = Some(input_info.report_metadata.clone());
        if self.json {
            self.json_writer = Some(Arc::new(Mutex::new(take_singleton(
                &mut output_files,
                JSON_ID,
            ))));
        }
        if self.html {
            self.html_writer = Some(Arc::new(Mutex::new(take_singleton(
                &mut output_files,
                HTML_ID,
            ))));
        }
        Ok(None)
    }

    fn apply(
        &self,
        block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> Result<(FastQBlocksCombined, bool)> {
        // Pure sink: reports are written in post_finalize, reads pass through.
        Ok((block, true))
    }

    fn post_finalize(&self, reports: &[FinalizeReportResult]) -> Result<()> {
        // Build the JSON once; HTML embeds the same JSON.
        let metadata = self
            .report_metadata
            .as_ref()
            .expect("report_metadata is set in init");
        let json = build_report_json(reports, metadata)?;

        if let Some(writer) = &self.json_writer
            && let Some(mut writer) = writer.lock().expect("lock poisoned").take()
        {
            writer.write_text_record(json.as_bytes())?;
            let () = writer.finish()?;
        }

        if let Some(writer) = &self.html_writer
            && let Some(mut writer) = writer.lock().expect("lock poisoned").take()
        {
            let html = render_html_report(&json)?;
            writer.write_text_record(html.as_bytes())?;
            let () = writer.finish()?;
        }

        Ok(())
    }
}
