use crate::transformations::prelude::Result;
use crate::transformations::prelude::*; // union_find_rs::prelude also exports a Result
use crate::verify_opt_path_component;

use bstr::{BStr, BString};
use fastqrab_io::{CompressionFormat, FileFormat};
use rayon::prelude::*;
use rustc_hash::FxHashSet;

type WriterHandle = Arc<Mutex<Option<ChunkedRecordWriter>>>;
type DataHandles = Arc<Mutex<DemultiplexedData<Option<ChunkedRecordWriter>>>>;

mod cellranger_like;
mod cluster;
mod helpers;

use cellranger_like::aggregate_to_matrix_cellranger_like;
use cluster::aggregate_to_matrix_cluster;
use helpers::{encode_umi, finish_writer, human_fmt_usize, take_singleton_writer, write_matrix};
//
//we need to keep these straight.

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Copy)]
struct CellIdx(u32);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Copy)]
struct GeneIdx(u32);

#[derive(Debug, Clone, Hash, Eq, PartialEq, PartialOrd, Ord, Copy)]
struct Umi(u32);

impl Umi {
    fn new_n() -> Self {
        Self(u32::MAX)
    }

    fn new_unmatched() -> Self {
        Self(0)
    }

    fn is_n(self) -> bool {
        self.0 == u32::MAX
    }

    #[expect(clippy::unreadable_literal, reason = "they're repeating")]
    #[mutants::skip] // all false positive
    fn is_homopolymer(self, umi_length: u16) -> bool {
        assert!(umi_length <= 16, "Max umi length exceeded");
        let x = self.0;
        let bits = u32::from(umi_length) * 2;
        //make sure we have no bits set above umi_length, so we detect if the umis are longer than
        //the length at least...
        assert!(
            !(umi_length < 16 && x & (u32::MAX << bits) != 0 && !self.is_n()),
            "Umi.his_homopolymer: bits set above umi_length"
        );
        let shift = 32 - bits;
        x == 0 // all A
            || x == 0b01010101010101010101010101010101 >> shift // all C 01
            || x == 0b10101010101010101010101010101010 >> shift // all G 10
            || x == 0b11111111111111111111111111111111 >> shift // all T 11
    }
}

impl GeneIdx {
    fn is_unmatched(self) -> bool {
        self.0 == 0
    }
}

impl CellIdx {
    fn is_unmatched(self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug)]
struct ObservedEvent {
    cell: CellIdx,
    gene: GeneIdx,
    umi: Umi,
}

#[derive(Debug, Clone, Copy)]
enum LookupMode {
    Barcode,
    Label,
}

