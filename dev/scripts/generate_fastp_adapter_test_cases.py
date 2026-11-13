#!/usr/bin/env python3
"""Generate test cases for ExtractAdapterLikeFastP using the real fastp binary.

Usage:
    python dev/scripts/generate_fastp_adapter_test_cases.py /path/to/fastp [n_reads_per_case] [n_reads_per_adapter_large]

The "large" cases share one input (reads built around 12 different adapters) and
compare only the per-read cut position against positions derived from fastp output.

The expected outputs were generated with fastp v1.4.0 (vendored source in .fastp_source).

For each adapter, a deterministic (seeded) set of reads is generated that exercises
fastp's AdapterTrimmer::trimBySequence code paths:
  - adapter at random positions, full or truncated at the read end
  - adapter missing its first 1..5 bases at the read start (negative positions)
  - substitutions, a single inserted / deleted base inside the adapter
  - very short reads, reads without adapter, N bases, lowercase reads

fastp is run with every other trimming/filtering step disabled. fastp drops reads
that are empty after trimming, so the fastqrab pipeline in input.toml ends
with FilterEmpty to produce the same output.
"""

import random
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
TARGET = ROOT / "test_cases" / "single_step" / "extraction" / "extract_adapter_like_fastp"

ADAPTERS = {
    # name: adapter - lengths chosen to hit fastp's different negative start offsets
    "adapter_33bp": "AGATCGGAAGAGCACACGTCTGAACTCCAGTCA",  # >= 16 -> start -4
    "adapter_16bp": "AGATCGGAAGAGCACA",  # boundary -> start -4
    "adapter_13bp": "AGATCGGAAGAGC",  # 12..15 -> start -3
    "adapter_10bp": "TGGAATTCTC",  # 8..11 -> start -2
    "adapter_6bp": "AGATCG",  # < 8 -> start 0
}

FASTP_ARGS = [
    "-Q",  # no quality filtering
    "-L",  # no length filtering
    "-G",  # no polyG trimming
    "--dimer_max_len",
    "-1",  # never classify as adapter dimer (would drop the read)
    "-w",
    "1",
]

TOML = """\
[input]
    read1 = 'input_read1.fq'

[[step]]
    action = 'ExtractAdapterLikeFastP'
    out_label = 'adapter'
    adapter = '{adapter}'

[[step]]
    action = 'TrimAtTag'
    in_label = 'adapter'
    direction = 'End'
    keep_tag = false

# fastp drops reads that are empty after trimming
[[step]]
    action = 'FilterEmpty'
    segment = 'read1'

[output]
    prefix = 'output'

[[step]]
    action = 'OutputFASTQ'
"""


def rand_seq(rng, n):
    return "".join(rng.choice("ACGT") for _ in range(n))


def mutate_adapter(rng, adapter):
    a = list(adapter)
    # skip leading bases (A-tailing etc.)
    if rng.random() < 0.15:
        a = a[rng.randint(1, min(5, len(a) - 1)) :]
    # substitutions
    n_subs = rng.choice([0, 0, 0, 1, 1, 2, 3, 4])
    for _ in range(n_subs):
        if not a:
            break
        i = rng.randrange(len(a))
        a[i] = rng.choice([c for c in "ACGT" if c != a[i]])
    # a single indel
    r = rng.random()
    if r < 0.15 and len(a) > 2:
        a.insert(rng.randint(1, len(a) - 1), rng.choice("ACGT"))
    elif r < 0.30 and len(a) > 2:
        del a[rng.randint(1, len(a) - 2)]
    return "".join(a)


