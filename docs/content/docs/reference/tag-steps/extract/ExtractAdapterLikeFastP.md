---
title: Extract adapter like fastp
weight: 52
---
# ExtractAdapterLikeFastP


```toml
[[step]]
    action = "ExtractAdapterLikeFastP"
    out_label = "adapter"
    adapter = "AGATCGGAAGAGCACACGTCTGAACTCCAGTCA" # A, C, G, T only, >= 4 bases
    segment = "read1"   # Any of your input segments (default: read1)

[[step]]
    action = "TrimAtTag"
    in_label = "adapter"
    direction = "End"
    keep_tag = false
```

Find a 3' adapter exactly like [fastp](https://github.com/OpenGene/fastp) (v1.4.0) does
when given a single `--adapter_sequence` (`AdapterTrimmer::trimBySequence`),
and store the region from the adapter start to the read end as a location tag.

The search, in fastp's order, stopping at the first hit:

1. Hamming-distance match, scanning from the read start, allowing one mismatch per 8 compared
   bases. The adapter must start at least 5 bases before the read end - a partial
   adapter of only 1-4 bases at the 3' end is never found (and the last 5 bases must
   then match exactly, since 5 / 8 rounds down to 0 allowed mismatches).
   Adapters >= 8 / 12 / 16 bp also match at the start of the read with their
   first 2 / 3 / 4 bases missing (Illumina A-tailing), which covers the whole read.
2. Matching with one base inserted in the read.
3. Matching with one base deleted from the read.

The indel searches (steps 2 & 3) only allow `compared_length / 8 - 1` mismatches
and therefore need at least 8 compared bases (zero mismatches allowed for 8-15). Faithful to fastp, they always compare the adapter
against the *start* of the read, regardless of the position being tested.

Comparison is byte-exact (as in fastp), so lowercase or `N` bases in the read count as mismatches.

Trim with [TrimAtTag]({{< relref "docs/reference/modification-steps/TrimAtTag.md" >}}).
Note that fastp drops reads that are empty after trimming - add a
[FilterEmpty]({{< relref "docs/reference/filter-steps/FilterEmpty.md" >}}) step to replicate that.

For partial adapter matches with configurable parameters, see
[ExtractIUPACSuffix]({{< relref "docs/reference/tag-steps/extract/ExtractIUPACSuffix.md" >}}).

For complete adapter matches, see
[ExtractIUPAC]({{< relref "docs/reference/tag-steps/extract/ExtractIUPAC.md" >}}).

