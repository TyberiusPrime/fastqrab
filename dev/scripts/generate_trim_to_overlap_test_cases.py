#!/usr/bin/env python3
"""Generate test cases for TrimToOverlap.

Usage:
    python dev/scripts/generate_trim_to_overlap_test_cases.py

Read pairs are sampled from random inserts, so the expected overlap follows from
the construction (insert coordinates), independent of the implementation:

    read1 covers insert[0 : min(I, L1)]          (+ adapter if I < L1)
    read2 covers insert[max(0, I - L2) : I]      (reverse complemented, + adapter if I < L2)
    overlap     = insert[max(0, I - L2) : min(I, L1)]

Overlaps are kept clear of min_overlap (>= min_overlap + 3, or < min_overlap - 3)
so fastp's scan boundaries don't matter. Read2 receives up to
max_mismatch_count substitutions inside the overlap, which must be preserved.
"""

import random
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
TARGET = ROOT / "test_cases" / "single_step" / "edits" / "trim_to_overlap"

MIN_OVERLAP = 10
MAX_MISMATCH_COUNT = 2

COMP = {"A": "T", "C": "G", "G": "C", "T": "A", "N": "N"}


def rc(s):
    return "".join(COMP[c] for c in reversed(s))


def rand_seq(rng, n):
    return "".join(rng.choice("ACGT") for _ in range(n))


def rand_qual(rng, n):
    return "".join(rng.choice("#+5?FIJ") for _ in range(n))


def make_pair(rng):
    """Returns (r1, q1, r2, q2, expected (r1_window, r2_window) or None)"""
    while True:
        l1 = rng.randint(20, 100)
        l2 = rng.randint(20, 100)
        insert_len = rng.randint(5, l1 + l2 + 20)
        ov_start = max(0, insert_len - l2)
        ov_end = min(insert_len, l1)
        ov = ov_end - ov_start
        if ov >= MIN_OVERLAP + 3 or ov < MIN_OVERLAP - 3:
            break
    insert = rand_seq(rng, insert_len)
    r1 = (insert[:l1] + rand_seq(rng, l1))[:l1]  # adapter read-through if insert is short
    r2_fwd = insert[ov_start:insert_len]  # read2 in read1 orientation
    # read2 = rc(insert end) followed by adapter
    r2 = (rc(r2_fwd) + rand_seq(rng, l2))[:l2]
    expected = None
    if ov >= MIN_OVERLAP:
        r2 = list(r2)
        # read2 index of insert position p: rc(read2)[k] = insert[ov_start + k] for
        # k < len(r2_fwd), and read2[i] = comp(rc(read2)[l2 - 1 - i])
        # -> the overlap occupies read2[len(r2_fwd) - (ov_end - ov_start) : len(r2_fwd)]
        r2_start = len(r2_fwd) - ov
        for _ in range(rng.randint(0, MAX_MISMATCH_COUNT)):
            i = rng.randrange(r2_start, r2_start + ov)
            r2[i] = rng.choice([c for c in "ACGT" if c != r2[i]])
        r2 = "".join(r2)
        expected = ((ov_start, ov), (r2_start, ov))
    return r1, rand_qual(rng, l1), r2, rand_qual(rng, l2), expected


def write_fastq(path, reads):
    with open(path, "w") as op:
        for name, seq, qual in reads:
            op.write(f"@{name}\n{seq}\n+\n{qual}\n")


TOML = """\
# expected output derived from the read construction in
# dev/scripts/generate_trim_to_overlap_test_cases.py
[input]
    read1 = 'input_read1.fq'
    read2 = 'input_read2.fq'

[[step]]
    action = 'TrimToOverlap'
    algorithm = 'Fastp'
    min_overlap = {min_overlap}
    max_mismatch_rate = 0.2
    max_mismatch_count = {max_mismatch_count}
    reverse_complement_segment2 = {rc}
    segment1 = 'read1'
    segment2 = 'read2'
    out_label = 'was_trimmed'

[[step]]
    action = 'StoreTagsInTable'
    in_labels = ['was_trimmed']

[output]
    prefix = 'output'

[[step]]
    action = 'OutputFASTQ'
"""


def generate(name, seed, n, reverse_complement_segment2):
    rng = random.Random(seed)
    pairs = [make_pair(rng) for _ in range(n)]
    # edge cases: empty read2, read2 shorter than min_overlap, both empty
    pairs.append(("ACGTACGTACGTACGTACGT", "I" * 20, "", "", None))
    pairs.append(("ACGTACGTACGTACGTACGT", "I" * 20, "CGTACG", "IIIIII", None))
    pairs.append(("", "", "", "", None))
    in1, in2, out1, out2, table = [], [], [], [], ["ReadName\twas_trimmed"]
    for ii, (r1, q1, r2, q2, expected) in enumerate(pairs):
        if not reverse_complement_segment2:
            # read2 already in read1 orientation (quality reversed along)
            r2, q2 = rc(r2), q2[::-1]
            if expected is not None:
                (s1, ov), (s2, _) = expected
                expected = ((s1, ov), (len(r2) - s2 - ov, ov))
        read_name = f"pair_{ii}"
        in1.append((read_name, r1, q1))
        in2.append((read_name, r2, q2))
        if expected is None:
            out1.append((read_name, r1, q1))
            out2.append((read_name, r2, q2))
        else:
            (s1, l1), (s2, l2) = expected
            out1.append((read_name, r1[s1 : s1 + l1], q1[s1 : s1 + l1]))
            out2.append((read_name, r2[s2 : s2 + l2], q2[s2 : s2 + l2]))
        table.append(f"{read_name}\t{1 if expected is not None else 0}")
    out_dir = TARGET / name
    out_dir.mkdir(parents=True, exist_ok=True)
    write_fastq(out_dir / "input_read1.fq", in1)
    write_fastq(out_dir / "input_read2.fq", in2)
    write_fastq(out_dir / "output_read1.fq", out1)
    write_fastq(out_dir / "output_read2.fq", out2)
    (out_dir / "output.tsv").write_text("\n".join(table) + "\n")
    (out_dir / "input.toml").write_text(
        TOML.format(
            min_overlap=MIN_OVERLAP,
            max_mismatch_count=MAX_MISMATCH_COUNT,
            rc="true" if reverse_complement_segment2 else "false",
        )
    )
    print(name, len(pairs), sum(p[4] is not None for p in pairs), "overlapping")


def main():
    generate("generated_revcomp", 1234, 300, True)
    generate("generated_no_revcomp", 4321, 300, False)


if __name__ == "__main__":
    main()
