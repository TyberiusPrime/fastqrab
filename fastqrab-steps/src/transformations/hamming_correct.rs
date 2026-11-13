use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use anyhow::anyhow;

use hamming_resonate::HammingResonator;
use indexmap::IndexMap;
use rustc_hash::FxHashMap;

use super::hamming_exact_counter::MajorityData;
use crate::transformations::prelude::*;
use fastqrab_config::tpd_adapt_u8_from_byte_or_char;
use fastqrab_dna::dna::init_hamming_resonator;

/// Correct a tag (extracted region) to known barcodes
///
#[derive(Debug, JsonSchema)]
#[tpd(no_verify)]
pub struct CountsFromReport {
    pub filename: String,
    pub report_name: String,
    pub tag_name: String,
}

/// `StepInputFiles` id linking `declare_input_files` to the handle taken in
/// `init`. It is the config field path so an open failure names what the user
/// actually wrote.
const COUNTS_FROM_REPORT_ID: &str = "on_tie_use_counts_from_report.filename";

#[derive(JsonSchema)]
#[tpd]
pub struct HammingCorrect {
    /// Input tag to correct
    pub in_label: TagLabel,
    /// Output tag to store corrected result
    pub out_label: TagLabel,
    /// Reference to barcodes section
    pub barcodes: TagLabel,
    /// Maximum hamming distance for correction
    pub max_hamming_distance: u8,

    #[tpd(default)]
    pub output: HammingOutput,

    /// What to do when no match is found
    pub on_no_match: OnNoMatch,

    /// What to do when more than one match an the same distance is found
    pub on_tie: OnTie,

    #[tpd(alias = "by_majority_min_molecules_to_start")]
    pub on_tie_min_molecules_to_start: usize,
    #[tpd(alias = "by_majority_threshold")]
    pub on_tie_threshold: f64,

    #[tpd(nested)]
    pub on_tie_use_counts_from_report: Option<CountsFromReport>,

    #[tpd(default)]
    #[schemars(skip)]
    pub on_tie_dump_counts: bool, //debug option.
    //
    #[tpd(skip)]
    #[schemars(skip)]
    count_writer: Arc<Mutex<Option<ChunkedRecordWriter>>>,

    /// names are considered identical if they match up to the first `name_split_character`
    #[tpd(with = "tpd_adapt_u8_from_byte_or_char", alias = "name_split_char")]
    pub name_split_character: Option<u8>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub(crate) seq_to_name: Arc<IndexMap<BString, String>>,

    /// FxHash-backed sequence -> position-in-`seq_to_name` lookup. Built once at
    /// verify-time; used for the hot exact-match path which is dominated by hashing
    /// short DNA keys (default `IndexMap` hasher is `SipHash`, far slower on tiny keys).
    #[tpd(skip)]
    #[schemars(skip)]
    pub(crate) seq_to_idx: Arc<FxHashMap<BString, usize>>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub(crate) resonator: Arc<HammingResonator>,

    #[tpd(skip)]
    #[schemars(skip)]
    pub majority_data: Option<Arc<MajorityData>>,

    /// Side channel from the parallel `_HammingPreMatch` step. Set by
    /// `expand_transformations` for ByMajority/ByEditProbability; `None` for
    /// non-counting modes (which do phase 1 inline).
    #[tpd(skip)]
    #[schemars(skip)]
    pub pre_match: Option<Arc<PreMatchData>>,

    #[tpd(skip)]
    #[schemars(skip)]
    reads_in_this_step: AtomicUsize, // for verification that we really count correctly between
    // HammingExactCounter and HammingCorrect in ByMajority mode
    #[tpd(skip)]
    #[schemars(skip)]
    final_block_seen: AtomicBool,
}

#[expect(
    clippy::missing_fields_in_debug,
    reason = "Thats the reason for the impl"
)]
impl std::fmt::Debug for HammingCorrect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HammingCorrect")
            .field("out_label", &self.out_label)
            .finish()
    }
}

#[tpd]
#[derive(Debug, JsonSchema, Default)]
pub enum HammingOutput {
    #[tpd(alias = "DNA", alias = "barcodes")]
    #[default]
    Barcode,
    #[tpd(alias = "labels")]
    Label,
}

impl VerifyIn<PartialConfig> for PartialHammingCorrect {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        //defaults first, error returns later
        self.reads_in_this_step = Some(AtomicUsize::new(0));
        self.on_tie_min_molecules_to_start.or(1_000_000);
        self.on_tie_threshold.or(0.975);

