use super::extract_region_tags_from_seq;
use crate::transformations::prelude::*;
use fastqrab_config::tpd_adapt_bstring_uppercase;

/// Find a 3' adapter exactly the way fastp (v1.4.0) does it for a single
/// `--adapter_sequence` (`AdapterTrimmer::trimBySequence`, `matchReq = 4`),
/// and store the adapter region (adapter start..read end) as a location tag.
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct AdapterLikeFastP {
    #[schemars(with = "String")]
    #[tpd(adapt_in_verify(String))]
    segment: SegmentIndex,

    pub out_label: TagLabel,

    #[tpd(with = "tpd_adapt_bstring_uppercase")]
    #[tpd(alias = "search")]
    #[tpd(alias = "query")]
    #[tpd(alias = "pattern")]
    #[schemars(with = "String")]
    adapter: BString,
}

/// fastp's `matchReq` for a single adapter sequence.
const MATCH_REQ: isize = 4;
/// fastp allows one mismatch per this many compared bases.
const ALLOW_ONE_MISMATCH_FOR_EACH: isize = 8;

impl VerifyIn<PartialConfig> for PartialAdapterLikeFastP {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        self.segment.validate_segment(parent);
        self.adapter.verify(|v| {
            if v.len() < 4 {
                return Err(ValidationFailure::new(
                    "Adapter must be at least 4 bases long",
                    Some("fastp rejects adapters of length <= 3"),
                ));
            }
            if let Some(c) = v.iter().find(|c| !matches!(c, b'A' | b'C' | b'G' | b'T')) {
                return Err(ValidationFailure::new(
                    format!(
                        "Invalid adapter base: '{}'",
                        std::char::from_u32((*c).into())
                            .unwrap_or(std::char::REPLACEMENT_CHARACTER)
                    ),
                    Some("Like fastp, only A, C, G and T are allowed".to_string()),
                ));
            }
            Ok(())
        });
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialAdapterLikeFastP> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        if let Some(inner) = self.toml_value.value.as_mut()
            && let Some(Some(segment)) = inner.segment.as_ref().map(|m| m.as_ref_post())
        {
            // Record the segment this location tag lives on, so a conditional
            // `Swap` on that segment can forget it.
            Some(TagUsageInfo {
                declared_tag: inner
                    .out_label
                    .to_declared_tag(TagValueType::Location)
                    .map(|dt| dt.with_segment(*segment)),
                ..Default::default()
            })
        } else {
            None // cov:excl-line
        }
    }
}

/// Equivalent of fastp's `Matcher::matchWithOneInsertion`, without allocations.
///
/// `ins_data` must hold at least `cmplen + 1` bytes, `normal_data` at least `cmplen`.
/// Is `normal_data[..cmplen]` equal to `ins_data[..cmplen+1]` with one base removed
/// (at position 1..cmplen), allowing `diff_limit` mismatches?
///
/// fastp fills two arrays (mismatches accumulated from the left / from the right)
/// with early breaks. The early breaks only ever affect entries whose sums already
/// exceed `diff_limit`, so computing the right-hand sums as
/// `total - prefix` gives identical results
/// (verified against a literal port in the tests).
fn match_with_one_insertion(
    ins_data: &[u8],
    normal_data: &[u8],
    cmplen: usize,
    diff_limit: isize,
) -> bool {
    if cmplen == 0 || diff_limit < 0 {
        // diff_limit < 0: fastp's check of `left[0] + right[last] > diff_limit`
        // fails immediately (or the loop doesn't run for cmplen == 1).
        return false;
    }
    let ins = &ins_data[..=cmplen];
    let normal = &normal_data[..cmplen];
    // right[i] = #mismatches of ins[j + 1] vs normal[j] for j in i..cmplen
    let right_total: isize = ins[1..]
        .iter()
        .zip(normal)
        .map(|(a, b)| isize::from(a != b))
        .sum();
    let right_last = isize::from(ins[cmplen] != normal[cmplen - 1]);
    let mut left = 0isize; // left[i - 1]
    let mut right_prefix = 0isize; // right[0] - right[i]
    for i in 1..cmplen {
        left += isize::from(ins[i - 1] != normal[i - 1]);
        right_prefix += isize::from(ins[i] != normal[i - 1]);
        if left + right_last > diff_limit {
            return false;
        }
        if left + right_total - right_prefix <= diff_limit {
            return true;
        }
    }
    false
}

