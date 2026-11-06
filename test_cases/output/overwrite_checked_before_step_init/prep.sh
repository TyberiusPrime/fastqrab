#!/usr/bin/env bash
# Pre-create the run's output file so the overwrite check has something to trip
# on. (The harness runs this in the same working directory as the processor.)
set -euo pipefail 2>/dev/null || set -eu
printf 'pre-existing output, must not be overwritten\n' > output_read1.fq