        if let Some(Some(_counts_from_report)) = self.on_tie_use_counts_from_report.as_ref() {
            self.on_tie_min_molecules_to_start.value = Some(0);
            if !matches!(
                self.on_tie.as_ref(),
                Some(OnTie::ByMajority | OnTie::ByEditProbability),
            ) {
                self.on_tie_use_counts_from_report.help = Some(
                    "on_tie_use_counts_from_report only makes sense if \
                        on_tie is ByMajority or ByEditProbability"
                        .to_string(),
                );
                let spans = vec![
                    (
                        self.on_tie_use_counts_from_report.span(),
                        "Wrong on_tie mode".to_string(),
                    ),
                    (
                        self.on_tie.span(),
                        "Incompatible with on_tie_use_counts_from_report".to_string(),
                    ),
                ];
                self.on_tie_use_counts_from_report.state = TomlValueState::Custom { spans };
            }
        }

        if let Some(out_label) = self.out_label.as_ref()
            && let Some(in_label) = self.in_label.as_ref()
            && out_label == in_label
        {
            let spans = vec![
                (self.in_label.span(), "The same as out_label".to_string()),
                (self.out_label.span(), "The same as in_label".to_string()),
            ];
            self.out_label.state = TomlValueState::Custom { spans };
            self.out_label.help =
                Some("Please use different tag names for the input and output labels to avoid overwriting the source tag.".to_string())
                ;
        }
        //0 for max_hamming_distance is a perfectly valid value if you want to use it as a lookup
        if let Some(barcodes_to_use) = self.barcodes.as_ref()
            && let Some(Some(barcodes_data)) = parent.barcodes.as_ref()
        {
            if let Some(barcodes_section) = barcodes_data.map.get(barcodes_to_use) {
                if let Some(barcodes_section) = barcodes_section.as_ref()
                    && let Some(seq_to_name) = &barcodes_section.seq_to_name
                    && let Some(max_hamming_distance) = self.max_hamming_distance.as_ref()
                {
                    self.resonator = Some(Arc::new(init_hamming_resonator(
                        seq_to_name,
                        *max_hamming_distance,
                    )?)); // cov:excl-line // we check length before, so this shouldn't fail.
                    self.seq_to_name = Some(seq_to_name.clone());
                    let idx_map: FxHashMap<BString, usize> = seq_to_name
                        .keys()
                        .enumerate()
                        .map(|(i, k)| (k.clone(), i))
                        .collect();
                    self.seq_to_idx = Some(Arc::new(idx_map));
                } //cov:excl-line
            // otherwise the barcode section wasn't ok and we'll never
            // be turned into a concrete HammingCorrect.
            } else {
                self.barcodes.help = Some(offer_alternatives(
                    barcodes_to_use.as_ref(),
                    &barcodes_data.map.keys().collect::<Vec<_>>(),
                ));

                self.barcodes.state = TomlValueState::ValidationFailed {
                    message: "Barcodes section not found".to_string(),
                };
                return Ok(());
            }
        } else if self.barcodes.as_ref().is_some()
            && !matches!(parent.barcodes.value.as_ref(), Some(Some(_)))
        {
            self.barcodes.help = Some(
                "Add a [barcodes.<name>] section in your TOML to define the barcodes.".to_string(),
            );
            self.barcodes.state =
                TomlValueState::new_validation_failed("No barcodes sections defined".to_string());
            return Ok(());
        }

        self.on_tie_threshold.verify(|x| {
            if *x < 0.0 || *x >= 1.0 {
                Err(ValidationFailure::new(
                    "Must be >= 0 <= 1".to_string(),
                    Some("Supply a valid fraction between (0..1)".to_string()),
                ))
            } else {
                Ok(())
            }
        });
        if matches!(self.on_tie.as_ref(), Some(OnTie::ByEditProbability)) {
            self.max_hamming_distance.verify(|dist| {
                if *dist != 1 {
                    Err(ValidationFailure::new(
                        "ByEditProbability requires max_hamming_distance == 1".to_string(),
                        Some("Set max_hamming_distance to 1".to_string()),
                    ))
                } else {
                    Ok(())
                }
            });
        }