/// Count mismatches, stopping once `limit` is exceeded (like fastp's `countMismatchesBounded`).
fn count_mismatches_bounded(a: &[u8], b: &[u8], limit: isize) -> isize {
    let mut mismatches = 0;
    for (x, y) in a.iter().zip(b) {
        if x != y {
            mismatches += 1;
            if mismatches > limit {
                break;
            }
        }
    }
    mismatches
}

impl AdapterLikeFastP {
    /// Port of fastp's `AdapterTrimmer::trimBySequence` (matchReq = 4).
    ///
    /// Returns the length fastp would truncate the read to,
    /// or None if no adapter was found.
    ///
    /// Note that this reproduces fastp's quirks faithfully, e.g. the indel
    /// searches always compare against the *start* of the read, no matter the
    /// position being tested.
    #[must_use]
    pub fn fastp_trim_position(read: &[u8], adapter: &[u8]) -> Option<usize> {
        #[expect(clippy::cast_possible_wrap, reason = "read lengths fit into isize")]
        let rlen = read.len() as isize;
        #[expect(clippy::cast_possible_wrap, reason = "adapter lengths fit into isize")]
        let alen = adapter.len() as isize;
        if alen < MATCH_REQ {
            return None; // cov:excl-line - rejected in verify.
        }

        // We start at negative positions since Illumina adapter dimers
        // usually have the first A skipped (A-tailing).
        let start: isize = if alen >= 16 {
            -4
        } else if alen >= 12 {
            -3
        } else if alen >= 8 {
            -2
        } else {
            0
        };

        // Hamming distance matches (no indels)
        for pos in start..rlen - MATCH_REQ {
            let cmplen = (rlen - pos).min(alen);
            let allowed_mismatch = cmplen / ALLOW_ONE_MISMATCH_FOR_EACH;
            let start_offset = (-pos).max(0);
            #[expect(clippy::cast_sign_loss, reason = "all values are >= 0 here")]
            let mismatches = count_mismatches_bounded(
                &adapter[start_offset as usize..cmplen as usize],
                &read[(start_offset + pos) as usize..],
                allowed_mismatch,
            );
            if mismatches <= allowed_mismatch {
                #[expect(clippy::cast_sign_loss, reason = "clamped to 0")]
                return Some(pos.max(0) as usize);
            }
        }

        // fastp's indel searches always compare against the read *start*, so the
        // outcome only depends on cmplen - which stays constant (= adapter length)
        // for most positions. We only evaluate each distinct cmplen once.
        // Below 8 compared bases, the allowed mismatch count is negative,
        // which can never match.

        // One insertion in the read
        let mut last_cmplen = -1;
        for pos in 0..rlen - MATCH_REQ - 1 {
            let cmplen = (rlen - pos - 1).min(alen);
            if cmplen < ALLOW_ONE_MISMATCH_FOR_EACH {
                break; // cmplen only shrinks from here on
            }
            if cmplen == last_cmplen {
                continue;
            }
            last_cmplen = cmplen;
            let allowed_mismatch = cmplen / ALLOW_ONE_MISMATCH_FOR_EACH - 1;
            #[expect(clippy::cast_sign_loss, reason = "cmplen > 0 here")]
            if match_with_one_insertion(read, adapter, cmplen as usize, allowed_mismatch) {
                #[expect(clippy::cast_sign_loss, reason = "pos >= 0")]
                return Some(pos as usize);
            }
        }

        // One deletion in the read
        let mut last_cmplen = -1;
        for pos in 0..rlen - MATCH_REQ {
            let cmplen = (rlen - pos).min(alen - 1);
            if cmplen < ALLOW_ONE_MISMATCH_FOR_EACH {
                break;
            }
            if cmplen == last_cmplen {
                continue;
            }
            last_cmplen = cmplen;
            let allowed_mismatch = cmplen / ALLOW_ONE_MISMATCH_FOR_EACH - 1;
            #[expect(clippy::cast_sign_loss, reason = "cmplen > 0 here")]
            if match_with_one_insertion(adapter, read, cmplen as usize, allowed_mismatch) {
                #[expect(clippy::cast_sign_loss, reason = "pos >= 0")]
                return Some(pos as usize);
            }
        }
        None
    }
}

