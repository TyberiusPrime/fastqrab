use crate::{no_barcode_infix, transformations::prelude::*};
use std::{cell::RefCell, collections::HashMap, thread::ThreadId};

thread_local! {
    // Keyed by step address so multiple histogram steps don't share accumulators.
    // The inner Mutex is per-thread, so it is never contended.
    static LOCAL: RefCell<HashMap<usize, Arc<Mutex<DemultiplexedData<HistogramData>>>>>
        = RefCell::new(HashMap::new());
}

/// Histogram data structure that can handle both String and Numeric tags
#[derive(Debug, Clone)]
pub enum HistogramData {
    /// String values mapped to their counts
    String(FxIndexMap<String, usize>),
    /// Numeric values bucketed into bins (value -> count)
    //Integer(FxIndexMap<i64, usize>),
    Float(FxIndexMap<NonNaN, usize>),
    /// Boolean values (false count, true count)
    Bool(usize, usize),
}

impl HistogramData {
    pub fn merge(&mut self, other: Self) {
        match (self, other) {
            (Self::String(a), Self::String(b)) => {
                for (k, v) in b {
                    *a.entry(k).or_insert(0) += v;
                }
            }
            // (Self::Integer(a), Self::Integer(b)) => {
            //     for (k, v) in b {
            //         *a.entry(k).or_insert(0) += v;
            //     }
            // }
            (Self::Float(a), Self::Float(b)) => {
                for (k, v) in b {
                    *a.entry(k).or_insert(0) += v;
                }
            }
            (Self::Bool(a_false, a_true), Self::Bool(b_false, b_true)) => {
                *a_false += b_false;
                *a_true += b_true;
            }
            _ => unreachable!(), // cov:excl-line
        }
    }

    // #[expect(
    //     clippy::cast_possible_truncation,
    //     reason = "precision loss for huge values is ok"
    // )]
    pub fn add_from_column(&mut self, col: &TagColumn, idx: usize) {
        match col {
            TagColumn::Location(col) => {
                if let HistogramData::String(map) = self {
                    let s = String::from_utf8_lossy(&col.joined_seq(idx, Some(b"_"))).into_owned();
                    *map.entry(s).or_insert(0) += 1;
                } else {
                    unreachable!()
                }
            }
            TagColumn::String(items) => {
                if let HistogramData::String(map) = self {
                    let s = match items.get_string(idx) {
                        None => "(missing)".to_string(),
                        Some(s) => s.to_string(),
                    };
                    *map.entry(s).or_insert(0) += 1;
                } else {
                    unreachable!()
                }
            }
            TagColumn::Numeric(items) => {
                let n = items[idx];
                match self {
                    // HistogramData::Integer(map) => {
                    //     *map.entry(n.round() as i64).or_insert(0) += 1;
                    // }
                    HistogramData::Float(map) => {
                        let bucket: NonNaN = ((n * 100.).round() / 100.0)
                            .try_into()
                            .expect("NaN value for histogram - not supported");
                        *map.entry(bucket).or_insert(0) += 1;
                    }
                    _ => unreachable!(),
                }
            }
            TagColumn::Bool(items) => {
                if let HistogramData::Bool(false_count, true_count) = self {
                    if items[idx] {
                        *true_count += 1;
                    } else {
                        *false_count += 1;
                    }
                } else {
                    unreachable!() // cov:excl-line
                }
            }
        }
    }
}

impl From<HistogramData> for serde_json::Value {
    fn from(value: HistogramData) -> Self {
        match value {
            HistogramData::String(map) => {
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();
                keys.iter()
                    .map(|k| ((*k).clone(), *map.get(*k).expect("Keys came from map")))
                    .collect()
            }
            //json only does string keys
            // HistogramData::Integer(map) => {
            //     let mut keys: Vec<_> = map.keys().collect();
            //     keys.sort();
            //     keys.iter()
            //         .map(|k| (k.to_string(), *map.get(*k).expect("Keys came from map")))
            //         .collect()
            // }
            HistogramData::Float(map) => {
                let mut keys: Vec<_> = map.keys().collect();
                keys.sort();
                keys.iter()
                    .map(|k| (format!("{k:.2}"), *map.get(*k).expect("Keys came from map")))
                    //2 digits incidentially end up without .00 in json
                    .collect()
            }

            HistogramData::Bool(false_count, true_count) => {
                let mut map = serde_json::Map::new();
                map.insert("true".into(), true_count.into());
                map.insert("false".into(), false_count.into());
                serde_json::Value::Object(map)
            }
        }
    }
}

#[derive(Clone)]
#[tpd(no_verify)]
#[derive(Debug)]
pub struct _ReportTagHistogram {
    pub report_no: usize,
    pub tag_name: TagLabel,
    #[tpd(skip)]
    pub tag_type: TagValueType,
    #[tpd(skip)]
    #[expect(
        clippy::type_complexity,
        reason = "Don't see teh point in refactoring it"
    )]
    pub data: Arc<Mutex<HashMap<ThreadId, Arc<Mutex<DemultiplexedData<HistogramData>>>>>>,
}

impl Partial_ReportTagHistogram {
    pub fn new(report_no: usize, tag_name: TomlValue<TagLabel>) -> Self {
        Self {
            report_no: TomlValue::new_ok_unplaced(report_no),
            tag_name,
            tag_type: None,
            data: Some(Arc::new(Mutex::new(HashMap::new()))),
        }
    }
}