        if matches!(
            self.on_tie.as_ref(),
            Some(OnTie::ByMajority | OnTie::ByEditProbability)
        ) {
            let reads_in_flight: usize = parent
                .options
                .as_ref()
                .and_then(|options| options.max_molecules_in_flight.as_ref())
                .copied()
                .unwrap_or_else(|| {
                    fastqrab_config::default_max_molecules_in_flight(
                        parent
                            .options
                            .as_ref()
                            .and_then(|x| x.block_size.as_ref())
                            .copied()
                            .unwrap_or_else(fastqrab_config::default_block_size),
                    )
                })
                .into();
            let reads_wanted = *self
                .on_tie_min_molecules_to_start
                .as_ref()
                .expect("just set above");

            if reads_in_flight < reads_wanted {
                return Err(ValidationFailure::new(
                    "Not enough reads 'in flight' for ByMajority|ByEditProbability".to_string(),
                    Some(format!(
                        "Using on_tie=ByMajority (or ByEditProbability) must first collect enough data. \n\
                    It is configured to require {reads_wanted} molecules.\n\
                    Your options.max_reads_in_flight is only {reads_in_flight} molecules.\n\
                    Increase it.\n\
                    Having a total number of reads below {reads_wanted} is not a problem,\n\
                    ByMajority|ByEditProbability will simply use all reads.",
                    )),
                ));
            }
        }
        self.majority_data = Some(None); //get's overwritten in expand_transformations for ByMajority, empty default otherwise
        self.count_writer = Some(Arc::new(Mutex::new(None)));
        self.pre_match = Some(None); //ditto
        self.final_block_seen = Some(AtomicBool::new(false));

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, JsonSchema)]
#[tpd]
pub enum OnNoMatch {
    Remove,
    Empty,
    Keep,
}

#[derive(Debug, Clone, Copy, JsonSchema)]
#[tpd]
pub enum OnTie {
    Remove,
    Empty,
    Keep,
    First,
    FirstStrict,
    Fail,
    ByMajority, // cell ranger like, 0.975, but we update blockwise, instead of reading everything
    // at once
    ByEditProbability,
}

impl TagUser for PartialTaggedVariant<PartialHammingCorrect> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut() {
            let input_kinds = if let Some(OnTie::ByEditProbability) = inner.on_tie.as_ref() {
                &[TagValueType::Location][..] //needs qualities
            } else {
                &[TagValueType::String, TagValueType::Location][..]
            };
            Some(TagUsageInfo {
                declared_tag: inner
                    .out_label
                    .to_declared_tag(TagValueType::String)
                    .map(|tag| {
                        tag.with_contents(
                            match inner.output.as_ref().unwrap_or(&HammingOutput::Barcode) {
                                HammingOutput::Barcode => StringTagContent::Barcodes,
                                HammingOutput::Label => StringTagContent::Labels,
                            },
                        )
                    }),
                used_tags: vec![inner.in_label.to_used_tag(input_kinds)],
                used_barcodes: inner.barcodes.as_ref().cloned().into_iter().collect(),
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }

    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        if let Some(inner) = self.toml_value.as_ref()
            && inner.on_tie_dump_counts.as_ref().is_some_and(|x| *x)
            && let Some(in_label) = inner.in_label.as_ref()
        {
            return Some(vec![OutputDeclaration {
                id: "counts".to_string(),
                target: WriteTargetConfig::new(
                    vec![format!("{}.counts", in_label)],
                    None,
                    "tsv".to_string(),
                ),
                sink_config: SinkConfig::new_uncompressed_unhashed(),
                format: FileFormat::Text,
                chunk_policy: ChunkPolicy::no_chunks(),
                bam_options: None,
                singleton: true,
                span: inner.on_tie_dump_counts.span(),
            }]);
        }

        None //doesn't count as an output job if unconfigured
    }

    fn declare_input_files(&self) -> Option<Vec<InputDeclaration>> {
        let inner = self.toml_value.as_ref()?;
        if let Some(Some(cfr)) = inner.on_tie_use_counts_from_report.as_ref()
            && let Some(filename) = cfr.filename.as_ref()
        {
            return Some(vec![InputDeclaration {
                id: COUNTS_FROM_REPORT_ID.to_string(),
                path: filename.clone().into(),
            }]);
        }
        None
    }
}

#[derive(Debug)]
pub enum MatchResultOwned {
    NoMatch,
    OneMatch {
        idx: usize,
        was_exact: bool,
    },
    /// Indices of tied candidates, sorted by (dist, seq) so position 0 is the
    /// canonical "first" choice for `OnTie::First`.
    Tie(Vec<usize>),
}

#[derive(Debug)]
pub struct MatchSlot {
    pub result: Option<MatchResultOwned>,
    /// Only filled for `Location` ties when `ByEditProbability` is in effect.
    pub quality: Option<BString>, //todo: Should just store the qual of the altered position
}

/// Shared state between `_HammingPreMatch` and the resolver `HammingCorrect`.
/// `pending` carries match results from the parallel matcher to the serial
/// resolver, keyed by `block_no`. The pipeline's per-step ordering guarantees
/// the resolver picks up the entry the matcher just inserted for that block.
#[derive(Debug)]
pub struct PreMatchData {
    pub seq_to_name: Arc<IndexMap<BString, String>>,
    pub seq_to_idx: Arc<FxHashMap<BString, usize>>,
    pub resonator: Arc<HammingResonator>,
    pub needs_qualities: bool,
    pub pending: Mutex<FxHashMap<usize, Vec<MatchSlot>>>,
}

