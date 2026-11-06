---
weight: 10
---
## Demultiplexed output


[Demultiplex]({{< relref "docs/reference/Demultiplex.md" >}}) is a 'magic' transformation that forks the output.

You receive one set of output files per barcode (combination) defined.

Transformations downstream are (virtually) duplicated,
so you can for example filter to the head reads in each barcode,
and get reports for both: all reads and each separate barcode.

Demultiplexing can be done on barcodes, or on boolean tags, and can happen multiple times.

### Based on barcodes

```toml
[[step]]
    action = "Demultiplex"
    in_label = "mytag"       # a location, string or boolean tag
    barcodes = "mybarcodes"
    output_unmatched  = true # if set, write reads not matching any barcode
                             #  to a file like ouput_prefix_no-barcode_1.fq
    in_label_contains_barcode = true # Whether the tag contains barcodes to lookup (true)
                                # or barcode-names (invalid names get translated to 'nobarcode')
                                # By default, auto detected on tag type: 
                                # location -> true,
                                # string -> auto set by HammingCorrect
                                #           manual specification necessary otherwise

[barcodes.mybarcodes] # can be before and after.
# separate multiple regions with a _
# a Mapping of barcode -> output name.
AAAAAA_CCCCCC = "sample-1" # output files are named prefix{ix_separator}barcode_prefix{ix_separator}segment.suffix
                           # with the separator defaulting to '_', e.g. output_sample-1_1.fq.gz
                           # or output_sample-1_report.fq.gz
```

### Based on boolean tags

```toml
[[step]]
    action = "TagOtherFile"
    source = "name:read1"
    # read names in the other file end at the first space; use `false` to match
    # the whole name (required whenever the source is a name).
    other_read_name_end_char = " "
    out_label = "a_bool_tag"
    filename = "path/to/boolean_tags.tsv"
    false_positive_rate = 0

[[step]]
    action = "Demultiplex"
    in_label = "a_bool_tag"
    # output_unmatched = is not valid for boolean tags
```


Note that this example does not 
extract the barcodes from the read 
(use an extract step, such as [ExtractRegion]({{< relref "docs/reference/tag-steps/extract/ExtractRegion.md" >}})).

Nor does it append the barcodes to the read name,
(use [StoreTagInComment]({{< relref "docs/reference/tag-steps/using/StoreTagInComment.md" >}}) for that) or remove the sequence from the reads
(combine with [CutStart]({{< relref "docs/reference/modification-steps/CutStart.md" >}})
/ [CutEnd]({{< relref "docs/reference/modification-steps/CutEnd.md" >}}) or perhaps 
[TrimAtTag]({{< relref "docs/reference/modification-steps/TrimAtTag.md" >}}).


Notes: 
- Query barcodes may use IUPAC codes. 
- IUPAC barcodes must be non-overlapping ( and this is enforced).
- Within one demultiplex step barcode must be of equal length.
- You can define multiple barcodes to go into the same output file.
- Multiple demultiplex steps per configuration are valid - you'll
  receive their product in terms of output files. There's a hard limit on the barcodes needing 
  64 bits, but you'll reach a limit on your patience for the compute to build all the output file names
  much before that.
- A demultiplex step matching zero barcodes (across all reads) will issue an error.

## Hamming Distance matching
Correcting a tag for hamming distance is a separate step. 
See [HammingCorrect]({{< relref "docs/reference/tag-steps/using/HammingCorrect.md" >}}).

You can also use [HammingCorrect]({{< relref "docs/reference/tag-steps/using/HammingCorrect.md" >}}).
if you want to assign a barcodes labels (store the lookup's result's in a tag).