impl Step for AdapterLikeFastP {
    fn apply(
        &self,
        mut block: FastQBlocksCombined,
        _input_info: &InputInfo,
        _demultiplex_info: &OptDemultiplex,
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        extract_region_tags_from_seq(&mut block, self.segment, &self.out_label, |seq| {
            #[expect(
                clippy::cast_possible_truncation,
                reason = "lengths are guaranteed to be within u32 range"
            )]
            Self::fastp_trim_position(seq, &self.adapter).map(|pos| pos as u32..seq.len() as u32)
        });
        Ok((block, true))
    }
}

#[cfg(test)]
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    reason = "test code, small values"
)]
mod tests {
    use super::*;

    /// Literal port of fastp's `Matcher::matchWithOneInsertion`.
    ///
    /// `ins_data` must hold at least `cmplen + 1` bytes, `normal_data` at least `cmplen`.
    /// Is `normal_data[..cmplen]` equal to `ins_data[..cmplen+1]` with one base removed
    /// (at position 1..cmplen), allowing `diff_limit` mismatches?
    fn reference_match_with_one_insertion(
        ins_data: &[u8],
        normal_data: &[u8],
        cmplen: usize,
        diff_limit: isize,
    ) -> bool {
        if cmplen == 0 {
            return false; // cov:excl-line - fastp never calls it like this.
        }
        let mut from_left = vec![0isize; cmplen];
        let mut from_right = vec![0isize; cmplen];
        from_left[0] = isize::from(ins_data[0] != normal_data[0]);
        from_right[cmplen - 1] = isize::from(ins_data[cmplen] != normal_data[cmplen - 1]);
        for i in 1..cmplen {
            from_left[i] = from_left[i - 1] + isize::from(ins_data[i] != normal_data[i]);
            if from_left[i] + from_right[cmplen - 1] > diff_limit {
                // fastp leaves the rest uninitialized - it is never read,
                // since the final loop returns false upon reaching index i.
                break;
            }
        }
        for i in (0..cmplen.saturating_sub(1)).rev() {
            from_right[i] = from_right[i + 1] + isize::from(ins_data[i + 1] != normal_data[i]);
            if from_right[i] + from_left[0] > diff_limit {
                for v in &mut from_right[..i] {
                    *v = diff_limit + 1;
                }
                break;
            }
        }
        for i in 1..cmplen {
            if from_left[i - 1] + from_right[cmplen - 1] > diff_limit {
                return false;
            }
            if from_left[i - 1] + from_right[i] <= diff_limit {
                return true;
            }
        }
        false
    }