fn match_sequence(
    seq_to_idx: &FxHashMap<BString, usize>,
    resonator: &HammingResonator,
    sequence: &BStr,
) -> Result<MatchResultOwned> {
    use MatchResultOwned::{NoMatch, OneMatch, Tie};

    if let Some(&idx) = seq_to_idx.get(sequence) {
        return Ok(OneMatch {
            idx,
            was_exact: true,
        });
    }
    let matched = resonator
        .query(sequence)
        .map_err(|e| anyhow::anyhow!("HammingCorrect query failed: {e}"))?;
    if matched.is_empty() {
        return Ok(NoMatch);
    }
    if matched.len() == 1 {
        let seq = matched[0].0;
        let idx = *seq_to_idx
            .get(seq)
            .expect("Resonator returned a sequence not in seq_to_idx");
        return Ok(OneMatch {
            idx,
            was_exact: matched[0].1 == 0,
        });
    }
    let mut indexed: Vec<(usize, &BStr, u32)> = matched
        .iter()
        .map(|(seq, dist)| {
            let idx = *seq_to_idx
                .get(*seq)
                .expect("Resonator returned a sequence not in seq_to_idx");
            (idx, *seq, *dist)
        })
        .collect();
    indexed.sort_by_key(|(_idx, seq, dist)| (*dist, *seq));
    if indexed[0].2 < indexed[1].2 {
        return Ok(OneMatch {
            idx: indexed[0].0,
            was_exact: indexed[0].2 == 0,
        });
    }
    let min_dist = indexed[0].2;
    let cut = indexed
        .iter()
        .position(|(_, _, d)| *d > min_dist) // mutants::skip - we only look at the first right now.
        .unwrap_or(indexed.len());
    indexed.truncate(cut);
    Ok(Tie(indexed.into_iter().map(|(idx, _, _)| idx).collect()))
}

fn run_match_phase(
    seq_to_idx: &FxHashMap<BString, usize>,
    resonator: &HammingResonator,
    input_tags: &TagColumn,
    needs_qualities: bool,
) -> Result<Vec<MatchSlot>> {
    let mut results: Vec<MatchSlot> = Vec::with_capacity(input_tags.len());
    match input_tags {
        TagColumn::Location(col) => {
            for hits in col {
                let result = if hits.0.is_empty() {
                    None
                } else {
                    Some(match_sequence(seq_to_idx, resonator, &hits.0)?)
                };
                results.push(MatchSlot {
                    result,
                    quality: None,
                });
            }
        }
        TagColumn::String(items) => {
            for item in items.iter() {
                let result = match item {
                    Some(bstring) => Some(match_sequence(seq_to_idx, resonator, bstring)?),
                    None => None,
                };
                results.push(MatchSlot {
                    result,
                    quality: None,
                });
            }
        }
        TagColumn::Numeric(_) | TagColumn::Bool(_) => {
            unreachable!("Validation was meant to prevent this situation. Bug?")
        } // cov:excl-line
    }
    if needs_qualities && let TagColumn::Location(col) = input_tags {
        for (hits, slot) in col.iter().zip(results.iter_mut()) {
            slot.quality = Some(hits.1.into_owned());
        }
    }

    Ok(results)
}

impl HammingCorrect {
    #[expect(clippy::unnecessary_wraps, reason = "caller ergonomics")]
    fn output_string(&self, matched_idx: usize, output_barcode: bool) -> Option<BString> {
        let (matched_seq, matched_name) = self
            .seq_to_name
            .get_index(matched_idx)
            .expect("seq_to_name index out of range");
        if output_barcode {
            Some(matched_seq.clone())
        } else {
            Some(matched_name.as_bytes().into())
        }
    }

    #[expect(clippy::unnecessary_wraps, reason = "caller ergonomics")]
    fn output_empty_string() -> Option<BString> {
        Some(Default::default())
    }
}

