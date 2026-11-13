---
weight: 2
---
# Implementing your own transformation

Let's implement an example step (also called a 'transformation') that converts your reads to FuNkYcAsE
by upper/lower-casing every other letter.

This guide assumes you have basic linux command line knowledge, and that you
can edit text files (source code).


We are going to start by devising a test case, making sure it fails,
and then step by step adding all the parts we need. This will illustrate
all the infrastructure the project has to support you in this.

(This guide is itself tested: `fastqrab/tests/custom_transformation_guide.rs` replays it
in a scratch copy of the repository, so the code below compiles and the test cases pass.)

## First things first
[Clone the repo]({{< relref "docs/development/getting_started.md" >}})
 and verify that you can build (perhaps
after entering the `nix develop` environment) using `cargo build`.

## Simple test case

To add a test case, we need to add a folder with an input.toml
somewhere below the `test_cases` directory. The input files
used by the test case must be named 'input*', while the output prefix can be anything,
though most existing test cases simply use 'output'. Our test runner will then verify
that fastqrab is producing exactly that output.

Create the test case folder, and use an existing short FASTQ file as the input
by symlinking it:

<!-- guide-test: run -->
```bash
mkdir -p test_cases/single_step/funky_case/basic
cd test_cases/single_step/funky_case/basic
ln -s ../../../sample_data/misc/input_read1_2.fq input.fq
```

The expected output goes into `test_cases/single_step/funky_case/basic/funky_read1.fq`
(copy `input.fq` and edit it in your editor):

<!-- guide-test: write test_cases/single_step/funky_case/basic/funky_read1.fq -->
```fastq
@test_read1
aGtCaGtCaGtCaGtC
+
IIIIIIIIIIIIIIII
@test_read2
tGaCtGaCtGaCtGaC
+
HHHHHHHHHHHHHHHH
```

Now we need to fill this into `test_cases/single_step/funky_case/basic/input.toml`:

<!-- guide-test: write test_cases/single_step/funky_case/basic/input.toml -->
```toml
[input]
    read1 = 'input.fq'

[[step]]
    action = "FunkyCase"
    start_with_lower = true

[output]
    prefix = "funky"

[[step]]
    action = "OutputFASTQ"
```

(Output is a step as well - `[output]` only sets the prefix, `OutputFASTQ` writes the reads.)

Now go back to the top level project directory and run `dev/ci/update_generated.sh`
which will discover your new test case and add it to our test harness (`fastqrab/tests/generated.rs`).
(If you omit this step, `all_test_cases_are_generated` will fail - after running
the script for you, so the next `cargo test` picks the test case up).

Now run `cargo test` (or just this test case with `cargo test funky_case`) and you should receive the following (expected) failure:

<!-- guide-test: build -->
<!-- guide-test: verify-fails test_cases/single_step/funky_case/basic Invalid tag value: 'FunkyCase' -->
```
---- test_cases_x_single_step_x_funky_case_x_basic stdout ----
...
# == Error Details ==
Configuration validation failed unexpectedly.

Caused by:
    Error 1/1
      ╭─config.toml
      ┆
    4 │ [[step]]
    5 │     action = "FunkyCase"
      ┆              ─────┬─────
      ┆                   │
      ┆                   ╰────── Invalid tag value: 'FunkyCase'
    ──╯
    Hint: Did you mean: Truncate, Rename, or Lowercase?
```

You can also run a single test case directly, without the test harness:
`cargo run -- verify test_cases/single_step/funky_case/basic/input.toml`.


## Adding the transformation in all the right places

We need to do three things:

