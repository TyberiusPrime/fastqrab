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

The `direction` parameter determines whether to trim from the start or end of the tag,
and `keep_tag` determines whether the tag sequence itself is retained.

Only extracted regions with exactly one hit are supported.

Has an implicit `if_tag` based on it's in_label, so reads that do not have the tag 
are not modified by `TrimAtTag`.
