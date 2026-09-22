---
weight: 55
---

# PickTag

Pick one value out of several tags per read, according to a policy.

```toml
# ignore_in_test
[[step]]
    action = "PickTag"
    in_labels = ["mytag", "mytag2"]  # tags to choose between (minimum 2, all the same type)
    out_label = "result"  # output tag name
    policy = "first_available"  # "first_available" | "equal_only" | "largest" | "smallest" | "left_most_region"
```

Every tag in `in_labels` must have the same tag type - Location, String or
Numeric (Bool tags are not supported). Unlike 
[`ConcatTags`]({{< relref "docs/reference/tag-steps/using/ConcatTags.md" >}})
or 
[`FillMissing`]({{< relref "docs/reference/modification-steps/FillMissing.md" >}})
`PickTag` does not
convert between types - it rejects at configuration time.

## Policies

### `first_available`

Picks the first tag (in `in_labels`) that has a value for that read.
Location, String or Numeric tags only - Numeric tags use `NaN` as their
"missing" value (same as `equal_only` below).
(You can do a logical operations on boolean tags using
[EvalExpression]({{< relref "docs/reference/tag-steps/convert/EvalExpression.md" >}}) )

```toml
# ignore_in_test
[[step]]
    action = "PickTag"
    in_labels = ["barcode1", "barcode2"]
    out_label = "barcode"
    policy = "first_available"
```

### `equal_only`

Picks a value only when every input tag agrees for that read; otherwise the
output is missing. Location, String or Numeric tags only:
- **Location/ String**: compares the extracted text (not the location).
- **Numeric**: compares the values; disagreement outputs `NaN`.

```toml
# ignore_in_test
[[step]]
    action = "PickTag"
    in_labels = ["caller_a", "caller_b"]
    out_label = "consensus"
    policy = "equal_only"
```

### `largest`

Location or Numeric tags only. Picks the tag with the largest measure for
that read:
- **Location**: the sum of that tag's region lengths.
- **Numeric**: the value itself.

```toml
# ignore_in_test
[[step]]
    action = "PickTag"
    in_labels = ["hit_a", "hit_b"]
    out_label = "best_hit"
    policy = "largest"
```

### `smallest`

The inverse of `largest` - picks the tag with the smallest measure for that
read. Location or Numeric tags only, same measures as `largest`.

```toml
# ignore_in_test
[[step]]
    action = "PickTag"
    in_labels = ["hit_a", "hit_b"]
    out_label = "shortest_hit"
    policy = "smallest"
```

### `left_most_region`

Location tags only. Picks the tag whose first region starts earliest,
comparing `(segment, start coordinate)` tuples - so this also picks between
tags living on different segments (a lower segment number always wins, no
matter where either hit starts).

```toml
# ignore_in_test
[[step]]
    action = "PickTag"
    in_labels = ["adapter_hit", "primer_hit"]
    out_label = "earliest_hit"
    policy = "left_most_region"
```

## Output type

- If every input tag is Location **and** shares one segment, the output stays
  a Location tag (coordinates preserved).
- If every input tag is Location but they live on different segments, the
  output is a String tag (a single Location tag column can only live on one
  segment), holding the winning tag's extracted text.
- If every input tag is String or Numeric, the output has that same type.

## Example: prefer an early, anchored barcode over a loose one

```toml
# ignore_in_test
[[step]]
    action = "ExtractIUPAC"
    segment = "read1"
    search = "AAAA"
    out_label = "barcode1"
    anchor = "Left"
    max_mismatches = 0

[[step]]
    action = "ExtractIUPAC"
    segment = "read2"
    search = "TTTT"
    out_label = "barcode2"
    anchor = "Anywhere"
    max_mismatches = 0

[[step]]
    action = "PickTag"
    in_labels = ["barcode1", "barcode2"]
    out_label = "barcode"
    policy = "first_available"
```

`barcode1` and `barcode2` live on different segments here, so `barcode`
ends up a String tag: `barcode1`'s text when read1 has a hit, else
`barcode2`'s text, else missing.

## Validation
- Requires at least 2 input tags
- Rejects duplicate input labels
- Requires every input tag to share the same tag type
- `left_most_region` rejects anything but Location tags; `largest` and
  `smallest` reject anything but Location and Numeric tags
