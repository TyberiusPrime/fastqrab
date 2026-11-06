---
weight: 3
not-a-transformation: true
---

# Input section

The `[input]` table enumerates all read sources that make up a fragment.
At least one segment must be declared.

```toml
[input]
    read1 = ['fileA_1.fastq', 'fileB_1.fastq.gz', 'fileC_1.fastq.zst'] # required: one or more paths
    # read2 = "fileA_2.fastq.gz"                                      # optional
    # index1 = ['index1_A.fastq.gz']                                   # optional
    # note that if using multiple files per segment, 
    # all segments must have the same number of files
    # interleaved = [...]                                            # optional, see below
```

| Key         | Required | Value type       | Notes |
|-------------|----------|------------------|-------|
| segment name (e.g. `read1`) | Yes (at least one) | string or array of strings | Each unique key defines a segment; arrays concatenate multiple files in order. |
| `interleaved` | No | array of strings | Enables interleaved reading; must list segment names in their in-file order. |

Additional points:

- fastqrab handles up to 256 [segments per read]({{< relref "docs/concepts/segments.md" >}})
- Segment names are user-defined and case sensitive. 
  Common conventions include `read1`, `read2`, `index1`, and `index2`. 
  They must conform to `[a-zA-Z0-9_]+$`.
- Compression is auto-detected for by inspecting file headers.
- Supported file formats are FASTQ, FASTA, and BAM. See [Input options](#input-options) below for format-specific settings.
- Every segment must provide the same number of reads. Cardinality mismatches raise a validation error.
- Multiple files per segment are concatenated virtually; the processor streams them sequentially.
- The names 'All', 'options' and 'interleaved' can not be used for segment names.

## File Formats

fastqrab supports multiple input formats with automatic detection
and transparent decompression.

### Supported Formats

| Format | Detection Method | Compression Support | Notes |
|--------|------------------|---------------------|-------|
| **FASTQ** | First byte (after decompression) is `@` | Raw, Gzip, Zstd | Primary format, fully optimized parser |
| **FASTA** | First byte (after decompression) is `>` | Raw, Gzip, Zstd | Converted to FASTQ with synthetic quality scores |
| **BAM** | Magic bytes `BAM\x01` | Built-in (BAM format) | Aligned and unaligned reads supported |

### Compression Formats

Compression is automatically detected by examining file headers—no need to specify format explicitly:

- **Raw** (uncompressed): `.fastq`, `.fq`, `.fasta`, `.fa`
- **Gzip**: `.gz`, `.gzip` (most common)
- **Zstandard**: `.zst`, `.zstd` (faster compression/decompression)

### FASTQ Format Requirements

FASTQ files should follow the standard format described by [Cock et al. (2010)](https://academic.oup.com/nar/article/38/6/1767/3112533):

```
@read_name optional_comment
ACGTACGTACGT
+
IIIIIIIIIIII
```

- **Line 1**: `@` followed by read identifier, optionally with comments after a separator (default: space)
- **Line 2**: DNA/RNA sequence (A, C, G, T, N, and IUPAC ambiguity codes)
- **Line 3**: `+` optionally followed by repeat of identifier (content ignored)
- **Line 4**: Quality scores (Phred+33 encoding standard)

**Line endings**: Both Unix (`\n`) and Windows (`\r\n`) line endings are automatically detected and handled correctly.

- No multi-line sequence / quality data ('wrapped FASTQ') is supported!

### FASTA Format

FASTA files are converted to FASTQ format for processing:

- Sequences are read normally
- Quality scores are synthesized using the `fasta_fake_quality` setting
- All downstream processing treats them as FASTQ
- Multi-line sequence data (wrapped FASTA) is supported, the whitespace is removed
  in processing

Required configuration when using FASTA:

```toml
[input.options]
    fasta_fake_quality = 'I'  # or numeric value (33-126)
```

The quality character should be chosen based on your quality filtering requirements. Common values:
- `'I'` (73): High quality (Q40)
- `'?'` (63): Medium quality (Q30)
- `'!'` (33): Minimum quality (Q0)

### BAM Format

BAM files (Binary Alignment Map) are supported with flexible filtering:

```toml
[input.options]
    bam_include_mapped = true      # Include aligned reads
    bam_include_unmapped = true    # Include unaligned reads
```

Both settings must be specified when using BAM input. At least one must be `true`.

**Use cases:**
- Extract unmapped reads from aligned BAM files for reanalysis
- Process all reads (mapped + unmapped) together
- Filter only aligned reads for downstream analysis

Quality scores are extracted directly from BAM records. Sequences are output in their stored orientation (may be reverse-complemented if aligned to reverse strand).

### Parser Architecture

For technical details about how parsing works, including the zero-copy design and handling of compressed files,
see [Parser Architecture]({{< relref "docs/development/parser-architecture.md" >}}).

## Input options

Format-specific behaviour is configured via the optional `[input.options]` table.
These knobs are required when the corresponding file types are present and ignored otherwise.

```toml # ignore_in_test
[input]
    read1 = ["reads.fasta"]

[input.options]
    threads_per_segment = 1       # (optional) how many threads to use for decompression. 
    fasta_fake_quality = 'a'      # required for FASTA inputs: synthetic Phred score to apply to every base. Used verbatim without further shifting.
    bam_include_mapped = true     # required for BAM inputs: include reads with a reference assignment
    bam_include_unmapped = true   # required for BAM inputs: include reads without a reference assignment
	read_comment_character = ' '       # defaults to ' '. The character seperating read name from the 'read comment'.
```

- `threads_per_segment` - see [threading]({{< relref "docs/reference/threading.md" >}}).
- `fasta_fake_quality` accepts a byte character or a number and is used verbatim. Stick to Phred ('!'/33 = worst).
  The value must be supplied whenever any FASTA source is detected.
- `bam_include_mapped` and `bam_include_unmapped` must both be defined when reading BAM files. At least one of them has to be `true`; disabling both would discard every record.
- Format detection is automatic and based on magic bytes: BAM (`BAM\x01`), FASTA (`>`), and FASTQ (`@`).
- The read_comment_char is used for input reads
    (e.g. when [`TagDeduplicate`]({{< relref "docs/reference/tag-steps/tag/TagDuplicates.md" >}}) with a name: source).
    The output step ([`StoreTagInComment`]({{< relref "docs/reference/tag-steps/using/StoreTagInComment.md" >}}) default to this setting, but allow overwriting.

## Interleaved input

Some data-sets store all segments in a single file.
Activate interleaved mode and describe how the segments are ordered:

```toml
[input]
    source = ['interleaved.fq'] # this 'virtual' segment will not be available for steps downstream
    interleaved = ["read1", "read2", "index1", "index2"]
```

Rules for interleaving:

- The `[input]` table must contain exactly **one** data source when `interleaved` is present.
- The `interleaved` list dictates how reads are grouped into fragments. The length of the list equals the number of segments.
- Downstream steps reference the declared segment names exactly as written in the list.


## Automatic segment (pair) name checking.

By default, if multiple segments are defined, every 1000th read pair is checked for the read name prefix
(up until the first /) matching, ensuring correctly paired reads.

This assumes Illumina style named reads ending e.g. '/1' and '/2'.

The automatism can be disabled with

```toml # ignore_in_test
[options]
    spot_check_read_pairing = false
```

To influence the character that delimits the read name prefix, or the sampling rate,
add an explicit [`ValidateReadPairing`]({{< relref "docs/reference/validation-steps/ValidateReadPairing.md" >}}) step.


## Named pipe input
Input files may be named pipes (FIFOs) - but only FASTQ formated data is supported in that case.


## Rapidgzip

fastqrab ships its own parallel gzip decompressor, `fastqrab-decompressor`, which
enables multi-core decompression of arbitrary gzip files instead of the built-in
single-threaded gzip decompressor. (The implementation is inspired by
[rapidgzip](https://github.com/mxmlnkn/rapidgzip), hence the option name.)

Since gzip decompression is often the single largest bottleneck in FASTQ processing,
this offers massive speed advantages.

The decompressor runs as a separate process, so all of the (necessarily `unsafe`)
decoder code is isolated from the main fastqrab process. The `fastqrab-decompressor`
binary must sit right next to the `fastqrab` binary: it is located by suffixing the
running executable's name with `-decompressor` (so `fastqrab` looks for
`fastqrab-decompressor`, and a renamed `fastqrab_0.9.1` looks for
`fastqrab_0.9.1-decompressor`).

By default, we use it if that binary is found next to fastqrab and there are
at least two threads available per segment for decompression (benchmarking indicates
parallel decompression is slower than our built-in gzip decompression otherwise).

You can force its use by setting `options.use_rapidgzip` to true; in that case a missing
decompressor binary will lead to an error. Likewise, you can disable it by setting it to false.


## Reading form stdin

You can read fastq & fasta input from stdin using `--stdin--` as file name.

For this to work, you must have only a single segment (or define interleaved input),
and `--stdin--` must be the only 'file name' provided.