#[derive(Debug, JsonSchema, Copy, Clone)]
#[tpd]
enum UMIAggregation {
    None,
    Exact,
    #[tpd(alias = "1MM")]
    Cluster,
    #[tpd(alias = "cellranger")]
    CellRangerLike,
}
/// Collect (`gene_idx`, `cell_idx`, `umi_2bit`) triples per read, then write binary
/// data file(s) and lookup tables on finalize.
///
/// Both cell and gene barcodes are looked up by sequence in [barcodes.*] tables.
/// Index 0 is reserved for "unmatched"; real barcodes are 1-indexed.
/// Output: one binary file per demultiplex group + shared lookup .txt files.
#[derive(JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct StoreSingleCellMatrix {
    /// Tag carrying the cell-barcode sequence (Location or String)
    cell_tag: TagLabel,

    /// Tag carrying the gene-barcode sequence (Location or String)
    gene_tag: TagLabel,

    /// Tag carrying the raw UMI sequence (Location or String)
    umi_tag: TagLabel,

    // how to aggregate UMIs into the matrix
    umi_aggregation: UMIAggregation,

    /// [barcode.*] sec listing all valid cell barcodes (sequence → name)
    cell_barcodes: TagLabel,

    /// [barcode.*] section listing all valid gene identifiers (sequence → name)
    gene_barcodes: TagLabel,

    /// Whether `cell_tag` values are barcode sequences (true) or corrected labels (false).
    /// Default: auto-detect — true for Location tags, false for String tags.
    #[tpd(default)]
    #[expect(
        dead_code,
        reason = "only read in get_tag_usage via PartialStoreSingleCellMatrix"
    )]
    cell_tag_contains_barcode: Option<bool>,
    ///
    /// Whether `gene_tag` values are barcode sequences (true) or corrected labels (false).
    /// Default: auto-detect — true for Location tags, false for String tags.
    #[tpd(default)]
    #[expect(
        dead_code,
        reason = "only read in get_tag_usage via PartialStoreSingleCellMatrix"
    )]
    gene_tag_contains_barcode: Option<bool>,

    /// Infix for output filenames
    #[expect(dead_code, reason = "only used in declare_output_files")]
    infix: Option<String>,

    /// Compression for the binary data file (lookup tables are always plain text)
    #[tpd(default)]
    #[expect(dead_code, reason = "only used in neclare_output_files")]
    compression: CompressionFormat,

    #[tpd(default)]
    #[expect(dead_code, reason = "only used in declare_output_files")]
    compression_level: Option<u8>,

    // ---- runtime-only fields ----
    #[tpd(skip, default)]
    #[schemars(skip)]
    cell_lookup: Arc<FxIndexMap<BString, String>>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    gene_lookup: Arc<FxIndexMap<BString, String>>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    entries: Arc<Mutex<DemultiplexedData<Vec<ObservedEvent>>>>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    max_umi_len: Arc<Mutex<u8>>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    data_writer: Option<DataHandles>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    stats_writer: Option<DataHandles>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    cell_barcodes_writer: Option<WriterHandle>,

    #[tpd(skip, default)]
    #[schemars(skip)]
    genes_writer: Option<WriterHandle>,

    #[tpd(skip)]
    #[schemars(skip)]
    cell_lookup_mode: LookupMode,

    #[tpd(skip)]
    #[schemars(skip)]
    gene_lookup_mode: LookupMode,
}

impl VerifyIn<PartialConfig> for PartialStoreSingleCellMatrix {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.infix.verify(verify_opt_path_component);
        Ok(())
    }
}

//todo: unify with demultiplex that makes the same decision, I suppose
fn determine_lookup_mode(
    upstream_label_meta: Option<&TagMetadata>,
    tag_contains_barcode: &TomlValue<Option<bool>>,
) -> Option<LookupMode> {
    if let Some(meta) = upstream_label_meta {
        match meta.tag_type {
            TagValueType::Location => Some(LookupMode::Barcode),
            TagValueType::String => {
                if let Some(Some(tag_contains_barcode)) = tag_contains_barcode.as_ref() {
                    //user has explicitly told us what the tag contains,
                    //barcodes or barcode-names
                    if *tag_contains_barcode {
                        Some(LookupMode::Barcode)
                    } else {
                        Some(LookupMode::Label)
                    }
                } else {
                    match meta.contents {
                        StringTagContent::Undefined => {
                            // require the user to set it
                            None // cov:excl-line
                        }
                        StringTagContent::Barcodes => Some(LookupMode::Barcode),
                        StringTagContent::Labels => Some(LookupMode::Label),
                    }
                }
                // complain otherwise.
            }
            // cov:excl-start
            _ => {
                //will be complained about because of allowed tag modes
                None
            } // cov:excl-stop
        }
    } else {
        None
    }
}