impl Step for HammingCorrect {
    fn init(
        &mut self,
        _input_info: &InputInfo,
        mut output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        if self.on_tie_dump_counts {
            let mut count_dump_file = output_files.take("counts");
            let writer = count_dump_file
                .remove(&0)
                .expect("tag 0 writer must exist, this is singleton:true");
            *self.count_writer.lock().expect("poisoned") = Some(writer);
        }
        if let Some(counts_from_report) = self.on_tie_use_counts_from_report.as_ref() {
            let file = input_files.take(COUNTS_FROM_REPORT_ID);
            let mj = self
                .majority_data
                .as_ref()
                .expect("ByMajority / counts_from_report means we have .majority");
            mj.load_from_report(file, counts_from_report)?;
        }

        Ok(None)
    }

    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> Result<(FastQBlocksCombined, bool)> {
        let input_tags = block.tags.get(&self.in_label).expect("Input tag not found");
        self.reads_in_this_step
            .fetch_add(input_tags.len(), Ordering::SeqCst);

        if block.is_final {
            self.final_block_seen.store(true, Ordering::SeqCst);
        }

        let (barcode_counts, count_here) =
            if matches!(self.on_tie, OnTie::ByMajority | OnTie::ByEditProbability) {
                let mj = self
                    .majority_data
                    .as_ref()
                    .expect("ByMajority means we have .majority");
                if mj.reads_to_count > 0 {
                    //mutants::skip - should be invisible
                    let (guard, cv) = &*mj.barrier.clone();
                    let _guard = cv.wait_while(
                            guard.lock().map_err(|err| {
                                anyhow!("Mutex poisoned while waiting for majority data to be ready: {err}") // cov:excl-line
                            })?, // cov:excl-line
                            |counting_done| !*counting_done,
                        )
                        .expect("mutex inside condvar poisoned");
                }
                let count_here = block.block_no()
                    > mj.start_counting_in_hamming_at_this_block_no
                        .load(Ordering::Acquire)
                    && self.on_tie_use_counts_from_report.is_none();
                if count_here {
                    mj.total_reads_considered
                        .fetch_add(input_tags.len(), Ordering::SeqCst);
                }
                (Some(&*mj.barcode_counts), count_here)
            } else {
                (None, false)
            };
        let output_barcode = matches!(self.output, HammingOutput::Barcode);
        let needs_qualities = matches!(self.on_tie, OnTie::ByEditProbability);

        // Phase 1: matching. If a `_HammingPreMatch` step ran ahead of us in
        // parallel, retreive its results from the side channel; otherwise match
        // inline (this path is `needs_serial=false` so the pipeline already
        // parallelizes across blocks).
        let results: Vec<MatchSlot> = if let Some(pre) = self.pre_match.as_ref() {
            pre.pending
                .lock()
                .map_err(|e| anyhow!("PreMatch pending mutex poisoned: {e}"))?
                .remove(&block.block_no())
                .expect("PreMatch results missing for this block — pipeline ordering bug")
        } else {
            run_match_phase(
                &self.seq_to_idx,
                &self.resonator,
                input_tags,
                needs_qualities,
            )? //cov:excl-line
        };

        // Phase 2: count updates + output construction (counts-hot).
        // let input_strings: Box<dyn Iterator<Item = Option<Cow<BStr>>>> = match input_tags {
        //     TagColumn::Location(in_col) => {
        //         Box::new(in_col.iter_seq().map(|x| (!x.is_empty()).then(|| x)))
        //     }
        //     TagColumn::String(bstrings) => Box::new(
        //         bstrings
        //             .iter()
        //             .map(|x| x.map(|y| Cow::Borrowed(y.as_ref()))),
        //     ),
        //     _ => unreachable!("Validation should have prevented non-String, non-Location tags"), //cov:excl-line
        // };
        let input_strings = input_tags.iter_stringified();
        let mut output_strings = StringColumnBuilder::new();

        for (read_idx, (input_tag, slot)) in input_strings.zip(results.into_iter()).enumerate() {
            let MatchSlot { result, .. } = slot;
            output_strings.push(match result {
                    None => None,
                    Some(hit) => match hit {
                        MatchResultOwned::NoMatch => match self.on_no_match {
                            OnNoMatch::Remove => None,
                            OnNoMatch::Empty => Self::output_empty_string(),
                            OnNoMatch::Keep => input_tag.map(|x| BString::from(x.to_vec())),
                        },
                        MatchResultOwned::OneMatch { idx, was_exact } => {
                            if was_exact && let Some(counts) = barcode_counts && count_here {
                                counts[idx].fetch_add(1, Ordering::Relaxed);
                            }
                            self.output_string(idx, output_barcode)
                        }
                        MatchResultOwned::Tie(items) => match self.on_tie {
                            OnTie::Remove => None,
                            OnTie::Empty => Self::output_empty_string(),
                            OnTie::Keep => input_tag.map(|x| BString::from(x.to_vec())),
                            OnTie::First => self.output_string(items[0], output_barcode),
                            OnTie::FirstStrict => {
                                let split = self.name_split_character;
                                let canonical_name = |idx: usize| -> &[u8] {
                                    let (_k, name) = self
                                        .seq_to_name
                                        .get_index(idx)
                                        .expect("seq_to_name index out of range");
                                    let bytes = name.as_bytes();
                                    match split {
                                        Some(split_char) => bytes
                                            .splitn(2, |&c| c == split_char)
                                            .next()
                                            .unwrap_or(bytes),
                                        None => bytes,
                                    }
                                };
                                let first = canonical_name(items[0]);
                                let all_the_same =
                                    items.iter().all(|&i| canonical_name(i) == first);
                                if all_the_same {
                                    self.output_string(items[0], output_barcode)
                                } else {
                                    None
                                }
                            }
                            OnTie::Fail => {
                                let query_seq = input_tag
                                    .as_deref()
                                    .map(|s| s.to_vec())
                                    .unwrap_or_default();
                                let display: Vec<_> = items
                                    .iter()
                                    .map(|&i| {
                                        let (k, n) = self
                                            .seq_to_name
                                            .get_index(i)
                                            .expect("seq_to_name index out of range");
                                        (BStr::new(k.as_slice()), n)
                                    })
                                    .collect();
                                bail!(
                                    "HammingCorrect on in_label={} \n\
                                     Uncorrectable sequence '{}', \n\
                                     matches multiple sequences within hamming distance: {:?}.\n\
                                     Set `on_tie` to one of Keep, Remove, Empty, First, or FirstStrict to resolve ties.\n\
                                     If using FirstStrict, consider setting `name_split_character`?",
                                    self.in_label,
                                    BStr::new(&query_seq),
                                    display
                                );
                            }
                            OnTie::ByMajority => {
                                let counts = barcode_counts
                                    .expect("Barcode_counts must be set in OnTie::ByMajority");
                                let mut best: Option<(usize, usize)> = None;
                                let mut total = 0;
                                for &idx in &items {
                                    let count = counts[idx].load(Ordering::Relaxed) + 1;
                                    best = Some(match best {
                                        Some(ibest) => {
                                            if ibest.1 < count { (idx, count) } else { ibest }
                                        }
                                        None => (idx, count),
                                    });
                                    total += count;
                                }
                                let best = best.expect("Items can't have been empty");
                                #[expect(clippy::cast_precision_loss, reason = "acceptable")]
                                if best.1 as f64 / total as f64 >= self.on_tie_threshold {
                                    self.output_string(best.0, output_barcode)
                                } else {
                                    None
                                }
                            }
                            OnTie::ByEditProbability => {
                                let counts = barcode_counts.expect("Barcode_counts must be set in OnTie::ByEditProbability");
                                let candidates: Vec<_> = items
                                    .iter()
                                    .map(|&i| {
                                        let (k, _n) = self
                                            .seq_to_name
                                            .get_index(i)
                                            .expect("seq_to_name index out of range");
                                        (
                                            BStr::new(k.as_slice()),
                                            i,
                                            counts[i].load(Ordering::Relaxed),
                                        )
                                    })
                                    .collect();
                                let candidates_for_likelihood: Vec<(&BStr, usize)> =
                                    candidates
                                        .iter()
                                        .map(|(seq, _idx, c)| (*seq, *c))
                                        .collect();
                                if let Some(input_tag) = input_tag && !input_tag.is_empty() {
                                    let (observed_sequence, observed_qualities) =
                                    (
                                        input_tag,
                                        input_tags
                                            .as_locations()
                                            .expect("By Edit Probability not supporte on string tags. Validation should have cought this")
                                        .joined_qual(read_idx, None)
                                    );

                                    if let Some(best_seq) =
                                        correct_barcode_via_base_editing_likelihood(
                                            self.on_tie_threshold,
                                            &observed_sequence,
                                            &observed_qualities,
                                            &candidates_for_likelihood,
                                        )
                                        {
                                            let best_idx = candidates
                                                .iter()
                                                .find(|(s, _, _)| *s == best_seq)
                                                .map(|(_, i, _)| *i)
                                                .expect("best_seq came from candidates");
                                            self.output_string(
                                                best_idx,
                                                output_barcode,
                                            )
                                        } else {
                                            Self::output_empty_string()
                                        }
                                } else {
                                    None // cov:excl-line
                                }
                            }
                        },
                    },
                }.map(Cow::Owned));
        }
        block.tags.insert(
            self.out_label.clone(),
            TagColumn::String(output_strings.finish()),
        );
        // Add the corrected tags to the output

        Ok((block, true))
    }

