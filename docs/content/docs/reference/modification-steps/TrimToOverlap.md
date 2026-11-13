# TrimToOverlap

```toml
[[step]]
    action = "TrimToOverlap"

    segment1 = "read1"                    # First segment
    segment2 = "read2"                    # Second segment
    reverse_complement_segment2 = true    # Reverse complement segment2 for the overlap search (suggested: true)

    algorithm = "Fastp"                   # Overlap detection algorithm (same as MergeReads)
    trim_mode = "AnyOverlap"              # "AnyOverlap" or "ReadThrough" (see below)
    min_overlap = 30                      # Minimum overlap length required (>= 5)
    max_mismatch_rate = 0.2               # Maximum allowed mismatch rate (0.0-1.0) (suggested: 0.2)
    max_mismatch_count = 5                # Maximum allowed absolute mismatches (suggested: 5)

    # out_label = "was_trimmed"           # (optional) bool tag: true if the reads were trimmed
```

The companion to [MergeReads]({{< relref "docs/reference/modification-steps/MergeReads.md" >}}):
finds the overlap of a read pair exactly like MergeReads does, but instead of merging the reads,
trims both segments to the overlapping bases. Mismatches in the overlap are preserved
(each read keeps its own bases and qualities), and segment2 stays in its original orientation.

`trim_mode` decides which overlapping pairs are trimmed:

- `AnyOverlap`: every pair with an overlap is trimmed to exactly the overlapping bases.
- `ReadThrough`: only pairs whose reads run past the end of the fragment into adapter
  (read2 starts before read1 - the fragment is shorter than the reads) are trimmed.
  Pairs with a regular overlap (fragment at least as long as the reads) are left alone.
  This reproduces fastp's overlap-based adapter trimming, including its detail of keeping
  the first 'overlap length' bases of both reads (which, when read1 is shorter than the
  fragment, is read2's 5' end rather than the bases overlapping read1).
  (fastp additionally trims by adapter sequence afterwards, see
  [ExtractAdapterLikeFastP]({{< relref "docs/reference/tag-steps/extract/ExtractAdapterLikeFastP.md" >}}).)

If no overlap is found, both reads are left unchanged.
Overlaps shorter than `min_overlap` (fastp reports those when read2 is shorter than `min_overlap`) count as no overlap.

Location tags on either segment survive the trimming (they are moved along, or lost per read if their bases were cut away).

If `out_label` is set, a boolean tag is created that is true for read pairs that were trimmed
(`AnyOverlap`: had an overlap, `ReadThrough`: ran into adapter), which you can use with `if_tag` or
[Demultiplex]({{< relref "docs/reference/Demultiplex.md" >}}).
