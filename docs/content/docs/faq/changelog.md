# Changelog

## v0.9.0

### General
- Renamed project from mbf-fastq-processor to fastqrab
- Much improved error messages pinpointing exactly what needs to change.
- [Virtual Tags]({{< relref "docs/concepts/tag.md#virtual-tags" >}}) introduced
- stdout output on any (infix based) output step with "infix='--stdout--'"
- Numeric tags now support range checking
- Added [G&uuml;nther]({{< relref "docs/concepts/mascot/_index.md" >}}) to the docs.

### New limitations
- Segment count limited to 255


### New steps
- New step: [ExtractAdapterLikeFastP]({{< relref "docs/reference/tag-steps/extract/ExtractAdapterLikeFastP.md" >}}) - fastp compatible adapter detection

- New step: [ValidateReadNamesPrintable]({{< relref "docs/reference/validation-steps/ValidateReadNamesPrintable.md" >}})

- New step: [CompareStringTags]({{< relref "docs/redirects/CompareStringTags.md" >}})

- New step: [ConcatTags]({{< relref "docs/reference/tag-steps/using/ConcatTags.md" >}}) -  concatenate multiple tags into one

- New step: [Lowercase]({{< relref "docs/reference/modification-steps/Lowercase.md" >}}) -unified replacement for LowercaseTag/LowercaseSequence

- New step: [ValidateReadPairing]({{< relref "docs/redirects/ValidateReadPairing.md" >}}) - Hamming-distance-based read pairing validation
 (This is the only automatically injected step in fastqrab!)

- New Step: [ConvertToRate]({{< relref "docs/reference/tag-steps/convert/ConvertToRate.md" >}}) 

- New step: [ExtractIUPACSuffix]({{< relref "docs/reference/tag-steps/extract/ExtractIUPACSuffix.md" >}})


- New step: [FillMissing]({{< relref "docs/reference/modification-steps/FillMissing.md" >}})
  - to, for example, choose 'either' found barcode.
- New step: [Matches]({{< relref "docs/reference/tag-steps/using/Matches.md" >}}) to restrict
  to a debug selection of reads.

- New step: [CalcWorstQuality]({{< relref "docs/reference/tag-steps/calc/CalcWorstQuality.md" >}})

- New step [AssignByHalves]({{< relref "docs/reference/tag-steps/using/AssignByHalves.md" >}}): for Cell Ranger like probe assignment

- New step: [StoreSingleCellMatrix]({{< relref "docs/reference/tag-steps/using/StoreSingleCellMatrix.md" >}})
to quickly calculate single cell matrix from a cell barcodes, assigned genes and UMIs.

- New step: [StoreTagInSequence]({{< relref "docs/redirects/StoreTagInSequence.md" >}}), allows storing at another tags location
  (contrast with [StoreTagBackInSequence]({{< relref "docs/reference/tag-steps/using/StoreTagBackInSequence.md" >}}))


### Step renames/merges
- ExtractAnchor merged into [ExtractRegions]({{< relref "docs/reference/tag-steps/extract/ExtractRegions.md" >}})

- NCount renamed [CalcNContent]({{< relref "docs/reference/tag-steps/calc/CalcNContent.md" >}})
to be in-line with [CalcGCContent]({{< relref "docs/reference/tag-steps/calc/CalcGCContent.md" >}})
and  [CalcBaseContent]({{< relref "docs/reference/tag-steps/calc/CalcBaseContent.md" >}})

- [TagOtherFile]({{< relref "docs/reference/tag-steps/tag/TagOtherFile.md" >}})
  unifies OtherFileByName and OtherFileBySequence into one step

- 'StoreTagInSequence' -> ['StoreTagBackInSequence']({{< relref "docs/redirects/StoreTagBackInSequence.md" >}})

### Step changes

- Conditional Swap/ReverseComplement variants merged into the main steps

- [if tag]({{< relref "docs/concepts/tag.md#conditional-processing" >}}) support on 8 core editing steps for conditional read editing

- min_length added to [ExtractRegions]({{< relref "docs/reference/tag-steps/extract/ExtractRegions.md" >}})

- (optional) Quality checking added to 
    [Prefix]({{< relref "docs/reference/modification-steps/Prefix.md" >}})
    /
    [Postfix]({{< relref "docs/reference/modification-steps/Postfix.md" >}})

- Tag value replacement within regular expressions on [ExtractRegex]({{< relref "docs/redirects/ExtractRegex.md" >}})

- [Swap]({{< relref "docs/reference/modification-steps/Swap.md" >}}) - Fixed a correctness bug.

- [Progress]({{< relref "docs/reference/report-steps/Progress.md" >}}) stdout handling unified, 
    reports core configuration and time taken by finalization of steps

- [ExtractRegex]({{< relref "docs/reference/tag-steps/extract/ExtractRegex.md" >}}) from tags

- [StoreTagsInTable]({{< relref "docs/reference/tag-steps/using/StoreTagsInTable.md" >}}) - optionally do not output read names.