impl TagUser for PartialTaggedVariant<PartialStoreSingleCellMatrix> {
    fn declare_output_files(&self) -> Option<Vec<OutputDeclaration>> {
        if let Some(inner) = self.toml_value.as_ref() {
            let infix = inner
                .infix
                .as_ref()
                .and_then(|x| x.as_ref())
                .cloned()
                .unwrap_or_default();
            let compression = inner.compression.as_ref().copied().unwrap_or_default();
            let compression_level = inner
                .compression_level
                .as_ref()
                .and_then(|x| x.as_ref())
                .copied();
            let data_suffix = compression.apply_suffix("matrix.mtx");
            let data_sink = SinkConfig {
                compression,
                compression_level,
                hash_uncompressed: false,
                hash_compressed: false,
                simulated_failure: None,
            };
            let span = inner.infix.span();
            Some(vec![
                OutputDeclaration {
                    id: "data".to_string(),
                    target: WriteTargetConfig::new(
                        vec![infix.clone(), "scd".to_string()],
                        None,
                        data_suffix,
                    ),
                    sink_config: data_sink.clone(),
                    format: FileFormat::Text,
                    chunk_policy: ChunkPolicy::default(),
                    bam_options: None,
                    singleton: false,
                    span: span.clone(),
                },
                OutputDeclaration {
                    id: "stats".to_string(),
                    target: WriteTargetConfig::new(
                        vec![infix.clone(), "scd".to_string()],
                        None,
                        compression.apply_suffix("matrix.mtx.stats.txt"),
                    ),
                    sink_config: data_sink.clone(),
                    format: FileFormat::Text,
                    chunk_policy: ChunkPolicy::default(),
                    bam_options: None,
                    singleton: false,
                    span: span.clone(),
                },
                OutputDeclaration {
                    id: "cell_barcodes".to_string(),
                    target: WriteTargetConfig::new(
                        vec![infix.clone(), "scd".to_string()],
                        None,
                        compression.apply_suffix("barcodes.txt"),
                    ),
                    sink_config: data_sink.clone(),
                    format: FileFormat::Text,
                    chunk_policy: ChunkPolicy::default(),
                    bam_options: None,
                    singleton: true,
                    span: span.clone(),
                },
                OutputDeclaration {
                    id: "genes".to_string(),
                    target: WriteTargetConfig::new(
                        vec![infix.clone(), "scd".to_string()],
                        None,
                        compression.apply_suffix("features.txt"),
                    ),
                    sink_config: data_sink,
                    format: FileFormat::Text,
                    chunk_policy: ChunkPolicy::default(),
                    bam_options: None,
                    singleton: true,
                    span,
                },
            ])
        } else {
            Some(vec![]) //there should be output files, but we can't name them.
        }
    }

    fn get_tag_usage(
        &mut self,
        tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        let inner = self.toml_value.value.as_mut()?;
        inner.cell_lookup_mode = determine_lookup_mode(
            tags_available.get(inner.cell_tag.as_ref().expect("parent was ok")),
            &inner.cell_tag_contains_barcode,
        );
        inner.gene_lookup_mode = determine_lookup_mode(
            tags_available.get(inner.gene_tag.as_ref().expect("parent was ok")),
            &inner.gene_tag_contains_barcode,
        );

        Some(TagUsageInfo {
            used_tags: vec![
                inner
                    .cell_tag
                    .to_used_tag(&[TagValueType::String, TagValueType::Location]),
                inner
                    .gene_tag
                    .to_used_tag(&[TagValueType::String, TagValueType::Location]),
                inner
                    .umi_tag
                    .to_used_tag(&[TagValueType::String, TagValueType::Location]),
            ],
            used_barcodes: [
                inner.cell_barcodes.as_ref().cloned(),
                inner.gene_barcodes.as_ref().cloned(),
            ]
            .into_iter()
            .flatten()
            .collect::<std::collections::HashSet<TagLabel>>(),
            ..Default::default()
        })
    }
}

fn seq_to_idx(seq: &[u8], map: &FxIndexMap<BString, String>) -> u32 {
    map.get_index_of(BStr::new(seq))
        .map_or(0, |i| u32::try_from(i + 1).expect("idx exceeds u32::MAX-1"))
}

impl StoreSingleCellMatrix {
    fn create_lookup(
        barcodes: &IndexMap<BString, String>,
        mode: LookupMode,
    ) -> Arc<FxIndexMap<BString, String>> {
        match mode {
            LookupMode::Barcode => Arc::new(
                barcodes
                    .iter()
                    .map(|(x, y)| (x.to_owned(), y.to_owned()))
                    .collect(),
            ),
            LookupMode::Label => Arc::new(
                barcodes
                    .iter()
                    .map(|(_k, label)| (BString::new(label.as_bytes().to_vec()), label.clone()))
                    .collect(),
            ),
        }
    }
}

