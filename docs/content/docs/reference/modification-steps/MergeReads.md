# MergeReads

```toml
[[step]]
    action = "MergeReads"

    segment1 = "read1"                    # First segment
    segment2 = "read2"                    # Second segment
    reverse_complement_segment2 = true    # Whether to reverse complement segment2 (suggested: true)

    algorithm = "FastpSeemsWeird"                   # Algorithm: "fastp_seems_weird". Further algorithms are in planning
    min_overlap = 30                      # Minimum overlap length required
    max_mismatch_rate = 0.2               # Maximum allowed mismatch rate (0.0-1.0) (suggested: 0.2)
    max_mismatch_count = 5                # Maximum allowed absolute mismatches (suggested: 5)
    no_overlap_strategy = "as_is"         # What to do when no overlap found: "as_is" or "concatenate"
    concatenate_spacer = "NNNN"           # (optional) Required if no_overlap_strategy = "concatenate". Spacer sequence to insert between reads
    spacer_quality_char = 33              # (optional) Quality score for spacer bases (suggested: 33 = Phred quality 0)

    # out_label = "merged"                      # (optional) output Tag label for boolean merge status
```

Merges paired-end reads from two segments by detecting their overlap and resolving mismatches.

To keep both reads, trimmed to just their overlap, see
[TrimToOverlap]({{< relref "docs/reference/modification-steps/TrimToOverlap.md" >}}).

Eventually will support multiple algorithms. Currently supports the ['fastp'](https://github.com/OpenGene/fastp) algorithm.

## How it works

1. Searches for overlap between segment1 and the (reverse complemented) segment2
3. If overlap is found:
   - Merges the reads 
   - Places merged sequence in segment1
   - Empties segment2 (ie. leaves an empty read)
4. If no overlap is found:
   - **as_is**: Leaves both segments unchanged
   - **concatenate**: Joins segment1 + spacer + processed segment2 into segment1, empties segment2
5. If `out_label` is specified, creates a boolean tag indicating merge status (true=merged, false=not merged),
   which you can [Demultiplex]({{< relref "docs/reference/Demultiplex.md" >}}) on.


## Parameters

- **segment1**, **segment2** (required): Names of segments to merge. 
- **reverse_complement_segment2** (required): Whether to reverse complement segment2 before processing.
- **algorithm** (required): Algorithm to use for overlap scoring and mismatch resolution. Options:
  - `"fastp_seems_weird"`: Reimplementation of the fastp algorithm. See below 
- **no_overlap_strategy** (required): Strategy when no overlap is detected:
  - `"as_is"` (suggested): Leave reads unchanged
  - `"concatenate"`: Join reads with spacer sequence
- **label** (optional): Tag label to store merge status as a boolean tag. If specified, creates a tag where `true` indicates the reads were merged and `false` indicates they were not merged. This tag can be used later for demultiplexing or filtering.
- **concatenate_spacer** (optional, required if no_overlap_strategy = "concatenate"): Sequence to insert between reads when concatenating. Common values: `""` (empty), `"NNNN"`.
- **spacer_quality_char** (optional): ASCII quality score character for spacer bases. Suggested: 33 (Phred quality 0).


### Fastp parameters

- **min_overlap** (required): Minimum number of overlapping bases required for merging. Suggested: 30.

- **max_mismatch_rate** (required): Maximum allowed mismatch rate in the overlap region (0.0 = perfect match, 1.0 = allow all mismatches). Suggested: 0.2 (20%).

- **max_mismatch_count** (required): Maximum allowed absolute number of mismatches in the overlap region. Suggested: 5.

## Algorithm notes


### fastp_seems_weird

This is a faithful re-implementation of the fastp algorithm, which is somewhat surprising in it's details.

It prefers to use read1 (segment1) bases, but if the read2 base is better than Phred 30 (ascii '?') 
and the read1 base is worse than Phred 14 (ascii '/'), it uses the read2 base instead. 
(The documentation claims that read1 is always preferred, but merging turns on base correction, 
and that uses these interesting thresholds before merging).

It will also merge reads with more than max_mismatch_rate/count if the first 50 bp of overlap are below these thresholds,
which might be surprising to the user.

Our implementation differs when either read1 or read2 is empty. Fastp then filters both reads. We output
a ('merged') read (ie. bases always in segment1 afterwards). If you want to reproduce the fastp behaviour,
include an
[FilterEmpty]({{< relref "docs/reference/filter-steps/FilterEmpty.md" >}})
step.


## Notes.

- Set `reverse_complement_segment2 = true` for standard paired-end reads where read2 is the reverse complement
- Use `concatenate_spacer = ""` for direct concatenation without spacer
