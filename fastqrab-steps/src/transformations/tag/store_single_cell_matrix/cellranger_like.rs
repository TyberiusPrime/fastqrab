use rayon::prelude::*;
use rustc_hash::FxHashSet;

use super::{CellIdx, GeneIdx, ObservedEvent, Umi};
use crate::transformations::prelude::*; // union_find_rs::prelude also exports a Result

pub fn aggregate_to_matrix_cellranger_like(
    entries: &[ObservedEvent],
    umi_length: u16,
) -> Vec<(GeneIdx, CellIdx, u32)> {
    //cell ranger overview:
    //a) for each umi, assign to highest 1hamming neighbour.
    //b) for each cell-barcode & umi, kick out low-count genes / on tie remove all
    //entries are [cell_idx, gene_idx, umi_enc], sorted.
    //
    //
    //save entries to debug.entries

    let entries: Vec<_> = entries
        .par_chunk_by(|a, b| a.cell == b.cell)
        .map(|cell_chunk| {
            let mut result = Vec::new();
            let cell_idx = cell_chunk[0].cell;
            if cell_idx.is_unmatched() {
                return result;
            }
            let mut counts = count_umis(cell_chunk, umi_length);
            let corrections = correct_umis_to_next_by_hamming(&counts, umi_length);
            apply_recount_adjustment(&mut counts, &corrections);

            let low_support_umigenes = find_umis_with_conflicting_genes(&counts);

            //now, all I care about is the number of remaining umis, per
            //gene, excluding the low_support_umigenes, right?
            for gene_chunk in cell_chunk.chunk_by(|a, b| a.gene == b.gene) {
                let mut distinct = FxHashSet::default();
                for event in gene_chunk {
                    if event.umi.is_homopolymer(umi_length)
                        || event.umi.is_n()
                        || event.gene.is_unmatched()
                    {
                        continue;
                    }
                    let corrected_gene_umi = corrections
                        .get(&(event.gene, event.umi))
                        .copied()
                        .unwrap_or((event.gene, event.umi));
                    if !low_support_umigenes.contains(&corrected_gene_umi) {
                        distinct.insert((event.gene, corrected_gene_umi.1));
                    }
                }
                let gene = gene_chunk[0].gene;
                result.push((
                    gene,
                    cell_idx,
                    distinct.len().try_into().expect("Exceeded u32"),
                ));
            }
            result
        })
        .collect();
    entries.into_iter().flatten().collect()
}

///cell ranger 3 special: one read has already been counted under `raw_key`
///before the correction to `corrected_key` was decided. Move that one count
///over so low-support UMI-genes are determined on the corrected tallies.
fn apply_recount_adjustment(
    counts: &mut FxIndexMap<(GeneIdx, Umi), u32>,
    corrections: &FxIndexMap<(GeneIdx, Umi), (GeneIdx, Umi)>,
) {
    for (raw_key, corrected_key) in corrections {
        *counts.get_mut(raw_key).expect("Key not found?") -= 1;
        *counts
            .get_mut(corrected_key)
            .expect("correcected key not found?") += 1;
    }
}

fn count_umis(
    cell_gene_umis: &[ObservedEvent],
    umi_length: u16,
) -> FxIndexMap<(GeneIdx, Umi), u32> {
    let mut umi_counts: FxIndexMap<(GeneIdx, Umi), u32> = FxIndexMap::default();
    for event in cell_gene_umis {
        if !event.umi.is_homopolymer(umi_length) && !event.umi.is_n() && !event.gene.is_unmatched()
        {
            umi_counts
                .entry((event.gene, event.umi))
                .and_modify(|c| *c = c.saturating_add(1u32))
                .or_insert(1);
        }
    }
    umi_counts
}

///correct within one cellbarcode/gene
fn correct_umis_to_next_by_hamming(
    umi_counts: &FxIndexMap<(GeneIdx, Umi), u32>,
    umi_length: u16,
) -> FxIndexMap<(GeneIdx, Umi), (GeneIdx, Umi)> {
    //first we need accurate counts.
    let mut umis: Vec<((GeneIdx, Umi), u32)> = umi_counts.iter().map(|(a, b)| (*a, *b)).collect();
    umis.sort_unstable();
    let mut corrections = FxIndexMap::default();
    // `==` vs `!=` is equivalent here: chunk_by always partitions the whole slice into
    // exactly one chunk per gene regardless of the 'sign' of the predicate,
    for gene_chunk in umis.chunk_by(|((gene, _), _), ((b_gene, _), _)| gene == b_gene) {
        //mutants::skip
        for &(x, x_count) in gene_chunk {
            let mut best = x;
            let mut best_count = x_count;

            let this_umi = x.1;
            for bp in 0..umi_length {
                let shift = bp * 2;
                let current = (this_umi.0 >> shift) & 0b11;

                for replacement in 0..4u32 {
                    if replacement == current {
                        continue;
                    }
                    // Clear the 2 bits at this basepair, then set the replacement
                    // `|` and `^` are equivalent here: the cleared bits are always 0
                    // where `replacement << shift` is nonzero, and vice versa.
                    let test_umi = (this_umi.0 & !(0b11 << shift)) | (replacement << shift); //mutants::skip
                    let test_umi = Umi(test_umi);

                    if let Some(&y_count) = umi_counts.get(&(x.0, test_umi)) {
                        let cmp = y_count.cmp(&best_count);
                        // `>` vs `>=` is equivalent: every test_umi generated in this
                        // double loop is distinct from every other one and from x.1
                        // itself (each is a single-base edit at a different position
                        // from a base guaranteed != current), so it can never equal best.1.
                        if (cmp == std::cmp::Ordering::Greater)
                            || (cmp == std::cmp::Ordering::Equal && test_umi > best.1)
                        //mutants::skip
                        {
                            best = (x.0, test_umi);
                            best_count = y_count;
                        }
                    }
                }
            }

            if best != x {
                corrections.insert(x, best);
            }
        }
    }
    corrections
}

