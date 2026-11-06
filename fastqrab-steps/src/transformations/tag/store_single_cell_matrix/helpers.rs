use anyhow::Result;
use fastqrab_io::io::output::chunked_writer::ChunkedRecordWriter;
use std::sync::{Arc, Mutex};

use super::{CellIdx, GeneIdx, Umi};
use crate::{
    demultiplex::StepOutputFiles, transformations::tag::store_single_cell_matrix::WriterHandle,
};

#[mutants::skip] // | is ^ on disjoint bytes.
pub fn encode_umi(umi: &[u8]) -> Umi {
    let mut v = 0u32;
    for &b in umi.iter().take(16) {
        v = (v << 2)
            | match b.to_ascii_uppercase() {
                b'A' => 0,
                b'C' => 1,
                b'G' => 2,
                b'T' => 3,
                _ => return Umi::new_n(), // any N -> becomes TTTTT
                                          // (which if you have less than 16 bp is distinguishable downstream
                                          // and if you don't you loose only a single umi
            };
    }
    Umi(v)
}

pub fn human_fmt_usize(n: usize) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            out.push('_');
        }
        out.push(c);
    }
    out.chars().rev().collect()
}

pub fn write_matrix(
    matrix: Vec<(GeneIdx, CellIdx, u32)>,
    n_genes: u32,
    n_cells: u32,
    writer: &mut ChunkedRecordWriter,
) -> Result<()> {
    writer.write_text_record(b"%%MatrixMarket matrix coordinate integer general\n")?;
    writer.write_text_record(b"%metadata_json: {\"software\": \"fastqrab\"}\n")?;
    let total: usize = matrix.len();
    writer.write_text_record(format!("{n_genes} {n_cells} {total}\n").as_bytes())?;
    for (gene, cell, count) in matrix {
        assert!(cell.0 <= n_cells, "n_cells and actual cells mismatch");
        assert!(gene.0 <= n_genes, "n_genes and actual genes mismatch");
        writer
            .write_text_record(format!("{} {} {}\n", gene.0 + 1, cell.0 + 1, count).as_bytes())?;
    }
    Ok(())
}

pub fn take_singleton_writer(output_files: &mut StepOutputFiles, id: &str) -> WriterHandle {
    let writers = output_files.take(id);
    let writer = writers
        .into_iter()
        .next()
        .map(|(_tag, w)| w)
        .expect("singleton writer must exist");
    Arc::new(Mutex::new(Some(writer)))
}

pub fn finish_writer(handle: &WriterHandle) -> Result<()> {
    handle
        .lock()
        .expect("lock poisoned")
        .take()
        .expect("SHould have had writer at this point")
        .finish()?;
    Ok(())
}

#[inline]
pub fn hamming_bp_16(a: Umi, b: Umi) -> u32 {
    // XOR marks differing bits within each 2-bit base.
    let x = a.0 ^ b.0;

    // Collapse each 2-bit lane to 1 bit:
    // 00 -> 0
    // 01,10,11 -> 1
    let y = (x | (x >> 1)) & 0x5555_5555;

    y.count_ones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_fmt_usize() {
        assert_eq!(human_fmt_usize(0), "0");
        assert_eq!(human_fmt_usize(1), "1");
        assert_eq!(human_fmt_usize(999), "999");
        assert_eq!(human_fmt_usize(1_000), "1_000");
        assert_eq!(human_fmt_usize(1_234), "1_234");
        assert_eq!(human_fmt_usize(999_999), "999_999");
        assert_eq!(human_fmt_usize(1_000_000), "1_000_000");
        assert_eq!(human_fmt_usize(1_234_567), "1_234_567");
        assert_eq!(human_fmt_usize(1_234_567_890), "1_234_567_890");
    }
}