    #[mutants::skip] //always true is safe.
    fn needs_serial(&self) -> bool {
        self.majority_data.is_some()
    }

    fn finalize(&self, _demultiplex_info: &OptDemultiplex) -> Result<Option<FinalizeReportResult>> {
        if let Some(mj) = self.majority_data.as_ref() {
            if self.on_tie_use_counts_from_report.is_none() {
                assert_eq!(
                    mj.total_reads_considered.load(Ordering::Acquire),
                    self.reads_in_this_step.load(Ordering::Acquire),
                    "Mismatch between OnTie::ByMajority considered reads and total reads in this step - bug in your count_here decision making"
                );
            } else {
                let total = mj.total_reads_considered.load(Ordering::Acquire);
                assert_eq!(
                    total, 0,
                    "In on_tie_use_counts_from_report mode, no reads should have been counted. But {total} were counted",
                );
            }
            if let Some(mut writer) = self.count_writer.lock().expect("Mutex poisoned").take() {
                let barcode_counts = mj.seq_to_name.keys().zip(mj.barcode_counts.iter());
                let mut records = barcode_counts
                    .map(|(seq, count)| format!("{}\t{}\n", seq, count.load(Ordering::Relaxed)))
                    .collect::<Vec<_>>();
                records.sort(); //sort by sequence for easier diffing between runs
                writer.write_text_record(b"Barcode\tCount\n")?;
                for record in records {
                    writer.write_text_record(record.as_bytes())?;
                }
                let () = writer.finish()?;
            }
            //assert that pre_match is empty
            if let Some(pre) = self.pre_match.as_ref()
                && self.final_block_seen.load(Ordering::Acquire)
            //we only check if we saw a final block,
            //something else paniced otherwise
            {
                let pending = pre.pending.lock().expect("Mutex poisoned");
                assert!(
                    pending.is_empty(),
                    "PreMatch pending should be empty at finalize, but has entries for blocks: {:?}",
                    pending.keys() //cov:excl-line
                );
            } // cov:excl-line
        }
        Ok(None)
    }
}

