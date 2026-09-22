// this file is written by dev/ci/_update_tests.py
// there is a test case that will inform you if tests are missing and you need
// to rerun dev/update_tests.py
#![expect(clippy::should_panic_without_expect, reason = "generated")]
#[path = "common/mod.rs"]
mod common;
use common::test_runner::run_test;

#[test]

fn test_cases_x_alloc_x_duplicate_input_allocation() {
    println!("Test case is in: test_cases/alloc/duplicate_input_allocation");
    run_test(
        std::path::Path::new("../test_cases/alloc/duplicate_input_allocation"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_bool() {
    println!("Test case is in: test_cases/demultiplex/bool");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/bool"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_bool_with_unmatched() {
    println!("Test case is in: test_cases/demultiplex/bool_with_unmatched");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/bool_with_unmatched"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_bool_with_unmatched_wrong_type() {
    println!("Test case is in: test_cases/demultiplex/bool_with_unmatched_wrong_type");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/bool_with_unmatched_wrong_type"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_chunked_interleaved_output_demultiplex() {
    println!("Test case is in: test_cases/demultiplex/chunked_interleaved_output_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/chunked_interleaved_output_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_chunked_output_demultiplex() {
    println!("Test case is in: test_cases/demultiplex/chunked_output_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/chunked_output_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_demultiplex_and_skip() {
    println!("Test case is in: test_cases/demultiplex/demultiplex_and_skip");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/demultiplex_and_skip"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_duplicates() {
    println!("Test case is in: test_cases/demultiplex/duplicates");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/duplicates"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_duplicates_with_initial_filter_capacity() {
    println!("Test case is in: test_cases/demultiplex/duplicates_with_initial_filter_capacity");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/duplicates_with_initial_filter_capacity"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_error_barcodes_undefined() {
    println!("Test case is in: test_cases/demultiplex/error_barcodes_undefined");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/error_barcodes_undefined"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_error_no_upstream() {
    println!("Test case is in: test_cases/demultiplex/error_no_upstream");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/error_no_upstream"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_error_unmatched_not_set() {
    println!("Test case is in: test_cases/demultiplex/error_unmatched_not_set");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/error_unmatched_not_set"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_head_with_index_and_demultiplex() {
    println!("Test case is in: test_cases/demultiplex/head_with_index_and_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/head_with_index_and_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_inspect() {
    println!("Test case is in: test_cases/demultiplex/inspect");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/inspect"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_interleaved_output_demultiplex() {
    println!("Test case is in: test_cases/demultiplex/interleaved_output_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/interleaved_output_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_interleaved_output_demultiplex_bam_output() {
    println!("Test case is in: test_cases/demultiplex/interleaved_output_demultiplex_bam_output");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/interleaved_output_demultiplex_bam_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_multiple_barcode_and_bool() {
    println!("Test case is in: test_cases/demultiplex/multiple_barcode_and_bool");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/multiple_barcode_and_bool"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_no_matching_barcodes() {
    println!("Test case is in: test_cases/demultiplex/no_matching_barcodes");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/no_matching_barcodes"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_no_matching_barcodes_no_lookup() {
    println!("Test case is in: test_cases/demultiplex/no_matching_barcodes_no_lookup");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/no_matching_barcodes_no_lookup"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_quantify_tag() {
    println!("Test case is in: test_cases/demultiplex/quantify_tag");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/quantify_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_reservoir_sample() {
    println!("Test case is in: test_cases/demultiplex/reservoir_sample");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/reservoir_sample"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_sample_names_invalid_filenames() {
    println!("Test case is in: test_cases/demultiplex/sample_names_invalid_filenames");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/sample_names_invalid_filenames"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_bam_output() {
    println!("Test case is in: test_cases/demultiplex/simple_bam_output");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_bam_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_basics() {
    println!("Test case is in: test_cases/demultiplex/simple_basics");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_basics"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_combined_outputs_x_order1() {
    println!("Test case is in: test_cases/demultiplex/simple_combined_outputs/order1");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_combined_outputs/order1"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_combined_outputs_x_order2_invariant() {
    println!("Test case is in: test_cases/demultiplex/simple_combined_outputs/order2_invariant");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_combined_outputs/order2_invariant"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_hamming() {
    println!("Test case is in: test_cases/demultiplex/simple_hamming");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_hamming"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_iupac() {
    println!("Test case is in: test_cases/demultiplex/simple_iupac");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_iupac"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_iupac_hamming() {
    println!("Test case is in: test_cases/demultiplex/simple_iupac_hamming");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_iupac_hamming"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_iupac_two_regions() {
    println!("Test case is in: test_cases/demultiplex/simple_iupac_two_regions");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_iupac_two_regions"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_multiple_into_one_output() {
    println!("Test case is in: test_cases/demultiplex/simple_multiple_into_one_output");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_multiple_into_one_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_no_unmatched() {
    println!("Test case is in: test_cases/demultiplex/simple_no_unmatched");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_no_unmatched"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_on_string_tag() {
    println!("Test case is in: test_cases/demultiplex/simple_on_string_tag");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_on_string_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_on_string_tag_contains_label_forced() {
    println!("Test case is in: test_cases/demultiplex/simple_on_string_tag_contains_label_forced");
    run_test(
        std::path::Path::new(
            "../test_cases/demultiplex/simple_on_string_tag_contains_label_forced",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_single_barcode() {
    println!("Test case is in: test_cases/demultiplex/simple_single_barcode");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_single_barcode"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_simple_single_barcode_no_unmatched_output() {
    println!("Test case is in: test_cases/demultiplex/simple_single_barcode_no_unmatched_output");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/simple_single_barcode_no_unmatched_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_two_barcodes() {
    println!("Test case is in: test_cases/demultiplex/two_barcodes");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/two_barcodes"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_two_barcodes_long() {
    println!("Test case is in: test_cases/demultiplex/two_barcodes_long");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/two_barcodes_long"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_demultiplex_x_unused_barcodes() {
    println!("Test case is in: test_cases/demultiplex/unused_barcodes");
    run_test(
        std::path::Path::new("../test_cases/demultiplex/unused_barcodes"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bad_options_and_sources_lateron() {
    println!("Test case is in: test_cases/error_handling/bad_options_and_sources_lateron");
    run_test(
        std::path::Path::new("../test_cases/error_handling/bad_options_and_sources_lateron"),
        "input.toml",
        1,
    );
}

#[test]
#[should_panic]
fn test_cases_x_error_handling_x_bam_x_disk_full_bam() {
    println!("Test case is in: test_cases/error_handling/bam/disk_full_bam");
    run_test(
        std::path::Path::new("../test_cases/error_handling/bam/disk_full_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_empty() {
    println!("Test case is in: test_cases/error_handling/bam/read_name_empty");
    run_test(
        std::path::Path::new("../test_cases/error_handling/bam/read_name_empty"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_exceeds_bam_length_x_just_fit() {
    println!(
        "Test case is in: test_cases/error_handling/bam/read_name_exceeds_bam_length/just_fit"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/bam/read_name_exceeds_bam_length/just_fit",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_exceeds_bam_length_x_just_ok() {
    println!("Test case is in: test_cases/error_handling/bam/read_name_exceeds_bam_length/just_ok");
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/bam/read_name_exceeds_bam_length/just_ok",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_exceeds_bam_length_x_just_too_long() {
    println!(
        "Test case is in: test_cases/error_handling/bam/read_name_exceeds_bam_length/just_too_long"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/bam/read_name_exceeds_bam_length/just_too_long",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_exceeds_bam_length_x_too_long() {
    println!(
        "Test case is in: test_cases/error_handling/bam/read_name_exceeds_bam_length/too_long"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/bam/read_name_exceeds_bam_length/too_long",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_exceeds_bam_length_x_with_comment_ok() {
    println!(
        "Test case is in: test_cases/error_handling/bam/read_name_exceeds_bam_length/with_comment_ok"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/bam/read_name_exceeds_bam_length/with_comment_ok",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_exceeds_bam_length_x_with_separation_char() {
    println!(
        "Test case is in: test_cases/error_handling/bam/read_name_exceeds_bam_length/with_separation_char"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/bam/read_name_exceeds_bam_length/with_separation_char",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_bam_x_read_name_non_printable() {
    println!("Test case is in: test_cases/error_handling/bam/read_name_non_printable");
    run_test(
        std::path::Path::new("../test_cases/error_handling/bam/read_name_non_printable"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_broken_input_validate_segment_called() {
    println!("Test case is in: test_cases/error_handling/broken_input_validate_segment_called");
    run_test(
        std::path::Path::new("../test_cases/error_handling/broken_input_validate_segment_called"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_declared_tag_an_int() {
    println!("Test case is in: test_cases/error_handling/declared_tag_an_int");
    run_test(
        std::path::Path::new("../test_cases/error_handling/declared_tag_an_int"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_duplicate_tag_name() {
    println!("Test case is in: test_cases/error_handling/duplicate_tag_name");
    run_test(
        std::path::Path::new("../test_cases/error_handling/duplicate_tag_name"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_broken_newline() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/broken_newline");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/broken_newline"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_broken_newline2() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/broken_newline2");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/broken_newline2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_broken_panics() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/broken_panics");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/broken_panics"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_crlf_no_newline_at_end() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/crlf_no_newline_at_end");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/crlf_no_newline_at_end"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_diff_ids() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_diff_ids");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_diff_ids"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_double_qual() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_double_qual");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_double_qual"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_double_seq() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_double_seq");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_double_seq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_long_qual() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_long_qual");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_long_qual"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_no_qual() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_no_qual");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_no_qual"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_qual_del() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_qual_del");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_qual_del"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_qual_escape() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_qual_escape");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_qual_escape"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_qual_null() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_qual_null");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_qual_null"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_qual_space() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_qual_space");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_qual_space"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_qual_tab() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_qual_tab");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_qual_tab"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_qual_unit_sep() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_qual_unit_sep");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_qual_unit_sep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_qual_vtab() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_qual_vtab");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_qual_vtab"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_short_qual() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_short_qual");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_short_qual"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_spaces() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_spaces");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_spaces"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_tabs() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_tabs");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_tabs"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_at_plus() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_at_plus");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_trunc_at_plus"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_at_qual() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_at_qual");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_trunc_at_qual"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_at_seq() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_at_seq");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_trunc_at_seq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_in_plus() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_in_plus");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_trunc_in_plus"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_in_qual() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_in_qual");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_trunc_in_qual"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_in_seq() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_in_seq");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_trunc_in_seq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_in_seq_tiny_blocksize() {
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_in_seq_tiny_blocksize"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/error_trunc_in_seq_tiny_blocksize",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_error_trunc_in_title() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/error_trunc_in_title");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/error_trunc_in_title"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_invalid_base() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/invalid_base");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/invalid_base"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_invalid_base_or_dot() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/invalid_base_or_dot");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/invalid_base_or_dot"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_invalid_base_or_dot_too_long() {
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/invalid_base_or_dot_too_long"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/invalid_base_or_dot_too_long",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_mismatched_seq_qual_len_1st_read_qual_too_long()
{
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_1st_read_qual_too_long"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_1st_read_qual_too_long",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_mismatched_seq_qual_len_1st_read_qual_too_short()
{
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_1st_read_qual_too_short"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_1st_read_qual_too_short",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_mismatched_seq_qual_len_2nd_read_qual_too_long()
{
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_2nd_read_qual_too_long"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_2nd_read_qual_too_long",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_mismatched_seq_qual_len_2nd_read_qual_too_short()
{
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_2nd_read_qual_too_short"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/mismatched_seq_qual_len_2nd_read_qual_too_short",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_mixed_newlines_carry() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/mixed_newlines_carry");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/mixed_newlines_carry"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_no_newline_and_truncated_qual() {
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/no_newline_and_truncated_qual"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/no_newline_and_truncated_qual",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_no_newline_at_all() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/no_newline_at_all");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/no_newline_at_all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_no_newline_at_end_ok() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/no_newline_at_end_ok");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/no_newline_at_end_ok"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_quality_starts_with_at() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/quality_starts_with_at");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/quality_starts_with_at"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_trailing_blank_line_own_chunk() {
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/trailing_blank_line_own_chunk"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/trailing_blank_line_own_chunk",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_truncated_after_at() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/truncated_after_at");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/truncated_after_at"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_windows_newlines() {
    println!("Test case is in: test_cases/error_handling/malformed_fastq/windows_newlines");
    run_test(
        std::path::Path::new("../test_cases/error_handling/malformed_fastq/windows_newlines"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_malformed_fastq_x_windows_newlines_but_broken() {
    println!(
        "Test case is in: test_cases/error_handling/malformed_fastq/windows_newlines_but_broken"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/malformed_fastq/windows_newlines_but_broken",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_disk_full_fastq() {
    println!("Test case is in: test_cases/error_handling/misc/disk_full_fastq");
    run_test(
        std::path::Path::new("../test_cases/error_handling/misc/disk_full_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_disk_full_store_in_table() {
    println!("Test case is in: test_cases/error_handling/misc/disk_full_store_in_table");
    run_test(
        std::path::Path::new("../test_cases/error_handling/misc/disk_full_store_in_table"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_missing_output_dir_will_be_created() {
    println!("Test case is in: test_cases/error_handling/misc/missing_output_dir_will_be_created");
    run_test(
        std::path::Path::new(
            "../test_cases/error_handling/misc/missing_output_dir_will_be_created",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_prefix_absolute() {
    println!("Test case is in: test_cases/error_handling/misc/prefix_absolute");
    run_test(
        std::path::Path::new("../test_cases/error_handling/misc/prefix_absolute"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_prefix_absolute_windows() {
    println!("Test case is in: test_cases/error_handling/misc/prefix_absolute_windows");
    run_test(
        std::path::Path::new("../test_cases/error_handling/misc/prefix_absolute_windows"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_prefix_drive_letter() {
    println!("Test case is in: test_cases/error_handling/misc/prefix_drive_letter");
    run_test(
        std::path::Path::new("../test_cases/error_handling/misc/prefix_drive_letter"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_prefix_go_back() {
    println!("Test case is in: test_cases/error_handling/misc/prefix_go_back");
    run_test(
        std::path::Path::new("../test_cases/error_handling/misc/prefix_go_back"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_misc_x_too_many_open_files() {
    println!("Test case is in: test_cases/error_handling/misc/too_many_open_files");
    run_test(
        std::path::Path::new("../test_cases/error_handling/misc/too_many_open_files"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_panic_in_thread() {
    println!("Test case is in: test_cases/error_handling/panic_in_thread");
    run_test(
        std::path::Path::new("../test_cases/error_handling/panic_in_thread"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_replace_tag_with_letter_x_no_letter() {
    println!("Test case is in: test_cases/error_handling/replace_tag_with_letter/no_letter");
    run_test(
        std::path::Path::new("../test_cases/error_handling/replace_tag_with_letter/no_letter"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_too_many_segments() {
    println!("Test case is in: test_cases/error_handling/too_many_segments");
    run_test(
        std::path::Path::new("../test_cases/error_handling/too_many_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_truncated_fastq_finish_eof_x_after_1() {
    println!("Test case is in: test_cases/error_handling/truncated_fastq_finish_eof/after_1");
    run_test(
        std::path::Path::new("../test_cases/error_handling/truncated_fastq_finish_eof/after_1"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_truncated_fastq_finish_eof_x_after_2() {
    println!("Test case is in: test_cases/error_handling/truncated_fastq_finish_eof/after_2");
    run_test(
        std::path::Path::new("../test_cases/error_handling/truncated_fastq_finish_eof/after_2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_truncated_fastq_finish_eof_x_after_3() {
    println!("Test case is in: test_cases/error_handling/truncated_fastq_finish_eof/after_3");
    run_test(
        std::path::Path::new("../test_cases/error_handling/truncated_fastq_finish_eof/after_3"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_truncated_fastq_finish_eof_x_within_4() {
    println!("Test case is in: test_cases/error_handling/truncated_fastq_finish_eof/within_4");
    run_test(
        std::path::Path::new("../test_cases/error_handling/truncated_fastq_finish_eof/within_4"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_error_handling_x_truncated_gzip_x_fasta() {
    println!("Test case is in: test_cases/error_handling/truncated_gzip/fasta");
    run_test(
        std::path::Path::new("../test_cases/error_handling/truncated_gzip/fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_from_the_field_x_basecount_expansion() {
    println!("Test case is in: test_cases/from_the_field/basecount_expansion");
    run_test(
        std::path::Path::new("../test_cases/from_the_field/basecount_expansion"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_if_tag_x_cut_start_conditional() {
    println!("Test case is in: test_cases/if_tag/cut_start_conditional");
    run_test(
        std::path::Path::new("../test_cases/if_tag/cut_start_conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_if_tag_x_if_tag_inverted() {
    println!("Test case is in: test_cases/if_tag/if_tag_inverted");
    run_test(
        std::path::Path::new("../test_cases/if_tag/if_tag_inverted"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_if_tag_x_if_tag_location_to_bool() {
    println!("Test case is in: test_cases/if_tag/if_tag_location_to_bool");
    run_test(
        std::path::Path::new("../test_cases/if_tag/if_tag_location_to_bool"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_compression_x_gz_input() {
    println!("Test case is in: test_cases/input/compression/gz_input");
    run_test(
        std::path::Path::new("../test_cases/input/compression/gz_input"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_compression_x_gzip_blocks_spliting_reads() {
    println!("Test case is in: test_cases/input/compression/gzip_blocks_spliting_reads");
    run_test(
        std::path::Path::new("../test_cases/input/compression/gzip_blocks_spliting_reads"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_compression_x_rapidgzip_x_basic() {
    println!("Test case is in: test_cases/input/compression/rapidgzip/basic");
    run_test(
        std::path::Path::new("../test_cases/input/compression/rapidgzip/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_compression_x_rapidgzip_x_single_thread() {
    println!("Test case is in: test_cases/input/compression/rapidgzip/single_thread");
    run_test(
        std::path::Path::new("../test_cases/input/compression/rapidgzip/single_thread"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_empty_input() {
    println!("Test case is in: test_cases/input/error_handling/empty_input");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/empty_input"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_array() {
    println!("Test case is in: test_cases/input/error_handling/input_array");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_array"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_bzip2() {
    println!("Test case is in: test_cases/input/error_handling/input_bzip2");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_bzip2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_gzip_but_not_fastq() {
    println!("Test case is in: test_cases/input/error_handling/input_gzip_but_not_fastq");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_gzip_but_not_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_nested_map() {
    println!("Test case is in: test_cases/input/error_handling/input_nested_map");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_nested_map"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_nested_map_output_mistake() {
    println!("Test case is in: test_cases/input/error_handling/input_nested_map_output_mistake");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_nested_map_output_mistake"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_non_str_key() {
    println!("Test case is in: test_cases/input/error_handling/input_non_str_key");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_non_str_key"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_non_str_value() {
    println!("Test case is in: test_cases/input/error_handling/input_non_str_value");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_non_str_value"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_non_str_value_nested() {
    println!("Test case is in: test_cases/input/error_handling/input_non_str_value_nested");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_non_str_value_nested"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_reserved_segment() {
    println!("Test case is in: test_cases/input/error_handling/input_reserved_segment");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_reserved_segment"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_input_str() {
    println!("Test case is in: test_cases/input/error_handling/input_str");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/input_str"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_non_string_values() {
    println!("Test case is in: test_cases/input/error_handling/non_string_values");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/non_string_values"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_error_handling_x_zero_blocksize() {
    println!("Test case is in: test_cases/input/error_handling/zero_blocksize");
    run_test(
        std::path::Path::new("../test_cases/input/error_handling/zero_blocksize"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_fasta_x_finish_stream() {
    println!("Test case is in: test_cases/input/fasta/finish_stream");
    run_test(
        std::path::Path::new("../test_cases/input/fasta/finish_stream"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_and_read_no() {
    println!("Test case is in: test_cases/input/interleaved/and_read_no");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/and_read_no"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_double() {
    println!("Test case is in: test_cases/input/interleaved/double");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/double"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_head() {
    println!("Test case is in: test_cases/input/interleaved/head");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/head"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved() {
    println!("Test case is in: test_cases/input/interleaved");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_just_one_segment() {
    println!("Test case is in: test_cases/input/interleaved/just_one_segment");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/just_one_segment"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_just_one_segment_x_vf() {
    println!("Test case is in: test_cases/input/interleaved/just_one_segment/vf");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/just_one_segment/vf"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_varying_formats() {
    println!("Test case is in: test_cases/input/interleaved/varying_formats");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/varying_formats"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_varying_formats_x_vf() {
    println!("Test case is in: test_cases/input/interleaved/varying_formats/vf");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/varying_formats/vf"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_varying_formats_bam() {
    println!("Test case is in: test_cases/input/interleaved/varying_formats_bam");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/varying_formats_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_interleaved_x_varying_formats_bam_x_vf() {
    println!("Test case is in: test_cases/input/interleaved/varying_formats_bam/vf");
    run_test(
        std::path::Path::new("../test_cases/input/interleaved/varying_formats_bam/vf"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_x_error_stdin_interleaved() {
    println!("Test case is in: test_cases/input/stdin/error_stdin_interleaved");
    run_test(
        std::path::Path::new("../test_cases/input/stdin/error_stdin_interleaved"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_x_error_stdin_regular_two_files_one_segement() {
    println!("Test case is in: test_cases/input/stdin/error_stdin_regular_two_files_one_segement");
    run_test(
        std::path::Path::new(
            "../test_cases/input/stdin/error_stdin_regular_two_files_one_segement",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_x_error_stdin_regular_two_files_one_segement_rev() {
    println!(
        "Test case is in: test_cases/input/stdin/error_stdin_regular_two_files_one_segement_rev"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/input/stdin/error_stdin_regular_two_files_one_segement_rev",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_x_error_stdin_regular_two_segments() {
    println!("Test case is in: test_cases/input/stdin/error_stdin_regular_two_segments");
    run_test(
        std::path::Path::new("../test_cases/input/stdin/error_stdin_regular_two_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_x_stdin_interleaved() {
    println!("Test case is in: test_cases/input/stdin/stdin_interleaved");
    run_test(
        std::path::Path::new("../test_cases/input/stdin/stdin_interleaved"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_x_stdin_regular() {
    println!("Test case is in: test_cases/input/stdin/stdin_regular");
    run_test(
        std::path::Path::new("../test_cases/input/stdin/stdin_regular"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_config_x_error_stdin_config_with_stdin_fastq() {
    println!("Test case is in: test_cases/input/stdin_config/error_stdin_config_with_stdin_fastq");
    run_test(
        std::path::Path::new(
            "../test_cases/input/stdin_config/error_stdin_config_with_stdin_fastq",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_config_x_stdin_config_process() {
    println!("Test case is in: test_cases/input/stdin_config/stdin_config_process");
    run_test(
        std::path::Path::new("../test_cases/input/stdin_config/stdin_config_process"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_config_x_stdin_config_validate_error() {
    println!("Test case is in: test_cases/input/stdin_config/stdin_config_validate_error");
    run_test(
        std::path::Path::new("../test_cases/input/stdin_config/stdin_config_validate_error"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_config_interleaved_x_error_stdin_config_with_stdin_fastq() {
    println!(
        "Test case is in: test_cases/input/stdin_config_interleaved/error_stdin_config_with_stdin_fastq"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/input/stdin_config_interleaved/error_stdin_config_with_stdin_fastq",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_config_interleaved_x_stdin_config_process() {
    println!("Test case is in: test_cases/input/stdin_config_interleaved/stdin_config_process");
    run_test(
        std::path::Path::new("../test_cases/input/stdin_config_interleaved/stdin_config_process"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_x_stdin_config_interleaved_x_stdin_config_validate_error() {
    println!(
        "Test case is in: test_cases/input/stdin_config_interleaved/stdin_config_validate_error"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/input/stdin_config_interleaved/stdin_config_validate_error",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_broken_gz() {
    println!("Test case is in: test_cases/input_validation/broken_gz");
    run_test(
        std::path::Path::new("../test_cases/input_validation/broken_gz"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_empty_bam_in_middle() {
    println!("Test case is in: test_cases/input_validation/empty_bam_in_middle");
    run_test(
        std::path::Path::new("../test_cases/input_validation/empty_bam_in_middle"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fasta_newline() {
    println!("Test case is in: test_cases/input_validation/fasta_newline");
    run_test(
        std::path::Path::new("../test_cases/input_validation/fasta_newline"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_breaking_after_sequence_in_partial() {
    println!(
        "Test case is in: test_cases/input_validation/fastq_breaking_after_sequence_in_partial"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/input_validation/fastq_breaking_after_sequence_in_partial",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_breakng_after_sequence_in_partial_but_stop_at_newline_so_check_in_spacer()
 {
    println!(
        "Test case is in: test_cases/input_validation/fastq_breakng_after_sequence_in_partial_but_stop_at_newline_so_check_in_spacer"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/input_validation/fastq_breakng_after_sequence_in_partial_but_stop_at_newline_so_check_in_spacer",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_breakng_after_sequence_in_partial_but_stop_at_newline_so_check_in_spacer_windows()
 {
    println!(
        "Test case is in: test_cases/input_validation/fastq_breakng_after_sequence_in_partial_but_stop_at_newline_so_check_in_spacer_windows"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/input_validation/fastq_breakng_after_sequence_in_partial_but_stop_at_newline_so_check_in_spacer_windows",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_invalid_bases() {
    println!("Test case is in: test_cases/input_validation/fastq_invalid_bases");
    run_test(
        std::path::Path::new("../test_cases/input_validation/fastq_invalid_bases"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_non_utf8_x_in_name() {
    println!("Test case is in: test_cases/input_validation/fastq_non_utf8/in_name");
    run_test(
        std::path::Path::new("../test_cases/input_validation/fastq_non_utf8/in_name"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_non_utf8_x_in_name_regx() {
    println!("Test case is in: test_cases/input_validation/fastq_non_utf8/in_name_regx");
    run_test(
        std::path::Path::new("../test_cases/input_validation/fastq_non_utf8/in_name_regx"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_non_utf8_x_in_sequence() {
    println!("Test case is in: test_cases/input_validation/fastq_non_utf8/in_sequence");
    run_test(
        std::path::Path::new("../test_cases/input_validation/fastq_non_utf8/in_sequence"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_fastq_non_utf8_x_in_sequence_validate_seq() {
    println!(
        "Test case is in: test_cases/input_validation/fastq_non_utf8/in_sequence_validate_seq"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/input_validation/fastq_non_utf8/in_sequence_validate_seq",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_not_fastq_nor_fasta_nor_bam() {
    println!("Test case is in: test_cases/input_validation/not_fastq_nor_fasta_nor_bam");
    run_test(
        std::path::Path::new("../test_cases/input_validation/not_fastq_nor_fasta_nor_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_too_many_segments() {
    println!("Test case is in: test_cases/input_validation/too_many_segments");
    run_test(
        std::path::Path::new("../test_cases/input_validation/too_many_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_input_validation_x_unsupported_compression() {
    println!("Test case is in: test_cases/input_validation/unsupported_compression");
    run_test(
        std::path::Path::new("../test_cases/input_validation/unsupported_compression"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_bam_merge_x_existing_files() {
    println!("Test case is in: test_cases/integration/bam_merge/existing_files");
    run_test(
        std::path::Path::new("../test_cases/integration/bam_merge/existing_files"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_basic_x_allow_overwrites() {
    println!("Test case is in: test_cases/integration/basic/allow_overwrites");
    run_test(
        std::path::Path::new("../test_cases/integration/basic/allow_overwrites"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_basic_x_noop() {
    println!("Test case is in: test_cases/integration/basic/noop");
    run_test(
        std::path::Path::new("../test_cases/integration/basic/noop"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_basic_x_noop_minimal() {
    println!("Test case is in: test_cases/integration/basic/noop_minimal");
    run_test(
        std::path::Path::new("../test_cases/integration/basic/noop_minimal"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_compatibility_x_fastp_416() {
    println!("Test case is in: test_cases/integration/compatibility/fastp_416");
    run_test(
        std::path::Path::new("../test_cases/integration/compatibility/fastp_416"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_compatibility_x_fastp_491() {
    println!("Test case is in: test_cases/integration/compatibility/fastp_491");
    run_test(
        std::path::Path::new("../test_cases/integration/compatibility/fastp_491"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_compatibility_x_fastp_606() {
    println!("Test case is in: test_cases/integration/compatibility/fastp_606");
    run_test(
        std::path::Path::new("../test_cases/integration/compatibility/fastp_606"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_compatibility_x_old_cli_format() {
    println!("Test case is in: test_cases/integration/compatibility/old_cli_format");
    run_test(
        std::path::Path::new("../test_cases/integration/compatibility/old_cli_format"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_complex_x_location_loss_on_conditional_trim() {
    println!("Test case is in: test_cases/integration/complex/location_loss_on_conditional_trim");
    run_test(
        std::path::Path::new("../test_cases/integration/complex/location_loss_on_conditional_trim"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_complex_x_order_maintained_in_single_core_transforms() {
    println!(
        "Test case is in: test_cases/integration/complex/order_maintained_in_single_core_transforms"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/integration/complex/order_maintained_in_single_core_transforms",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_complex_x_ten_segments_creative_transforms() {
    println!("Test case is in: test_cases/integration/complex/ten_segments_creative_transforms");
    run_test(
        std::path::Path::new("../test_cases/integration/complex/ten_segments_creative_transforms"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_duplicate_fragment_count_and_demultiplex() {
    println!("Test case is in: test_cases/integration/duplicate_fragment_count_and_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/integration/duplicate_fragment_count_and_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_edge_cases_x_max_one_block_in_flight() {
    println!("Test case is in: test_cases/integration/edge_cases/max_one_block_in_flight");
    run_test(
        std::path::Path::new("../test_cases/integration/edge_cases/max_one_block_in_flight"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_edge_cases_x_mega_long_reads() {
    println!("Test case is in: test_cases/integration/edge_cases/mega_long_reads");
    run_test(
        std::path::Path::new("../test_cases/integration/edge_cases/mega_long_reads"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_edge_cases_x_very_long_reads() {
    println!("Test case is in: test_cases/integration/edge_cases/very_long_reads");
    run_test(
        std::path::Path::new("../test_cases/integration/edge_cases/very_long_reads"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_head_x_tag_histogram_before_head() {
    println!("Test case is in: test_cases/integration/head/tag_histogram_before_head");
    run_test(
        std::path::Path::new("../test_cases/integration/head/tag_histogram_before_head"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_io_x_cut_end_named_pipes_x_both() {
    println!("Test case is in: test_cases/integration/io/cut_end_named_pipes/both");
    run_test(
        std::path::Path::new("../test_cases/integration/io/cut_end_named_pipes/both"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_io_x_cut_end_named_pipes_x_input_pipe() {
    println!("Test case is in: test_cases/integration/io/cut_end_named_pipes/input_pipe");
    run_test(
        std::path::Path::new("../test_cases/integration/io/cut_end_named_pipes/input_pipe"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_io_x_cut_end_named_pipes_x_named_output_pipe_and_chunked_fails() {
    println!(
        "Test case is in: test_cases/integration/io/cut_end_named_pipes/named_output_pipe_and_chunked_fails"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/integration/io/cut_end_named_pipes/named_output_pipe_and_chunked_fails",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_io_x_mixed_input_files() {
    println!("Test case is in: test_cases/integration/io/mixed_input_files");
    run_test(
        std::path::Path::new("../test_cases/integration/io/mixed_input_files"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_misc_x_head_with_index() {
    println!("Test case is in: test_cases/integration/misc/head_with_index");
    run_test(
        std::path::Path::new("../test_cases/integration/misc/head_with_index"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_regex_and_replace_in_seq_x_basic() {
    println!("Test case is in: test_cases/integration/regex_and_replace_in_seq/basic");
    run_test(
        std::path::Path::new("../test_cases/integration/regex_and_replace_in_seq/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_regex_and_replace_in_seq_x_multi_location() {
    println!("Test case is in: test_cases/integration/regex_and_replace_in_seq/multi_location");
    run_test(
        std::path::Path::new("../test_cases/integration/regex_and_replace_in_seq/multi_location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_reverse_complement_and_lowercase() {
    println!("Test case is in: test_cases/integration/reverse_complement_and_lowercase");
    run_test(
        std::path::Path::new("../test_cases/integration/reverse_complement_and_lowercase"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_x_tag_histogram_and_demultiplex() {
    println!("Test case is in: test_cases/integration/tag_histogram_and_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/integration/tag_histogram_and_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_tests_x_calc_x_quantify_regions_multi() {
    println!("Test case is in: test_cases/integration_tests/calc/quantify_regions_multi");
    run_test(
        std::path::Path::new("../test_cases/integration_tests/calc/quantify_regions_multi"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_tests_x_calc_x_quantify_regions_simple() {
    println!("Test case is in: test_cases/integration_tests/calc/quantify_regions_simple");
    run_test(
        std::path::Path::new("../test_cases/integration_tests/calc/quantify_regions_simple"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_tests_x_integration_tests_x_input_is_symlink() {
    println!("Test case is in: test_cases/integration_tests/integration_tests/input_is_symlink");
    run_test(
        std::path::Path::new("../test_cases/integration_tests/integration_tests/input_is_symlink"),
        "input_prelink.toml",
        1,
    );
}

#[test]

fn test_cases_x_integration_tests_x_quality_base_replacement() {
    println!("Test case is in: test_cases/integration_tests/quality_base_replacement");
    run_test(
        std::path::Path::new("../test_cases/integration_tests/quality_base_replacement"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_new_x_bam_stdout() {
    println!("Test case is in: test_cases/new/bam_stdout");
    run_test(
        std::path::Path::new("../test_cases/new/bam_stdout"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_new_x_bam_uncompressed_output() {
    println!("Test case is in: test_cases/new/bam_uncompressed_output");
    run_test(
        std::path::Path::new("../test_cases/new/bam_uncompressed_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_new_x_interleave_output_overlap() {
    println!("Test case is in: test_cases/new/interleave_output_overlap");
    run_test(
        std::path::Path::new("../test_cases/new/interleave_output_overlap"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_new_x_merge_reads_invalid_spacer_quality() {
    println!("Test case is in: test_cases/new/merge_reads_invalid_spacer_quality");
    run_test(
        std::path::Path::new("../test_cases/new/merge_reads_invalid_spacer_quality"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_new_x_merge_reads_missing_spacer() {
    println!("Test case is in: test_cases/new/merge_reads_missing_spacer");
    run_test(
        std::path::Path::new("../test_cases/new/merge_reads_missing_spacer"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_new_x_merge_reads_same_segment() {
    println!("Test case is in: test_cases/new/merge_reads_same_segment");
    run_test(
        std::path::Path::new("../test_cases/new/merge_reads_same_segment"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_new_x_stdout_with_explicit_output() {
    println!("Test case is in: test_cases/new/stdout_with_explicit_output");
    run_test(
        std::path::Path::new("../test_cases/new/stdout_with_explicit_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_basic() {
    println!("Test case is in: test_cases/output/bam/basic");
    run_test(
        std::path::Path::new("../test_cases/output/bam/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_basic_higher_compression() {
    println!("Test case is in: test_cases/output/bam/basic_higher_compression");
    run_test(
        std::path::Path::new("../test_cases/output/bam/basic_higher_compression"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_basic_higher_compression_wrong_level() {
    println!("Test case is in: test_cases/output/bam/basic_higher_compression_wrong_level");
    run_test(
        std::path::Path::new("../test_cases/output/bam/basic_higher_compression_wrong_level"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_interleaved() {
    println!("Test case is in: test_cases/output/bam/interleaved");
    run_test(
        std::path::Path::new("../test_cases/output/bam/interleaved"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_basic() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/basic");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_basic_interleaved_output() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/basic_interleaved_output");
    run_test(
        std::path::Path::new(
            "../test_cases/output/bam/merge_demultiplexed/basic_interleaved_output",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_basic_inv_ref_order() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/basic_inv_ref_order");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/basic_inv_ref_order"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_basic_multi_segment() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/basic_multi_segment");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/basic_multi_segment"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_configured_but_no_demultiplex() {
    println!(
        "Test case is in: test_cases/output/bam/merge_demultiplexed/configured_but_no_demultiplex"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/output/bam/merge_demultiplexed/configured_but_no_demultiplex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_just_one_barcode() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/just_one_barcode");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/just_one_barcode"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_just_one_per_pipeline() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/just_one_per_pipeline");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/just_one_per_pipeline"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_no_output() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/no_output");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/no_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_no_output_but_interleave() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/no_output_but_interleave");
    run_test(
        std::path::Path::new(
            "../test_cases/output/bam/merge_demultiplexed/no_output_but_interleave",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_no_tag_set() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/no_tag_set");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/no_tag_set"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_sep_in_sample_names() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/sep_in_sample_names");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/sep_in_sample_names"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_three_levels_bool() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/three_levels_bool");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/three_levels_bool"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_twice_on_same_label() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/twice_on_same_label");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/twice_on_same_label"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_two_levels() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/two_levels");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/two_levels"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_two_levels_after() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/two_levels_after");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/two_levels_after"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_typoed_demultiplex_step() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/typoed_demultiplex_step");
    run_test(
        std::path::Path::new(
            "../test_cases/output/bam/merge_demultiplexed/typoed_demultiplex_step",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_merge_demultiplexed_x_wrong_reference_tag() {
    println!("Test case is in: test_cases/output/bam/merge_demultiplexed/wrong_reference_tag");
    run_test(
        std::path::Path::new("../test_cases/output/bam/merge_demultiplexed/wrong_reference_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_reference_but_no_tag() {
    println!("Test case is in: test_cases/output/bam/reference_but_no_tag");
    run_test(
        std::path::Path::new("../test_cases/output/bam/reference_but_no_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_bam_tag() {
    println!("Test case is in: test_cases/output/bam/tag_to_bam_tag");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_bam_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_bam_tag_and_reference() {
    println!("Test case is in: test_cases/output/bam/tag_to_bam_tag_and_reference");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_bam_tag_and_reference"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_bam_tag_invalid_name() {
    println!("Test case is in: test_cases/output/bam/tag_to_bam_tag_invalid_name");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_bam_tag_invalid_name"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_bam_tag_missing_tag() {
    println!("Test case is in: test_cases/output/bam/tag_to_bam_tag_missing_tag");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_bam_tag_missing_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_bam_tag_must_not_be_comment() {
    println!("Test case is in: test_cases/output/bam/tag_to_bam_tag_must_not_be_comment");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_bam_tag_must_not_be_comment"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_bam_tag_output_tag_twice() {
    println!("Test case is in: test_cases/output/bam/tag_to_bam_tag_output_tag_twice");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_bam_tag_output_tag_twice"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_bam_tag_too_long() {
    println!("Test case is in: test_cases/output/bam/tag_to_bam_tag_too_long");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_bam_tag_too_long"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_referenc_no_ref_defined() {
    println!("Test case is in: test_cases/output/bam/tag_to_referenc_no_ref_defined");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_referenc_no_ref_defined"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_barcodes() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_barcodes");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_barcodes"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_barcodes_missing_tag() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_barcodes_missing_tag");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_barcodes_missing_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_barcodes_no_correction() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_barcodes_no_correction");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_barcodes_no_correction"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_barcodes_no_seq() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_barcodes_no_seq");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_barcodes_no_seq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_barcodes_no_such_barcode() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_barcodes_no_such_barcode");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_barcodes_no_such_barcode"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_from_bam() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_from_bam");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_from_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_from_bam_and_barcodes() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_from_bam_and_barcodes");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_from_bam_and_barcodes"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_from_bam_broken_bam() {
    println!("Test case is in: test_cases/output/bam/tag_to_reference_from_bam_broken_bam");
    run_test(
        std::path::Path::new("../test_cases/output/bam/tag_to_reference_from_bam_broken_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_bam_x_tag_to_reference_from_bam_non_existant_reference() {
    println!(
        "Test case is in: test_cases/output/bam/tag_to_reference_from_bam_non_existant_reference"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/output/bam/tag_to_reference_from_bam_non_existant_reference",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_chunked_x_bam() {
    println!("Test case is in: test_cases/output/chunked/bam");
    run_test(
        std::path::Path::new("../test_cases/output/chunked/bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_chunked_x_error_zero_chunk_size() {
    println!("Test case is in: test_cases/output/chunked/error_zero_chunk_size");
    run_test(
        std::path::Path::new("../test_cases/output/chunked/error_zero_chunk_size"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_chunked_x_fastq_exceeding_100_chunks() {
    println!("Test case is in: test_cases/output/chunked/fastq_exceeding_100_chunks");
    run_test(
        std::path::Path::new("../test_cases/output/chunked/fastq_exceeding_100_chunks"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_chunked_x_fastq_gzip() {
    println!("Test case is in: test_cases/output/chunked/fastq_gzip");
    run_test(
        std::path::Path::new("../test_cases/output/chunked/fastq_gzip"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_chunked_x_table() {
    println!("Test case is in: test_cases/output/chunked/table");
    run_test(
        std::path::Path::new("../test_cases/output/chunked/table"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_custom_ix_separator() {
    println!("Test case is in: test_cases/output/custom_ix_separator");
    run_test(
        std::path::Path::new("../test_cases/output/custom_ix_separator"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_custom_ix_separator_table_no_infix() {
    println!("Test case is in: test_cases/output/custom_ix_separator_table_no_infix");
    run_test(
        std::path::Path::new("../test_cases/output/custom_ix_separator_table_no_infix"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_backslash_in_x_sep() {
    println!("Test case is in: test_cases/output/error_handling/backslash_in_x_sep");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/backslash_in_x_sep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_colon_in_ix_sep() {
    println!("Test case is in: test_cases/output/error_handling/colon_in_ix_sep");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/colon_in_ix_sep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_compression_threads_0() {
    println!("Test case is in: test_cases/output/error_handling/compression_threads_0");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/compression_threads_0"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_infix_path_sep() {
    println!("Test case is in: test_cases/output/error_handling/infix_path_sep");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/infix_path_sep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_ix_sep_empty() {
    println!("Test case is in: test_cases/output/error_handling/ix_sep_empty");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/ix_sep_empty"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_prefix_empty() {
    println!("Test case is in: test_cases/output/error_handling/prefix_empty");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/prefix_empty"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_slash_in_ix_sep() {
    println!("Test case is in: test_cases/output/error_handling/slash_in_ix_sep");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/slash_in_ix_sep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_suffix_empty() {
    println!("Test case is in: test_cases/output/error_handling/suffix_empty");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/suffix_empty"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_error_handling_x_suffix_path_sep() {
    println!("Test case is in: test_cases/output/error_handling/suffix_path_sep");
    run_test(
        std::path::Path::new("../test_cases/output/error_handling/suffix_path_sep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_hash_output_both() {
    println!("Test case is in: test_cases/output/hash_output_both");
    run_test(
        std::path::Path::new("../test_cases/output/hash_output_both"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_hash_output_compressed() {
    println!("Test case is in: test_cases/output/hash_output_compressed");
    run_test(
        std::path::Path::new("../test_cases/output/hash_output_compressed"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_interleaved_empty_output_empty() {
    println!("Test case is in: test_cases/output/interleaved_empty_output_empty");
    run_test(
        std::path::Path::new("../test_cases/output/interleaved_empty_output_empty"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_interleaved_empty_output_ok() {
    println!("Test case is in: test_cases/output/interleaved_empty_output_ok");
    run_test(
        std::path::Path::new("../test_cases/output/interleaved_empty_output_ok"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_interleaved_output() {
    println!("Test case is in: test_cases/output/interleaved_output");
    run_test(
        std::path::Path::new("../test_cases/output/interleaved_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_multiple_x_bam_head_fastq() {
    println!("Test case is in: test_cases/output/multiple/bam_head_fastq");
    run_test(
        std::path::Path::new("../test_cases/output/multiple/bam_head_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_multiple_x_fasta_head_bam() {
    println!("Test case is in: test_cases/output/multiple/fasta_head_bam");
    run_test(
        std::path::Path::new("../test_cases/output/multiple/fasta_head_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_multiple_x_fastq_head_fasta() {
    println!("Test case is in: test_cases/output/multiple/fastq_head_fasta");
    run_test(
        std::path::Path::new("../test_cases/output/multiple/fastq_head_fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_multiple_x_report_head_fastq() {
    println!("Test case is in: test_cases/output/multiple/report_head_fastq");
    run_test(
        std::path::Path::new("../test_cases/output/multiple/report_head_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_one_report_is_enough_x_html() {
    println!("Test case is in: test_cases/output/one_report_is_enough/html");
    run_test(
        std::path::Path::new("../test_cases/output/one_report_is_enough/html"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_one_report_is_enough_x_json() {
    println!("Test case is in: test_cases/output/one_report_is_enough/json");
    run_test(
        std::path::Path::new("../test_cases/output/one_report_is_enough/json"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_x_chunked_x_fastq() {
    println!("Test case is in: test_cases/output/output/chunked/fastq");
    run_test(
        std::path::Path::new("../test_cases/output/output/chunked/fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_x_chunked_x_fastq_but_2nd_file_unwritable() {
    println!("Test case is in: test_cases/output/output/chunked/fastq_but_2nd_file_unwritable");
    run_test(
        std::path::Path::new("../test_cases/output/output/chunked/fastq_but_2nd_file_unwritable"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_x_fastq() {
    println!("Test case is in: test_cases/output/output/fastq");
    run_test(
        std::path::Path::new("../test_cases/output/output/fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_x_twice() {
    println!("Test case is in: test_cases/output/output/twice");
    run_test(
        std::path::Path::new("../test_cases/output/output/twice"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_x_twice_with_head_between() {
    println!("Test case is in: test_cases/output/output/twice_with_head_between");
    run_test(
        std::path::Path::new("../test_cases/output/output/twice_with_head_between"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_compression_gzip_level() {
    println!("Test case is in: test_cases/output/output_compression_gzip_level");
    run_test(
        std::path::Path::new("../test_cases/output/output_compression_gzip_level"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_compression_single_thread() {
    println!("Test case is in: test_cases/output/output_compression_single_thread");
    run_test(
        std::path::Path::new("../test_cases/output/output_compression_single_thread"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_compression_single_thread_with_compression_level() {
    println!(
        "Test case is in: test_cases/output/output_compression_single_thread_with_compression_level"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/output/output_compression_single_thread_with_compression_level",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_compression_single_thread_with_compression_level_fasta() {
    println!(
        "Test case is in: test_cases/output/output_compression_single_thread_with_compression_level_fasta"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/output/output_compression_single_thread_with_compression_level_fasta",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_compression_single_thread_zst() {
    println!("Test case is in: test_cases/output/output_compression_single_thread_zst");
    run_test(
        std::path::Path::new("../test_cases/output/output_compression_single_thread_zst"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_different_suffix() {
    println!("Test case is in: test_cases/output/output_different_suffix");
    run_test(
        std::path::Path::new("../test_cases/output/output_different_suffix"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_neither_r1_nor_r2() {
    println!("Test case is in: test_cases/output/output_neither_r1_nor_r2");
    run_test(
        std::path::Path::new("../test_cases/output/output_neither_r1_nor_r2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_neither_r1_nor_r2_but_index2() {
    println!("Test case is in: test_cases/output/output_neither_r1_nor_r2_but_index2");
    run_test(
        std::path::Path::new("../test_cases/output/output_neither_r1_nor_r2_but_index2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_r1_only() {
    println!("Test case is in: test_cases/output/output_r1_only");
    run_test(
        std::path::Path::new("../test_cases/output/output_r1_only"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_r2_only() {
    println!("Test case is in: test_cases/output/output_r2_only");
    run_test(
        std::path::Path::new("../test_cases/output/output_r2_only"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_suffix_x_chunked_x_fastq() {
    println!("Test case is in: test_cases/output/output_suffix/chunked/fastq");
    run_test(
        std::path::Path::new("../test_cases/output/output_suffix/chunked/fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_suffix_x_fasta() {
    println!("Test case is in: test_cases/output/output_suffix/fasta");
    run_test(
        std::path::Path::new("../test_cases/output/output_suffix/fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_output_suffix_x_fastq() {
    println!("Test case is in: test_cases/output/output_suffix/fastq");
    run_test(
        std::path::Path::new("../test_cases/output/output_suffix/fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_progress_x_basic() {
    println!("Test case is in: test_cases/output/progress/basic");
    run_test(
        std::path::Path::new("../test_cases/output/progress/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_progress_x_followed_by_head() {
    println!("Test case is in: test_cases/output/progress/followed_by_head");
    run_test(
        std::path::Path::new("../test_cases/output/progress/followed_by_head"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_progress_x_stdout() {
    println!("Test case is in: test_cases/output/progress/stdout");
    run_test(
        std::path::Path::new("../test_cases/output/progress/stdout"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_output_x_stdout_output_interleaved() {
    println!("Test case is in: test_cases/output/stdout_output_interleaved");
    run_test(
        std::path::Path::new("../test_cases/output/stdout_output_interleaved"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calccomplexity() {
    println!("Test case is in: test_cases/single_step/CalcComplexity");
    run_test(
        std::path::Path::new("../test_cases/single_step/CalcComplexity"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_assign_by_halves_x_based_on_string_tag() {
    println!("Test case is in: test_cases/single_step/assign_by_halves/based_on_string_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/assign_by_halves/based_on_string_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_assign_by_halves_x_basic() {
    println!("Test case is in: test_cases/single_step/assign_by_halves/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/assign_by_halves/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_assign_by_halves_x_broken_barcodes() {
    println!("Test case is in: test_cases/single_step/assign_by_halves/broken_barcodes");
    run_test(
        std::path::Path::new("../test_cases/single_step/assign_by_halves/broken_barcodes"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_assign_by_halves_x_empty_barcodes_defined() {
    println!("Test case is in: test_cases/single_step/assign_by_halves/empty_barcodes_defined");
    run_test(
        std::path::Path::new("../test_cases/single_step/assign_by_halves/empty_barcodes_defined"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_assign_by_halves_x_in_must_be_unequal_out() {
    println!("Test case is in: test_cases/single_step/assign_by_halves/in_must_be_unequal_out");
    run_test(
        std::path::Path::new("../test_cases/single_step/assign_by_halves/in_must_be_unequal_out"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_assign_by_halves_x_missing_barcode_section() {
    println!("Test case is in: test_cases/single_step/assign_by_halves/missing_barcode_section");
    run_test(
        std::path::Path::new("../test_cases/single_step/assign_by_halves/missing_barcode_section"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_assign_by_halves_x_no_barcodes_defined() {
    println!("Test case is in: test_cases/single_step/assign_by_halves/no_barcodes_defined");
    run_test(
        std::path::Path::new("../test_cases/single_step/assign_by_halves/no_barcodes_defined"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_auto_x_detect_x_on_x_stderr() {
    println!("Test case is in: test_cases/single_step/auto-detect-on-stderr");
    run_test(
        std::path::Path::new("../test_cases/single_step/auto-detect-on-stderr"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_convert_to_rate_x_all_segments() {
    println!("Test case is in: test_cases/single_step/calc/convert_to_rate/all_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/convert_to_rate/all_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_convert_to_rate_x_basic() {
    println!("Test case is in: test_cases/single_step/calc/convert_to_rate/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/convert_to_rate/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_expected_error_x_basic() {
    println!("Test case is in: test_cases/single_step/calc/expected_error/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/expected_error/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_expected_error_x_basic_all_segments() {
    println!("Test case is in: test_cases/single_step/calc/expected_error/basic_all_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/expected_error/basic_all_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_expected_error_x_input_error_invalid_quality() {
    println!(
        "Test case is in: test_cases/single_step/calc/expected_error/input_error_invalid_quality"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/calc/expected_error/input_error_invalid_quality",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_expected_error_x_input_error_invalid_quality_single_segment() {
    println!(
        "Test case is in: test_cases/single_step/calc/expected_error/input_error_invalid_quality_single_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/calc/expected_error/input_error_invalid_quality_single_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_expected_error_x_max() {
    println!("Test case is in: test_cases/single_step/calc/expected_error/max");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/expected_error/max"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_expected_error_x_max_all_segments() {
    println!("Test case is in: test_cases/single_step/calc/expected_error/max_all_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/expected_error/max_all_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_basic() {
    println!("Test case is in: test_cases/single_step/calc/kmer/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_basic_higher_min_count() {
    println!("Test case is in: test_cases/single_step/calc/kmer/basic_higher_min_count");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/basic_higher_min_count"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_basic_query_matches_k() {
    println!("Test case is in: test_cases/single_step/calc/kmer/basic_query_matches_k");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/basic_query_matches_k"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_basic_segments() {
    println!("Test case is in: test_cases/single_step/calc/kmer/basic_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/basic_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_files_as_sequence() {
    println!("Test case is in: test_cases/single_step/calc/kmer/files_as_sequence");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/files_as_sequence"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_min_count_zero_error() {
    println!("Test case is in: test_cases/single_step/calc/kmer/min_count_zero_error");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/min_count_zero_error"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_multiple_kmer_files() {
    println!("Test case is in: test_cases/single_step/calc/kmer/multiple_kmer_files");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/multiple_kmer_files"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_kmer_x_phix() {
    println!("Test case is in: test_cases/single_step/calc/kmer/phix");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/kmer/phix"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_all() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/all");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_all_with_offset() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/all_with_offset");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/all_with_offset"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_basic() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_basic_with_offset() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/basic_with_offset");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/basic_with_offset"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_empty_read() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/empty_read");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/empty_read"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_empty_tag() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/empty_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/empty_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_location_tag() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/location_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/location_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_missing_tag() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/missing_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/missing_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_wrong_tag_kind() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/wrong_tag_kind");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/wrong_tag_kind"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_calc_x_worst_quality_x_wrong_tag_kind_string() {
    println!("Test case is in: test_cases/single_step/calc/worst_quality/wrong_tag_kind_string");
    run_test(
        std::path::Path::new("../test_cases/single_step/calc/worst_quality/wrong_tag_kind_string"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compare_string_tags_x_basic() {
    println!("Test case is in: test_cases/single_step/compare_string_tags/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/compare_string_tags/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compare_string_tags_x_location() {
    println!("Test case is in: test_cases/single_step/compare_string_tags/location");
    run_test(
        std::path::Path::new("../test_cases/single_step/compare_string_tags/location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compare_string_tags_x_missing() {
    println!("Test case is in: test_cases/single_step/compare_string_tags/missing");
    run_test(
        std::path::Path::new("../test_cases/single_step/compare_string_tags/missing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compare_string_tags_x_mixed_1() {
    println!("Test case is in: test_cases/single_step/compare_string_tags/mixed_1");
    run_test(
        std::path::Path::new("../test_cases/single_step/compare_string_tags/mixed_1"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compare_string_tags_x_mixed_2() {
    println!("Test case is in: test_cases/single_step/compare_string_tags/mixed_2");
    run_test(
        std::path::Path::new("../test_cases/single_step/compare_string_tags/mixed_2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compare_string_tags_x_unequal_len() {
    println!("Test case is in: test_cases/single_step/compare_string_tags/unequal_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/compare_string_tags/unequal_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compression_x_zstd_input() {
    println!("Test case is in: test_cases/single_step/compression/zstd_input");
    run_test(
        std::path::Path::new("../test_cases/single_step/compression/zstd_input"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compression_x_zstd_input_gzip_output() {
    println!("Test case is in: test_cases/single_step/compression/zstd_input_gzip_output");
    run_test(
        std::path::Path::new("../test_cases/single_step/compression/zstd_input_gzip_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compression_x_zstd_input_read_swap() {
    println!("Test case is in: test_cases/single_step/compression/zstd_input_read_swap");
    run_test(
        std::path::Path::new("../test_cases/single_step/compression/zstd_input_read_swap"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_compression_x_zstd_input_zst_output() {
    println!("Test case is in: test_cases/single_step/compression/zstd_input_zst_output");
    run_test(
        std::path::Path::new("../test_cases/single_step/compression/zstd_input_zst_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_concat_tags_too_few_labels() {
    println!("Test case is in: test_cases/single_step/concat_tags/concat_tags_too_few_labels");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/concat_tags_too_few_labels"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_location_and_string_concat() {
    println!("Test case is in: test_cases/single_step/concat_tags/location_and_string_concat");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/location_and_string_concat"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_location_and_string_concat_does_not_provide_location() {
    println!(
        "Test case is in: test_cases/single_step/concat_tags/location_and_string_concat_does_not_provide_location"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/concat_tags/location_and_string_concat_does_not_provide_location",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_location_and_string_concat_outputs_location() {
    println!(
        "Test case is in: test_cases/single_step/concat_tags/location_and_string_concat_outputs_location"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/concat_tags/location_and_string_concat_outputs_location",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_location_concat() {
    println!("Test case is in: test_cases/single_step/concat_tags/location_concat");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/location_concat"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_multiple_hits_per_tag() {
    println!("Test case is in: test_cases/single_step/concat_tags/multiple_hits_per_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/multiple_hits_per_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_multiple_hits_per_tag_anchor_right() {
    println!(
        "Test case is in: test_cases/single_step/concat_tags/multiple_hits_per_tag_anchor_right"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/concat_tags/multiple_hits_per_tag_anchor_right",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_repeated_tag_fails() {
    println!("Test case is in: test_cases/single_step/concat_tags/repeated_tag_fails");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/repeated_tag_fails"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_string_string_concat() {
    println!("Test case is in: test_cases/single_step/concat_tags/string_string_concat");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/string_string_concat"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_three_tags() {
    println!("Test case is in: test_cases/single_step/concat_tags/three_tags");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/three_tags"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_three_tags_set_missing() {
    println!("Test case is in: test_cases/single_step/concat_tags/three_tags_set_missing");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/three_tags_set_missing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_with_missing_and_separator() {
    println!("Test case is in: test_cases/single_step/concat_tags/with_missing_and_separator");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/with_missing_and_separator"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_with_missing_and_separator_merge_present() {
    println!(
        "Test case is in: test_cases/single_step/concat_tags/with_missing_and_separator_merge_present"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/concat_tags/with_missing_and_separator_merge_present",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_with_missing_and_separator_merge_present_mixed() {
    println!(
        "Test case is in: test_cases/single_step/concat_tags/with_missing_and_separator_merge_present_mixed"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/concat_tags/with_missing_and_separator_merge_present_mixed",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_with_missing_tag_merge_present() {
    println!("Test case is in: test_cases/single_step/concat_tags/with_missing_tag_merge_present");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/concat_tags/with_missing_tag_merge_present",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_concat_tags_x_with_missing_tag_set_missing() {
    println!("Test case is in: test_cases/single_step/concat_tags/with_missing_tag_set_missing");
    run_test(
        std::path::Path::new("../test_cases/single_step/concat_tags/with_missing_tag_set_missing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_convert_to_rate_x_all_segments() {
    println!("Test case is in: test_cases/single_step/convert/convert_to_rate/all_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert/convert_to_rate/all_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_regions_to_length_x_basic() {
    println!("Test case is in: test_cases/single_step/convert/regions_to_length/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert/regions_to_length/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_regions_to_length_x_error_in_equal_out() {
    println!(
        "Test case is in: test_cases/single_step/convert/regions_to_length/error_in_equal_out"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/convert/regions_to_length/error_in_equal_out",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_regions_to_length_x_multiple_regions() {
    println!("Test case is in: test_cases/single_step/convert/regions_to_length/multiple_regions");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/convert/regions_to_length/multiple_regions",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_regions_to_length_x_polyx() {
    println!("Test case is in: test_cases/single_step/convert/regions_to_length/polyx");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert/regions_to_length/polyx"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_to_rate_x_basic() {
    println!("Test case is in: test_cases/single_step/convert/to_rate/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert/to_rate/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_to_rate_x_denominator_tag() {
    println!("Test case is in: test_cases/single_step/convert/to_rate/denominator_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert/to_rate/denominator_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_x_to_rate_x_log_variants() {
    println!("Test case is in: test_cases/single_step/convert/to_rate/log_variants");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert/to_rate/log_variants"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_quality_x_convert_phred() {
    println!("Test case is in: test_cases/single_step/convert_quality/convert_phred");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert_quality/convert_phred"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_quality_x_convert_phred_broken_input() {
    println!("Test case is in: test_cases/single_step/convert_quality/convert_phred_broken_input");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/convert_quality/convert_phred_broken_input",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_quality_x_convert_phred_multi() {
    println!("Test case is in: test_cases/single_step/convert_quality/convert_phred_multi");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert_quality/convert_phred_multi"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_convert_quality_x_error_convert_to_same() {
    println!("Test case is in: test_cases/single_step/convert_quality/error_convert_to_same");
    run_test(
        std::path::Path::new("../test_cases/single_step/convert_quality/error_convert_to_same"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_basic() {
    println!("Test case is in: test_cases/single_step/duplicates/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_all_segments() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_all_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_all_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_all_segments_name() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_all_segments_name");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_all_segments_name"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_exact() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_exact");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_exact"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_keep_duplicates() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_keep_duplicates");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_keep_duplicates"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_read2() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_read2");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_read2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_read_combo() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_read_combo");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_read_combo"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_read_combo_demultiplex() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_read_combo_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_read_combo_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_dedup_read_combo_incl_index() {
    println!("Test case is in: test_cases/single_step/duplicates/dedup_read_combo_incl_index");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/dedup_read_combo_incl_index"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_error_fpr_x_too_high() {
    println!("Test case is in: test_cases/single_step/duplicates/error_fpr/too_high");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/error_fpr/too_high"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_error_fpr_x_too_low() {
    println!("Test case is in: test_cases/single_step/duplicates/error_fpr/too_low");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/error_fpr/too_low"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_error_no_seed() {
    println!("Test case is in: test_cases/single_step/duplicates/error_no_seed");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/error_no_seed"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_exact_location_tag() {
    println!("Test case is in: test_cases/single_step/duplicates/exact_location_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/exact_location_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_exact_name() {
    println!("Test case is in: test_cases/single_step/duplicates/exact_name");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/exact_name"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_exact_tag() {
    println!("Test case is in: test_cases/single_step/duplicates/exact_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/exact_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_duplicates_x_exact_tag_missing_values() {
    println!("Test case is in: test_cases/single_step/duplicates/exact_tag_missing_values");
    run_test(
        std::path::Path::new("../test_cases/single_step/duplicates/exact_tag_missing_values"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_illumina_x_cat() {
    println!("Test case is in: test_cases/single_step/edge_cases/challenging_formats/illumina/cat");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/illumina/cat",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_illumina_x_to_sanger() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/illumina/to_sanger"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/illumina/to_sanger",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_illumina_x_to_solexa() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/illumina/to_solexa"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/illumina/to_solexa",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_longreads_x_cat() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/longreads/cat"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/longreads/cat",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_misc_dna_x_as_illumina() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/misc_dna/as_illumina"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/misc_dna/as_illumina",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_misc_dna_x_as_solexa() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/misc_dna/as_solexa"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/misc_dna/as_solexa",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_misc_dna_x_cat() {
    println!("Test case is in: test_cases/single_step/edge_cases/challenging_formats/misc_dna/cat");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/misc_dna/cat",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_misc_rna_x_as_illumina() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/misc_rna/as_illumina"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/misc_rna/as_illumina",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_misc_rna_x_as_solexa() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/misc_rna/as_solexa"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/misc_rna/as_solexa",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_misc_rna_x_cat() {
    println!("Test case is in: test_cases/single_step/edge_cases/challenging_formats/misc_rna/cat");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/misc_rna/cat",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_sanger_full_range_x_as_illumina() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/sanger_full_range/as_illumina"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/sanger_full_range/as_illumina",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_sanger_full_range_x_as_solexa() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/sanger_full_range/as_solexa"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/sanger_full_range/as_solexa",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_sanger_full_range_x_cat() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/sanger_full_range/cat"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/sanger_full_range/cat",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_solexa_x_as_illumina() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/solexa/as_illumina"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/solexa/as_illumina",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_solexa_x_as_sanger() {
    println!(
        "Test case is in: test_cases/single_step/edge_cases/challenging_formats/solexa/as_sanger"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/solexa/as_sanger",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_solexa_x_cat() {
    println!("Test case is in: test_cases/single_step/edge_cases/challenging_formats/solexa/cat");
    run_test(
        std::path::Path::new("../test_cases/single_step/edge_cases/challenging_formats/solexa/cat"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edge_cases_x_challenging_formats_x_wrapping_x_cat() {
    println!("Test case is in: test_cases/single_step/edge_cases/challenging_formats/wrapping/cat");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edge_cases/challenging_formats/wrapping/cat",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_lowercase_name() {
    println!("Test case is in: test_cases/single_step/edits/lowercase_name");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/lowercase_name"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_lowercase_name_autodetect_segment() {
    println!("Test case is in: test_cases/single_step/edits/lowercase_name_autodetect_segment");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/lowercase_name_autodetect_segment"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_lowercase_sequence() {
    println!("Test case is in: test_cases/single_step/edits/lowercase_sequence");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/lowercase_sequence"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_lowercase_tag() {
    println!("Test case is in: test_cases/single_step/edits/lowercase_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/lowercase_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_dont_revcomp_segment2() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/dont_revcomp_segment2");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/dont_revcomp_segment2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_fastp_but_short() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/fastp_but_short");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/fastp_but_short"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_forgets_location_tag() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/forgets_location_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/forgets_location_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_location_tag_lost_after_merge() {
    println!(
        "Test case is in: test_cases/single_step/edits/merge_reads/location_tag_lost_after_merge"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/merge_reads/location_tag_lost_after_merge",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_location_tag_lost_after_merge_other_segment() {
    println!(
        "Test case is in: test_cases/single_step/edits/merge_reads/location_tag_lost_after_merge_other_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/merge_reads/location_tag_lost_after_merge_other_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_no_overlap_concatenate() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/no_overlap_concatenate");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/no_overlap_concatenate"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_no_overlap_keep() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/no_overlap_keep");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/no_overlap_keep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_overlap_perfect_match() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/overlap_perfect_match");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/overlap_perfect_match"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_overlap_with_tag() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/overlap_with_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/overlap_with_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_vs_fastp() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/vs_fastp");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/vs_fastp"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_vs_fastp_systematic() {
    println!("Test case is in: test_cases/single_step/edits/merge_reads/vs_fastp_systematic");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/merge_reads/vs_fastp_systematic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_merge_reads_x_vs_fastp_systematic_but_limit_is_percentage() {
    println!(
        "Test case is in: test_cases/single_step/edits/merge_reads/vs_fastp_systematic_but_limit_is_percentage"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/merge_reads/vs_fastp_systematic_but_limit_is_percentage",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_basic() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/reverse_complement/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_conditional() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/conditional");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/reverse_complement/conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_name_not_supported() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/name_not_supported");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/reverse_complement/name_not_supported",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/reverse_complement/tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag_conditional() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/tag_conditional");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/reverse_complement/tag_conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag_lost_location() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/tag_lost_location");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/reverse_complement/tag_lost_location",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag_lost_location_after_edits() {
    println!(
        "Test case is in: test_cases/single_step/edits/reverse_complement/tag_lost_location_after_edits"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/reverse_complement/tag_lost_location_after_edits",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag_lost_location_complain() {
    println!(
        "Test case is in: test_cases/single_step/edits/reverse_complement/tag_lost_location_complain"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/reverse_complement/tag_lost_location_complain",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag_split() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/tag_split");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/reverse_complement/tag_split"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag_string_value() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/tag_string_value");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/reverse_complement/tag_string_value"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_tag_string_value_conditional() {
    println!(
        "Test case is in: test_cases/single_step/edits/reverse_complement/tag_string_value_conditional"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/edits/reverse_complement/tag_string_value_conditional",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_reverse_complement_x_with_missing() {
    println!("Test case is in: test_cases/single_step/edits/reverse_complement/with_missing");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/reverse_complement/with_missing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_auto_detect_two_segments() {
    println!("Test case is in: test_cases/single_step/edits/swap/auto_detect_two_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/auto_detect_two_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_conditional() {
    println!("Test case is in: test_cases/single_step/edits/swap/conditional");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_conditional_extended() {
    println!("Test case is in: test_cases/single_step/edits/swap/conditional_extended");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/conditional_extended"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_conditional_majority() {
    println!("Test case is in: test_cases/single_step/edits/swap/conditional_majority");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/conditional_majority"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_conditional_minority() {
    println!("Test case is in: test_cases/single_step/edits/swap/conditional_minority");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/conditional_minority"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_conditional_with_name_region() {
    println!("Test case is in: test_cases/single_step/edits/swap/conditional_with_name_region");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/conditional_with_name_region"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_conditional_with_regions() {
    println!("Test case is in: test_cases/single_step/edits/swap/conditional_with_regions");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/conditional_with_regions"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_four_segments() {
    println!("Test case is in: test_cases/single_step/edits/swap/four_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/four_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_unequal_lengths() {
    println!("Test case is in: test_cases/single_step/edits/swap/unequal_lengths");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/unequal_lengths"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_swap_x_unequal_lengths_conditional() {
    println!("Test case is in: test_cases/single_step/edits/swap/unequal_lengths_conditional");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/swap/unequal_lengths_conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_uppercase_location_tag() {
    println!("Test case is in: test_cases/single_step/edits/uppercase_location_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/uppercase_location_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_uppercase_location_tag_conditional() {
    println!("Test case is in: test_cases/single_step/edits/uppercase_location_tag_conditional");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/uppercase_location_tag_conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_uppercase_sequence() {
    println!("Test case is in: test_cases/single_step/edits/uppercase_sequence");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/uppercase_sequence"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_uppercase_sequence_conditional() {
    println!("Test case is in: test_cases/single_step/edits/uppercase_sequence_conditional");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/uppercase_sequence_conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_edits_x_uppercase_string_conditional() {
    println!("Test case is in: test_cases/single_step/edits/uppercase_string_conditional");
    run_test(
        std::path::Path::new("../test_cases/single_step/edits/uppercase_string_conditional"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_spotcheckreadpairing_x_empty_read_name() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/SpotCheckReadPairing/empty_read_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/SpotCheckReadPairing/empty_read_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_spotcheckreadpairing_x_not_paired() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/SpotCheckReadPairing/not_paired"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/SpotCheckReadPairing/not_paired",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_spotcheckreadpairing_x_sample_stride_0() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/SpotCheckReadPairing/sample_stride_0"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/SpotCheckReadPairing/sample_stride_0",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_bam_x_bam_missing_input_settings_x_both_false() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/bam/bam_missing_input_settings/both_false"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/bam/bam_missing_input_settings/both_false",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_bam_x_bam_missing_input_settings_x_mapped() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/bam/bam_missing_input_settings/mapped"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/bam/bam_missing_input_settings/mapped",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_bam_x_bam_missing_input_settings_x_unmapped() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/bam/bam_missing_input_settings/unmapped"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/bam/bam_missing_input_settings/unmapped",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_bam_x_bam_output_uncompressed_hash() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/bam/bam_output_uncompressed_hash"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/bam/bam_output_uncompressed_hash",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_check_name_collisions_x_segment_barcode() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/check_name_collisions/segment_barcode"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/check_name_collisions/segment_barcode",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_check_name_collisions_x_segment_barcode_interleaved()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/check_name_collisions/segment_barcode_interleaved"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/check_name_collisions/segment_barcode_interleaved",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_check_name_collisions_x_segment_tag() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/check_name_collisions/segment_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/check_name_collisions/segment_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_check_name_collisions_x_tag_barcode() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/check_name_collisions/tag_barcode"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/check_name_collisions/tag_barcode",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_compression_x_compression_detection_wrong_extension()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/compression/compression_detection_wrong_extension"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/compression/compression_detection_wrong_extension",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_compression_x_invalid_compression_levels_x_inspect_gzip_level_too_high()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/compression/invalid_compression_levels/inspect_gzip_level_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/compression/invalid_compression_levels/inspect_gzip_level_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_compression_x_invalid_compression_levels_x_inspect_zstd_level_zero()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/compression/invalid_compression_levels/inspect_zstd_level_zero"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/compression/invalid_compression_levels/inspect_zstd_level_zero",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_compression_x_invalid_compression_levels_x_raw_with_compression_level()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/compression/invalid_compression_levels/raw_with_compression_level"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/compression/invalid_compression_levels/raw_with_compression_level",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_demultiplex_x_barcodes_x_bad_barcode_name() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/demultiplex/barcodes/bad_barcode_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/demultiplex/barcodes/bad_barcode_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_demultiplex_x_barcodes_x_different_barcode_lengths()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/demultiplex/barcodes/different_barcode_lengths"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/demultiplex/barcodes/different_barcode_lengths",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_demultiplex_x_barcodes_x_different_files() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/demultiplex/barcodes/different_files"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/demultiplex/barcodes/different_files",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_demultiplex_x_barcodes_x_empty() {
    println!("Test case is in: test_cases/single_step/error_handling/demultiplex/barcodes/empty");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/demultiplex/barcodes/empty"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_demultiplex_x_barcodes_x_non_iupac() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/demultiplex/barcodes/non_iupac"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/demultiplex/barcodes/non_iupac",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_demultiplex_x_barcodes_x_same_files() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/demultiplex/barcodes/same_files"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/demultiplex/barcodes/same_files",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_dna_validation_x_dna_validation_count_oligos_non_agtc()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/dna_validation/dna_validation_count_oligos_non_agtc"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/dna_validation/dna_validation_count_oligos_non_agtc",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_dna_validation_x_dna_validation_count_oligos_non_empty()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/dna_validation/dna_validation_count_oligos_non_empty"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/dna_validation/dna_validation_count_oligos_non_empty",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_all_on_non_all_segments() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/all_on_non_all_segments"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/all_on_non_all_segments",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_all_on_non_segment_or_name() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/all_on_non_segment_or_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/all_on_non_segment_or_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_barcodes_as_list() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/barcodes_as_list"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/barcodes_as_list",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_empty_tag_name() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/empty_tag_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/empty_tag_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_no_source_on_source_all() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/no_source_on_source_all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/no_source_on_source_all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_show_step_template_on_error() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/show_step_template_on_error"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/show_step_template_on_error",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_tag_not_defined() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/tag_not_defined"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/tag_not_defined",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_two_mistakes_eserde() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/two_mistakes_eserde"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/two_mistakes_eserde",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_error_messages_x_two_mistakes_post_deserialization()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/error_messages/two_mistakes_post_deserialization"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/error_messages/two_mistakes_post_deserialization",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_eval_expr_x_len_from_not_a_len_tag() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/eval_expr/len_from_not_a_len_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/eval_expr/len_from_not_a_len_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_base_content_absolute_with_ignore()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_base_content_absolute_with_ignore"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_base_content_absolute_with_ignore",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_base_content_empty_count() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_base_content_empty_count"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_base_content_empty_count",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_base_content_invalid_letters() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_base_content_invalid_letters"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_base_content_invalid_letters",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_gc_panic_on_store_in_seq() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_gc_panic_on_store_in_seq"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_gc_panic_on_store_in_seq",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_iupac_no_segment() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_iupac_no_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_iupac_no_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_iupac_suffix_empty_search() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_iupac_suffix_empty_search"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_iupac_suffix_empty_search",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_iupac_suffix_min_length_too_high()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_iupac_suffix_min_length_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_iupac_suffix_min_length_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_iupac_suffix_min_length_too_low()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_iupac_suffix_min_length_too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_iupac_suffix_min_length_too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_iupac_suffix_too_many_mismatches()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_iupac_suffix_too_many_mismatches"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_iupac_suffix_too_many_mismatches",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_iupac_suffix_uneven_len() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_iupac_suffix_uneven_len"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_iupac_suffix_uneven_len",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_regex_x_from_name_followed_by_uppercase()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_regex/from_name_followed_by_uppercase"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_regex/from_name_followed_by_uppercase",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_regex_x_label_starts_with_name()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_regex/label_starts_with_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_regex/label_starts_with_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_extract_tag_from_i1_i2_no_i1_i2() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/extract_tag_from_i1_i2_no_i1_i2"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/extract_tag_from_i1_i2_no_i1_i2",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_filter_by_tag_numeric_rejection() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/filter_by_tag_numeric_rejection"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/filter_by_tag_numeric_rejection",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_filter_no_such_tag() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/filter_no_such_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/filter_no_such_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_numeric_filter_wrong_tag_type() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/numeric_filter_wrong_tag_type"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/numeric_filter_wrong_tag_type",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_numeric_filter_wrong_tag_type_location()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/numeric_filter_wrong_tag_type_location"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/numeric_filter_wrong_tag_type_location",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_store_tag_in_comment_x_insert_char_in_value()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/store_tag_in_comment/insert_char_in_value"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/store_tag_in_comment/insert_char_in_value",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_store_tag_in_comment_x_seperator_in_label()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/store_tag_in_comment/seperator_in_label"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/store_tag_in_comment/seperator_in_label",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_store_tag_in_comment_x_seperator_in_value()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/store_tag_in_comment/seperator_in_value"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/store_tag_in_comment/seperator_in_value",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_tag_name_x_tag_name_not_len() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/tag_name/tag_name_not_len"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/tag_name/tag_name_not_len",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_tag_name_x_tag_name_not_location() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/tag_name/tag_name_not_location"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/tag_name/tag_name_not_location",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_unused_extract_tag_x_one_unused_tag() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/unused_extract_tag/one_unused_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/unused_extract_tag/one_unused_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_unused_extract_tag_x_two_unused_tags() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/unused_extract_tag/two_unused_tags"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/unused_extract_tag/two_unused_tags",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_extraction_x_validate_regex_fail() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/extraction/validate_regex_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/extraction/validate_regex_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_filter_x_bynumerictagminormax() {
    println!("Test case is in: test_cases/single_step/error_handling/filter/ByNumericTagMinOrMax");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/filter/ByNumericTagMinOrMax",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_filter_x_other_file_by_seq_x_input_is_output() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/filter/other_file_by_seq/input_is_output"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/filter/other_file_by_seq/input_is_output",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_filter_x_other_file_by_seq_x_negative_fpr() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/filter/other_file_by_seq/negative_fpr"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/filter/other_file_by_seq/negative_fpr",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_filter_x_other_file_by_seq_x_no_seed() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/filter/other_file_by_seq/no_seed"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/filter/other_file_by_seq/no_seed",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_invalid_segment_names_x_all() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_config/invalid_segment_names/all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_config/invalid_segment_names/all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_invalid_segment_names_x_internal() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_config/invalid_segment_names/internal"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_config/invalid_segment_names/internal",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_no_segments() {
    println!("Test case is in: test_cases/single_step/error_handling/input_config/no_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/input_config/no_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_segment_name_x_duplicated_after_trim()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_config/segment_name/duplicated_after_trim"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_config/segment_name/duplicated_after_trim",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_segment_name_x_empty() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_config/segment_name/empty"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_config/segment_name/empty",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_segment_name_x_invalid_path() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_config/segment_name/invalid_path"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_config/segment_name/invalid_path",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_segment_name_x_invalid_path2() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_config/segment_name/invalid_path2"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_config/segment_name/invalid_path2",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_config_x_segment_name_x_whitespace_only() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_config/segment_name/whitespace_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_config/segment_name/whitespace_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_ermission_denied_input_file_interleaved()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/ermission_denied_input_file_interleaved"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/ermission_denied_input_file_interleaved",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_fake_fasta_missing() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/fake_fasta_missing"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/fake_fasta_missing",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_fake_fasta_too_high() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/fake_fasta_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/fake_fasta_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_fake_fasta_too_low() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/fake_fasta_too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/fake_fasta_too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_filter_missing_tag() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/filter_missing_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/filter_missing_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_index1_file_does_not_exist() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/index1_file_does_not_exist"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/index1_file_does_not_exist",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_index2_file_does_not_exist() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/index2_file_does_not_exist"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/index2_file_does_not_exist",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_input_file_is_output_file() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/input_file_is_output_file"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/input_file_is_output_file",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_missing_input_file() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/missing_input_file"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/missing_input_file",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_output_x_interleave_x_missing_target()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/output/interleave/missing_target"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/output/interleave/missing_target",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_paired_end_unqueal_read_count_x_read1_more_than_read2()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/paired_end_unqueal_read_count/read1_more_than_read2"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/paired_end_unqueal_read_count/read1_more_than_read2",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_paired_end_unqueal_read_count_x_read2_more_than_read1()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/paired_end_unqueal_read_count/read2_more_than_read1"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/paired_end_unqueal_read_count/read2_more_than_read1",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_permission_denied_input_file() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/permission_denied_input_file"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/permission_denied_input_file",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_permission_denied_output_dir() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/permission_denied_output_dir"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/permission_denied_output_dir",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_permission_denied_output_dir_direct() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/permission_denied_output_dir_direct"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/permission_denied_output_dir_direct",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_permission_denied_output_dir_direct_x_permission_denied_output_dir()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/permission_denied_output_dir_direct/permission_denied_output_dir"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/permission_denied_output_dir_direct/permission_denied_output_dir",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_permission_denied_output_file() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/permission_denied_output_file"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/permission_denied_output_file",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_permission_denied_read1() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/permission_denied_read1"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/permission_denied_read1",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_read1_empty_list() {
    println!("Test case is in: test_cases/single_step/error_handling/input_files/read1_empty_list");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/read1_empty_list",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_read1_len_neq_index1_len() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/read1_len_neq_index1_len"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/read1_len_neq_index1_len",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_read1_len_neq_index2_len() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/read1_len_neq_index2_len"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/read1_len_neq_index2_len",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_read1_len_neq_read2_len() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/read1_len_neq_read2_len"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/read1_len_neq_read2_len",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_read1_not_a_string() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/read1_not_a_string"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/read1_not_a_string",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_read2_file_does_not_exist() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/read2_file_does_not_exist"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/read2_file_does_not_exist",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_read2_not_a_string() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/read2_not_a_string"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/read2_not_a_string",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_repeated_filenames() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/repeated_filenames"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/repeated_filenames",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_repeated_filenames_index1() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/repeated_filenames_index1"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/repeated_filenames_index1",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_repeated_filenames_index2() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/repeated_filenames_index2"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/repeated_filenames_index2",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_repeated_filenames_one_key() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/repeated_filenames_one_key"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/repeated_filenames_one_key",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_swap_x_swap_missing_segment_a() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/swap/swap_missing_segment_a"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/swap/swap_missing_segment_a",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_input_files_x_swap_x_swap_missing_segment_b() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/input_files/swap/swap_missing_segment_b"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/input_files/swap/swap_missing_segment_b",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_io_x_stdin_multiple_files() {
    println!("Test case is in: test_cases/single_step/error_handling/io/stdin_multiple_files");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/io/stdin_multiple_files"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_kmer_x_from_stdin() {
    println!("Test case is in: test_cases/single_step/error_handling/kmer/from_stdin");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/kmer/from_stdin"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_kmer_x_kmer_zero() {
    println!("Test case is in: test_cases/single_step/error_handling/kmer/kmer_zero");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/kmer/kmer_zero"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_kmer_x_no_file() {
    println!("Test case is in: test_cases/single_step/error_handling/kmer/no_file");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/kmer/no_file"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_kmer_x_wrong_type_for_files() {
    println!("Test case is in: test_cases/single_step/error_handling/kmer/wrong_type_for_files");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/kmer/wrong_type_for_files"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_kmer_x_wrong_type_for_files_nested() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/kmer/wrong_type_for_files_nested"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/kmer/wrong_type_for_files_nested",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_empty_list_of_files() {
    println!("Test case is in: test_cases/single_step/error_handling/misc/empty_list_of_files");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/misc/empty_list_of_files"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_empty_name_input() {
    println!("Test case is in: test_cases/single_step/error_handling/misc/empty_name_input");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/misc/empty_name_input"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_mixed_input_formats() {
    println!("Test case is in: test_cases/single_step/error_handling/misc/mixed_input_formats");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/misc/mixed_input_formats"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_postfix_len_mismatch() {
    println!("Test case is in: test_cases/single_step/error_handling/misc/postfix_len_mismatch");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/misc/postfix_len_mismatch"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_prefix_len_mismatch() {
    println!("Test case is in: test_cases/single_step/error_handling/misc/prefix_len_mismatch");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/misc/prefix_len_mismatch"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_read_with_comment_in_line_3() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/misc/read_with_comment_in_line_3"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/misc/read_with_comment_in_line_3",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_report_names_distinct() {
    println!("Test case is in: test_cases/single_step/error_handling/misc/report_names_distinct");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/misc/report_names_distinct"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_u8_from_char_number_to_large() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/misc/u8_from_char_number_to_large"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/misc/u8_from_char_number_to_large",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_misc_x_u8_from_char_too_many_chars() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/misc/u8_from_char_too_many_chars"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/misc/u8_from_char_too_many_chars",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_barcode_outputs_not_named_no_barcode()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/barcode_outputs_not_named_no_barcode"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/barcode_outputs_not_named_no_barcode",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_compression_level_on_uncompressed() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/compression_level_on_uncompressed"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/compression_level_on_uncompressed",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_duplicated_segment() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/duplicated_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/duplicated_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_empty_output() {
    println!("Test case is in: test_cases/single_step/error_handling/output_config/empty_output");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/output_config/empty_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_extract_regex_x_name_invalid_segment()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/extract_regex/name_invalid_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/extract_regex/name_invalid_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_extract_regex_x_name_no_segment_specified()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/extract_regex/name_no_segment_specified"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/extract_regex/name_no_segment_specified",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_extract_tag_i1_i2_but_not_output() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/extract_tag_i1_i2_but_not_output"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/extract_tag_i1_i2_but_not_output",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_input_interleaved_multiple_segment_files()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/input_interleaved_multiple_segment_files"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/input_interleaved_multiple_segment_files",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_invalid_compression_levels_x_output_gzip_level_too_high()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/invalid_compression_levels/output_gzip_level_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/invalid_compression_levels/output_gzip_level_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_invalid_compression_levels_x_output_zstd_level_too_high()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/invalid_compression_levels/output_zstd_level_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/invalid_compression_levels/output_zstd_level_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_invalid_compression_levels_x_output_zstd_level_zero()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/invalid_compression_levels/output_zstd_level_zero"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/invalid_compression_levels/output_zstd_level_zero",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_no_output_no_reports_x_empty_output()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/no_output_no_reports/empty_output"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/no_output_no_reports/empty_output",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_no_output_no_reports_x_format_raw() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/no_output_no_reports/format_raw"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/no_output_no_reports/format_raw",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_output_x_chunked_fifo() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/output/chunked_fifo"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/output/chunked_fifo",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_output_x_chunked_stdout() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/output/chunked_stdout"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/output/chunked_stdout",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_output_x_interleave_x_duplicated_target()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/output/interleave/duplicated_target"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/output/interleave/duplicated_target",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_output_x_interleave_x_just_one_target()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/output/interleave/just_one_target"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/output/interleave/just_one_target",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_report_without_output_flags() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/report_without_output_flags"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/report_without_output_flags",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_segment_defaults_multiple_segments_fails()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/segment_defaults_multiple_segments_fails"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/segment_defaults_multiple_segments_fails",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_segment_duplicated_interleave() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/segment_duplicated_interleave"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/segment_duplicated_interleave",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_stdin_multiple_segments() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/stdin_multiple_segments"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/stdin_multiple_segments",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_stdout_conflict() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/stdout_conflict"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/stdout_conflict",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_undefined_segments() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/undefined_segments"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/undefined_segments",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_undefined_segments_repeated() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/undefined_segments_repeated"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/undefined_segments_repeated",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_unwritable_output_dir() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/unwritable_output_dir"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/unwritable_output_dir",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_output_config_x_validate_name_needs_multiple_segments()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/output_config/validate_name_needs_multiple_segments"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/output_config/validate_name_needs_multiple_segments",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_paired_end_x_paired_end_unqueal_read_count_x_read3_more_than_1_2()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/paired_end/paired_end_unqueal_read_count/read3_more_than_1_2"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/paired_end/paired_end_unqueal_read_count/read3_more_than_1_2",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_quality_scores_x_convert_phred_raises() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/quality_scores/convert_phred_raises"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/quality_scores/convert_phred_raises",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_reports_x_report_but_no_report_step_html() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/reports/report_but_no_report_step_html"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/reports/report_but_no_report_step_html",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_reports_x_report_but_no_report_step_json() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/reports/report_but_no_report_step_json"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/reports/report_but_no_report_step_json",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_all_in_place_of_noall() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/all_in_place_of_noall"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/all_in_place_of_noall",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_empty_name_source_no_all() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/empty_name_source_no_all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/empty_name_source_no_all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_empty_name_source_no_all_single_segment()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/empty_name_source_no_all_single_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/empty_name_source_no_all_single_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_empty_tag_source_no_all() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/empty_tag_source_no_all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/empty_tag_source_no_all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_missing_segment_index_or_all() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/missing_segment_index_or_all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/missing_segment_index_or_all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_missing_segment_or_name_index() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/missing_segment_or_name_index"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/missing_segment_or_name_index",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_no_such_tag_source_no_all() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/no_such_tag_source_no_all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/no_such_tag_source_no_all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_source_all_name_empty() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/source_all_name_empty"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/source_all_name_empty",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_source_all_name_segment_not_found() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/source_all_name_segment_not_found"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/source_all_name_segment_not_found",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_source_all_segment_not_foudn() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/source_all_segment_not_foudn"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/source_all_segment_not_foudn",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_source_all_tag_empty() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/source_all_tag_empty"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/source_all_tag_empty",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_source_all_tag_not_found() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/source_all_tag_not_found"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/source_all_tag_not_found",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_source_no_all_name_segmentnot_found() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/source_no_all_name_segmentnot_found"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/source_no_all_name_segmentnot_found",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_segments_x_source_no_all_segment_not_found() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/segments/source_no_all_segment_not_found"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/segments/source_no_all_segment_not_found",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_showing_docs_x_from_action() {
    println!("Test case is in: test_cases/single_step/error_handling/showing_docs/from_action");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/showing_docs/from_action"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_showing_docs_x_from_braces() {
    println!("Test case is in: test_cases/single_step/error_handling/showing_docs/from_braces");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/showing_docs/from_braces"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_swap_x_swap_auto_detect_too_few_segments() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/swap/swap_auto_detect_too_few_segments"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/swap/swap_auto_detect_too_few_segments",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_swap_x_swap_auto_detect_too_many_segments() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/swap/swap_auto_detect_too_many_segments"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/swap/swap_auto_detect_too_many_segments",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_swap_x_swap_broken_a() {
    println!("Test case is in: test_cases/single_step/error_handling/swap/swap_broken_a");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/swap/swap_broken_a"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_swap_x_swap_partial_specification_a_only() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/swap/swap_partial_specification_a_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/swap/swap_partial_specification_a_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_swap_x_swap_partial_specification_b_only() {
    println!(
        "Test case is in: test_cases/single_step/error_handling/swap/swap_partial_specification_b_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/swap/swap_partial_specification_b_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_swap_x_swap_same_segment() {
    println!("Test case is in: test_cases/single_step/error_handling/swap/swap_same_segment");
    run_test(
        std::path::Path::new("../test_cases/single_step/error_handling/swap/swap_same_segment"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_validation_errors_x_merge_reads_min_overlap_too_low()
{
    println!(
        "Test case is in: test_cases/single_step/error_handling/validation_errors/merge_reads_min_overlap_too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/validation_errors/merge_reads_min_overlap_too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_validation_errors_x_merge_reads_mismatch_rate_too_high()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/validation_errors/merge_reads_mismatch_rate_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/validation_errors/merge_reads_mismatch_rate_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_error_handling_x_validation_errors_x_merge_reads_mismatch_rate_too_low()
 {
    println!(
        "Test case is in: test_cases/single_step/error_handling/validation_errors/merge_reads_mismatch_rate_too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/error_handling/validation_errors/merge_reads_mismatch_rate_too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_err_empty_expr() {
    println!("Test case is in: test_cases/single_step/eval/eval_expr/err_empty_expr");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/eval_expr/err_empty_expr"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_eval_expression_basic() {
    println!("Test case is in: test_cases/single_step/eval/eval_expr/eval_expression_basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/eval_expr/eval_expression_basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_eval_expression_bool() {
    println!("Test case is in: test_cases/single_step/eval/eval_expr/eval_expression_bool");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/eval_expr/eval_expression_bool"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_eval_expression_bool_as_float() {
    println!(
        "Test case is in: test_cases/single_step/eval/eval_expr/eval_expression_bool_as_float"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/eval/eval_expr/eval_expression_bool_as_float",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_eval_expression_complex() {
    println!("Test case is in: test_cases/single_step/eval/eval_expr/eval_expression_complex");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/eval_expr/eval_expression_complex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_eval_expression_force_type_bool() {
    println!(
        "Test case is in: test_cases/single_step/eval/eval_expr/eval_expression_force_type_bool"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/eval/eval_expr/eval_expression_force_type_bool",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_eval_expression_force_type_numeric() {
    println!(
        "Test case is in: test_cases/single_step/eval/eval_expr/eval_expression_force_type_numeric"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/eval/eval_expr/eval_expression_force_type_numeric",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_missing_result_type() {
    println!("Test case is in: test_cases/single_step/eval/eval_expr/missing_result_type");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/eval_expr/missing_result_type"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_eval_expr_x_syntax_error() {
    println!("Test case is in: test_cases/single_step/eval/eval_expr/syntax_error");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/eval_expr/syntax_error"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_location() {
    println!("Test case is in: test_cases/single_step/eval/location");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_location_len() {
    println!("Test case is in: test_cases/single_step/eval/location_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/location_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_location_len_with_replacement() {
    println!("Test case is in: test_cases/single_step/eval/location_len_with_replacement");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/location_len_with_replacement"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_segment_len() {
    println!("Test case is in: test_cases/single_step/eval/segment_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/segment_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_str() {
    println!("Test case is in: test_cases/single_step/eval/str");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/str"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_str_len() {
    println!("Test case is in: test_cases/single_step/eval/str_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/str_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_threshold() {
    println!("Test case is in: test_cases/single_step/eval/threshold");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/threshold"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_eval_x_varname_all() {
    println!("Test case is in: test_cases/single_step/eval/varname_all");
    run_test(
        std::path::Path::new("../test_cases/single_step/eval/varname_all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_polytail_x_error_fastp_mode_non_g_base() {
    println!(
        "Test case is in: test_cases/single_step/extraction/PolyTail/error_fastp_mode_non_g_base"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/PolyTail/error_fastp_mode_non_g_base",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_polytail_x_error_max_mismatch_rate_too_high() {
    println!(
        "Test case is in: test_cases/single_step/extraction/PolyTail/error_max_mismatch_rate_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/PolyTail/error_max_mismatch_rate_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_polytail_x_error_max_mismatch_rate_too_low() {
    println!(
        "Test case is in: test_cases/single_step/extraction/PolyTail/error_max_mismatch_rate_too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/PolyTail/error_max_mismatch_rate_too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_polytail_x_error_min_length() {
    println!("Test case is in: test_cases/single_step/extraction/PolyTail/error_min_length");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/PolyTail/error_min_length"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_polytail_x_fastp_mode() {
    println!("Test case is in: test_cases/single_step/extraction/PolyTail/fastp_mode");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/PolyTail/fastp_mode"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_cut_end_inside_tag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/cut_end_inside_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/cut_end_inside_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_cut_start_inside_tag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/cut_start_inside_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/cut_start_inside_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_extract_trim_end_false() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_end_false"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_end_false",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_extract_trim_end_true() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_end_true"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_end_true",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_extract_trim_start_false()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_start_false"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_start_false",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_extract_trim_start_true()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_start_true"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/extract_trim_start_true",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_max_len_after_tag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/max_len_after_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/max_len_after_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_max_len_before_tag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/max_len_before_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/max_len_before_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_max_len_inside_tag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/max_len_inside_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/max_len_inside_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_prefix() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/prefix"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/prefix",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_rev_complement() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/rev_complement"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/rev_complement",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_rev_complement_all() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/rev_complement_all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/rev_complement_all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_swap() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/swap"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/swap",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_swap_conditional() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/swap_conditional"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/swap_conditional",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_swap_conditional_third() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/swap_conditional_third"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/swap_conditional_third",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_swap_conditional_third_other_tag()
 {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/swap_conditional_third_other_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/swap_conditional_third_other_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_swap_third() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/swap_third"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/swap_third",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_edits_altering_tag_locations_x_trim_quality_start() {
    println!(
        "Test case is in: test_cases/single_step/extraction/edits_altering_tag_locations/trim_quality_start"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/edits_altering_tag_locations/trim_quality_start",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_anchor_x_hamming() {
    println!("Test case is in: test_cases/single_step/extraction/extract_anchor/hamming");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_anchor/hamming"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_anchor_x_leftmost_verification() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_anchor/leftmost_verification"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_anchor/leftmost_verification",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_anchor_x_right_most_verification() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_anchor/right_most_verification"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_anchor/right_most_verification",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_anchor_x_simple() {
    println!("Test case is in: test_cases/single_step/extraction/extract_anchor/simple");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_anchor/simple"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_anchor_x_too_far() {
    println!("Test case is in: test_cases/single_step/extraction/extract_anchor/too_far");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_anchor/too_far"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_anchor_x_too_far_left() {
    println!("Test case is in: test_cases/single_step/extraction/extract_anchor/too_far_left");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_anchor/too_far_left"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_base_content_simple_test() {
    println!("Test case is in: test_cases/single_step/extraction/extract_base_content_simple_test");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_base_content_simple_test",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_filter_x_keep() {
    println!("Test case is in: test_cases/single_step/extraction/extract_filter/keep");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_filter/keep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_filter_x_remove() {
    println!("Test case is in: test_cases/single_step/extraction/extract_filter/remove");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_filter/remove"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_gc_x_after_trim() {
    println!("Test case is in: test_cases/single_step/extraction/extract_gc/after_trim");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_gc/after_trim"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_gc_x_segment_all_full_data() {
    println!("Test case is in: test_cases/single_step/extraction/extract_gc/segment_all_full_data");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_gc/segment_all_full_data",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_gc_x_segment_all_read1_only() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_gc/segment_all_read1_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_gc/segment_all_read1_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_gc_x_simple_test() {
    println!("Test case is in: test_cases/single_step/extraction/extract_gc/simple_test");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_gc/simple_test"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_growing() {
    println!("Test case is in: test_cases/single_step/extraction/extract_growing");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_growing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_growing_from_nothing() {
    println!("Test case is in: test_cases/single_step/extraction/extract_growing_from_nothing");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_growing_from_nothing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_highlight_x_basic() {
    println!("Test case is in: test_cases/single_step/extraction/extract_highlight/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_highlight/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_highlight_x_regex() {
    println!("Test case is in: test_cases/single_step/extraction/extract_highlight/regex");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_highlight/regex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_ambigous() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/ambigous");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_iupac/ambigous"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_anchor_variations() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/anchor_variations");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/anchor_variations",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_anywhere_and_max_anchor_dist() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/anywhere_and_max_anchor_dist"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/anywhere_and_max_anchor_dist",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_error_suffix_using_iupac_non_iupac() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/error_suffix_using_iupac_non_iupac"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/error_suffix_using_iupac_non_iupac",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_multiple() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/multiple");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_iupac/multiple"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_multiple_left_most() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/multiple_left_most");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/multiple_left_most",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_multiple_patterns_x_multiple_patterns_different_positions()
 {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/multiple_patterns/multiple_patterns_different_positions"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/multiple_patterns/multiple_patterns_different_positions",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_multiple_patterns_x_nested_patterns() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/multiple_patterns/nested_patterns"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/multiple_patterns/nested_patterns",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_multiple_patterns_x_overlapping_patterns()
 {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/multiple_patterns/overlapping_patterns"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/multiple_patterns/overlapping_patterns",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_multiple_right_most() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/multiple_right_most"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/multiple_right_most",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_non_dna_query() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/non_dna_query");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_iupac/non_dna_query"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_suffix() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/suffix");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_iupac/suffix"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_suffix_and_cut() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/suffix_and_cut");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_iupac/suffix_and_cut"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_suffix_multiple_using_iupac() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/suffix_multiple_using_iupac"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/suffix_multiple_using_iupac",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_suffix_using_iupac() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/suffix_using_iupac");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/suffix_using_iupac",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_with_indel() {
    println!("Test case is in: test_cases/single_step/extraction/extract_iupac/with_indel");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_iupac/with_indel"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_with_indel_anchor_left() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/with_indel_anchor_left"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/with_indel_anchor_left",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_with_indel_anchor_right() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/with_indel_anchor_right"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/with_indel_anchor_right",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_with_indel_empty_reads() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/with_indel_empty_reads"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/with_indel_empty_reads",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_iupac_x_with_indel_empty_search() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_iupac/with_indel_empty_search"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_iupac/with_indel_empty_search",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_label_must_not_be_empty() {
    println!("Test case is in: test_cases/single_step/extraction/extract_label_must_not_be_empty");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_label_must_not_be_empty",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_length_x_after_trim() {
    println!("Test case is in: test_cases/single_step/extraction/extract_length/after_trim");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_length/after_trim"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_length_x_segment_all_full_data() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_length/segment_all_full_data"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_length/segment_all_full_data",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_length_x_segment_all_read1_only() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_length/segment_all_read1_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_length/segment_all_read1_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_length_x_simple_test() {
    println!("Test case is in: test_cases/single_step/extraction/extract_length/simple_test");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_length/simple_test"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_any_base() {
    println!("Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/any_base");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/any_base",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_any_base_all_monomer() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/any_base_all_monomer"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/any_base_all_monomer",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_any_base_mismatch() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/any_base_mismatch"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/any_base_mismatch",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_basic() {
    println!("Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_longest_poly_x/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_basic_no_mismatches() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/basic_no_mismatches"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/basic_no_mismatches",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_error_min_length() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/error_min_length"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/error_min_length",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_error_mismatch_rate_x_too_high()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/error_mismatch_rate/too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/error_mismatch_rate/too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_error_mismatch_rate_x_too_low()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/error_mismatch_rate/too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/error_mismatch_rate/too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_longest_poly_x_x_min_length_below_read_length() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_longest_poly_x/min_length_below_read_length"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_longest_poly_x/min_length_below_read_length",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_n_x_count_after_trim() {
    println!("Test case is in: test_cases/single_step/extraction/extract_n/count_after_trim");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_n/count_after_trim"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_n_x_count_simple_test() {
    println!("Test case is in: test_cases/single_step/extraction/extract_n/count_simple_test");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_n/count_simple_test"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_n_x_count_target_all_full_data() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_n/count_target_all_full_data"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_n/count_target_all_full_data",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_n_x_count_target_all_read1_only() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_n/count_target_all_read1_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_n/count_target_all_read1_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_basic() {
    println!("Test case is in: test_cases/single_step/extraction/extract_regex/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_regex/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_extract_regex_from_name() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regex/extract_regex_from_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regex/extract_regex_from_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_extract_regex_from_name_multi_segment() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regex/extract_regex_from_name_multi_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regex/extract_regex_from_name_multi_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_extract_regex_from_name_no_replacement()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regex/extract_regex_from_name_no_replacement"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regex/extract_regex_from_name_no_replacement",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_extract_regex_no_replacement() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regex/extract_regex_no_replacement"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regex/extract_regex_no_replacement",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_from_tag() {
    println!("Test case is in: test_cases/single_step/extraction/extract_regex/from_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_regex/from_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_regex_len_table() {
    println!("Test case is in: test_cases/single_step/extraction/extract_regex/regex_len_table");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_regex/regex_len_table"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_tag_replacement() {
    println!("Test case is in: test_cases/single_step/extraction/extract_regex/tag_replacement");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_regex/tag_replacement"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_tag_replacement_no_suchtag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regex/tag_replacement_no_suchtag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regex/tag_replacement_no_suchtag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_x_tag_replacement_string_source() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regex/tag_replacement_string_source"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regex/tag_replacement_string_source",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_underscores() {
    println!("Test case is in: test_cases/single_step/extraction/extract_regex_underscores");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_regex_underscores"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regex_underscores_x_ok_works() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regex_underscores/ok_works"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regex_underscores/ok_works",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_and_replace_multiple() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_region/and_replace_multiple"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_region/and_replace_multiple",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_before_read() {
    println!("Test case is in: test_cases/single_step/extraction/extract_region/before_read");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_region/before_read"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_beyond_read_len() {
    println!("Test case is in: test_cases/single_step/extraction/extract_region/beyond_read_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_region/beyond_read_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_beyond_read_len_and_trim() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_region/beyond_read_len_and_trim"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_region/beyond_read_len_and_trim",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_empty_region_definition() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_region/empty_region_definition"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_region/empty_region_definition",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_extract_from_tag_with_cleared_location()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_region/extract_from_tag_with_cleared_location"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_region/extract_from_tag_with_cleared_location",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_from_location_tag() {
    println!("Test case is in: test_cases/single_step/extraction/extract_region/from_location_tag");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_region/from_location_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_from_string_tag() {
    println!("Test case is in: test_cases/single_step/extraction/extract_region/from_string_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_region/from_string_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_from_string_tag_outside_of_tag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_region/from_string_tag_outside_of_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_region/from_string_tag_outside_of_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_read_too_short() {
    println!("Test case is in: test_cases/single_step/extraction/extract_region/read_too_short");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_region/read_too_short"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_region_x_trim_at_tag_multi_location_region() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_region/trim_at_tag_multi_location_region"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_region/trim_at_tag_multi_location_region",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regions_of_low_quality_x_min_length_filter() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_filter"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_filter",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regions_of_low_quality_x_min_length_missing() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_missing"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_missing",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regions_of_low_quality_x_min_length_negative() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_negative"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_negative",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_regions_of_low_quality_x_min_length_zero() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_zero"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_regions_of_low_quality/min_length_zero",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_shrinking() {
    println!("Test case is in: test_cases/single_step/extraction/extract_shrinking");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_shrinking"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_tag() {
    println!("Test case is in: test_cases/single_step/extraction/extract_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_tag_duplicate_name_panics() {
    println!(
        "Test case is in: test_cases/single_step/extraction/extract_tag_duplicate_name_panics"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_tag_duplicate_name_panics",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_tag_i1_i2() {
    println!("Test case is in: test_cases/single_step/extraction/extract_tag_i1_i2");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_tag_i1_i2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_tag_r2() {
    println!("Test case is in: test_cases/single_step/extraction/extract_tag_r2");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_tag_r2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_tag_reserved_name_panics() {
    println!("Test case is in: test_cases/single_step/extraction/extract_tag_reserved_name_panics");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/extract_tag_reserved_name_panics",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_trim_x_end_false() {
    println!("Test case is in: test_cases/single_step/extraction/extract_trim/end_false");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_trim/end_false"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_trim_x_end_true() {
    println!("Test case is in: test_cases/single_step/extraction/extract_trim/end_true");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_trim/end_true"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_trim_x_start_false() {
    println!("Test case is in: test_cases/single_step/extraction/extract_trim/start_false");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_trim/start_false"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_extract_trim_x_start_true() {
    println!("Test case is in: test_cases/single_step/extraction/extract_trim/start_true");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/extract_trim/start_true"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_overlapping_regions_trim_conflict() {
    println!(
        "Test case is in: test_cases/single_step/extraction/overlapping_regions_trim_conflict"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/overlapping_regions_trim_conflict",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_remove_nonexistant_tag() {
    println!("Test case is in: test_cases/single_step/extraction/remove_nonexistant_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/remove_nonexistant_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_comment_x_insert_char_default() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_comment/insert_char_default"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_comment/insert_char_default",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_comment_x_multiple_tags() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_comment/multiple_tags"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_comment/multiple_tags",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_comment_x_store_in_not_output_segment() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_comment/store_in_not_output_segment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_comment/store_in_not_output_segment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_basic() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tag_in_fastq/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_basic_many_blocks_expose_order() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/basic_many_blocks_expose_order"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/basic_many_blocks_expose_order",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_demultiplex_output_unmatched_x_false()
 {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/demultiplex_output_unmatched=false"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/demultiplex_output_unmatched=false",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_error_bam_output() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/error_bam_output"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/error_bam_output",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_error_no_output_def() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/error_no_output_def"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/error_no_output_def",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_fasta() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/fasta");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tag_in_fastq/fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_gzipped() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/gzipped");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tag_in_fastq/gzipped"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_head_after() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/head_after");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tag_in_fastq/head_after"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_no_location() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/no_location");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tag_in_fastq/no_location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_repeat_between_inlabel_and_comment()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/repeat_between_inlabel_and_comment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/repeat_between_inlabel_and_comment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_with_comments() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/with_comments");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/with_comments",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_with_comments_conflict_value_insert()
 {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/with_comments_conflict_value_insert"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/with_comments_conflict_value_insert",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_with_comments_conflict_value_sep() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/with_comments_conflict_value_sep"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/with_comments_conflict_value_sep",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_with_location() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/with_location");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/with_location",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_fastq_x_with_location_multi() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_fastq/with_location_multi"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_fastq/with_location_multi",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_basic() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tag_in_sequence/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_empty_value_insert_noop() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/empty_value_insert_noop"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/empty_value_insert_noop",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_end_anchor() {
    println!("Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/end_anchor");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/end_anchor",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_insert_after_read_end() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/insert_after_read_end"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/insert_after_read_end",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_insert_lost_location_complain() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/insert_lost_location_complain"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/insert_lost_location_complain",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_replace_disjoint_length_error() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/replace_disjoint_length_error"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/replace_disjoint_length_error",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_replace_zero_width_lost_after_cut()
 {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/replace_zero_width_lost_after_cut"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/replace_zero_width_lost_after_cut",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_replace_zero_width_start_noop() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/replace_zero_width_start_noop"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/replace_zero_width_start_noop",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_replacement_basic() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/replacement_basic"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/replacement_basic",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_replacement_with_location_shift()
{
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/replacement_with_location_shift"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/replacement_with_location_shift",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_string_value() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/string_value"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/string_value",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_with_unrelated_tags() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/with_unrelated_tags"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/with_unrelated_tags",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tag_in_sequence_x_with_unrelated_tags_after() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tag_in_sequence/with_unrelated_tags_after"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tag_in_sequence/with_unrelated_tags_after",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_empty_inlabels() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tags_in_table/empty_inlabels"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/empty_inlabels",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_head_after() {
    println!("Test case is in: test_cases/single_step/extraction/store_tags_in_table/head_after");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tags_in_table/head_after"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv() {
    println!("Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tags_in_table/in_tsv"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv_demultiplex() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv_demultiplex"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/in_tsv_demultiplex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv_gz() {
    println!("Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv_gz");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/store_tags_in_table/in_tsv_gz"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv_no_read_name() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv_no_read_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/in_tsv_no_read_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv_string_tag() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv_string_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/in_tsv_string_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv_validate_compression() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv_validate_compression"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/in_tsv_validate_compression",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv_virtual_tags() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv_virtual_tags"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/in_tsv_virtual_tags",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_in_tsv_virtual_tags_missing() {
    println!(
        "Test case is in: test_cases/single_step/extraction/store_tags_in_table/in_tsv_virtual_tags_missing"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/in_tsv_virtual_tags_missing",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_store_tags_in_table_x_missing_tag() {
    println!("Test case is in: test_cases/single_step/extraction/store_tags_in_table/missing_tag");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/store_tags_in_table/missing_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_umi_extract() {
    println!("Test case is in: test_cases/single_step/extraction/umi_extract");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/umi_extract"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_umi_extract_store_in_all_read_names() {
    println!(
        "Test case is in: test_cases/single_step/extraction/umi_extract_store_in_all_read_names"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/umi_extract_store_in_all_read_names",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_umi_extract_with_existing_comment() {
    println!(
        "Test case is in: test_cases/single_step/extraction/umi_extract_with_existing_comment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/umi_extract_with_existing_comment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_use_forget_all_tags() {
    println!("Test case is in: test_cases/single_step/extraction/use_forget_all_tags");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/use_forget_all_tags"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_use_removed_tag() {
    println!("Test case is in: test_cases/single_step/extraction/use_removed_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/use_removed_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_use_tag_after_forget() {
    println!("Test case is in: test_cases/single_step/extraction/use_tag_after_forget");
    run_test(
        std::path::Path::new("../test_cases/single_step/extraction/use_tag_after_forget"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_extraction_x_with_demultiplex_x_store_tag_in_fastq_x_demultiplex() {
    println!(
        "Test case is in: test_cases/single_step/extraction/with_demultiplex/store_tag_in_fastq/demultiplex"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/extraction/with_demultiplex/store_tag_in_fastq/demultiplex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_bam_but_neither_mapped_nor_unmapped() {
    println!(
        "Test case is in: test_cases/single_step/fileformats/bam_but_neither_mapped_nor_unmapped"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/fileformats/bam_but_neither_mapped_nor_unmapped",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_bam_to_fastq() {
    println!("Test case is in: test_cases/single_step/fileformats/bam_to_fastq");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/bam_to_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_bam_with_index_to_fastq_x_both() {
    println!("Test case is in: test_cases/single_step/fileformats/bam_with_index_to_fastq/both");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/bam_with_index_to_fastq/both"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_bam_with_index_to_fastq_x_mapped() {
    println!("Test case is in: test_cases/single_step/fileformats/bam_with_index_to_fastq/mapped");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/fileformats/bam_with_index_to_fastq/mapped",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_bam_with_index_to_fastq_x_unmapped() {
    println!(
        "Test case is in: test_cases/single_step/fileformats/bam_with_index_to_fastq/unmapped"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/fileformats/bam_with_index_to_fastq/unmapped",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_bam_without_quals() {
    println!("Test case is in: test_cases/single_step/fileformats/bam_without_quals");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/bam_without_quals"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_empty_bam() {
    println!("Test case is in: test_cases/single_step/fileformats/empty_bam");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/empty_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_fasta_gz_to_fastq() {
    println!("Test case is in: test_cases/single_step/fileformats/fasta_gz_to_fastq");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/fasta_gz_to_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_fasta_long_line_to_fastq() {
    println!("Test case is in: test_cases/single_step/fileformats/fasta_long_line_to_fastq");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/fasta_long_line_to_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_fasta_to_fastq() {
    println!("Test case is in: test_cases/single_step/fileformats/fasta_to_fastq");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/fasta_to_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_fasta_with_desc_to_fastq() {
    println!("Test case is in: test_cases/single_step/fileformats/fasta_with_desc_to_fastq");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/fasta_with_desc_to_fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_fasta_with_empty_desc_to_fastq() {
    println!("Test case is in: test_cases/single_step/fileformats/fasta_with_empty_desc_to_fastq");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/fileformats/fasta_with_empty_desc_to_fastq",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_fastq_to_bam() {
    println!("Test case is in: test_cases/single_step/fileformats/fastq_to_bam");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/fastq_to_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_fastq_to_fasta() {
    println!("Test case is in: test_cases/single_step/fileformats/fastq_to_fasta");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/fastq_to_fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_stdout_x_fasta() {
    println!("Test case is in: test_cases/single_step/fileformats/stdout/fasta");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/stdout/fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_stdout_x_fasta_compressed() {
    println!("Test case is in: test_cases/single_step/fileformats/stdout/fasta_compressed");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/stdout/fasta_compressed"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fileformats_x_stdout_x_fastq() {
    println!("Test case is in: test_cases/single_step/fileformats/stdout/fastq");
    run_test(
        std::path::Path::new("../test_cases/single_step/fileformats/stdout/fastq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_basic() {
    println!("Test case is in: test_cases/single_step/fill_missing/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/fill_missing/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_location_location() {
    println!("Test case is in: test_cases/single_step/fill_missing/location_location");
    run_test(
        std::path::Path::new("../test_cases/single_step/fill_missing/location_location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_location_location_different_sources_x_fail() {
    println!(
        "Test case is in: test_cases/single_step/fill_missing/location_location_different_sources/fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/fill_missing/location_location_different_sources/fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_location_location_different_sources_x_ok() {
    println!(
        "Test case is in: test_cases/single_step/fill_missing/location_location_different_sources/ok"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/fill_missing/location_location_different_sources/ok",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_mixed_location_string() {
    println!("Test case is in: test_cases/single_step/fill_missing/mixed_location_string");
    run_test(
        std::path::Path::new("../test_cases/single_step/fill_missing/mixed_location_string"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_mixed_string_location() {
    println!("Test case is in: test_cases/single_step/fill_missing/mixed_string_location");
    run_test(
        std::path::Path::new("../test_cases/single_step/fill_missing/mixed_string_location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_number_number() {
    println!("Test case is in: test_cases/single_step/fill_missing/number_number");
    run_test(
        std::path::Path::new("../test_cases/single_step/fill_missing/number_number"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_fill_missing_x_string_string() {
    println!("Test case is in: test_cases/single_step/fill_missing/string_string");
    run_test(
        std::path::Path::new("../test_cases/single_step/fill_missing/string_string"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_len_threshold_too_low() {
    println!("Test case is in: test_cases/single_step/filter/len_threshold_too_low");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/len_threshold_too_low"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_x_all_segments() {
    println!("Test case is in: test_cases/single_step/filter/low_complexity/all_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/low_complexity/all_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_x_basic() {
    println!("Test case is in: test_cases/single_step/filter/low_complexity/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/low_complexity/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_x_empty_read() {
    println!("Test case is in: test_cases/single_step/filter/low_complexity/empty_read");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/low_complexity/empty_read"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_x_empty_read_all() {
    println!("Test case is in: test_cases/single_step/filter/low_complexity/empty_read_all");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/low_complexity/empty_read_all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_max_below_min() {
    println!(
        "Test case is in: test_cases/single_step/filter/low_complexity_range_tests/max_below_min"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/max_below_min",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_max_is_ok() {
    println!("Test case is in: test_cases/single_step/filter/low_complexity_range_tests/max_is_ok");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/max_is_ok",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_max_too_high() {
    println!(
        "Test case is in: test_cases/single_step/filter/low_complexity_range_tests/max_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/max_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_max_too_low() {
    println!(
        "Test case is in: test_cases/single_step/filter/low_complexity_range_tests/max_too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/max_too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_min_above_max() {
    println!(
        "Test case is in: test_cases/single_step/filter/low_complexity_range_tests/min_above_max"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/min_above_max",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_min_is_ok() {
    println!("Test case is in: test_cases/single_step/filter/low_complexity_range_tests/min_is_ok");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/min_is_ok",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_min_too_high() {
    println!(
        "Test case is in: test_cases/single_step/filter/low_complexity_range_tests/min_too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/min_too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_low_complexity_range_tests_x_min_too_low() {
    println!(
        "Test case is in: test_cases/single_step/filter/low_complexity_range_tests/min_too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/low_complexity_range_tests/min_too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_max_len() {
    println!("Test case is in: test_cases/single_step/filter/max_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/max_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_max_len_implicit_tag_x_all_segments() {
    println!("Test case is in: test_cases/single_step/filter/max_len_implicit_tag/all_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/max_len_implicit_tag/all_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_max_len_implicit_tag_x_basic() {
    println!("Test case is in: test_cases/single_step/filter/max_len_implicit_tag/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/max_len_implicit_tag/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_min_len() {
    println!("Test case is in: test_cases/single_step/filter/min_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/min_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_distinct_separators() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/distinct_separators"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/distinct_separators",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_distinct_separators_conflict_with_store_in_comment()
 {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/distinct_separators_conflict_with_store_in_comment"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/distinct_separators_conflict_with_store_in_comment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_error_bzip2() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_name/error_bzip2");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_name/error_bzip2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_error_false_positive_rate_x_too_high() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/error_false_positive_rate/too_high"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/error_false_positive_rate/too_high",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_error_false_positive_rate_x_too_low() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/error_false_positive_rate/too_low"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/error_false_positive_rate/too_low",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_error_no_reads_in_other_file() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/error_no_reads_in_other_file"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/error_no_reads_in_other_file",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_keep() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_name/keep");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_name/keep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_keep_zstd() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_name/keep_zstd");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_name/keep_zstd"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_remove() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_name/remove");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_name/remove"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_remove_bam_approximate() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/remove_bam_approximate"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/remove_bam_approximate",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_remove_bam_approximate_broken_bai() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/remove_bam_approximate_broken_bai"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/remove_bam_approximate_broken_bai",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_remove_bam_approximate_no_bai() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/remove_bam_approximate_no_bai"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/remove_bam_approximate_no_bai",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_remove_bam_both() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_name/remove_bam_both");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_name/remove_bam_both"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_remove_bam_mapped_only() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/remove_bam_mapped_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/remove_bam_mapped_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_remove_bam_unmapped_only() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/remove_bam_unmapped_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/remove_bam_unmapped_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_name_x_wrong_compression_format() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_name/wrong_compression_format"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_name/wrong_compression_format",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_error_no_reads_in_other_file() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_seq/error_no_reads_in_other_file"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_seq/error_no_reads_in_other_file",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_keep() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_seq/keep");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_seq/keep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_remove() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_seq/remove");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_seq/remove"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_remove_bam_both() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_seq/remove_bam_both");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_seq/remove_bam_both"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_remove_bam_mapped_not_set() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_seq/remove_bam_mapped_not_set"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_seq/remove_bam_mapped_not_set",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_remove_bam_mapped_only() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_seq/remove_bam_mapped_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_seq/remove_bam_mapped_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_remove_bam_neither_errors() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_seq/remove_bam_neither_errors"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_seq/remove_bam_neither_errors",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_remove_bam_unmapped_not_set() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_seq/remove_bam_unmapped_not_set"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_seq/remove_bam_unmapped_not_set",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_seq_x_remove_bam_unmapped_only() {
    println!(
        "Test case is in: test_cases/single_step/filter/other_file_by_seq/remove_bam_unmapped_only"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/filter/other_file_by_seq/remove_bam_unmapped_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_tag() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_tag_multi_regions() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_tag_multi_regions");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_tag_multi_regions"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_other_file_by_tag_string_location() {
    println!("Test case is in: test_cases/single_step/filter/other_file_by_tag_string_location");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/other_file_by_tag_string_location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_qualified_bases_x_above() {
    println!("Test case is in: test_cases/single_step/filter/qualified_bases/above");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/qualified_bases/above"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_qualified_bases_x_above_all() {
    println!("Test case is in: test_cases/single_step/filter/qualified_bases/above_all");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/qualified_bases/above_all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_qualified_bases_x_above_or_equal() {
    println!("Test case is in: test_cases/single_step/filter/qualified_bases/above_or_equal");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/qualified_bases/above_or_equal"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_qualified_bases_x_below() {
    println!("Test case is in: test_cases/single_step/filter/qualified_bases/below");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/qualified_bases/below"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_qualified_bases_x_below_or_equal() {
    println!("Test case is in: test_cases/single_step/filter/qualified_bases/below_or_equal");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/qualified_bases/below_or_equal"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_too_many_n_x_all() {
    println!("Test case is in: test_cases/single_step/filter/too_many_n/all");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/too_many_n/all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_too_many_n_x_basic() {
    println!("Test case is in: test_cases/single_step/filter/too_many_n/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/too_many_n/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_filter_x_too_many_n_x_segments_vs_all() {
    println!("Test case is in: test_cases/single_step/filter/too_many_n/segments_vs_all");
    run_test(
        std::path::Path::new("../test_cases/single_step/filter/too_many_n/segments_vs_all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_forget_all_tags() {
    println!("Test case is in: test_cases/single_step/forget_all_tags");
    run_test(
        std::path::Path::new("../test_cases/single_step/forget_all_tags"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_forget_tags_x_set_two_forget_all() {
    println!("Test case is in: test_cases/single_step/forget_tags/set_two_forget_all");
    run_test(
        std::path::Path::new("../test_cases/single_step/forget_tags/set_two_forget_all"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_forget_tags_x_set_two_forget_one_access() {
    println!("Test case is in: test_cases/single_step/forget_tags/set_two_forget_one_access");
    run_test(
        std::path::Path::new("../test_cases/single_step/forget_tags/set_two_forget_one_access"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_forget_tags_x_set_two_forget_one_filter_one() {
    println!("Test case is in: test_cases/single_step/forget_tags/set_two_forget_one_filter_one");
    run_test(
        std::path::Path::new("../test_cases/single_step/forget_tags/set_two_forget_one_filter_one"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_0_distance_is_ok() {
    println!("Test case is in: test_cases/single_step/hamming/0_distance_is_ok");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/0_distance_is_ok"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_basic() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/assign_to_reference/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_basic_bam() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/basic_bam");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/assign_to_reference/basic_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_duplicate_reference() {
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/duplicate_reference"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/duplicate_reference",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_file_not_found() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/file_not_found");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/file_not_found",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_from_name_regex() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/from_name_regex");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/from_name_regex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_fuzzy() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/fuzzy");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/assign_to_reference/fuzzy"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_in_out_problem() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/in_out_problem");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/in_out_problem",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_no_match() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/no_match");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/assign_to_reference/no_match"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_reference_not_enough_hamming_distance()
 {
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/reference_not_enough_hamming_distance"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/reference_not_enough_hamming_distance",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_reference_not_enough_hamming_distance_but_same_prefix()
 {
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/reference_not_enough_hamming_distance_but_same_prefix"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/reference_not_enough_hamming_distance_but_same_prefix",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_runtime_not_distinct() {
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/runtime_not_distinct"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/runtime_not_distinct",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_runtime_not_distinct_but_equivalent()
{
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/runtime_not_distinct_but_equivalent"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/runtime_not_distinct_but_equivalent",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_runtime_not_distinct_but_equivalent_rev_order()
 {
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/runtime_not_distinct_but_equivalent_rev_order"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/runtime_not_distinct_but_equivalent_rev_order",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_unequal_len_references() {
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/unequal_len_references"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/unequal_len_references",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_unique() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/unique");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/assign_to_reference/unique"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_unreadable_input_file() {
    println!(
        "Test case is in: test_cases/single_step/hamming/assign_to_reference/unreadable_input_file"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/assign_to_reference/unreadable_input_file",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_assign_to_reference_x_wrong_nesting() {
    println!("Test case is in: test_cases/single_step/hamming/assign_to_reference/wrong_nesting");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/assign_to_reference/wrong_nesting"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_barcodes_from_both() {
    println!("Test case is in: test_cases/single_step/hamming/barcodes_from_both");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/barcodes_from_both"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_barcodes_not_found() {
    println!("Test case is in: test_cases/single_step/hamming/barcodes_not_found");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/barcodes_not_found"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_basic_correction() {
    println!("Test case is in: test_cases/single_step/hamming/basic_correction");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/basic_correction"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_basic_correction_empty() {
    println!("Test case is in: test_cases/single_step/hamming/basic_correction_empty");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/basic_correction_empty"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_basic_correction_keep() {
    println!("Test case is in: test_cases/single_step/hamming/basic_correction_keep");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/basic_correction_keep"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_basic_correction_split_regions() {
    println!("Test case is in: test_cases/single_step/hamming/basic_correction_split_regions");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/basic_correction_split_regions"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_basic_correction_too_close() {
    println!("Test case is in: test_cases/single_step/hamming/basic_correction_too_close");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/basic_correction_too_close"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_basic_correction_too_close_but_same_label() {
    println!(
        "Test case is in: test_cases/single_step/hamming/basic_correction_too_close_but_same_label"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/basic_correction_too_close_but_same_label",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_basic_correction_too_close_but_same_label_after_split() {
    println!(
        "Test case is in: test_cases/single_step/hamming/basic_correction_too_close_but_same_label_after_split"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/basic_correction_too_close_but_same_label_after_split",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_best_match_at_distance_1_of_2() {
    println!("Test case is in: test_cases/single_step/hamming/best_match_at_distance_1_of_2");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/best_match_at_distance_1_of_2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_basic() {
    println!("Test case is in: test_cases/single_step/hamming/by_edit_probability/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_edit_probability/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_dump_counts() {
    println!("Test case is in: test_cases/single_step/hamming/by_edit_probability/dump_counts");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_edit_probability/dump_counts"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_from_report() {
    println!("Test case is in: test_cases/single_step/hamming/by_edit_probability/from_report");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_edit_probability/from_report"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_from_report_but_wrong_method() {
    println!(
        "Test case is in: test_cases/single_step/hamming/by_edit_probability/from_report_but_wrong_method"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_edit_probability/from_report_but_wrong_method",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_from_report_cantread_file_x_missing_histogram()
 {
    println!(
        "Test case is in: test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/missing_histogram"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/missing_histogram",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_from_report_cantread_file_x_missing_report_name()
 {
    println!(
        "Test case is in: test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/missing_report_name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/missing_report_name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_from_report_cantread_file_x_nofile() {
    println!(
        "Test case is in: test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/nofile"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/nofile",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_from_report_cantread_file_x_notjson()
{
    println!(
        "Test case is in: test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/notjson"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_edit_probability/from_report_cantread_file/notjson",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_edit_probability_x_must_have_dist_1() {
    println!(
        "Test case is in: test_cases/single_step/hamming/by_edit_probability/must_have_dist_1"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_edit_probability/must_have_dist_1",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_dump_counts() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/dump_counts");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/dump_counts"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_never_seen() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/never_seen");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/never_seen"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_no_pretrain() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/no_pretrain");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/no_pretrain"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_not_enough_blocks_in_flight() {
    println!(
        "Test case is in: test_cases/single_step/hamming/by_majority/not_enough_blocks_in_flight"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_majority/not_enough_blocks_in_flight",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_reaching_read_count() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/reaching_read_count");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/reaching_read_count"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_reaching_read_count_partial_block() {
    println!(
        "Test case is in: test_cases/single_step/hamming/by_majority/reaching_read_count_partial_block"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/by_majority/reaching_read_count_partial_block",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_simple() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/simple");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/simple"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_string_tag() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/string_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/string_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_threshold_too_high() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/threshold_too_high");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/threshold_too_high"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_by_majority_x_threshold_too_low() {
    println!("Test case is in: test_cases/single_step/hamming/by_majority/threshold_too_low");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/by_majority/threshold_too_low"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_duplicate_barcode() {
    println!("Test case is in: test_cases/single_step/hamming/duplicate_barcode");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/duplicate_barcode"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_duplicate_barcode_from_file() {
    println!("Test case is in: test_cases/single_step/hamming/duplicate_barcode_from_file");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/duplicate_barcode_from_file"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_empty_barcode_file() {
    println!("Test case is in: test_cases/single_step/hamming/empty_barcode_file");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/empty_barcode_file"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_from_file_x_txt() {
    println!("Test case is in: test_cases/single_step/hamming/from_file/txt");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/from_file/txt"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_from_file_x_txt_gz() {
    println!("Test case is in: test_cases/single_step/hamming/from_file/txt_gz");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/from_file/txt_gz"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_from_file_x_txt_unequal_lines() {
    println!("Test case is in: test_cases/single_step/hamming/from_file/txt_unequal_lines");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/from_file/txt_unequal_lines"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_input_label_is_output_label() {
    println!("Test case is in: test_cases/single_step/hamming/input_label_is_output_label");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/input_label_is_output_label"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_iupac_overlap_from_file() {
    println!("Test case is in: test_cases/single_step/hamming/iupac_overlap_from_file");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/iupac_overlap_from_file"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_missing_barcode() {
    println!("Test case is in: test_cases/single_step/hamming/missing_barcode");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/missing_barcode"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_missing_barcodes_empty_section() {
    println!("Test case is in: test_cases/single_step/hamming/missing_barcodes_empty_section");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/missing_barcodes_empty_section"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_on_tie() {
    println!("Test case is in: test_cases/single_step/hamming/on_tie");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/on_tie"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_reference_not_enough_hamming_distance() {
    println!(
        "Test case is in: test_cases/single_step/hamming/reference_not_enough_hamming_distance"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/reference_not_enough_hamming_distance",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_reference_not_enough_hamming_distance_but_same_seq() {
    println!(
        "Test case is in: test_cases/single_step/hamming/reference_not_enough_hamming_distance_but_same_seq"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/reference_not_enough_hamming_distance_but_same_seq",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_string_tag_barcodes_too_close() {
    println!("Test case is in: test_cases/single_step/hamming/string_tag_barcodes_too_close");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/string_tag_barcodes_too_close"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_string_tag_barcodes_too_close_but_same_label() {
    println!(
        "Test case is in: test_cases/single_step/hamming/string_tag_barcodes_too_close_but_same_label"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/string_tag_barcodes_too_close_but_same_label",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_string_tag_barcodes_too_close_but_same_label_after_split() {
    println!(
        "Test case is in: test_cases/single_step/hamming/string_tag_barcodes_too_close_but_same_label_after_split"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/string_tag_barcodes_too_close_but_same_label_after_split",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_string_tag_barcodes_too_close_still_different_after_split()
{
    println!(
        "Test case is in: test_cases/single_step/hamming/string_tag_barcodes_too_close_still_different_after_split"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/hamming/string_tag_barcodes_too_close_still_different_after_split",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_string_tag_correction() {
    println!("Test case is in: test_cases/single_step/hamming/string_tag_correction");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/string_tag_correction"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_unequal_len_barcodes_x_from_file() {
    println!("Test case is in: test_cases/single_step/hamming/unequal_len_barcodes/from_file");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/unequal_len_barcodes/from_file"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_hamming_x_unequal_len_barcodes_x_in_toml() {
    println!("Test case is in: test_cases/single_step/hamming/unequal_len_barcodes/in_toml");
    run_test(
        std::path::Path::new("../test_cases/single_step/hamming/unequal_len_barcodes/in_toml"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_after_quantify() {
    println!("Test case is in: test_cases/single_step/head/head_after_quantify");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_after_quantify"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_after_report() {
    println!("Test case is in: test_cases/single_step/head/head_after_report");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_after_report"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_after_report_base_stats() {
    println!("Test case is in: test_cases/single_step/head/head_after_report_base_stats");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_after_report_base_stats"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_after_report_duplicate_count() {
    println!("Test case is in: test_cases/single_step/head/head_after_report_duplicate_count");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_after_report_duplicate_count"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_after_report_duplicate_fragment_count() {
    println!(
        "Test case is in: test_cases/single_step/head/head_after_report_duplicate_fragment_count"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/head/head_after_report_duplicate_fragment_count",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_after_report_length_distribution() {
    println!("Test case is in: test_cases/single_step/head/head_after_report_length_distribution");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/head/head_after_report_length_distribution",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_after_report_oligos() {
    println!("Test case is in: test_cases/single_step/head/head_after_report_oligos");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_after_report_oligos"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_before_quantify() {
    println!("Test case is in: test_cases/single_step/head/head_before_quantify");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_before_quantify"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_before_report() {
    println!("Test case is in: test_cases/single_step/head/head_before_report");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_before_report"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_stops_reading() {
    println!("Test case is in: test_cases/single_step/head/head_stops_reading");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_stops_reading"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_stops_reading_multiple() {
    println!("Test case is in: test_cases/single_step/head/head_stops_reading_multiple");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_stops_reading_multiple"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_head_with_tags() {
    println!("Test case is in: test_cases/single_step/head/head_with_tags");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/head_with_tags"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_multi_stage_head() {
    println!("Test case is in: test_cases/single_step/head/multi_stage_head");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/multi_stage_head"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_multi_stage_head_report_bottom() {
    println!("Test case is in: test_cases/single_step/head/multi_stage_head_report_bottom");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/multi_stage_head_report_bottom"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_multi_stage_head_report_middle() {
    println!("Test case is in: test_cases/single_step/head/multi_stage_head_report_middle");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/multi_stage_head_report_middle"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_multi_stage_head_report_middle_bottom() {
    println!("Test case is in: test_cases/single_step/head/multi_stage_head_report_middle_bottom");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/head/multi_stage_head_report_middle_bottom",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_multi_stage_head_report_top() {
    println!("Test case is in: test_cases/single_step/head/multi_stage_head_report_top");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/multi_stage_head_report_top"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_head_x_multi_stage_head_spot_check_pairing() {
    println!("Test case is in: test_cases/single_step/head/multi_stage_head_spot_check_pairing");
    run_test(
        std::path::Path::new("../test_cases/single_step/head/multi_stage_head_spot_check_pairing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_all_interleaved() {
    println!("Test case is in: test_cases/single_step/inspect/all_interleaved");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/all_interleaved"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_all_interleaved_reversed() {
    println!("Test case is in: test_cases/single_step/inspect/all_interleaved_reversed");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/all_interleaved_reversed"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_compression_zstd_level() {
    println!("Test case is in: test_cases/single_step/inspect/compression_zstd_level");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/compression_zstd_level"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_x_inspect_broken_config() {
    println!("Test case is in: test_cases/single_step/inspect/inspect/inspect_broken_config");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect/inspect_broken_config"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_x_inspect_index1() {
    println!("Test case is in: test_cases/single_step/inspect/inspect/inspect_index1");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect/inspect_index1"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_x_inspect_index2() {
    println!("Test case is in: test_cases/single_step/inspect/inspect/inspect_index2");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect/inspect_index2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_x_inspect_read1() {
    println!("Test case is in: test_cases/single_step/inspect/inspect/inspect_read1");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect/inspect_read1"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_x_inspect_read2() {
    println!("Test case is in: test_cases/single_step/inspect/inspect/inspect_read2");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect/inspect_read2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_x_inspect_read2_fasta() {
    println!("Test case is in: test_cases/single_step/inspect/inspect/inspect_read2_fasta");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect/inspect_read2_fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_x_inspect_read2_fasta_demultiplex() {
    println!(
        "Test case is in: test_cases/single_step/inspect/inspect/inspect_read2_fasta_demultiplex"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/inspect/inspect/inspect_read2_fasta_demultiplex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_no_bam() {
    println!("Test case is in: test_cases/single_step/inspect/inspect_no_bam");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect_no_bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_inspect_x_inspect_read1_compressed() {
    println!("Test case is in: test_cases/single_step/inspect/inspect_read1_compressed");
    run_test(
        std::path::Path::new("../test_cases/single_step/inspect/inspect_read1_compressed"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_output_x_allow_override_but_file_is_read_only() {
    println!("Test case is in: test_cases/single_step/output/allow_override_but_file_is_read_only");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/output/allow_override_but_file_is_read_only",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_output_x_cut_end_named_pipes_x_output_pipe() {
    println!("Test case is in: test_cases/single_step/output/cut_end_named_pipes/output_pipe");
    run_test(
        std::path::Path::new("../test_cases/single_step/output/cut_end_named_pipes/output_pipe"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_output_x_output_bam_x_basic() {
    println!("Test case is in: test_cases/single_step/output/output_bam/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/output/output_bam/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_output_x_output_fasta_x_basic() {
    println!("Test case is in: test_cases/single_step/output/output_fasta/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/output/output_fasta/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_output_x_output_fastq_x_basic() {
    println!("Test case is in: test_cases/single_step/output/output_fastq/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/output/output_fastq/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_output_x_output_neither_r1_nor_r2_but_index() {
    println!("Test case is in: test_cases/single_step/output/output_neither_r1_nor_r2_but_index");
    run_test(
        std::path::Path::new("../test_cases/single_step/output/output_neither_r1_nor_r2_but_index"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_quantify_tag() {
    println!("Test case is in: test_cases/single_step/quantify_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/quantify_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_rename_x_rename_read_index_placeholder() {
    println!("Test case is in: test_cases/single_step/rename/rename_read_index_placeholder");
    run_test(
        std::path::Path::new("../test_cases/single_step/rename/rename_read_index_placeholder"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_rename_x_rename_regex() {
    println!("Test case is in: test_cases/single_step/rename/rename_regex");
    run_test(
        std::path::Path::new("../test_cases/single_step/rename/rename_regex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_rename_x_rename_regex_gets_longer() {
    println!("Test case is in: test_cases/single_step/rename/rename_regex_gets_longer");
    run_test(
        std::path::Path::new("../test_cases/single_step/rename/rename_regex_gets_longer"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_rename_x_rename_regex_shorter() {
    println!("Test case is in: test_cases/single_step/rename/rename_regex_shorter");
    run_test(
        std::path::Path::new("../test_cases/single_step/rename/rename_regex_shorter"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_duplicate_fragment_count_is_stable() {
    println!("Test case is in: test_cases/single_step/reports/duplicate_fragment_count_is_stable");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/reports/duplicate_fragment_count_is_stable",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_duplication_count_is_stable() {
    println!("Test case is in: test_cases/single_step/reports/duplication_count_is_stable");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/duplication_count_is_stable"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_oligo_counts_x_2() {
    println!("Test case is in: test_cases/single_step/reports/oligo_counts/2");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/oligo_counts/2"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_oligo_counts_x_bad_query() {
    println!("Test case is in: test_cases/single_step/reports/oligo_counts/bad_query");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/oligo_counts/bad_query"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_oligo_counts_x_basic() {
    println!("Test case is in: test_cases/single_step/reports/oligo_counts/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/oligo_counts/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_oligo_counts_x_demultiplex() {
    println!("Test case is in: test_cases/single_step/reports/oligo_counts/demultiplex");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/oligo_counts/demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_progress_error_check() {
    println!("Test case is in: test_cases/single_step/reports/progress_error_check");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/progress_error_check"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_progress_init_messages() {
    println!("Test case is in: test_cases/single_step/reports/progress_init_messages");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/progress_init_messages"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_read_length_reporting() {
    println!("Test case is in: test_cases/single_step/reports/read_length_reporting");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/read_length_reporting"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_bam() {
    println!("Test case is in: test_cases/single_step/reports/report/bam");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/bam"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_basic() {
    println!("Test case is in: test_cases/single_step/reports/report/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_depduplication_per_fragment() {
    println!("Test case is in: test_cases/single_step/reports/report/depduplication_per_fragment");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/reports/report/depduplication_per_fragment",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_duplicate_label() {
    println!("Test case is in: test_cases/single_step/reports/report/duplicate_label");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/duplicate_label"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_empty_name() {
    println!("Test case is in: test_cases/single_step/reports/report/empty_name");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/empty_name"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_fasta() {
    println!("Test case is in: test_cases/single_step/reports/report/fasta");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/fasta"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_fasta_more_than_one_block() {
    println!("Test case is in: test_cases/single_step/reports/report/fasta_more_than_one_block");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/fasta_more_than_one_block"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_no_output() {
    println!("Test case is in: test_cases/single_step/reports/report/no_output");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/no_output"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_ordering() {
    println!("Test case is in: test_cases/single_step/reports/report/ordering");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/ordering"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_pe() {
    println!("Test case is in: test_cases/single_step/reports/report/pe");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/pe"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_tag_histogram_x_basic() {
    println!("Test case is in: test_cases/single_step/reports/report/tag_histogram/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/tag_histogram/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_tag_histogram_x_missing_strings() {
    println!(
        "Test case is in: test_cases/single_step/reports/report/tag_histogram/missing_strings"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/reports/report/tag_histogram/missing_strings",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_tag_histogram_x_non_integer_values() {
    println!(
        "Test case is in: test_cases/single_step/reports/report/tag_histogram/non_integer_values"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/reports/report/tag_histogram/non_integer_values",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_tag_histogram_x_tag_not_defined() {
    println!(
        "Test case is in: test_cases/single_step/reports/report/tag_histogram/tag_not_defined"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/reports/report/tag_histogram/tag_not_defined",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_reports_x_report_x_two_outputs() {
    println!("Test case is in: test_cases/single_step/reports/report/two_outputs");
    run_test(
        std::path::Path::new("../test_cases/single_step/reports/report/two_outputs"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_reservoir_x_error_n_too_low() {
    println!("Test case is in: test_cases/single_step/sampling/reservoir/error_n_too_low");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/reservoir/error_n_too_low"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_reservoir_x_sample() {
    println!("Test case is in: test_cases/single_step/sampling/reservoir/sample");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/reservoir/sample"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_reservoir_x_sample_multi_segments() {
    println!("Test case is in: test_cases/single_step/sampling/reservoir/sample_multi_segments");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/reservoir/sample_multi_segments"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_reservoir_x_sample_twice() {
    println!("Test case is in: test_cases/single_step/sampling/reservoir/sample_twice");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/reservoir/sample_twice"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_sample_x_basic() {
    println!("Test case is in: test_cases/single_step/sampling/sample/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/sample/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_sample_x_error_p_too_high() {
    println!("Test case is in: test_cases/single_step/sampling/sample/error_p_too_high");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/sample/error_p_too_high"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_sample_x_error_p_too_low() {
    println!("Test case is in: test_cases/single_step/sampling/sample/error_p_too_low");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/sample/error_p_too_low"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_sampling_x_skip() {
    println!("Test case is in: test_cases/single_step/sampling/skip");
    run_test(
        std::path::Path::new("../test_cases/single_step/sampling/skip"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_comment_no_insert_char_present() {
    println!("Test case is in: test_cases/single_step/store_tag/in_comment_no_insert_char_present");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/in_comment_no_insert_char_present",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_comment_tag_comment_separator_conflict() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/in_comment_tag_comment_separator_conflict"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/in_comment_tag_comment_separator_conflict",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_comment_with_insert_char_present() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/in_comment_with_insert_char_present"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/in_comment_with_insert_char_present",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_sequence_exceeding_read_length() {
    println!("Test case is in: test_cases/single_step/store_tag/in_sequence_exceeding_read_length");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/in_sequence_exceeding_read_length",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_sequence_missing() {
    println!("Test case is in: test_cases/single_step/store_tag/in_sequence_missing");
    run_test(
        std::path::Path::new("../test_cases/single_step/store_tag/in_sequence_missing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_table_identical_infix_x_not_set() {
    println!("Test case is in: test_cases/single_step/store_tag/in_table_identical_infix/not_set");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/in_table_identical_infix/not_set",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_table_identical_infix_x_set() {
    println!("Test case is in: test_cases/single_step/store_tag/in_table_identical_infix/set");
    run_test(
        std::path::Path::new("../test_cases/single_step/store_tag/in_table_identical_infix/set"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_table_output_file_unwritable() {
    println!("Test case is in: test_cases/single_step/store_tag/in_table_output_file_unwritable");
    run_test(
        std::path::Path::new("../test_cases/single_step/store_tag/in_table_output_file_unwritable"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_table_output_stdout() {
    println!("Test case is in: test_cases/single_step/store_tag/in_table_output_stdout");
    run_test(
        std::path::Path::new("../test_cases/single_step/store_tag/in_table_output_stdout"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_table_output_stdout_demultiplex() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/in_table_output_stdout_demultiplex"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/in_table_output_stdout_demultiplex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_in_table_output_stdout_twice() {
    println!("Test case is in: test_cases/single_step/store_tag/in_table_output_stdout_twice");
    run_test(
        std::path::Path::new("../test_cases/single_step/store_tag/in_table_output_stdout_twice"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_infix_invalid() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/infix_invalid"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/infix_invalid",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cellranger_x_basic() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/basic"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/basic",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cellranger_x_with_conflict_loser_first()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_conflict_loser_first"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_conflict_loser_first",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cellranger_x_with_conflict_winner()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_conflict_winner"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_conflict_winner",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cellranger_x_with_conflicts()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_conflicts"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_conflicts",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cellranger_x_with_n_and_unmatched()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_n_and_unmatched"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/with_n_and_unmatched",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cellranger_x_zero_reads()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/zero_reads"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cellranger/zero_reads",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cluster_x_basic() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cluster/basic"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cluster/basic",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cluster_x_with_n_umi()
{
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cluster/with_n_umi"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cluster/with_n_umi",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_cluster_x_zero_reads()
{
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cluster/zero_reads"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_cluster/zero_reads",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_exact_x_basic() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_exact/basic"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_exact/basic",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_exact_x_with_n_umi() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_exact/with_n_umi"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_exact/with_n_umi",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_exact_x_zero_reads() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_exact/zero_reads"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_exact/zero_reads",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_basic() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/basic"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/basic",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_cell_label_lookup()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/cell_label_lookup"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/cell_label_lookup",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_cell_label_lookup_fail()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/cell_label_lookup_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/cell_label_lookup_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_compressed() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/compressed"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/compressed",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_gene_label_lookup()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/gene_label_lookup"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/gene_label_lookup",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_gene_label_lookup_fail()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/gene_label_lookup_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/gene_label_lookup_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_missing_tags()
{
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/missing_tags"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/missing_tags",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_missing_umi() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/missing_umi"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/missing_umi",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_string_tags() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/string_tags"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/string_tags",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_umi_nonuniform()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/umi_nonuniform"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/umi_nonuniform",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_umi_too_long()
{
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/umi_too_long"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/umi_too_long",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_with_demultiplex()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/with_demultiplex"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/with_demultiplex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_with_unmatched()
 {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/with_unmatched"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/with_unmatched",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_aggr_none_x_zero_reads() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/zero_reads"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_aggr_none/zero_reads",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_missing_string_tag() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_missing_string_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_missing_string_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_too_long_x_16bp_ok() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_too_long/16bp_ok"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_too_long/16bp_ok",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_too_long_x_basic() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_too_long/basic"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_too_long/basic",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_single_cell_matrix_x_umi_too_long_x_string_tag() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_single_cell_matrix/umi_too_long/string_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_single_cell_matrix/umi_too_long/string_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_store_tag_x_store_tags_in_table_no_tags_defined() {
    println!(
        "Test case is in: test_cases/single_step/store_tag/store_tags_in_table_no_tags_defined"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/store_tag/store_tags_in_table_no_tags_defined",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_tag_matches_x_bad_tags() {
    println!("Test case is in: test_cases/single_step/tag_matches/bad_tags");
    run_test(
        std::path::Path::new("../test_cases/single_step/tag_matches/bad_tags"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_tag_matches_x_basic() {
    println!("Test case is in: test_cases/single_step/tag_matches/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/tag_matches/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_transform_x_max_len() {
    println!("Test case is in: test_cases/single_step/transform/max_len");
    run_test(
        std::path::Path::new("../test_cases/single_step/transform/max_len"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_lowqualityend() {
    println!("Test case is in: test_cases/single_step/trim/LowQualityEnd");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/LowQualityEnd"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_lowqualityendwrongtypeforminqual() {
    println!("Test case is in: test_cases/single_step/trim/LowQualityEndWrongTypeForMinQual");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/LowQualityEndWrongTypeForMinQual"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_cut_end_x_basic() {
    println!("Test case is in: test_cases/single_step/trim/cut_end/basic");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/cut_end/basic"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_cut_end_x_if_tag() {
    println!("Test case is in: test_cases/single_step/trim/cut_end/if_tag");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/cut_end/if_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_cut_end_x_lifting_location() {
    println!("Test case is in: test_cases/single_step/trim/cut_end/lifting_location");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/cut_end/lifting_location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_cut_start() {
    println!("Test case is in: test_cases/single_step/trim/cut_start");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/cut_start"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_at_tag_lifting_location() {
    println!("Test case is in: test_cases/single_step/trim/trim_at_tag_lifting_location");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_at_tag_lifting_location"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_at_tag_multi_region_x_end_false() {
    println!("Test case is in: test_cases/single_step/trim/trim_at_tag_multi_region/end_false");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_at_tag_multi_region/end_false"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_at_tag_multi_region_x_end_true() {
    println!("Test case is in: test_cases/single_step/trim/trim_at_tag_multi_region/end_true");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_at_tag_multi_region/end_true"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_at_tag_multi_region_x_start_false() {
    println!("Test case is in: test_cases/single_step/trim/trim_at_tag_multi_region/start_false");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_at_tag_multi_region/start_false"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_at_tag_multi_region_x_start_true() {
    println!("Test case is in: test_cases/single_step/trim/trim_at_tag_multi_region/start_true");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_at_tag_multi_region/start_true"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_poly_tail_x_detail() {
    println!("Test case is in: test_cases/single_step/trim/trim_poly_tail/detail");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_poly_tail/detail"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_poly_tail_x_detail_g() {
    println!("Test case is in: test_cases/single_step/trim/trim_poly_tail/detail_g");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_poly_tail/detail_g"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_poly_tail_x_long() {
    println!("Test case is in: test_cases/single_step/trim/trim_poly_tail/long");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_poly_tail/long"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_poly_tail_x_n() {
    println!("Test case is in: test_cases/single_step/trim/trim_poly_tail/n");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_poly_tail/n"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_poly_tail_x_n_keep_end() {
    println!("Test case is in: test_cases/single_step/trim/trim_poly_tail/n_keep_end");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_poly_tail/n_keep_end"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_qual_x_end() {
    println!("Test case is in: test_cases/single_step/trim/trim_qual/end");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_qual/end"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_trim_x_trim_qual_x_start() {
    println!("Test case is in: test_cases/single_step/trim/trim_qual/start");
    run_test(
        std::path::Path::new("../test_cases/single_step/trim/trim_qual/start"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_spot_check_read_pairing_x_disabled() {
    println!("Test case is in: test_cases/single_step/validation/spot_check_read_pairing/disabled");
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/spot_check_read_pairing/disabled",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_spot_check_read_pairing_x_fail() {
    println!("Test case is in: test_cases/single_step/validation/spot_check_read_pairing/fail");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/spot_check_read_pairing/fail"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_spot_check_read_pairing_x_fail_unequal_len() {
    println!(
        "Test case is in: test_cases/single_step/validation/spot_check_read_pairing/fail_unequal_len"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/spot_check_read_pairing/fail_unequal_len",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_spot_check_read_pairing_x_illumina_2026() {
    println!(
        "Test case is in: test_cases/single_step/validation/spot_check_read_pairing/illumina_2026"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/spot_check_read_pairing/illumina_2026",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_spot_check_read_pairing_x_not_sampled_no_error() {
    println!(
        "Test case is in: test_cases/single_step/validation/spot_check_read_pairing/not_sampled_no_error"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/spot_check_read_pairing/not_sampled_no_error",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_spot_check_read_pairing_x_simple() {
    println!("Test case is in: test_cases/single_step/validation/spot_check_read_pairing/simple");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/spot_check_read_pairing/simple"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_all() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_all_fail() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/all_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/all_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_basic() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/basic"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/basic",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_fail() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_name() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/name"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/name",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_name_all() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/name_all"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/name_all",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_name_all_fail() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/name_all_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/name_all_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_name_fail() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/name_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/name_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_no_source() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/no_source"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/no_source",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_with_location_tag_missing()
 {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/with_location_tag_missing"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/with_location_tag_missing",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_with_tag() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/with_tag"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/with_tag",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_with_tag_fail() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/with_tag_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/with_tag_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_with_tag_missing() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/with_tag_missing"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/with_tag_missing",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_with_tag_missing_fail()
{
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/with_tag_missing_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/with_tag_missing_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_with_tag_string_type() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/with_tag_string_type"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/with_tag_string_type",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_all_reads_same_length_x_with_tag_string_type_fail()
 {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_all_reads_same_length/with_tag_string_type_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_all_reads_same_length/with_tag_string_type_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_name_x_simple() {
    println!("Test case is in: test_cases/single_step/validation/validate_name/simple");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/validate_name/simple"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_name_x_stride_eq_0() {
    println!("Test case is in: test_cases/single_step/validation/validate_name/stride_eq_0");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/validate_name/stride_eq_0"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_name_x_validate_name_custom_separator() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_name/validate_name_custom_separator"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_name/validate_name_custom_separator",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_name_x_validate_name_fail_x_bad_prefix() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_name/validate_name_fail/bad_prefix"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_name/validate_name_fail/bad_prefix",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_name_x_validate_name_fail_x_missing_end_char_in_seq1()
 {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_name/validate_name_fail/missing_end_char_in_seq1"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_name/validate_name_fail/missing_end_char_in_seq1",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_name_x_validate_name_fail_x_missing_end_char_in_seq2()
 {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_name/validate_name_fail/missing_end_char_in_seq2"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_name/validate_name_fail/missing_end_char_in_seq2",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_phred() {
    println!("Test case is in: test_cases/single_step/validation/validate_phred");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/validate_phred"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_phred_fail() {
    println!("Test case is in: test_cases/single_step/validation/validate_phred_fail");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/validate_phred_fail"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_read_names_x_just_ok_length() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_read_names/just_ok_length"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_read_names/just_ok_length",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_read_names_x_printable() {
    println!("Test case is in: test_cases/single_step/validation/validate_read_names/printable");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/validate_read_names/printable"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_read_names_x_printable_fail() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_read_names/printable_fail"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_read_names/printable_fail",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_read_names_x_printable_fail_too_long() {
    println!(
        "Test case is in: test_cases/single_step/validation/validate_read_names/printable_fail_too_long"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/single_step/validation/validate_read_names/printable_fail_too_long",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_seq() {
    println!("Test case is in: test_cases/single_step/validation/validate_seq");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/validate_seq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_single_step_x_validation_x_validate_seq_fail() {
    println!("Test case is in: test_cases/single_step/validation/validate_seq_fail");
    run_test(
        std::path::Path::new("../test_cases/single_step/validation/validate_seq_fail"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tag_labels_x_len_of_tag() {
    println!("Test case is in: test_cases/tag_labels/len_of_tag");
    run_test(
        std::path::Path::new("../test_cases/tag_labels/len_of_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tag_labels_x_len_of_tag_not_existant() {
    println!("Test case is in: test_cases/tag_labels/len_of_tag_not_existant");
    run_test(
        std::path::Path::new("../test_cases/tag_labels/len_of_tag_not_existant"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tag_labels_x_location_of_tag() {
    println!("Test case is in: test_cases/tag_labels/location_of_tag");
    run_test(
        std::path::Path::new("../test_cases/tag_labels/location_of_tag"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tag_labels_x_location_of_tag_non_existant() {
    println!("Test case is in: test_cases/tag_labels/location_of_tag_non_existant");
    run_test(
        std::path::Path::new("../test_cases/tag_labels/location_of_tag_non_existant"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_head() {
    println!("Test case is in: test_cases/tags_and_readset_modifying_steps/head");
    run_test(
        std::path::Path::new("../test_cases/tags_and_readset_modifying_steps/head"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_head_demultiplex() {
    println!("Test case is in: test_cases/tags_and_readset_modifying_steps/head_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/tags_and_readset_modifying_steps/head_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_report_demultiplex_even() {
    println!(
        "Test case is in: test_cases/tags_and_readset_modifying_steps/report_demultiplex_even"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/tags_and_readset_modifying_steps/report_demultiplex_even",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_reservoir_sample() {
    println!("Test case is in: test_cases/tags_and_readset_modifying_steps/reservoir_sample");
    run_test(
        std::path::Path::new("../test_cases/tags_and_readset_modifying_steps/reservoir_sample"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_reservoir_sample_demultiplex() {
    println!(
        "Test case is in: test_cases/tags_and_readset_modifying_steps/reservoir_sample_demultiplex"
    );
    run_test(
        std::path::Path::new(
            "../test_cases/tags_and_readset_modifying_steps/reservoir_sample_demultiplex",
        ),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_sample() {
    println!("Test case is in: test_cases/tags_and_readset_modifying_steps/sample");
    run_test(
        std::path::Path::new("../test_cases/tags_and_readset_modifying_steps/sample"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_skip() {
    println!("Test case is in: test_cases/tags_and_readset_modifying_steps/skip");
    run_test(
        std::path::Path::new("../test_cases/tags_and_readset_modifying_steps/skip"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_tags_and_readset_modifying_steps_x_skip_demultiplex() {
    println!("Test case is in: test_cases/tags_and_readset_modifying_steps/skip_demultiplex");
    run_test(
        std::path::Path::new("../test_cases/tags_and_readset_modifying_steps/skip_demultiplex"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_transform_x_prefix_and_postfix() {
    println!("Test case is in: test_cases/transform/prefix_and_postfix");
    run_test(
        std::path::Path::new("../test_cases/transform/prefix_and_postfix"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_transform_x_prefix_and_postfix_not_dna() {
    println!("Test case is in: test_cases/transform/prefix_and_postfix_not_dna");
    run_test(
        std::path::Path::new("../test_cases/transform/prefix_and_postfix_not_dna"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_transform_x_prefix_and_postfix_qual_range() {
    println!("Test case is in: test_cases/transform/prefix_and_postfix_qual_range");
    run_test(
        std::path::Path::new("../test_cases/transform/prefix_and_postfix_qual_range"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_transform_x_prefix_and_postfix_qual_range_solexa() {
    println!("Test case is in: test_cases/transform/prefix_and_postfix_qual_range_solexa");
    run_test(
        std::path::Path::new("../test_cases/transform/prefix_and_postfix_qual_range_solexa"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_01_x_basic_x_quality_x_report() {
    println!("Test case is in: cookbooks/01-basic-quality-report");
    run_test(
        std::path::Path::new("../cookbooks/01-basic-quality-report"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_02_x_umi_x_extraction() {
    println!("Test case is in: cookbooks/02-umi-extraction");
    run_test(
        std::path::Path::new("../cookbooks/02-umi-extraction"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_03_x_lexogen_x_quantseq() {
    println!("Test case is in: cookbooks/03-lexogen-quantseq");
    run_test(
        std::path::Path::new("../cookbooks/03-lexogen-quantseq"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_04_x_phix_x_removal() {
    println!("Test case is in: cookbooks/04-phiX-removal");
    run_test(
        std::path::Path::new("../cookbooks/04-phiX-removal"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_04_x_phix_x_removal_2() {
    println!("Test case is in: cookbooks/04-phiX-removal");
    run_test(
        std::path::Path::new("../cookbooks/04-phiX-removal"),
        "input_demultiplex.toml",
        2,
    );
}

#[test]

fn test_cases_x_05_x_quality_x_filtering() {
    println!("Test case is in: cookbooks/05-quality-filtering");
    run_test(
        std::path::Path::new("../cookbooks/05-quality-filtering"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_06_x_adapter_x_trimming() {
    println!("Test case is in: cookbooks/06-adapter-trimming");
    run_test(
        std::path::Path::new("../cookbooks/06-adapter-trimming"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_07_x_demultiplexing() {
    println!("Test case is in: cookbooks/07-demultiplexing");
    run_test(
        std::path::Path::new("../cookbooks/07-demultiplexing"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_08_x_length_x_filtering() {
    println!("Test case is in: cookbooks/08-length-filtering");
    run_test(
        std::path::Path::new("../cookbooks/08-length-filtering"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_09_x_fastp_x_equivalent() {
    println!("Test case is in: cookbooks/09-fastp-equivalent");
    run_test(
        std::path::Path::new("../cookbooks/09-fastp-equivalent"),
        "input.toml",
        1,
    );
}

#[test]

fn test_cases_x_10_x_adapter_x_identification() {
    println!("Test case is in: cookbooks/10-adapter-identification");
    run_test(
        std::path::Path::new("../cookbooks/10-adapter-identification"),
        "input.toml",
        1,
    );
}
