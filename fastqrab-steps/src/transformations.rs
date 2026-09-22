#![expect(non_camel_case_types, reason = "Derived Partial_* is ok for internals")]

use anyhow::Result;
use enum_dispatch::enum_dispatch;
use indexmap::IndexMap;
use prelude::TagMetadata;
use rand::SeedableRng;
use scalable_cuckoo_filter::ScalableCuckooFilter;
use schemars::JsonSchema;
use toml_pretty_deser::prelude::*;

use crate::{
    config::ThreadingConfiguration,
    demultiplex::{DemultiplexBarcodes, OptDemultiplex, StepOutputFiles},
    input_files::{InputDeclaration, StepInputFiles},
};
use fastqrab_config::{DeclaredTag, RemovedTags, TagLabel, UsedTag, segments::SegmentIndex};
use fastqrab_io::io::FastQBlocksCombined;
use fastqrab_io::io::output::chunked_writer::OutputDeclaration;
use std::collections::HashSet;
use std::sync::Arc;

pub(crate) mod calc;
pub(crate) mod convert;
pub(crate) mod debug;
pub(crate) mod demultiplex;
pub(crate) mod edits;
pub(crate) mod extract;
pub(crate) mod filters;
pub(crate) mod hamming_correct;
pub(crate) mod hamming_exact_counter;
mod internal_steps;
pub(crate) mod output;
pub(crate) mod prelude;
pub(crate) mod reports;
pub(crate) mod tag;
pub(crate) mod validation;
pub use internal_steps::{
    _InduceFailure, _InternalDelay, _InternalReadCount, Partial_InduceFailure,
    Partial_InternalDelay, Partial_InternalReadCount,
};

#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct RegionDefinition {
    pub start: isize,
    #[tpd(alias = "len")]
    #[tpd(alias = "count")]
    #[tpd(alias = "n")]
    pub length: usize,

    /// Whether the start position is anchored to the start (default) or end of the region
    pub anchor: RegionAnchor,
}

impl<R> VerifyIn<R> for PartialRegionDefinition {
    fn verify(
        &mut self,
        _parent: &R,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        Ok(())
    }
}

#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub enum RegionAnchor {
    #[tpd(alias = "left")]
    Start,
    #[tpd(alias = "right")]
    End,
}

/// turn a u64 seed into a 32 byte seed for chacha
fn extend_seed(seed: u64) -> [u8; 32] {
    let seed_bytes = seed.to_le_bytes();

    // Extend the seed_bytes to 32 bytes
    let mut extended_seed = [0u8; 32];
    extended_seed[..8].copy_from_slice(&seed_bytes);
    extended_seed
}

pub(crate) fn reproducible_cuckoofilter<T: std::hash::Hash + ?Sized>(
    seed: u64,
    initial_capacity: usize,
    false_positive_probability: f64,
) -> ScalableCuckooFilter<T, scalable_cuckoo_filter::DefaultHasher, rand_chacha::ChaChaRng> {
    let rng = rand_chacha::ChaChaRng::from_seed(extend_seed(seed));
    scalable_cuckoo_filter::ScalableCuckooFilterBuilder::new()
        .initial_capacity(initial_capacity)
        .false_positive_probability(false_positive_probability)
        .rng(rng)
        .finish()
}

pub(crate) type OurCuckCooFilter<T> = scalable_cuckoo_filter::ScalableCuckooFilter<
    T,
    scalable_cuckoo_filter::DefaultHasher,
    rand_chacha::ChaChaRng,
>;

#[derive(Hash, Debug)]
pub struct FragmentEntry<'a>(&'a [&'a [u8]]);

#[derive(Hash, Debug)]
pub struct FragmentEntryForCuckooFilter(FragmentEntry<'static>);

// cov:excl-start
// not sure why coverage isn't seeing it, we need it though to compile.
impl<'a> std::borrow::Borrow<FragmentEntry<'a>> for FragmentEntryForCuckooFilter {
    fn borrow(&self) -> &FragmentEntry<'a> {
        &self.0
    }
}
// cov:excl-stop

impl FragmentEntry<'_> {
    fn to_continuous_vec(&self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        for v in self.0 {
            res.extend(*v);
        }
        res
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, JsonSchema)]
#[tpd]
pub enum KeepOrRemove {
    #[tpd(alias = "keep")]
    Keep,
    #[tpd(alias = "remove")]
    Remove,
}

