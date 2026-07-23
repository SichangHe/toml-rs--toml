use toml_parser::Source;
use toml_parser::lexer::TokenKind;

// @ASSUME:whitespace_model_harness
// @ASSUME:rust_str_utf8
fn verified_whitespace_end(bytes: &[u8]) -> usize {
    let mut end = 0;
    while end < bytes.len() && matches!(bytes[end], b' ' | b'\t') {
        end += 1;
    }
    end
}

fn assert_lexer_matches_verified_model(input: &str, whitespace_start: usize) {
    assert!(matches!(input.as_bytes()[whitespace_start], b' ' | b'\t'));
    let expected_end =
        whitespace_start + verified_whitespace_end(&input.as_bytes()[whitespace_start..]);
    let source = Source::new(input);
    let mut lexer = source.lex();
    let whitespace = lexer
        .find(|token| token.kind() == TokenKind::Whitespace)
        .expect("whitespace token");

    assert_eq!(whitespace.kind(), TokenKind::Whitespace, "input {input:?}");
    assert_eq!(
        whitespace.span().start(),
        whitespace_start,
        "input {input:?}"
    );
    assert_eq!(whitespace.span().end(), expected_end, "input {input:?}");
    assert_eq!(
        source.get(whitespace).expect("whitespace span").as_str(),
        &input[whitespace_start..expected_end],
        "input {input:?}"
    );
    assert!(input.is_char_boundary(expected_end), "input {input:?}");

    let next = lexer.next().expect("next token");
    assert_eq!(next.span().start(), expected_end, "input {input:?}");
}

#[test]
fn all_unicode_scalars_match_verified_model() {
    let mut input = String::with_capacity(10);
    for scalar in 0..=0x10_ffff {
        let Some(ch) = char::from_u32(scalar) else {
            continue;
        };

        input.clear();
        input.push(' ');
        input.push('\t');
        input.push(ch);
        assert_lexer_matches_verified_model(&input, 0);

        input.insert(0, 'x');
        assert_lexer_matches_verified_model(&input, 1);
    }
}

#[test]
fn maximal_ascii_runs_and_terminators_match_verified_model() {
    for (input, whitespace_start) in [
        (" \t \tx", 0),
        ("x\t \r\nrest", 1),
        ("\t ", 0),
        ("x \té", 1),
    ] {
        assert_lexer_matches_verified_model(input, whitespace_start);
    }
}
