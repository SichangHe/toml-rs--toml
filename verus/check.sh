#!/usr/bin/env bash
set -eu

run() {
    if ! check_output=$("$@" 2>&1); then
        printf 'failed: %s\n%s\n' "$*" "$check_output" >&2
        return 1
    fi
}

midas_lex_bin=${MIDAS_LEX_BIN:-midas-lex}
verus_bin=${VERUS_BIN:-verus-0.2026.03.17}
run "$midas_lex_bin" --version
run "$midas_lex_bin" docs read verus_termination_measures
run "$verus_bin" verus/whitespace_boundary.rs
run cargo test --quiet -p toml_parser --test whitespace_boundary
run cargo test --quiet -p toml_parser --test whitespace_boundary --features unsafe
run cargo test --quiet -p toml_parser --lib lexer::test::test_lex_whitespace
run cargo test --quiet -p toml_parser --lib lexer::test::test_lex_whitespace --features unsafe
run cargo fmt --all --check
run cargo clippy --quiet -p toml_parser --all-targets --all-features -- -D warnings
run assumls check .