#[derive(Debug)]
pub struct FinalizeReportResult {
    pub report_no: usize,
    pub contents: serde_json::Value,
}

#[derive(Debug, Clone)]
pub struct InputInfo {
    pub segment_order: Vec<String>,
    pub barcodes_data: IndexMap<TagLabel, crate::config::Barcodes>,
    pub comment_insert_char: u8,
    pub initial_filter_capacity: Option<usize>,
    pub threading_configuration: ThreadingConfiguration,
    /// Live gauge of blocks currently inside the work pool (stalled + being
    /// worked on), maintained by the `WorkpoolCoordinator`. Shared so the
    /// `Progress` step can report pipeline occupancy alongside throughput,
    /// to tell a real stage stall (gauge drains) from a measurement artifact
    /// (gauge stays pinned at capacity while the `max_reads_in_flight` budget
    /// is saturated).
    pub blocks_in_flight: Arc<std::sync::atomic::AtomicUsize>,
    /// Report metadata, used by the `OutputReport` step to build the combined
    /// report (label-per-`report_no`, input-file config and the raw config TOML
    /// the legacy `[output]` renderer embedded). Shared so cloning `InputInfo`
    /// into every worker stays cheap.
    pub report_metadata: Arc<ReportMetadata>,
}

/// Pipeline-level data the combined run report embeds. Collected once during
/// config check and handed to the `OutputReport` step via [`InputInfo`].
#[derive(Debug, Default)]
pub struct ReportMetadata {
    /// Maps a report's `report_no` to the label it should be keyed under, and
    /// defines the `report_order` array. Indexed by `report_no`.
    pub report_labels: Vec<String>,
    /// The input-file configuration as a compact JSON string, embedded as the
    /// report's `__.input_files`. Kept as a single small allocation (rather than
    /// a `serde_json::Value` tree) so it does not duplicate per-input-file data
    /// in memory during processing; parsed back to a value at report time.
    pub input_files: Arc<str>,
    /// The raw config TOML, embedded as `run_info.input_toml`. Shared (rather
    /// than cloned) so it is not duplicated in memory during processing.
    pub raw_config: Arc<str>,
}

#[derive(Default, Debug)]
pub struct TagUsageInfo<'a> {
    pub used_tags: Vec<Option<UsedTag<'a>>>, //an option as convenience to the declaring steps
    pub used_barcodes: HashSet<TagLabel>,
    pub removed_tags: RemovedTags<'a>,
    pub declared_tag: Option<DeclaredTag<'a>>,
    pub must_see_all_tags: bool,
}

#[enum_dispatch(PartialTransformation)]
pub trait TagUser {
    // not part of step since it needs to be done on the pre-verified transformations
    #[mutants::skip] // this *is* Default::default()
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        Some(TagUsageInfo::default())
    }

    // we can not do this in the regular verify,
    // for at that point, .transform is still NotSet
    fn verify_others(
        &mut self,
        _input_def: Option<&crate::config::PartialInput>,
        _output_def: Option<&crate::config::PartialOutput>,
        _transformations_before_this_one: &[TomlValue<PartialTransformation>],
    ) {
    }

    /// Return declarations for each output file this step would create.
    /// Called at config-verification time to detect filename conflicts.
    /// Each declaration's `span` should point at the config field most
    /// responsible for the filename (e.g. `infix`, `in_label`), falling
    /// back to the step's own top-level span when no better field exists.
    /// Steps that produce side-output files override this; the default
    /// returns an empty vec.
    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        None
    }

    /// Return declarations for each auxiliary input file this step reads,
    /// besides the pipeline `[input]` reads. Called at config-verification
    /// time so the full set of out-of-tree reads is known before processing;
    /// the runtime opens each declared path and hands the handles to
    /// `Step::init` via [`StepInputFiles`]. Mirrors `declare_output_files`.
    /// Steps that read side-input files override this; the default returns
    /// `None`.
    #[mutants::skip]
    fn declare_input_files(&self) -> Option<Vec<InputDeclaration>> {
        None
    }
}