impl TagUser for PartialTaggedVariant<Box<Partial_ReportTagHistogram>> {
    fn get_tag_usage(
        &mut self,
        tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            if let Some(tag_meta) =
                tags_available.get(inner.tag_name.as_ref().expect("parent was ok?"))
            {
                inner.tag_type = Some(tag_meta.tag_type);
            } else {
                //no need to set it, missing tag will fail before the 'tag_type not set in verify'
                //if that's happening at all for our dynamically generated one.
            }
            Some(TagUsageInfo {
                used_tags: vec![inner.tag_name.to_used_tag(&[
                    TagValueType::String,
                    TagValueType::Numeric((None, None)),
                    TagValueType::Bool,
                    TagValueType::Location,
                ])],
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

impl _ReportTagHistogram {
    fn new_histogram(&self) -> HistogramData {
        match self.tag_type {
            TagValueType::Location | TagValueType::String => {
                HistogramData::String(FxIndexMap::default())
            }
            TagValueType::Numeric(_) => HistogramData::Float(FxIndexMap::default()),
            TagValueType::Bool => HistogramData::Bool(0, 0),
        }
    }

    // Returns this thread's accumulator Arc, registering it with the shared
    // registry on first call. The returned Arc is never held by any other
    // thread, so locking it is always uncontended.
    fn get_or_create_local(&self) -> Arc<Mutex<DemultiplexedData<HistogramData>>> {
        #[expect(
            clippy::ref_as_ptr,
            reason = "we are (ab)using the pointer as index into a hashmap"
        )]
        //todo: can we do this prettier? thread id or such?
        let step_addr = self as *const _ReportTagHistogram as usize;
        LOCAL.with(|local| {
            let mut cache = local.borrow_mut();
            if let Some(arc) = cache.get(&step_addr) {
                return arc.clone();
            }
            let new_arc: Arc<Mutex<DemultiplexedData<HistogramData>>> =
                Arc::new(Mutex::new(DemultiplexedData::default()));
            self.data
                .lock()
                .expect("Lock poisoned")
                .insert(std::thread::current().id(), new_arc.clone());
            cache.insert(step_addr, new_arc.clone());
            new_arc
        })
    }
}

impl Step for Box<_ReportTagHistogram> {
    fn transmits_premature_termination(&self) -> bool {
        false
    }

    #[mutants::skip]
    fn needs_serial(&self) -> bool {
        false
    }

    fn init(
        &mut self,
        _input_info: &InputInfo,
        _output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        _input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        Ok(None)
    }

    fn apply(
        &self,
        block: FastQBlocksCombined,
        _input_info: &InputInfo,
        demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        if let Some(tag_values) = block.tags.get(&self.tag_name) {
            // Each thread accumulates into its own DemultiplexedData.
            // get_or_create_local only locks the shared registry on first call;
            // after that the inner lock is always uncontended.
            let local_arc = self.get_or_create_local();
            let mut local = local_arc.lock().expect("Lock poisoned");

            match demultiplex_info {
                OptDemultiplex::No => {
                    let histogram = local.entry(0).or_insert_with(|| self.new_histogram());
                    for read_idx in 0..tag_values.len() {
                        histogram.add_from_column(tag_values, read_idx);
                    }
                }
                OptDemultiplex::Yes(_) => {
                    if let Some(output_tags) = &block.output_tags {
                        for (read_idx, &demux_tag) in output_tags.iter().enumerate() {
                            local
                                .entry(demux_tag)
                                .or_insert_with(|| self.new_histogram())
                                .add_from_column(tag_values, read_idx);
                        }
                    } // cov:excl-line
                }
            }
        } // cov:excl-line
        Ok((block, true))
    }

    fn finalize(&self, demultiplex_info: &OptDemultiplex) -> Result<Option<FinalizeReportResult>> {
        // Collect and merge all per-thread accumulators. Called after all
        // apply() calls complete, so no thread is writing concurrently.
        let thread_map = self.data.lock().expect("Lock poisoned");
        let mut merged: DemultiplexedData<HistogramData> = DemultiplexedData::default();
        for thread_data in thread_map.values() {
            let taken: DemultiplexedData<HistogramData> =
                std::mem::take(&mut *thread_data.lock().expect("Lock poisoned"));
            for (tag, hist) in taken {
                merged
                    .entry(tag)
                    .or_insert_with(|| self.new_histogram())
                    .merge(hist);
            }
        }
        drop(thread_map);

        let mut contents = serde_json::Map::new();
        let histogram_key = self.tag_name.clone();

        match demultiplex_info {
            OptDemultiplex::No => {
                let histogram = merged.remove(&0).unwrap_or_else(|| self.new_histogram());
                let mut histogram_contents = serde_json::Map::new();
                histogram_contents.insert(histogram_key.as_ref().to_string(), histogram.into());
                contents.insert(
                    "histogram".to_string(),
                    serde_json::Value::Object(histogram_contents),
                );
            }

            OptDemultiplex::Yes(demultiplex_info) => {
                // Place histogram nested inside each barcode bucket so that after
                // pipeline_workpool wraps everything in "multiplexed", the histogram
                // lives alongside molecule_count inside the per-bucket object and the
                // HTML template can find it via addSectionTable().
                for (tag, name) in &demultiplex_info.tag_to_name {
                    let barcode_key = name.as_ref().map_or(no_barcode_infix(), |x| x.as_str());
                    let histogram = merged.remove(tag).unwrap_or_else(|| self.new_histogram());
                    let mut inner = serde_json::Map::new();
                    inner.insert(histogram_key.as_ref().to_string(), histogram.into());
                    let mut barcode_contents = serde_json::Map::new();
                    barcode_contents
                        .insert("histogram".to_string(), serde_json::Value::Object(inner));
                    contents.insert(
                        barcode_key.to_string(),
                        serde_json::Value::Object(barcode_contents),
                    );
                }
            }
        }

        Ok(Some(FinalizeReportResult {
            report_no: self.report_no,
            contents: serde_json::Value::Object(contents),
        }))
    }
}
