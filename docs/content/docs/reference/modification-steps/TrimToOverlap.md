# TrimToOverlap

```toml
[[step]]
    action = "TrimToOverlap"

    segment1 = "read1"                    # First segment
    segment2 = "read2"                    # Second segment
    reverse_complement_segment2 = true    # Reverse complement segment2 for the overlap search (suggested: true)

    algorithm = "Fastp"                   # Overlap detection algorithm (same as MergeReads)
    min_overlap = 30                      # Minimum overlap length required (>= 5)
    max_mismatch_rate = 0.2               # Maximum allowed mismatch rate (0.0-1.0) (suggested: 0.2)
    max_mismatch_count = 5                # Maximum allowed absolute mismatches (suggested: 5)

    # out_label = "was_trimmed"           # (optional) bool tag: true if an overlap was found
```

The companion to [MergeReads]({{< relref "docs/reference/modification-steps/MergeReads.md" >}}):
finds the overlap of a read pair exactly like MergeReads does, but instead of merging the reads,
trims both segments to the overlapping bases. Mismatches in the overlap are preserved
(each read keeps its own bases and qualities), and segment2 stays in its original orientation.

If no overlap is found, both reads are left unchanged.
Overlaps shorter than `min_overlap` (fastp reports those when read2 is shorter than `min_overlap`) count as no overlap.

Location tags on either segment survive the trimming (they are moved along, or lost per read if their bases were cut away).

If `out_label` is set, a boolean tag is created that is true for read pairs that had an overlap
(and were trimmed to it), which you can use with `if_tag` or
[Demultiplex]({{< relref "docs/reference/Demultiplex.md" >}}).