#[enum_dispatch(Transformation)]
pub trait Step {
    fn store_progress_output(&mut self, _progress: &crate::transformations::reports::Progress) {
        //default does nothing
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

    /// finish up what you were doing
    fn finalize(&self, _demultiplex_info: &OptDemultiplex) -> Result<Option<FinalizeReportResult>> {
        Ok(None)
    }

    /// called after all `finalize` have happened - for progress to report and
    /// for output steps to emit aggregate artifacts.
    ///
    /// `reports` is the full set of reports collected from every step's
    /// `finalize`; passing it here (rather than sharing the collector `Arc`
    /// widely) lets steps like `OutputReport` write a combined report without
    /// needing access to the pipeline-level collector.
    fn post_finalize(&self, _reports: &[FinalizeReportResult]) -> Result<()> {
        Ok(())
    }

    /// handle one block
    fn apply(
        &self,
        block: FastQBlocksCombined,
        input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)>;

    /// does this transformation need to see all reads, or is it fine to run it in multiple
    /// threads in parallel?
    #[mutants::skip] // since setting this to true will not lead to an error, just longer runtime
    fn needs_serial(&self) -> bool {
        false
    }

    /// When we have a transformation that says 'Enough reads'
    /// like Head, that sends the 'end transmission' signal
    /// upstream by closing it's receiver.
    /// Then the next upstream stage detects that, and can
    /// close it's receiver in turn (and ending it's processing).
    /// Except if this returns false,
    /// because then we stop the breakage here,
    /// accepting all incoming reads and discarding them after processing
    fn transmits_premature_termination(&self) -> bool {
        true
    }
}

#[enum_dispatch]
#[tpd(tag = "action", further_attr = "enum_dispatch")]
#[derive(Debug, JsonSchema)]
// tpd deserializes this enum internally tagged on `action`; mirror that in the
// schema so steps validate as `{ action = "...", ... }` rather than the
// externally-tagged `{ VariantName = { ... } }` schemars emits by default.
#[schemars(tag = "action")]
pub enum Transformation {
    //Edits
    CutStart(edits::CutStart),
    CutEnd(edits::CutEnd),
    Truncate(edits::Truncate),
    Prefix(edits::Prefix),
    Postfix(edits::Postfix),
    ConvertQuality(edits::ConvertQuality),
    ReverseComplement(edits::ReverseComplement),
    Rename(edits::Rename),
    Swap(edits::Swap),
    Lowercase(edits::Lowercase),
    Uppercase(edits::Uppercase),
    #[tpd(skip)]
    #[schemars(skip)]
    _ChangeCase(edits::_ChangeCase), // public interface is Lowercase/Uppercase
    TrimAtTag(edits::TrimAtTag),
    MergeReads(edits::MergeReads),
    //
    #[tpd(alias = "TagMatches")]
    Matches(filters::TagMatches),
    FilterByTag(filters::ByTag),
    FilterByNumericTag(filters::ByNumericTag),
    //
    // //Filters
    Head(filters::Head),
    Skip(filters::Skip),
    FilterEmpty(filters::Empty),
    #[tpd(alias = "sample")]
    FilterSample(filters::Sample),
    #[tpd(alias = "reservoirsample")]
    FilterReservoirSample(filters::ReservoirSample),
    // //
    // //Validation
    #[tpd(alias = "SpotCheckReadNames")]
    ValidateReadPairing(validation::ValidateReadPairing),
    #[tpd(alias = "ValidateSequence")]
    ValidateSeq(validation::ValidateSeq),
    #[tpd(alias = "ValidateQual")]
    ValidateQuality(validation::ValidateQuality),
    ValidateName(validation::ValidateName),
    ValidateAllReadsSameLength(validation::ValidateAllReadsSameLength),
    ValidateReadNamesPrintable(validation::ValidateReadNamesPrintable),
    //
    // // tag based stuff
    ExtractIUPAC(extract::IUPAC),
    ExtractIUPACWithIndel(extract::IUPACWithIndel),
    ExtractRegex(extract::Regex),
    ExtractRegion(extract::Region), //gets converted into ExtractRegions
    ExtractRegions(extract::Regions),
    CalcLength(calc::Length),
    CalcBaseContent(calc::BaseContent),
    #[tpd(alias = "CalcGCCount")]
    CalcGCContent(calc::GCContent),
    #[tpd(alias = "CalcNCount")]
    CalcNContent(calc::NContent),
    CalcComplexity(calc::Complexity),
    CalcQualifiedBases(calc::QualifiedBases),
    CalcExpectedError(calc::ExpectedError),
    CalcWorstQuality(calc::WorstQuality),
    CalcKmers(calc::Kmers),
    ConvertToRate(calc::ConvertToRate),
    //
    ConvertRegionsToLength(convert::RegionsToLength),
    #[tpd(alias = "EvaluateExpression")]
    EvalExpression(Box<convert::EvalExpression>),
    CompareStringTags(convert::CompareStringTags),
    ExtractRegionsOfLowQuality(extract::RegionsOfLowQuality),
    ExtractLongestPolyX(extract::LongestPolyX),
    ExtractPolyTail(extract::PolyTail),
    ExtractIUPACSuffix(extract::IUPACSuffix),
    ExtractLowQualityStart(extract::LowQualityStart),
    ExtractLowQualityEnd(extract::LowQualityEnd),
    // // bool tags
    TagDuplicates(extract::tag::Duplicates),
    TagOtherFile(extract::tag::OtherFile),
    //
    // //edit
    StoreTagBackInSequence(tag::StoreTagBackInSequence),
    StoreTagInSequence(tag::StoreTagInSequence),
    ReplaceTagWithLetter(tag::ReplaceTagWithLetter),
    //
    // //store
    ConcatTags(tag::ConcatTags),
    FillMissing(tag::FillMissing),
    PickTag(tag::PickTag),
    ForgetAllTags(tag::ForgetAllTags),
    ForgetTag(tag::ForgetTag),
    #[tpd(alias = "StoreTagsInComment")]
    StoreTagInComment(tag::StoreTagInComment),
    #[tpd(alias = "StoreTagsInFastQ")]
    StoreTagInFastQ(tag::StoreTagInFastQ),
    #[tpd(alias = "StoreTagInTable")]
    #[tpd(alias = "OutputTable")]
    #[tpd(alias = "OutputTSV")]
    StoreTagsInTable(tag::StoreTagsInTable),
    StoreSingleCellMatrix(tag::StoreSingleCellMatrix),
    // //other
    QuantifyTag(tag::QuantifyTag),
    //
    Progress(reports::Progress),
    Report(reports::Report),
    #[schemars(skip)]
    #[tpd(skip)]
    _ReportCount(Box<reports::_ReportCount>),
    #[schemars(skip)]
    _ReportLengthDistribution(Box<reports::_ReportLengthDistribution>),
    #[schemars(skip)]
    #[tpd(skip)]
    _ReportDuplicateCount(Box<reports::_ReportDuplicateCount>),
    #[schemars(skip)]
    #[tpd(skip)]
    _ReportDuplicateFragmentCount(Box<reports::_ReportDuplicateFragmentCount>),
    #[schemars(skip)] // nodefault
    #[tpd(skip)]
    _ReportBaseStatisticsPart1(Box<reports::_ReportBaseStatisticsPart1>),
    #[schemars(skip)] // nodefault
    #[tpd(skip)]
    _ReportBaseStatisticsPart2(Box<reports::_ReportBaseStatisticsPart2>),
    #[schemars(skip)] // nodefault
    #[tpd(skip)]
    _ReportCountOligos(Box<reports::_ReportCountOligos>),
    #[schemars(skip)]
    #[tpd(skip)]
    _ReportTagHistogram(Box<reports::_ReportTagHistogram>),
    //
    Inspect(reports::Inspect),
    //
    // Output steps
    #[tpd(alias = "OutputFastq")]
    OutputFASTQ(output::OutputFASTQ),
    #[tpd(alias = "OutputFasta")]
    OutputFASTA(output::OutputFASTA),
    #[tpd(alias = "OutputBam")]
    OutputBAM(output::OutputBAM),
    OutputReport(output::OutputReport),
    //
    Demultiplex(demultiplex::Demultiplex),
    HammingCorrect(hamming_correct::HammingCorrect),
    _HammingExactCounter(hamming_exact_counter::HammingExactCounter),
    #[schemars(skip)]
    #[tpd(skip)]
    _HammingPreMatch(hamming_correct::_HammingPreMatch),
    AssignByHalves(tag::AssignByHalves),
    //
    #[schemars(skip)]
    _InternalDelay(Box<_InternalDelay>),
    #[schemars(skip)]
    _InternalReadCount(Box<_InternalReadCount>),
    //
    #[schemars(skip)]
    _InduceFailure(Box<_InduceFailure>),