impl Step for StoreSingleCellMatrix {
    fn init(
        &mut self,
        input_info: &InputInfo,
        mut output_files: StepOutputFiles,
        _demultiplex_info: &OptDemultiplex,
        _input_files: &mut StepInputFiles,
    ) -> Result<Option<DemultiplexBarcodes>> {
        let cell_bc = input_info
            .barcodes_data
            .get(&self.cell_barcodes)
            .expect("cell_barcodes section missing from InputInfo");
        let gene_bc = input_info
            .barcodes_data
            .get(&self.gene_barcodes)
            .expect("gene_barcodes section missing from InputInfo");

        if cell_bc.seq_to_name.len() >= (u32::MAX - 1) as usize {
            //mutants::skip
            //cov:excl-start
            anyhow::bail!(
                "Too many cell barcodes: {} (max {})",
                cell_bc.seq_to_name.len(),
                u32::MAX as usize - 1
            );
            //cov:excl-stop
        }
        if gene_bc.seq_to_name.len() >= (u32::MAX - 1) as usize {
            //mutants::skip
            //cov:excl-start
            anyhow::bail!(
                "Too many gene barcodes: {} (max {})",
                gene_bc.seq_to_name.len(),
                u32::MAX as usize - 1
            );
            //cov:excl-stop
        }
        self.gene_lookup = Self::create_lookup(&gene_bc.seq_to_name, self.gene_lookup_mode);
        self.cell_lookup = Self::create_lookup(&cell_bc.seq_to_name, self.cell_lookup_mode);

        let per_tag = output_files.take("data");
        let mut entries_map = DemultiplexedData::new();
        let mut data_map = DemultiplexedData::new();
        for (tag, writer) in per_tag {
            entries_map.insert(tag, Vec::new());
            data_map.insert(tag, Some(writer));
        }
        let mut stats_map = DemultiplexedData::new();
        for (tag, writer) in output_files.take("stats") {
            stats_map.insert(tag, Some(writer));
        }

        self.entries = Arc::new(Mutex::new(entries_map));
        self.data_writer = Some(Arc::new(Mutex::new(data_map)));
        self.stats_writer = Some(Arc::new(Mutex::new(stats_map)));

        self.cell_barcodes_writer = Some(take_singleton_writer(&mut output_files, "cell_barcodes"));
        self.genes_writer = Some(take_singleton_writer(&mut output_files, "genes"));

        Ok(None)
    }

    fn apply(
        &self,
        block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        let cell_tags = block
            .tags
            .get(&self.cell_tag)
            .expect("cell_tag not in block");
        let gene_tags = block
            .tags
            .get(&self.gene_tag)
            .expect("gene_tag not in block");
        let umi_tags = block.tags.get(&self.umi_tag).expect("umi_tag not in block");

        let cell_map = &*self.cell_lookup;
        let gene_map = &*self.gene_lookup;
        let output_tags = block.output_tags.as_ref();

        //todo: this is accidentially single threaded...
        let mut expected_umi_len = *self.max_umi_len.lock().expect("lock poisoned");

        let mut local_entries: DemultiplexedData<Vec<ObservedEvent>> = DemultiplexedData::new();

        for ii in 0..cell_tags.len() {
            let cell_idx = match cell_tags {
                TagColumn::String(items) => match items.get_string(ii) {
                    Some(s) => seq_to_idx(s, cell_map),
                    None => 0, // cov:excl-line
                },
                TagColumn::Location(col) => {
                    let seq = col.joined_seq(ii, None);
                    if seq.is_empty() {
                        0 // cov:excl-line
                    } else {
                        seq_to_idx(&seq, cell_map)
                    }
                }
                _ => 0, // cov:excl-line
            };

            let gene_idx = match gene_tags {
                TagColumn::String(items) => match items.get_string(ii) {
                    Some(s) => seq_to_idx(s, gene_map),
                    None => 0, // cov:excl-line
                },
                TagColumn::Location(col) => {
                    let seq = col.joined_seq(ii, None);
                    if seq.is_empty() {
                        0 // cov:excl-line
                    } else {
                        seq_to_idx(&seq, gene_map)
                    }
                }
                _ => 0, // cov:excl-line
            };

            #[expect(clippy::cast_possible_truncation, reason = "umi max length 16")]
            let (umi_enc, umi_len) = match umi_tags {
                TagColumn::String(items) => match items.get_string(ii) {
                    Some(s) => {
                        if s.len() > 16 {
                            anyhow::bail!("UMI is {}bp, maximum supported length is 16bp", s.len());
                        }
                        (encode_umi(s), s.len() as u8)
                    }
                    None => (Umi::new_unmatched(), 0u8),
                },
                TagColumn::Location(col) => {
                    let seq = col.joined_seq(ii, None);
                    if seq.is_empty() {
                        (Umi::new_unmatched(), 0u8)
                    } else {
                        if seq.len() > 16 {
                            anyhow::bail!(
                                "UMI is {}bp, maximum supported length is 16bp",
                                seq.len()
                            );
                        }
                        (encode_umi(&seq), seq.len() as u8)
                    }
                }
                _ => unreachable!("Colum type was verified to be string or location"),
            };

            {
                if expected_umi_len == 0 && umi_len > 0 {
                    //mutants::skip
                    *self.max_umi_len.lock().expect("lock poisoned") = umi_len;
                    expected_umi_len = umi_len;
                } else if umi_len > 0 && expected_umi_len != umi_len {
                    anyhow::bail!(
                        "UMI lengths are not uniform: expected {expected_umi_len}bp, got {umi_len}bp",
                    );
                }
            }

            let output_tag = output_tags.map_or(0, |x| x[ii]);
            match local_entries.entry(output_tag) {
                std::collections::btree_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(vec![ObservedEvent {
                        cell: CellIdx(cell_idx),
                        gene: GeneIdx(gene_idx),
                        umi: umi_enc,
                    }]);
                }
                std::collections::btree_map::Entry::Occupied(occupied_entry) => {
                    occupied_entry.into_mut().push(ObservedEvent {
                        cell: CellIdx(cell_idx),
                        gene: GeneIdx(gene_idx),
                        umi: umi_enc,
                    });
                }
            }
        }

