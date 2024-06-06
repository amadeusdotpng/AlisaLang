// similar to how the Rust compiler does it.

use super::*;

fn check(s: &str, expected: TokenKind) {
    let mut lex = Lexer::new(s);
    assert_eq!(lex.next_token().kind, expected);
}

#[test]
fn single_character_tokens() {
    check(";", TokenKind::Semi);
    check(":", TokenKind::Colon);
    check(",", TokenKind::Comma);
    check(".", TokenKind::Dot);
    check("(", TokenKind::OpenParen);
    check(")", TokenKind::CloseParen);
    check("{", TokenKind::OpenBrace);
    check("}", TokenKind::CloseBrace);
    check("\\", TokenKind::BSlash);
    check("=", TokenKind::Eq);
    check("<", TokenKind::Lt);
    check(">", TokenKind::Gt);
    check("|", TokenKind::Pipe);
    check("&", TokenKind::And);
    check("^", TokenKind::Caret);
    check("+", TokenKind::Plus);
    check("-", TokenKind::Minus);
    check("*", TokenKind::Star);
    check("/", TokenKind::FSlash);
    check("%", TokenKind::Percent);
    check("!", TokenKind::Bang);
    check("~", TokenKind::Tilde);
    check("\\", TokenKind::BSlash);
    check("`", TokenKind::Unknown);
    check("", TokenKind::EOF);
}

#[test]
fn character_literal_tokens() {
    check(
        "'a'",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: true },
        },
    );
    // Unterminated character literals.
    check(
        "''",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: false },
        },
    );
    check(
        "'a",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: false },
        },
    );
    check(
        "'",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: false },
        },
    );
}

#[test]
fn string_literal_tokens() {
    check(
        "\"foobar\"",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: true },
        },
    );
    check(
        "\"\"",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: true },
        },
    );
    // Unterminated strings
    check(
        "\"foobar",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: false },
        },
    );
    check(
        "\"",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: false },
        },
    );
}

#[test]
fn number_literal_tokens() {
    check(
        "1234",
        TokenKind::Literal {
            kind: LiteralKind::Int,
        },
    );

    check(
        "1234.",
        TokenKind::Literal {
            kind: LiteralKind::Float,
        },
    );
    check(
        ".1234",
        TokenKind::Literal {
            kind: LiteralKind::Float,
        },
    );
    check(
        "12.34",
        TokenKind::Literal {
            kind: LiteralKind::Float,
        },
    );
}
