# whitespace_model_harness

`crates/toml_parser/tests/whitespace_boundary.rs::verified_whitespace_end` is the
executable projection of `verus/whitespace_boundary.rs::whitespace_end`: both scan
from zero while each byte is a space or horizontal tab. The bounded differential
test, not Verus, compares that projection with `Source::lex`.

# rust_str_utf8

`Source::new` accepts `&str`, whose Rust type invariant is valid UTF-8. The Verus
byte model does not formalize that invariant. The model proves that it consumes
only ASCII whitespace; the harness checks that the resulting production offset is
a Rust character boundary.
