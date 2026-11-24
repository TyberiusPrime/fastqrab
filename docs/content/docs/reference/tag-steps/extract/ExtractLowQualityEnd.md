# ExctractLowQualityEnd


```toml
[[step]]
    action = "ExtractLowQualityEnd"
    out_label = "low_quality_ends"
    min_qual = 20 # u8, minimum quality to keep (in whatever your score is encoded in)
             # either a char like 'A' or a number 0..128 (typical phred score is 33..75)
    segment = "read1" # Any of your input segments
```

Define a region of low quality bases at the end of reads.


## Corresponding options in other software 
- Trimmomatic: TRAILING (if paired with [TrimAtTag]({{< relref "docs/reference/modification-steps/TrimAtTag.md" >}}))
