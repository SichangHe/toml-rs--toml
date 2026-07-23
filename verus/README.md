# whitespace byte boundary

- requirement `WHITESPACE-BOUNDARY`
  - status: bounded no-defect evidence complete when `check.sh` passes
  - public operation: `Source::lex`
  - executable path: `Source::lex` -> `Lexer::next` -> `process_token` -> `lex_whitespace`
  - specification: `whitespace_end_spec`
  - input: a valid Rust string whose next token begins with space or horizontal tab
  - result: one `Whitespace` token ending before the first byte that is neither space nor horizontal tab, or at EOF
  - state change: consume exactly the maximal whitespace run and leave the next byte unconsumed
  - frame: bytes at and after the returned offset remain unconsumed
  - verified model: `verus/whitespace_boundary.rs::whitespace_end`
  - bounded differential bridge: `whitespace_boundary::all_unicode_scalars_match_verified_model`
  - manual bridges: `@ASSUME:whitespace_model_harness`, `@ASSUME:rust_str_utf8`
  - source evidence: `crates/toml_parser/src/lexer/mod.rs::lex_whitespace`
  - existing test evidence: `crates/toml_parser/src/lexer/test.rs::test_lex_whitespace`

- public trace
  - `Source::lex` constructs a `Lexer` over the caller's `&str`
  - `Lexer::next` reads the next byte and routes space or tab to `lex_whitespace`
  - `lex_whitespace` finds the first non-whitespace byte or EOF and advances by that offset
  - `next_slice_unchecked` relies on the offset being a UTF-8 boundary when the `unsafe` feature is enabled
  - the returned `Whitespace` span exposes the consumed byte range to the caller
  - the bounded harness compares that public result with the separately verified byte model

- proof obligations
  - the returned offset is at most the input byte length
  - every earlier byte is space or horizontal tab
  - the returned offset is EOF or points to a byte that is neither space nor horizontal tab
  - valid `&str` input plus consumption of only ASCII bytes makes the resulting production offset a Rust UTF-8 character boundary

- result boundary
  - Verus proves the byte model for arbitrary byte vectors, not production's winnow call or Rust's `&str` invariant
  - the bounded safe/unsafe differential harness places every Unicode scalar immediately after a two-byte whitespace run
  - focused cases cover longer alternating runs, CRLF, EOF, a nonzero public-stream prefix, and a multibyte scalar
  - arbitrary longer runs and suffixes are proved only in the model, not exhaustively compared with production

- checks
  - `./verus/check.sh`