    #[schemars(skip)]
    DebugBlockSizes(debug::BlockSizes),
    #[schemars(skip)]
    _InducePanic(debug::InducedPanic),
}

#[derive(Debug)]
pub struct Coords {
    pub segment_index: SegmentIndex,
    pub start: usize,
    pub length: usize,
}

#[expect(clippy::cast_sign_loss, reason = "Checked before")]
fn extract_from_sequence(
    seq_len: usize,
    sub_sequence_start: usize,
    sub_sequence_end: usize,
    out_start: isize,
    out_length: usize,
    anchor: &RegionAnchor,
) -> Option<(u32, u32)> {
    assert!(
        sub_sequence_start == 0,
        "sub_sequence_start != 0 was never tested and needs verification"
    );
    // let sub_sequence_start: isize = sub_sequence_start
    //     .try_into()
    //     .expect("sub_sequence_start did not fit into isize");
    let sub_sequence_end: isize = sub_sequence_end
        .try_into()
        .expect("sub_sequence_end did not fit into isize");

    // Calculate the actual start position
    let actual_start: isize = match anchor {
        RegionAnchor::Start => {
            // For start anchoring, negative values count from the beginning
            (out_start) // + sub_sequence_start)
                .min(seq_len.try_into().expect("seq_len did not fit into isize"))
        }
        RegionAnchor::End => {
            // For end anchoring, negative values count from the end
            sub_sequence_end + out_start
        }
    };
    if actual_start < 0 {
        return None;
    }
    let actual_start = actual_start as usize; // verified to be >= 0

    if actual_start >= seq_len {
        None
    } else {
        // Ensure we don't go beyond sequence bounds
        let end_pos = actual_start
            .checked_add(out_length)
            .expect("end_pos overflowed usize");
        if end_pos > seq_len {
            return None;
        }
        let length = end_pos - actual_start;
        Some((
            actual_start
                .try_into()
                .expect("actual_start did not fit into u32"),
            length.try_into().expect("length did not fit into u32"),
        ))
    }
}

#[must_use]
pub fn read_name_canonical_prefix(name: &[u8], readname_end_char: Option<u8>) -> &[u8] {
    if let Some(separator) = readname_end_char {
        if let Some(position) = memchr::memchr(separator, name) {
            &name[..position]
        } else {
            name
        }
    } else {
        name
    }
}

#[must_use]
pub fn read_name_canonical_prefix_strict(
    name: &[u8],
    readname_end_char: Option<u8>,
) -> Option<&[u8]> {
    if let Some(separator) = readname_end_char {
        if let Some(position) = memchr::memchr(separator, name) {
            Some(&name[..position])
        } else {
            None
        }
    } else {
        Some(name)
    }
}

#[cfg(test)]
mod tests {
    use super::{read_name_canonical_prefix, read_name_canonical_prefix_strict};