fn find_umis_with_conflicting_genes(
    umi_counts: &FxIndexMap<(GeneIdx, Umi), u32>,
) -> FxHashSet<(GeneIdx, Umi)> {
    let mut low_support_umigenes: FxHashSet<(GeneIdx, Umi)> = FxHashSet::default();
    let mut umigene_count_vec: Vec<_> = umi_counts
        .iter()
        .map(|(&(gene, umi), &count)| (umi, gene, count))
        .collect();
    umigene_count_vec.sort();
    //chunk by umi...
    for gene_counts in umigene_count_vec.chunk_by(|x, y| x.0 == y.0) {
        let (max_count, max_is_tied) = gene_counts
            .iter()
            .copied()
            .fold(None, |acc, (_umi, _gene, count)| match acc {
                None => Some((count, false)),
                Some((m, tied)) => match count.cmp(&m) {
                    std::cmp::Ordering::Greater => Some((count, false)),
                    std::cmp::Ordering::Equal => Some((count, true)),
                    std::cmp::Ordering::Less => Some((m, tied)),
                },
            })
            .expect("ALways at least one");
        for (umi, gene, count) in gene_counts {
            if max_is_tied || *count < max_count {
                low_support_umigenes.insert((*gene, *umi));
            }
        }
    }
    low_support_umigenes
}

#[test]
fn test_find_umis_with_conflicting_genes() {
    let hm: FxIndexMap<(GeneIdx, Umi), u32> = vec![
        ((GeneIdx(773), Umi(1)), 2u32),
        ((GeneIdx(6396), Umi(1)), 1u32),
    ]
    .into_iter()
    .collect();
    let result = find_umis_with_conflicting_genes(&hm);
    assert_eq!(result.len(), 1);
}

#[test]
fn test_apply_recount_adjustment() {
    let mut counts: FxIndexMap<(GeneIdx, Umi), u32> =
        vec![((GeneIdx(1), Umi(0)), 3u32), ((GeneIdx(1), Umi(1)), 5u32)]
            .into_iter()
            .collect();
    let corrections: FxIndexMap<(GeneIdx, Umi), (GeneIdx, Umi)> =
        vec![((GeneIdx(1), Umi(0)), (GeneIdx(1), Umi(1)))]
            .into_iter()
            .collect();
    apply_recount_adjustment(&mut counts, &corrections);
    assert_eq!(counts[&(GeneIdx(1), Umi(0))], 2);
    assert_eq!(counts[&(GeneIdx(1), Umi(1))], 6);
}

#[test]
fn test_umi_correction() {
    let counts: FxIndexMap<(GeneIdx, Umi), u32> = vec![
        ((GeneIdx(772), Umi(10_849_786)), 2),
        ((GeneIdx(772), Umi(10_849_738)), 1),
    ]
    .into_iter()
    .collect();
    let corrections = correct_umis_to_next_by_hamming(&counts, 12);
    assert_eq!(
        corrections.get(&(GeneIdx(772), Umi(10_849_738))),
        Some(&(GeneIdx(772), Umi(10_849_786)))
    );
    assert!(!corrections.contains_key(&(GeneIdx(772), Umi(10_849_786))));
}

