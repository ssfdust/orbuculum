#!/bin/sh
set -e
rm -f profiles/*
files=$(RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="profiles/network-%p-%m.profraw" cargo test --tests --no-run --message-format=json 2>/dev/null | jq -r "select(.profile.test == true) | .filenames[]" | grep -v dSYM -)
RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="profiles/network-%p-%m.profraw" cargo test --tests
llvm-profdata merge -sparse profiles/*.profraw -o profiles/network.profdata
llvm-cov report $( \
    for file in $files; \
    do \
        printf "%s %s " -object $file; \
    done \
) \
--use-color --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='/.cargo/git' --ignore-filename-regex='/rustc/' \
--instr-profile=profiles/network.profdata
llvm-cov show --format html $( \
    for file in $files; \
    do \
        printf "%s %s " -object $file; \
    done \
) \
--use-color --ignore-filename-regex='/.cargo/registry' --ignore-filename-regex='/.cargo/git' --ignore-filename-regex='/rustc/' \
--instr-profile=profiles/network.profdata --show-instantiations --show-line-counts-or-regions > coverage.html
rm -f profiles/*