        let mut entries = self.entries.lock().expect("lock poisoned");
        for (demultiplex_tag, local) in local_entries {
            entries
                .get_mut(&demultiplex_tag)
                .expect("Entries were created in init")
                .extend(local);
        }

        Ok((block, true))
    }

    fn finalize(&self, _demultiplex_info: &OptDemultiplex) -> Result<Option<FinalizeReportResult>> {
        // Sort and write one binary file per demultiplex tag
        {
            let entries_map = {
                let mut locked = self.entries.lock().expect("lock poisoned");
                std::mem::take(&mut *locked)
            };

            let mut data_guard = self
                .data_writer
                .as_ref()
                .expect("data_writer set in init")
                .lock()
                .expect("lock poisoned");
            let mut stat_guard = self
                .stats_writer
                .as_ref()
                .expect("data_writer set in init")
                .lock()
                .expect("lock poisoned");

            let mut any_gene_matches = false;
            let mut any_cell_matches = false;
            for (tag, mut entries) in entries_map {
                let mut stats = Vec::new();
                if let Some(Some(writer)) = data_guard.get_mut(&tag) {
                    //entries.par_sort_by_key(|&[g, c, _]| (g, c)); it's not measurably faster.

                    // how many different gene/cell combinations are there.
                    //let combos: HashSet<_> = entries.iter().map(|e| (e[0], e[1])).collect();
                    // dbg!(
                    //     "Tag {}, total entries: {}, unique gene/cell combos: {}",
                    //     tag,
                    //     entries.len(),
                    //     combos.len()
                    // );
                    //entries is a vec of [cell_idx, gene_idx, umi_enc]
                    entries.par_sort_unstable_by_key(|read| (read.cell, read.gene, read.umi)); //must also sort by umi to be reproducible in output
                    let total = entries.len();
                    stats.push(("Input reads", total));

                    let matrix = aggregate_to_matrix(
                        &entries,
                        self.umi_aggregation,
                        u16::from(*self.max_umi_len.lock().expect("lock poisoned")),
                    );

                    let unique_genes: FxHashSet<_> = matrix
                        .iter()
                        .filter_map(|(gene, _, _count)| {
                            if !gene.is_unmatched() {
                                Some(gene)
                            } else {
                                None
                            }
                        })
                        .collect();

                    let unique_cells: FxHashSet<_> = matrix
                        .iter()
                        .filter_map(|(_, cell, _count)| {
                            if !cell.is_unmatched() {
                                Some(cell)
                            } else {
                                None
                            }
                        })
                        .collect();

                    let matrix_matched = matrix
                        .iter()
                        .filter(|(gene, cell, _count)| !gene.is_unmatched() && !cell.is_unmatched())
                        .count();

                    any_gene_matches |= !unique_genes.is_empty();
                    any_cell_matches |= !unique_cells.is_empty();
                    stats.push(("Matrix size", matrix.len()));
                    stats.push(("Matrix matched", matrix_matched));
                    stats.push(("Observed genes", unique_genes.len()));
                    stats.push(("Observed cells", unique_cells.len()));
                    let reads_in_matrix = matrix
                        .iter()
                        .map(|(_gene, _cell, count)| *count as usize)
                        .sum();
                    let reads_in_matrix_matched = matrix
                        .iter()
                        .filter(|(gene, cell, _count)| !gene.is_unmatched() && !cell.is_unmatched())
                        .map(|(_gene, _cell_, count)| *count as usize)
                        .sum();
                    stats.push(("Reads in matrix", reads_in_matrix));
                    stats.push((
                        "Reads in matrix with matched gene and cell",
                        reads_in_matrix_matched,
                    ));
                    let reads_lost_due_to_umi_cluster = total - reads_in_matrix as usize;
                    stats.push((
                        "Reads lost due to UMI deduplication",
                        reads_lost_due_to_umi_cluster,
                    ));

                    write_matrix(
                        matrix,
                        //+1 for unmatched. Cast is safe, we
                        //checked this before
                        (self.gene_lookup.len() + 1)
                            .try_into()
                            .expect("gene_lookup.len() + 1 exceeds u32::MAX"),
                        (self.cell_lookup.len() + 1)
                            .try_into()
                            .expect("cell_lookup.len() + 1 exceeds u32::MAX"),
                        writer,
                    )?; // cov:excl-line
                } // cov:excl-line
                if let Some(Some(writer)) = stat_guard.get_mut(&tag) {
                    writer.write_text_record(b"Metric\tValue\n")?;

                    // Pre-format so we can measure the widest value.
                    let rows: Vec<(&str, String)> = stats
                        .iter()
                        .map(|(metric, value)| (*metric, human_fmt_usize(*value)))
                        .collect();

                    let col_width = rows.iter().map(|(_, v)| v.len()).max().unwrap_or(0);

                    for (metric, value) in &rows {
                        writer.write_text_record(
                            format!("{metric}\t{value:>col_width$}\n").as_bytes(),
                        )?; // cov:excl-line
                    }
                } // cov:excl-line
            }
            if !any_gene_matches && !any_cell_matches {
                bail!(
                    "No reads matched the gene nor the cell barcode/label. \
                    Check your barcodes and possibly set `gene_tag_contains_barcode` \
                    and `cell_tag_contains_barcode` to either 'barcode' or 'label' \
                    if the auto-detection has failed you.
                    "
                );
            }
            if !any_gene_matches {
                bail!(
                    "No reads matched the gene barcode/label. \
                    Check your barcodes and possibly set `gene_tag_contains_barcode` \
                    to either 'barcode' or 'label' if the auto-detection has failed you.
                    "
                );
            }
            if !any_cell_matches {
                bail!(
                    "No reads matched the cell barcode/label. \
                    Check your barcodes and possibly set `cell_tag_contains_barcode` \
                    to either 'barcode' or 'label' if the auto-detection has failed you.
                    "
                );
            }
        }
        for (_tag, writer) in self
            .data_writer
            .as_ref()
            .expect("data_writer set in init")
            .lock()
            .expect("lock poisoned")
            .iter_mut()
        {
            if let Some(w) = writer.take() {
                w.finish()?;
            } // cov:excl-line
        }

        // Write cell barcode lookup table (line 0 = "unmatched", then ordered names)
        {
            let handle = self
                .cell_barcodes_writer
                .as_ref()
                .expect("cell_barcodes_writer set in init");
            let mut guard = handle.lock().expect("lock poisoned");
            let writer = guard.as_mut().expect("writer not yet finished");
            writer.write_text_record(b"unmatched\n")?;
            for name in self.cell_lookup.values() {
                let mut line = name.as_bytes().to_vec();
                line.push(b'\n');
                writer.write_text_record(&line)?;
            } // cov:excl-line
        }
        finish_writer(
            self.cell_barcodes_writer
                .as_ref()
                .expect("cell_barcodes_writer set in init"),
        )?; // cov:excl-line

        // Write gene lookup table
        {
            let handle = self
                .genes_writer
                .as_ref()
                .expect("genes_writer set in init");
            let mut guard = handle.lock().expect("lock poisoned");
            let writer = guard.as_mut().expect("writer not yet finished");
            writer.write_text_record(b"unmatched\n")?;
            for name in self.gene_lookup.values() {
                let mut line = name.as_bytes().to_vec();
                line.push(b'\n');
                writer.write_text_record(&line)?;
            }
        }
        finish_writer(
            self.genes_writer
                .as_ref()
                .expect("genes_writer set in init"),
        )?; // cov:excl-line

        Ok(None)
    }
}