/// Parallel matcher half of the split `HammingCorrect` pipeline. Created in
/// `expand_transformations` for `OnTie::ByMajority | OnTie::ByEditProbability`
/// and inserted between `_HammingExactCounter` and `HammingCorrect`.
///
/// Runs phase 1 (the resonator-hot lookup) without `needs_serial` so the
/// pipeline can fan it across worker threads. Results land in `shared.pending`
/// keyed by `block_no`; the downstream serial `HammingCorrect` step pops them.
#[tpd(no_verify)]
#[derive(JsonSchema, Debug, Clone)]
pub struct _HammingPreMatch {
    in_label: TagLabel,

    #[tpd(skip)]
    #[schemars(skip)]
    pub shared: Arc<PreMatchData>,
}

impl Partial_HammingPreMatch {
    pub(crate) fn new(in_label: TagLabel, shared: Arc<PreMatchData>) -> Self {
        Self {
            in_label: TomlValue::new_ok_unplaced(in_label),
            shared: Some(shared),
        }
    }
}

impl TagUser for PartialTaggedVariant<Partial_HammingPreMatch> {
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
                        .to_used_tag(&[TagValueType::String, TagValueType::Location]),
                ],
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

impl Step for _HammingPreMatch {
    fn apply(
        &self,
        block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> Result<(FastQBlocksCombined, bool)> {
        let input_tags = block.tags.get(&self.in_label).expect("Input tag not found");
        let results = run_match_phase(
            &self.shared.seq_to_idx,
            &self.shared.resonator,
            input_tags,
            self.shared.needs_qualities,
        )?; //cov:excl-line
        let block_no = block.block_no();
        self.shared
            .pending
            .lock()
            .map_err(|e| anyhow!("PreMatch pending mutex poisoned: {e}"))?
            .insert(block_no, results);
        Ok((block, true))
    }
}

/// Compute the posterior over Hamming-1 'known' neighbors of `observed`,
/// weighting each by the per-base error probability at the differing position
/// and a Laplace-smoothed prior count. Returns the corrected barcode if its
/// posterior mass exceeds `p_threshold`, otherwise `None` (ambiguous).
///
/// `qual` is Illumina Phred+33 (clamped to Q66, matching `BC_MAX_QV`).
/// `candidates` must each be at Hamming distance exactly 1 from `observed`
/// and have the same length.
pub fn correct_barcode_via_base_editing_likelihood<'a>(
    p_threshold: f64,
    observed: &BStr,
    qual: &[u8],
    candidates: &'a [(&'a BStr, usize)],
) -> Option<&'a BStr> {
    debug_assert_eq!(observed.len(), qual.len());

    let mut total = 0.0_f64;
    let mut best: Option<(f64, &'a BStr)> = None;

    for &(cand, raw_count) in candidates {
        debug_assert_eq!(cand.len(), observed.len());

        let diff_pos = observed
            .iter()
            .zip(cand.iter())
            .position(|(a, b)| a != b)
            .expect("Candidates must be at 1 hamming distance from observed sequence.");

        let qv = qual[diff_pos].min(66); // we clamp it here to prevent sequencer overconfidence /
        // actually let the prior from the barcode count count.
        let phred_edit_probability = 10f64.powf(-f64::from(qv.saturating_sub(33)) / 10.0);
        #[expect(
            clippy::cast_precision_loss,
            reason = "If counts reach f64 imprecison region, precision loss would be acceptable"
        )]
        let likelihood = phred_edit_probability * (1 + raw_count) as f64;

        total += likelihood;

        // Tiebreak on likelihood ties by lex-greater sequence
        // hence tuple comparison.
        let tup = Some((likelihood, cand));
        if tup > best {
            //mutants::skip - we keep the first
            best = tup;
        }
    }

    let (best_like, best_seq) = best?;
    (best_like / total >= p_threshold).then_some(best_seq)
}