    /// Literal port of fastp's trimBySequence (no shortcuts), as reference.
    fn reference_trim_position(read: &[u8], adapter: &[u8]) -> Option<usize> {
        let rlen = read.len() as isize;
        let alen = adapter.len() as isize;
        let start: isize = if alen >= 16 {
            -4
        } else if alen >= 12 {
            -3
        } else if alen >= 8 {
            -2
        } else {
            0
        };
        for pos in start..rlen - 4 {
            let cmplen = (rlen - pos).min(alen);
            let so = (-pos).max(0);
            let mm = adapter[so as usize..cmplen as usize]
                .iter()
                .zip(&read[(so + pos) as usize..])
                .filter(|(a, b)| a != b)
                .count() as isize;
            if mm <= cmplen / 8 {
                return Some(pos.max(0) as usize);
            }
        }
        for pos in 0..rlen - 5 {
            let cmplen = (rlen - pos - 1).min(alen);
            if reference_match_with_one_insertion(read, adapter, cmplen as usize, cmplen / 8 - 1) {
                return Some(pos as usize);
            }
        }
        for pos in 0..rlen - 4 {
            let cmplen = (rlen - pos).min(alen - 1);
            if reference_match_with_one_insertion(adapter, read, cmplen as usize, cmplen / 8 - 1) {
                return Some(pos as usize);
            }
        }
        None
    }

    #[test]
    fn matches_literal_port() {
        // small xorshift - no rand dependency needed
        let mut state = 0x2545_f491_4f6c_dd1du64;
        let mut next = move |n: usize| {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            (state % n as u64) as usize
        };
        let full = b"AGATCGGAAGAGCACACGTCTGAACTCCAGTCA";
        for round in 0..50_000 {
            let alen = 4 + next(full.len() - 3);
            let adapter = &full[..alen];
            // low-entropy alphabet in half the rounds to provoke near-matches
            let alphabet: &[u8] = if round % 2 == 0 { b"ACGT" } else { b"AG" };
            let mut read: Vec<u8> = (0..next(60))
                .map(|_| alphabet[next(alphabet.len())])
                .collect();
            if next(2) == 0 {
                // plant a mutated adapter copy
                let at = next(read.len() + 1);
                let mut a = adapter[next(5).min(alen - 1)..].to_vec();
                for _ in 0..next(4) {
                    let i = next(a.len());
                    a[i] = b"ACGT"[next(4)];
                }
                match next(3) {
                    0 if a.len() > 2 => {
                        let i = 1 + next(a.len() - 1);
                        a.insert(i, b"ACGT"[next(4)]);
                    }
                    1 if a.len() > 2 => {
                        a.remove(1 + next(a.len() - 2));
                    }
                    _ => {}
                }
                read.splice(at..at, a);
                read.truncate(next(read.len() + 1).max(read.len() / 2));
            }
            assert_eq!(
                AdapterLikeFastP::fastp_trim_position(&read, adapter),
                reference_trim_position(&read, adapter),
                "read {:?} adapter {:?}",
                std::str::from_utf8(&read),
                std::str::from_utf8(adapter)
            );
        }
    }

    #[test]
    fn fastp_self_test_single() {
        // from fastp's AdapterTrimmer::test()
        let read = b"TTTTAACCCCCCCCCCCCCCCCCCCCCCCCCCCCAATTTTAAAATTTTCCCCGGGG";
        let pos = AdapterLikeFastP::fastp_trim_position(read, b"TTTTCCACGGGGATACTACTG");
        assert_eq!(
            &read[..pos.unwrap()],
            b"TTTTAACCCCCCCCCCCCCCCCCCCCCCCCCCCCAATTTTAAAA"
        );
    }

    #[test]
    fn negative_start_trims_everything() {
        // adapter missing its first 3 bases at the read start
        let adapter = b"AGATCGGAAGAGCACACGTCTGAACTCCAGTCA";
        let read = &adapter[3..];
        assert_eq!(
            AdapterLikeFastP::fastp_trim_position(read, adapter),
            Some(0)
        );
    }

    #[test]
    fn no_hit() {
        assert_eq!(
            AdapterLikeFastP::fastp_trim_position(b"CCCCCCCCCCCCCCCCCCCC", b"AGATCGGAAGAGC"),
            None
        );
        // too short to scan at all
        assert_eq!(
            AdapterLikeFastP::fastp_trim_position(b"AGAT", b"AGATCGGAAGAGC"),
            None
        );
    }
}