def make_read(rng, adapter):
    kind = rng.random()
    if kind < 0.05:
        read_len = rng.randint(1, 8)
    elif kind < 0.25:
        read_len = rng.randint(9, 40)
    else:
        read_len = rng.randint(41, 151)

    style = rng.random()
    if style < 0.1:
        seq = rand_seq(rng, read_len)  # no adapter
    elif style < 0.2:
        # adapter at the very start (dimer-like)
        seq = mutate_adapter(rng, adapter) + rand_seq(rng, read_len)
    else:
        insert_len = rng.randint(0, read_len)
        seq = (
            rand_seq(rng, insert_len)
            + mutate_adapter(rng, adapter)
            + rand_seq(rng, read_len)
        )
    seq = seq[:read_len]
    if rng.random() < 0.05:
        seq = "".join(c if rng.random() > 0.1 else "N" for c in seq)
    if rng.random() < 0.02:
        seq = seq.lower()
    qual = "".join(rng.choice("#+5?FIJ") for _ in seq)
    return seq, qual


def handcrafted(adapter):
    a = adapter
    seqs = [
        a,  # adapter only
        a[1:],  # adapter missing first base
        a[2:],
        a[3:],
        a[4:],
        a[5:],
        "CCCCC" + a,
        "CCCCCCCCCCCCCCCCCCCC" + a[:5],  # minimal partial (5 bases)
        "CCCCCCCCCCCCCCCCCCCC" + a[:4],  # too short to be found
        "CCCCCCCCCCCCCCCCCCCC" + a[:6],
        "CCCCCCCCCCCCCCCCCCCC" + a + "GGGGGGGGGG",
        "CCCCCCCCCCCCCCCCCCCC" + a[:8] + "T" + a[8:] + "GGGG",  # insertion
        "CCCCCCCCCCCCCCCCCCCC" + a[:8] + a[9:] + "GGGG",  # deletion
        a[:8] + "T" + a[8:] + "GGGG",  # insertion at read start
        a[:8] + a[9:] + "GGGG",  # deletion at read start
        "ACGT",
        "A",
    ]
    return [(s, "I" * len(s)) for s in seqs]


def write_fastq(path, reads, prefix):
    with open(path, "w") as op:
        for ii, (seq, qual) in enumerate(reads):
            op.write(f"@{prefix}_{ii}\n{seq}\n+\n{qual}\n")


def run_fastp(fastp, adapter, input_fq, output_fq):
    with tempfile.TemporaryDirectory() as tmp:
        subprocess.check_call(
            [
                fastp,
                "-i",
                str(input_fq),
                "-o",
                str(output_fq),
                "-a",
                adapter,
                *FASTP_ARGS,
                "-j",
                str(Path(tmp) / "fastp.json"),
                "-h",
                str(Path(tmp) / "fastp.html"),
            ],
            stderr=subprocess.DEVNULL,
        )


LARGE_ADAPTERS = dict(ADAPTERS)
LARGE_ADAPTERS.update(
    {
        "adapter_4bp": "AGAT",
        "adapter_7bp": "AGATCGG",
        "adapter_8bp": "AGATCGGA",
        "adapter_11bp": "AGATCGGAAGA",
        "adapter_12bp": "AGATCGGAAGAG",
        "adapter_15bp": "AGATCGGAAGAGCAC",
        "nextera_19bp": "CTGTCTCTTATACACATCT",
    }
)

LARGE_TOML = """\
[input]
    read1 = 'input_read1.fq.zst' # symlink to ../shared_input_read1.fq.zst

[[step]]
    action = 'ExtractAdapterLikeFastP'
    out_label = 'adapter'
    adapter = '{adapter}'

# position fastp cuts the read at, -1 if no adapter was found
[[step]]
    action = 'EvalExpression'
    out_label = 'pos'
    expression = 'adapter * (len_read1 - len_adapter) - (adapter == 0)'
    result_type = 'Numeric'

[[step]]
    action = 'StoreTagsInTable'
    infix = 'positions'
    in_labels = ['pos']
    include_read_name = false
    compression = 'Zstd'
    compression_level = 5

[output]
    prefix = 'output'
"""


def parse_fastq(path):
    lines = Path(path).read_text().split("\n")
    return [(lines[i][1:], lines[i + 1]) for i in range(0, len(lines) - 3, 4)]


