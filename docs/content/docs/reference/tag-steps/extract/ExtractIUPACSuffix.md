---
title: Extract IUPAC suffix
weight: 51
---
# ExtractIUPACSuffix


```toml
[[step]]
    action = "ExtractIUPACSuffix"
    out_label = "mytag"
    search = "AGTCA"  # the adapter(s) to trim. Full IUPAC, lists of same-length queries allowed
    segment = "read1"   # Any of your input segments (default: read1)
    min_length = 3     # uint, the minimum length of match between the end of the read and
                       # the start of the adapter
    max_mismatches = 0 # required. How many mismatches to accept
```

Find one or more potentially truncated [IUPAC
strings](https://doi.org/10.1093%2Fnar%2F13.9.3021) sequence at the end of a
read.

Simple comparison with a max mismatch hamming distance, 
requiring only the first min length bases of the query to match at the end of the read.


Tested for each 'search' entry separately, the longest suffix within the given
`max_mismatches` is chosen.

Trim with [TrimAtTag]({{< relref
"docs/reference/modification-steps/TrimAtTag.md" >}}) if you want to remove the
found suffix.