- [StoreTagInComment]({{< relref "docs/reference/tag-steps/using/StoreTagInComment.md" >}}) now supports multiple tags at once

- [ReverseComplement]({{< relref "docs/reference/modification-steps/ReverseComplement.md" >}}) can now be applied to tags

- [CalcBaseContent]({{< relref "docs/reference/tag-steps/calc/CalcBaseContent.md" >}}) outputs 0..=1, not 0..=100

- More steps support 'relative' as an option (CalcGCContent, CalcNContent, CalcQualifiedBases)


#### [ExtractIUPAC]({{< relref "docs/reference/tag-steps/extract/ExtractIUPAC.md" >}})
- Multiple queries in one step, max_mismatches now required, large performance improvements
- Tie handling for multiple queries 
- Can now also search within a (small) distance from start/end.

#### [HammingCorrect]({{< relref "docs/reference/tag-steps/using/HammingCorrect.md" >}})
- Can now assign either barcodes or barcode labels
- Uses the [hamming-resonate crate](https://crates.io/crates/hamming-resonate/).
- Gained support for explicit on_no_match and on_tie handling
  in multiple variations, with or without name-prefix based equivalence classes.
- multi-core
- on_tie=ByEditProbability supports reading the barcode distribution from a previously generated report.




### Output changes

- Tag histogram reports; demultiplex data nested under 'demultiplex' key in reports

#### [BAM output]({{< relref "docs/reference/output-section.md#bam-output" >}})
- Store tags in BAM tags
- Store one tag in BAM reference field.
- Corrected error message when read length > max bam read length
- better error messages when the read names can't be represented in BAM.
  (see also [ValidateReadNamesPrintable]({{< relref "docs/reference/validation-steps/ValidateReadNamesPrintable.md" >}})

### Performance

- Redesigned multi-core engine: workpool based, better controllable, better documented
- Default thread count now uses all available CPU cores
- Rapidgzip for parallel gzip decompression, now also for FASTA; auto-detected; included in Nix builds
- Arena-based parsers for FASTA and BAM
- Parallel BAM decoding
- Multicore EvalExpression, ReportCountOligos, ReportLengthDistribution
- Prefix/Postfix massively improved performance
- Merge base statistics ~80% faster
- ConcatTags ~15% faster
- IUPAC matching: replaced Sassy with optimized pure-Rust implementation
- Optimized SwapConditional, TrimAtTag, StoreTagBackInSequence, FilterReservoirSample, Rename
- Dynamic cuckoo filter sizing; initial_filter_capacity documented; read count estimation
- [ExtractLongestPolyX]({{< relref "docs/reference/tag-steps/extract/ExtractLongestPolyX.md" >}}) no longer O(n^2)


### Other
- verify command: validates a pipeline produces expected output; auto-detects config, captures stdout/stderr
- configuration TOML can now be read from stdin (incompatible with reads from stdin).
- barcode sections can now read from files (FASTA, txt)
- Shell autocompletion for bash, fish, and zsh
- benchmark mode and per-step benchmark harness
- template command: shows help on error
- LLM configuration guide and template.toml rewrite for LLM-assisted config generation
- TagLabel type: all tag names are now strongly typed; duplicate tag names produce a clear error
- IndexMap replaces HashMap everywhere for deterministic output order
- unwrap() replaced with expect() throughout; clippy::unwrap_used now denied
- MSRV pinned to match flake.nix Rust version
- Security: upgraded bytes crate (GHSA-434x-w66g-qw3r)
- Upgraded dependencies
- Reports: Prevent script injection.
- Report's count_oligos now supports named oligos.

### Documentation
- Four new cookbooks for common FastQ processing tasks
- Copy-to-clipboard button in docs
- Documentation URLs included in validation failure messages
- Added mascot
- [New Cookbook: #10 - Adapter identification]({{< relref "docs/how-to/cookbooks/10-adapter-identification.md" >}})

### Correctness

- The parser has been fuzzed.
- CI now tests docker images
- Barcode sections now must be used (or removed).
- Removed unsafe in parallel gzp writing, interactive mode

### Bug fixes
- Fixed fastp merge algorithm (replaced with direct port of the reference C++ algorithm)
- Fixed invalid FASTQ detection when comment line doesn't start with '+'
- Fixed Windows newline detection edge case in parser
- Fixed Local-Local FastQElement swap
- Fixed demultiplex & fragment count in reports
- Fixed Head short-circuit (broken by SpotCheckReadPairing)
- Fixed ignore_unaligned → now include_mapped / include_unmapped
- Fixed barcode overlapping multiple matches
- Fixed a bug n [ExtractIUPACSuffix]({{< relref "docs/reference/tag-steps/extract/ExtractIUPACSuffix.md" >}}
- Fixed a potential panic in [ExtractLongestPolyX]({{< relref "docs/reference/tag-steps/extract/ExtractLongestPolyX.md" >}})



## v0.8.1
   
- Github release workflow test


## v0.8.0

- Versioned documentation
- First revision where very major feature is in place. Changelog starts here.