#[cfg(test)]
mod test_correct_barcode_via_base_editing_likelihood {
    use super::correct_barcode_via_base_editing_likelihood;
    use bstr::BStr;

    fn b(s: &[u8]) -> &BStr {
        BStr::new(s)
    }

    #[test]
    fn single_candidate_passes_threshold() {
        // With one candidate, mass = 1.0, regardless of quality / count.
        let observed = b(b"ACGT");
        let qual = b"IIII"; // Q40
        let candidates = [(b(b"ACGA"), 0usize)];
        let res = correct_barcode_via_base_editing_likelihood(0.5, observed, qual, &candidates);
        assert_eq!(res, Some(b(b"ACGA")));
    }

    #[test]
    fn no_candidates_returns_none() {
        let observed = b(b"ACGT");
        let qual = b"IIII";
        let candidates: [(&BStr, usize); 0] = [];
        let res = correct_barcode_via_base_editing_likelihood(0.5, observed, qual, &candidates);
        assert_eq!(res, None);
    }

    #[test]
    fn picks_lower_quality_position_when_counts_equal() {
        // Differing positions get different qualities.
        // Higher edit-probability (lower Q) wins when counts match.
        let observed = b(b"AAAAAAAA");
        // pos 0 = '!' (Q0, p_edit = 1.0); pos 4 = 'I' (Q40, p_edit = 1e-4).
        let qual: &[u8] = b"!bIbIbIb";
        // cand1 differs at pos 0 (low quality -> high p_edit -> wins)
        // cand2 differs at pos 4 (high quality -> low p_edit -> loses)
        let candidates = [(b(b"TAAAAAAA"), 0usize), (b(b"AAAATAAA"), 0usize)];
        let res = correct_barcode_via_base_editing_likelihood(0.5, observed, qual, &candidates);
        assert_eq!(res, Some(b(b"TAAAAAAA")));
    }

    #[test]
    fn higher_count_wins_when_qualities_equal() {
        let observed = b(b"AAAAAAAA");
        let qual: &[u8] = &[b'I'; 8];
        // Both differ at pos 0; counts decide.
        let candidates = [(b(b"TAAAAAAA"), 1usize), (b(b"CAAAAAAA"), 99usize)];
        let res = correct_barcode_via_base_editing_likelihood(0.5, observed, qual, &candidates);
        assert_eq!(res, Some(b(b"CAAAAAAA")));
    }

    #[test]
    fn ambiguous_returns_none_below_threshold() {
        // Two equal candidates -> each gets 0.5 of the mass; 0.99 threshold not reached.
        let observed = b(b"AAAAAAAA");
        let qual: &[u8] = b"!III!III";
        // Both diff positions are Q0 -> p_edit = 1, counts equal -> 50/50 split.
        let candidates = [(b(b"TAAAAAAA"), 5usize), (b(b"AAAATAAA"), 5usize)];
        let res = correct_barcode_via_base_editing_likelihood(0.99, observed, qual, &candidates);
        assert_eq!(res, None);
        //but if both of them reach the threshold, we take the lexicographic first one
        let res = correct_barcode_via_base_editing_likelihood(0.50, observed, qual, &candidates);
        assert_eq!(res, Some(b(b"TAAAAAAA")));
    }

    #[test]
    fn quality_clamped_to_q66() {
        // Quality bytes above Q66 (b'!' + 66 = 99 = 'c') get clamped to Q66.
        // Two candidates at the same diff position, with extremely high quality,
        // should still be distinguishable by count.
        let observed = b(b"AAAA");
        let qual: &[u8] = b"~~~~"; // '~' = 126, beyond Q66
        let candidates = [(b(b"TAAA"), 0usize), (b(b"CAAA"), 10usize)];
        let res = correct_barcode_via_base_editing_likelihood(0.5, observed, qual, &candidates);
        assert_eq!(res, Some(b(b"CAAA")));
    }
}
