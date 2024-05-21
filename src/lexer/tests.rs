// similar to how the Rust compiler does it.

use super::*;

fn check(s: &str, expected: TokenKind) {
    let mut lex = Lexer::new(s);
    assert_eq!(lex.next_token().kind, expected);
}
#[test]
fn single_character_tokens() {
    check(">", TokenKind::Gt);
    check("<", TokenKind::Lt);
    check("=", TokenKind::Eq);
    check("!", TokenKind::Bang);
    check("/", TokenKind::Slash);
    check("*", TokenKind::Star);
    check("-", TokenKind::Minus);
    check("+", TokenKind::Plus);
    check("%", TokenKind::Percent);
    check("^", TokenKind::Caret);
    check("&", TokenKind::And);
    check("|", TokenKind::Or);
    check("}", TokenKind::CloseBrace);
    check("{", TokenKind::OpenBrace);
    check(")", TokenKind::CloseParen);
    check("(", TokenKind::OpenParen);
    check(".", TokenKind::Dot);
    check(",", TokenKind::Comma);
    check(":", TokenKind::Colon);
    check(";", TokenKind::Semi);
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
