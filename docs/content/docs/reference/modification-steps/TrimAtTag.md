---
weight: 50
---

# TrimAtTag

Trim the read at the position of a tag.

```toml
[[step]]
    action = "TrimAtTag"
    in_label = "mytag" # must be a location tag
    direction = "Start" # or "End"
    keep_tag = false # if true, the tag sequence is kept in the read
    if_tag = "other_tag" # (optional) only trim reads where this tag is true (prefix with ! to negate)
```

This transformation trims the read at the position where a tag was found.

`direction` names the part of the read that is **removed**:

- `Start`: everything *before* the tag is removed; the part after the tag is kept.
- `End`: everything *after* the tag is removed; the part before the tag is kept.

`keep_tag` decides whether the tag's own bases survive the cut
(`true`) or are removed along with the trimmed part (`false`).
Quality scores are trimmed together with the sequence (as always).

## Examples

Given a read where the tag `mytag` matched `AGATC`:

```text
read:   TTTTTAGATCGGGGG
             ^^^^^
             mytag
```

| `direction` | `keep_tag` | removed      | result       |
|-------------|------------|--------------|--------------|
| `Start`     | `false`    | `TTTTTAGATC` | `GGGGG`      |
| `Start`     | `true`     | `TTTTT`      | `AGATCGGGGG` |
| `End`       | `false`    | `AGATCGGGGG` | `TTTTT`      |
| `End`       | `true`     | `GGGGG`      | `TTTTTAGATC` |

Typical uses:

- Remove a 3' adapter and everything following it:
  `direction = "End"`, `keep_tag = false`.
- Remove a 5' primer/linker and everything preceding it, so the read starts
  right after it: `direction = "Start"`, `keep_tag = false`.

Only extracted regions with exactly one hit are supported.

Has an implicit `if_tag` based on its `in_label`, so reads that do not have the tag 
are not modified by `TrimAtTag`.