def fastp_positions(input_reads, fastp_output):
    """Derive the cut position from fastp's output.

    fastp only ever truncates the read end, and never at the read length itself
    (the scan stops 5 bases before the end), so:
    same length -> no adapter (-1), shorter -> cut position, missing -> cut at 0
    (except for empty input reads, which fastp drops anyway -> -1).
    """
    out = dict(parse_fastq(fastp_output))
    res = []
    for name, seq in input_reads:
        if not seq:
            res.append(-1)  # fastp drops empty reads regardless - nothing to find
        elif name not in out:
            res.append(0)
        elif len(out[name]) == len(seq):
            res.append(-1)
        else:
            res.append(len(out[name]))
    return res


def generate_large(fastp, version, n):
    """One shared input (reads built around every adapter), each adapter tested on all of it.
    Expected output: the position fastp cuts at, per read."""
    large = TARGET / "large"
    large.mkdir(parents=True, exist_ok=True)
    reads = []
    for ii, (name, adapter) in enumerate(LARGE_ADAPTERS.items()):
        rng = random.Random(9000 + ii)
        reads.extend(handcrafted(adapter))
        reads.extend(make_read(rng, adapter) for _ in range(n))
    # quality is irrelevant to adapter detection - keep it constant to compress well
    reads = [(seq, "I" * len(seq)) for seq, _ in reads]
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        write_fastq(tmp / "input.fq", reads, "r")
        names = [(f"r_{ii}", seq) for ii, (seq, _) in enumerate(reads)]
        shared = large / "shared_input_read1.fq.zst"
        subprocess.check_call(["zstd", "-q", "-f", "-19", str(tmp / "input.fq"), "-o", str(shared)])
        for name, adapter in LARGE_ADAPTERS.items():
            run_fastp(fastp, adapter, tmp / "input.fq", tmp / "out.fq")
            positions = fastp_positions(names, tmp / "out.fq")
            out_dir = large / name
            out_dir.mkdir(exist_ok=True)
            link = out_dir / "input_read1.fq.zst"
            if link.is_symlink() or link.exists():
                link.unlink()
            link.symlink_to("../shared_input_read1.fq.zst")
            table = tmp / "positions.tsv"
            table.write_text("pos\n" + "".join(f"{p}\n" for p in positions))
            subprocess.check_call(
                ["zstd", "-q", "-f", "-5", str(table), "-o", str(out_dir / "output_positions.tsv.zst")]
            )
            (out_dir / "input.toml").write_text(
                f"# expected positions derived from {version} output via\n"
                f"# dev/scripts/generate_fastp_adapter_test_cases.py\n"
                f"# fastp -i input_read1.fq -o out.fq -a {adapter} {' '.join(FASTP_ARGS)}\n"
                + LARGE_TOML.format(adapter=adapter)
            )
            print("large", name, adapter, len(reads), sum(p >= 0 for p in positions))


def main():
    fastp = sys.argv[1]
    n = int(sys.argv[2]) if len(sys.argv) > 2 else 300
    n_large = int(sys.argv[3]) if len(sys.argv) > 3 else 20_000
    version = subprocess.run(
        [fastp, "--version"], capture_output=True, text=True
    )
    version = (version.stdout + version.stderr).strip() or "unknown"
    generate_large(fastp, version, n_large)
    for ii, (name, adapter) in enumerate(ADAPTERS.items()):
        rng = random.Random(4242 + ii)
        reads = handcrafted(adapter) + [make_read(rng, adapter) for _ in range(n)]
        out_dir = TARGET / name
        out_dir.mkdir(parents=True, exist_ok=True)
        write_fastq(out_dir / "input_read1.fq", reads, name)
        run_fastp(fastp, adapter, out_dir / "input_read1.fq", out_dir / "output_read1.fq")
        (out_dir / "input.toml").write_text(
            f"# expected output generated by {version} via\n"
            f"# dev/scripts/generate_fastp_adapter_test_cases.py\n"
            f"# fastp -i input_read1.fq -o output_read1.fq -a {adapter} {' '.join(FASTP_ARGS)}\n"
            + TOML.format(adapter=adapter)
        )
        print(name, adapter, len(reads))


if __name__ == "__main__":
    main()