/// Regression test: `current` must be the *actual* base at this bp, not some
/// garbage derived from OR-ing or XOR-ing with 0b11. Uses umi_length == 1 so
/// there are no higher-order bits to mask off, making the true base's effect
/// on which replacement gets (wrongly) skipped visible.
#[test]
fn test_umi_correction_uses_actual_current_base() {
    let counts: FxIndexMap<(GeneIdx, Umi), u32> = vec![
        // gene 1: current base is 1 (0b01); the only better neighbour is base 3.
        // A mutant that ORs with 0b11 always thinks `current == 3` and would
        // skip ever testing replacement 3, missing this correction.
        ((GeneIdx(1), Umi(1)), 1u32),
        ((GeneIdx(1), Umi(3)), 50u32),
        // gene 2: current base is 1 (0b01); the only better neighbour is base 2,
        // the complement of 1 (1 ^ 0b11 == 2). A mutant that XORs with 0b11
        // would think `current == 2` and skip testing replacement 2.
        ((GeneIdx(2), Umi(1)), 1u32),
        ((GeneIdx(2), Umi(2)), 50u32),
    ]
    .into_iter()
    .collect();
    let corrections = correct_umis_to_next_by_hamming(&counts, 1);
    assert_eq!(
        corrections.get(&(GeneIdx(1), Umi(1))),
        Some(&(GeneIdx(1), Umi(3)))
    );
    assert_eq!(
        corrections.get(&(GeneIdx(2), Umi(1))),
        Some(&(GeneIdx(2), Umi(2)))
    );
}

/// Regression test: the mask/replacement shifts must move with `bp`. At
/// bp == 0 (shift == 0) `<<` and `>>` coincide, so this only exercises bp > 0
/// (umi_length == 2, correction needed on the higher-order base).
#[test]
fn test_umi_correction_shifts_by_bp() {
    // x = 0b0101 (base0 = 1, base1 = 1). Correcting base1 (2 -> value 0b1001 = 9)
    // is only reachable if both the clear-mask and the replacement are shifted
    // by `shift`, not left un-shifted or shifted the wrong way.
    let counts: FxIndexMap<(GeneIdx, Umi), u32> = vec![
        ((GeneIdx(1), Umi(0b0101)), 1u32),
        ((GeneIdx(1), Umi(0b1001)), 50u32),
    ]
    .into_iter()
    .collect();
    let corrections = correct_umis_to_next_by_hamming(&counts, 2);
    assert_eq!(
        corrections.get(&(GeneIdx(1), Umi(0b0101))),
        Some(&(GeneIdx(1), Umi(0b1001)))
    );
}

/// Regression test: `current` at `bp > 0` must be extracted with `>>`, not `<<`.
/// A `<<`-based `current` computes the wrong base at this position (0 instead of
/// the true value 1), so it wrongly skips testing replacement 0 (thinking it's a
/// no-op) and instead "tests" replacement 1, which just reproduces `x` itself.
/// The one candidate that actually has a much higher count (replacement 0) is
/// never tried, so no correction is found at all.
#[test]
fn test_umi_correction_uses_actual_current_base_at_higher_bp() {
    let counts: FxIndexMap<(GeneIdx, Umi), u32> = vec![
        ((GeneIdx(1), Umi(4)), 1u32),  // base0 = 0, base1 = 1 (0b0100)
        ((GeneIdx(1), Umi(0)), 50u32), // same but base1 = 0 (0b0000)
    ]
    .into_iter()
    .collect();
    let corrections = correct_umis_to_next_by_hamming(&counts, 2);
    assert_eq!(
        corrections.get(&(GeneIdx(1), Umi(4))),
        Some(&(GeneIdx(1), Umi(0)))
    );
}

/// Regression test: on a tied count, the tie-break must switch to a *later*,
/// lexically-larger candidate (`test_umi > best.1`), not just keep the first one
/// found (which a `==` mutant would do) and not prefer the *smaller* one (which a
/// `<` mutant would do). Umi(3) is found first (bp 0) and Umi(4) second (bp 1),
/// both tied at count 10, with Umi(4) > Umi(3).
#[test]
fn test_umi_correction_tie_break_prefers_larger_umi_found_later() {
    let counts: FxIndexMap<(GeneIdx, Umi), u32> = vec![
        ((GeneIdx(1), Umi(0)), 1u32),
        ((GeneIdx(1), Umi(3)), 10u32),
        ((GeneIdx(1), Umi(4)), 10u32),
    ]
    .into_iter()
    .collect();
    let corrections = correct_umis_to_next_by_hamming(&counts, 2);
    assert_eq!(
        corrections.get(&(GeneIdx(1), Umi(0))),
        Some(&(GeneIdx(1), Umi(4)))
    );
}

/// Regression test: a higher-count neighbour must win over a merely
/// lexically-larger one, and a lower count must never win outright.
#[test]
fn test_umi_correction_picks_highest_count_not_lexical_max() {
    let counts: FxIndexMap<(GeneIdx, Umi), u32> = vec![
        ((GeneIdx(1), Umi(1)), 1u32),
        ((GeneIdx(1), Umi(0)), 5u32), // strictly higher count, lexically smaller
        ((GeneIdx(1), Umi(2)), 3u32), // lower count, lexically larger
    ]
    .into_iter()
    .collect();
    let corrections = correct_umis_to_next_by_hamming(&counts, 1);
    assert_eq!(
        corrections.get(&(GeneIdx(1), Umi(1))),
        Some(&(GeneIdx(1), Umi(0)))
    );
}