    #[test]
    fn canonical_prefix_stops_at_first_separator() {
        assert_eq!(
            read_name_canonical_prefix(b"Sample_1_2", Some(b'_')),
            b"Sample"
        );
    }

    #[test]
    fn canonical_prefix_uses_full_name_when_separator_missing() {
        assert_eq!(read_name_canonical_prefix(b"Sample", None), b"Sample");
    }

    #[test]
    fn custom_separator_is_respected() {
        assert_eq!(read_name_canonical_prefix(b"Run/42", Some(b'/')), b"Run");
    }

    #[test]
    fn missing_separator_configuration_defaults_to_exact_match() {
        assert_eq!(read_name_canonical_prefix(b"Exact", None), b"Exact");
    }

    #[test]
    fn canonical_prefix_stops_at_first_separator_strict() {
        assert_eq!(
            read_name_canonical_prefix_strict(b"Sample_1_2", Some(b'_')),
            Some(&b"Sample"[..])
        );
    }

    #[test]
    fn canonical_prefix_uses_full_name_when_separator_missing_strict() {
        assert_eq!(
            read_name_canonical_prefix_strict(b"Sample", None),
            Some(&b"Sample"[..])
        );
    }

    #[test]
    fn custom_separator_is_respected_strict() {
        assert_eq!(
            read_name_canonical_prefix_strict(b"Run/42", Some(b'/')),
            Some(&b"Run"[..])
        );
    }

    #[test]
    fn missing_separator_configuration_defaults_to_exact_match_strict() {
        assert_eq!(
            read_name_canonical_prefix_strict(b"Exact", None),
            Some(&b"Exact"[..])
        );
    }
}
