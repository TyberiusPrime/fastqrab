---
weight: 50
---

# TagOtherFile

Marks reads based on whether 'they' are present in another file.

Supports comparing by read sequence, read name, and tags.


```toml
[[step]]
    action = "TagOtherFile"
    source = 'read1' # <segment>, name:<segment> or tag<tag_name>
    out_label = "present_in_other_file"
    filename = "names.fastq" # Can read fastq (also compressed), or SAM/BAM, or fasta files
    false_positive_rate = 0.01 # false positive rate (0..1)
    seed = 42 # seed for randomness
    include_mapped = true # in case of BAM/SAM, whether to include aligned reads
    include_unmapped = true # in case of BAM/SAM, whether to include unaligned reads
    # other_readname_end_character = " " # (required/accepted only in source=name: mode), 
                                       # Cut the other files read names at this character.
                                       # set to false to not cut. 


```

This step annotates reads by comparing them to another file.

With false_positive_rate > 0, uses a cuckoo filter, otherwise an exact hash set.
Please note our [remarks about cuckoo filters]({{< relref "docs/faq/_index.md" >}}#cuckoo-filtering).

We can compare reads based on sequencs, names, or extracted sequences (=string & location tags),
by using [source]({{< relref "docs/concepts/source.md" >}}) concept.

In name mode, our read's names are *always* cut 
at [input.options.read_name_end_character]({{< relref "docs/reference/input-section.md" >}}). 

The other files read names are only cut if other_read_name_end_character is set to a character / byte value.
The value _false_ is used to signal 'do not cut'.

In tag mode, missing tags are treated as 'not present in other file'.