- Write a [struct](https://doc.rust-lang.org/book/ch05-00-structs.html) & [trait](https://doc.rust-lang.org/book/ch10-02-traits.html) implementation of our transformation in a new rust file
- Hook it into [rust's module system](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html)
- Add it to the central [enum](https://doc.rust-lang.org/book/ch06-00-enums.html) that lists all transformations.

We're going to start with the last step.


### Add a transformation to the central enum that lists all transformations

Edit the file `fastqrab-steps/src/transformations.rs` using your favorite editor.

You are looking for
<!-- guide-test: excerpt fastqrab-steps/src/transformations.rs -->
```rust
pub enum Transformation {
    //Edits
    CutStart(edits::CutStart),
    CutEnd(edits::CutEnd),
    Truncate(edits::Truncate),
    ...
```

And you want to add `FunkyCase(edits::FunkyCase)` so it looks like
<!-- guide-test: insert fastqrab-steps/src/transformations.rs -->
```rust
pub enum Transformation {
    //Edits
    CutStart(edits::CutStart),
    CutEnd(edits::CutEnd),
    Truncate(edits::Truncate),
    FunkyCase(edits::FunkyCase),
    ...
```

`edits::` in this case refers to a module below the `transformation` module,
which brings us to our next step:


### Hook a step into the module system.


We are going to tell the `edits` module that it has a submodule `funky_case`,
and reexport two types from `funky_case` so that the
rest of the rust code can use it: the concrete `FunkyCase` type used at runtime,
and the `PartialFunkyCase` type used during TOML parsing.

Open `fastqrab-steps/src/transformations/edits.rs` and add

<!-- guide-test: append fastqrab-steps/src/transformations/edits.rs -->
```rust
mod funky_case; // declare that we have a module funky_case(.rs)

pub use funky_case::{FunkyCase, PartialFunkyCase}; // export our types
```

### Write the transformation

Every transformation consists of three parts:

- A struct with the `#[tpd]` attribute macro (which generates the `PartialFunkyCase` type used for TOML parsing)
- An `impl VerifyIn<PartialConfig> for PartialFunkyCase` block for config-time validation
- An `impl Step for FunkyCase` block for the actual runtime logic

Additionally, the TOML framework requires that each step declares which tags it reads
and writes, via `impl TagUser for PartialTaggedVariant<PartialFunkyCase>`.
A step that uses no tags at all can rely on the default implementation (an empty `impl` block).

To be usable, your struct needs to be included in the large `Transformation` enum listing all steps,
which we accomplished in the previous steps.

Now it's time to actually write the struct. Create a new file
`fastqrab-steps/src/transformations/edits/funky_case.rs`
and put the following minimal example into it:

<!-- guide-test: write fastqrab-steps/src/transformations/edits/funky_case.rs -->
```rust
use crate::transformations::prelude::*;

/// Convert reads to FuNkYcAsE
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct FunkyCase {
    // a mandatory TOML option - whether to start with a lower case letter.
    pub start_with_lower: bool,
}

impl VerifyIn<PartialConfig> for PartialFunkyCase {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        Ok(())
    }
}

// we use no tags (yet) - the default implementation says so
impl TagUser for PartialTaggedVariant<PartialFunkyCase> {}

impl Step for FunkyCase {
    fn apply(
        &self,
        block: FastQBlocksCombined, // that's where the read data lives
        _input_info: &InputInfo,    //ignore for now
        _demultiplex_info: &OptDemultiplex, //ignore for now
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        //this doesn't do anything.
        // The bool says 'continue processing' - returning false
        // would stop reading more input (see Head)
        Ok((block, true))
    }
}
```

At this point `cargo check` should show no errors.

<!-- guide-test: check -->
<!-- guide-test: verify-fails test_cases/single_step/funky_case/basic funky_read1.fq: Content mismatch at byte 12 -->

Our test case however will now fail with a different message:

```
# == Error Details ==
Output verification failed:
  funky_read1.fq: Content mismatch at byte 12: expected 0x61, got 0x41
```

Which is to be expected, since we're not actually changing the reads yet.

You can compare the expected and actual output. For failing test cases, after a run,
there is a folder 'actual' in the test case directory that has the produced output.

You can simply compare them with diff

```
> cd test_cases/single_step/funky_case/basic
> diff actual/funky_read1.fq funky_read1.fq
2c2
< AGTCAGTCAGTCAGTC
---
> aGtCaGtCaGtCaGtC
6c6
< TGACTGACTGACTGAC
---
> tGaCtGaCtGaCtGaC
```

### Modifying the reads

To actually change the reads, we are going to use a function
that takes a callback that modifies each read in turn.

Replace the contents of `fastqrab-steps/src/transformations/edits/funky_case.rs` with this
<!-- guide-test: write fastqrab-steps/src/transformations/edits/funky_case.rs -->
```rust
use crate::transformations::prelude::*;

/// Convert reads to FuNkYcAsE
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct FunkyCase {
    // a mandatory TOML option - whether to start with a lower case letter.
    pub start_with_lower: bool,
}

impl VerifyIn<PartialConfig> for PartialFunkyCase {
    fn verify(
        &mut self,
        _parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialFunkyCase> {}

impl Step for FunkyCase {
    fn apply(
        &self,
        mut block: FastQBlocksCombined, // that's where the read data lives
        _input_info: &InputInfo,        //ignore for now
        _demultiplex_info: &OptDemultiplex, //ignore for now
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        //apply funky casing to all reads
        block.apply_in_place_wrapped_plus_all(
            // in the first segment (read1), see below for configurability
            SegmentIndexOrAll::Indexed(SegmentIndex(0)),
            |read| {
                // a lambda function taking a FastQReadMut mutable reference
                let mut lower = self.start_with_lower; //so we can alternate
                for b in read.seq.iter_mut() {
                    //for every character in the sequence
                    *b = if lower {
                        b.to_ascii_lowercase()
                    } else {
                        b.to_ascii_uppercase()
                    };
                    lower = !lower;
                }
            },
            None, // if_tag support, see below
        );

        Ok((block, true))
    }
}
```

Note that this callback may only change the read's bytes, not its length.
Steps that change the length (like `CutStart` or `Prefix`) need to go through the
segment's `seq_quals` so that location tags are kept in sync.

<!-- guide-test: check -->
<!-- guide-test: verify test_cases/single_step/funky_case/basic -->

If you run `cargo test` now, our funky case test will pass,
and it will fail later on with the tests that verify our documentation:

```
---- test_every_transformation_has_documentation stdout ----
The following transformations are missing documentation files:
FunkyCase

---- test_every_transformation_has_benchmark stdout ----
The following transformations are missing benchmarks in simple_benchmarks.rs:
FunkyCase

---- test_llm_guide_covers_all_transformations stdout ----
LLM guide validation failed:
Transformation 'FunkyCase' is not documented in llm-guide.md

---- test_every_step_has_a_template_section stdout ----
Template validation failed:
The following transformations are missing in template.toml:
FunkyCase

---- test_every_link_docs_target_has_a_redirect_page stdout ----
The following link_docs() targets have no redirect page in docs/reference/redirects/.
...
```

(We'll get to those at the end.)

### Making the transformation configurable.

We'll add the documentation later, for now let's make FunkyCase more configurable.

We want it to work on any segment (not just the first one), support `if_tag` like
all the other read editing transformations, and allow starting with either a lower or upper
case letter.

The last one we already have - every field in the struct is a TOML option
(mandatory, unless it has a default or is an `Option`), so let's add a test case
for `start_with_lower = false` by copying the basic test case

<!-- guide-test: run -->
```bash
cp -r test_cases/single_step/funky_case/basic test_cases/single_step/funky_case/upper_first
```

and editing `start_with_lower = true` to `start_with_lower = false` in that test case's input.toml.

<!-- guide-test: run -->
```bash
sed -i 's/start_with_lower = true/start_with_lower = false/' test_cases/single_step/funky_case/upper_first/input.toml
```

Don't forget to change the expected output in
`test_cases/single_step/funky_case/upper_first/funky_read1.fq`:

<!-- guide-test: write test_cases/single_step/funky_case/upper_first/funky_read1.fq -->
```fastq
@test_read1
AgTcAgTcAgTcAgTc
+
IIIIIIIIIIIIIIII
@test_read2
TgAcTgAcTgAcTgAc
+
HHHHHHHHHHHHHHHH
```

Let it find the new test (`dev/ci/update_generated.sh`), and `cargo test` should show it passing.

<!-- guide-test: verify test_cases/single_step/funky_case/upper_first -->

What happens if you leave out the option? Comment out `start_with_lower` in
`test_cases/single_step/funky_case/basic/input.toml` and run
`cargo run -- verify test_cases/single_step/funky_case/basic/input.toml`:

<!-- guide-test: run -->
```bash
sed -i 's/^    start_with_lower = true/#    start_with_lower = true/' test_cases/single_step/funky_case/basic/input.toml
```

<!-- guide-test: verify-fails test_cases/single_step/funky_case/basic Missing required key: 'start_with_lower' -->
```
    Error 1/1
      ╭─config.toml
      ┆
    4 │ [[step]]
    5 │     action = "FunkyCase"
      ┆              ─────┬─────┬
      ┆                   │     │
      ┆                   ╰─────── In this step
      ┆                         │
      ┆                         ╰─ Missing required key: 'start_with_lower'.
    ──╯
    Hint: See https://tyberiusprime.github.io/fastqrab/v0.9.1/docs/redirects/FunkyCase
          Or run: `fastqrab template FunkyCase` for more information.
```

(Undo that before continuing.)

<!-- guide-test: run -->
```bash
sed -i 's/^#    start_with_lower = true/    start_with_lower = true/' test_cases/single_step/funky_case/basic/input.toml
```


### If_tag and segment support


For a full transformation we're still missing two ingredients:
the choice of what segment to work on, and the `if_tag`  support allowing
it to work on a subset of reads.

We're going to do both at once now.

Duplicate the basic test case once more:
<!-- guide-test: run -->
```bash
cp -r test_cases/single_step/funky_case/basic test_cases/single_step/funky_case/if_tag_segment
```

Place the following in `test_cases/single_step/funky_case/if_tag_segment/funky_read2.fq`

<!-- guide-test: write test_cases/single_step/funky_case/if_tag_segment/funky_read2.fq -->
```fastq
@test_read1
AgTcAgTcAgTcAgTc
+
IIIIIIIIIIIIIIII
@test_read2
TGACTGACTGACTGAC
+
HHHHHHHHHHHHHHHH
```

and the following in its `input.toml`

<!-- guide-test: write test_cases/single_step/funky_case/if_tag_segment/input.toml -->
```toml
[options]
    accept_duplicate_files = true

[input]
    read1 = 'input.fq'
    read2 = 'input.fq'

[[step]]
    action = "FunkyCase"
    start_with_lower = true
    segment = 'read1'

[[step]]
    action = "EvalExpression"
    out_label = "apply_funky_to_read2"
    expression = "read_no < 1"
    result_type = "bool"

[[step]]
    action = "FunkyCase"
    start_with_lower = false
    segment = 'read2'
    if_tag = 'apply_funky_to_read2'

[output]
    prefix = "funky"

[[step]]
    action = "OutputFASTQ"
```

Update the tests once more with `dev/ci/update_generated.sh` and observe it failing, because we
haven't added the options yet:

<!-- guide-test: verify-fails test_cases/single_step/funky_case/if_tag_segment Unknown key. -->
```
    Error 1/4
       ╭─config.toml
       ┆
     8 │ [[step]]
     9 │     action = "FunkyCase"
       ┆              ─────┬─────
       ┆                   │
       ┆                   ╰────── In this step
       ┆
    10 │     start_with_lower = true
    11 │     segment = 'read1'
       ┆     ───┬───
       ┆        │
       ┆        ╰────────────── Unknown key.
    ───╯
    ...
    Error 4/4
       ╭─config.toml
       ┆
    13 │ [[step]]
    14 │     action = "EvalExpression"
    15 │     out_label = "apply_funky_to_read2"
       ┆                 ───────────┬──────────
       ┆                            │
       ┆                            ╰─────────── Unused tag
    ───╯
    Hint: Make sure to either use, or forget (using ForgetTag) all your tags.
```

(Errors 2 and 3 complain about `segment` and `if_tag` in the second FunkyCase step. Every tag
must be used by some step, so the now 'unused' `apply_funky_to_read2` is an error as well.)


Open `fastqrab-steps/src/transformations/edits/funky_case.rs` and replace it with the
full version that has segment and `if_tag` support:

<!-- guide-test: write fastqrab-steps/src/transformations/edits/funky_case.rs -->
```rust
use crate::transformations::prelude::*;

/// Convert reads to FuNkYcAsE
#[derive(Clone, JsonSchema)]
#[tpd]
#[derive(Debug)]
pub struct FunkyCase {
    start_with_lower: bool,

    // The user writes a segment name ('read1', or 'all'), which is
    // turned into an index during verification.
    #[schemars(with = "String")]
    #[tpd(adapt_in_verify(String))]
    segment: SegmentIndexOrAll,

    // Option<...> fields are optional in the TOML
    #[tpd(alias = "if_label")]
    if_tag: Option<ConditionalTagLabel>,
}

impl VerifyIn<PartialConfig> for PartialFunkyCase {
    fn verify(
        &mut self,
        parent: &PartialConfig,
        _options: &VerifyOptions,
    ) -> std::result::Result<(), ValidationFailure>
    where
        Self: Sized + toml_pretty_deser::Visitor,
    {
        // turn the segment name into an index - or report an error
        // if the segment does not exist in the input config
        self.segment.validate_segment(parent);
        Ok(())
    }
}

impl TagUser for PartialTaggedVariant<PartialFunkyCase> {
    fn get_tag_usage(
        &mut self,
        _tags_available: &IndexMap<TagLabel, TagMetadata>,
        _segment_order: &[String],
    ) -> Option<TagUsageInfo<'_>> {
        // value is None if the step failed to parse / verify.
        if let Some(inner) = self.toml_value.value.as_mut() {
            Some(TagUsageInfo {
                // the if_tag (if set) is a tag we read.
                // The (empty) slice would restrict the accepted tag types -
                // conditional tags are converted to bool anyway.
                used_tags: vec![inner.if_tag.to_used_tag(&[])],
                ..Default::default()
            })
        } else {
            None
        }
    }
}

impl Step for FunkyCase {
    fn apply(
        &self,
        mut block: FastQBlocksCombined, // that's where the read data lives
        _input_info: &InputInfo,        //ignore for now
        _demultiplex_info: &OptDemultiplex, //ignore for now
    ) -> anyhow::Result<(FastQBlocksCombined, bool)> {
        // one bool per read - or None if there is no if_tag
        let condition = self
            .if_tag
            .as_ref()
            .map(|tag| get_bool_vec_from_tag(&block, tag));

        block.apply_in_place_wrapped_plus_all(
            self.segment,
            |read| {
                let mut lower = self.start_with_lower;
                for b in read.seq.iter_mut() {
                    *b = if lower {
                        b.to_ascii_lowercase()
                    } else {
                        b.to_ascii_uppercase()
                    };
                    lower = !lower;
                }
            },
            condition.as_deref(),
        );

        Ok((block, true))
    }
}
```

Note that `segment` is now mandatory, so add `segment = 'read1'` to the `basic` and `upper_first` test cases,

<!-- guide-test: run -->
```bash
for tc in basic upper_first; do
    sed -i 's/^    action = "FunkyCase"$/    action = "FunkyCase"\n    segment = "read1"/' \
        test_cases/single_step/funky_case/$tc/input.toml
done
```

or give it a default in `verify` with `self.segment.or(SegmentIndexOrAll::Indexed(SegmentIndex(0)));`.

<!-- guide-test: check -->
<!-- guide-test: verify test_cases/single_step/funky_case/basic -->
<!-- guide-test: verify test_cases/single_step/funky_case/upper_first -->
<!-- guide-test: verify test_cases/single_step/funky_case/if_tag_segment -->

At this stage you have a working 'Step' (verify with `cargo test`) that has all
the usual amenities, but lacks documentation.

To add this is left as an exercise for the reader,
but you'll need to edit `fastqrab/src/template.toml`,
`docs/content/docs/reference/llm-guide.md` and
add a file `docs/content/docs/reference/modification-steps/FunkyCase.md`
which need to include a valid TOML block with `action = "FunkyCase"` and all available options documented.
Run `dev/ci/update_generated.sh` once more afterwards, it also generates a redirect page
for the new step's documentation and updates the JSON schema.

You'll also need to add a microbenchmark to
`fastqrab/benches/simple_benchmarks.rs`.

Congratulations, you just wrote your first transformation for fastqrab!


## Further notes

When taking a segment, name or tag, have a look at `ResolvedSourceAll`/`ResolvedSourceNoAll`
(used by e.g. `Lowercase`).

To produce a tag instead of modifying reads, look at the `Extract*` steps
(e.g. `fastqrab-steps/src/transformations/extract/iupac_suffix.rs`), which declare
the tag they create in `get_tag_usage` and use helpers like `extract_region_tags_from_seq`.