fn aggregate_to_matrix(
    entries: &[ObservedEvent],
    umi_aggregation: UMIAggregation,
    umi_length: u16,
) -> Vec<(GeneIdx, CellIdx, u32)> {
    if entries.is_empty() {
        Vec::new()
    } else {
        match umi_aggregation {
            UMIAggregation::None => aggregate_to_matrix_none(entries),
            UMIAggregation::Exact => aggregate_to_matrix_exact(entries),
            UMIAggregation::Cluster => aggregate_to_matrix_cluster(entries, umi_length),
            UMIAggregation::CellRangerLike => {
                aggregate_to_matrix_cellranger_like(entries, umi_length)
            }
        }
    }
}

fn aggregate_to_matrix_none(entries: &[ObservedEvent]) -> Vec<(GeneIdx, CellIdx, u32)> {
    let mut matrix = Vec::new();
    let mut counter = 0u32; //doesn't matter though, overwritten in first loop.
    let mut last: Option<(GeneIdx, CellIdx)> = None;
    for entry in entries {
        //let umi = entry[2];
        let key = (entry.gene, entry.cell);
        match last {
            Some(last_key) if last_key == key => {
                //same gene & cell
                counter = counter.saturating_add(1);
            }
            Some(last_key) => {
                // different gene & cell -> push last
                matrix.push((last_key.0, last_key.1, counter));
                last = Some(key);
                counter = 1;
            }
            None => {
                //first trip through the loop
                last = Some(key);
                counter = 1;
            }
        }
    }
    if let Some(last) = last {
        matrix.push((last.0, last.1, counter));
    }
    matrix
}

fn aggregate_to_matrix_exact(entries: &[ObservedEvent]) -> Vec<(GeneIdx, CellIdx, u32)> {
    let mut matrix = Vec::new();
    let mut counter = 0u32;
    let mut last: Option<(GeneIdx, CellIdx, Umi)> = None;
    for entry in entries {
        let umi = entry.umi;
        if umi.is_n() {
            //invalid umi / any N
            continue;
        }
        //we again use that we're umi sorted
        let key = (entry.gene, entry.cell, umi);
        match last {
            Some(last_key) if last_key == key => {
                //same gene & cell & umi. don't count
            }
            Some(last_key) if last_key.0 == key.0 && last_key.1 == key.1 => {
                //same cell and gene, different umi.
                counter = counter.saturating_add(1);
                last = Some(key);
            }
            Some(last_key) => {
                // different gene & cell -> push last
                matrix.push((last_key.0, last_key.1, counter));
                last = Some(key);
                counter = 1;
            }
            None => {
                //first trip through the loop
                last = Some(key);
                counter = 1;
            }
        }
    }
    if let Some(last_key) = last {
        matrix.push((last_key.0, last_key.1, counter));
    }
    matrix
}

// fn write_entries_to_debug_file(entries: &Vec<ObservedEvent>) {
//     use std::io::Write;
//     let file = ex::fs::File::create("debug_entries.dat").expect("Failed to create debug file");
//     let mut buffer = std::io::BufWriter::new(file);
//     let data = entries;
//     let len = data.len() as u64;
//     buffer.write_all(&len.to_le_bytes()).unwrap();
//
//     for triple in data {
//         buffer.write_all(&triple.gene.0.to_le_bytes()).unwrap();
//         buffer.write_all(&triple.cell.0.to_le_bytes()).unwrap();
//         buffer.write_all(&triple.umi.0.to_le_bytes()).unwrap();
//     }
//     buffer.flush().unwrap();
//     panic!("Done dumping entries.dat");
// }
