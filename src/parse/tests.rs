use super::stream::TokenStream;
use crate::ast::token::*;

fn stream_check(s: &str, expected: TokenKind) {
    let mut stream = TokenStream::new(s);
    assert_eq!(stream.next_token().kind, expected);
}

//
#[test]
fn identifier_keyword_tokens() {
    stream_check("fn", TokenKind::Fn);
    stream_check("struct", TokenKind::Struct);
    stream_check("enum", TokenKind::Enum);
    stream_check("let", TokenKind::Let);
    stream_check("->", TokenKind::RArrow);
    stream_check("identifier", TokenKind::Identifier);
}

//
#[test]
fn character_literal_tokens() {
    stream_check(
        "'a'",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: true },
        },
    );
    // Unterminated character literals.
    stream_check(
        "''",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: false },
        },
    );
    stream_check(
        "'a",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: false },
        },
    );
    stream_check(
        "'",
        TokenKind::Literal {
            kind: LiteralKind::Char { terminated: false },
        },
    );
}

//
#[test]
fn string_literal_tokens() {
    stream_check(
        "\"foobar\"",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: true },
        },
    );
    stream_check(
        "\"\"",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: true },
        },
    );
    // Unterminated strings
    stream_check(
        "\"foobar",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: false },
        },
    );
    stream_check(
        "\"",
        TokenKind::Literal {
            kind: LiteralKind::Str { terminated: false },
        },
    );
}

//
#[test]
fn number_literal_tokens() {
    stream_check(
        "1234",
        TokenKind::Literal {
            kind: LiteralKind::Int,
        },
    );

    stream_check(
        "1234.",
        TokenKind::Literal {
            kind: LiteralKind::Float,
        },
    );
    stream_check(
        ".1234",
        TokenKind::Literal {
            kind: LiteralKind::Float,
        },
    );
    stream_check(
        "12.34",
        TokenKind::Literal {
            kind: LiteralKind::Float,
        },
    );
}

#[test]
fn operator_tokens() {
    // Boolean Operators
    stream_check("||", TokenKind::PipePipe);
    stream_check("&&", TokenKind::AndAnd);
    stream_check("!", TokenKind::Bang);
    stream_check("==", TokenKind::EqEq);
    stream_check("!=", TokenKind::BangEq);
    stream_check(">=", TokenKind::GtEq);
    stream_check("<=", TokenKind::LtEq);
    stream_check(">", TokenKind::Gt);
    stream_check("<", TokenKind::Lt);


    // Arithmetic Operators
    stream_check("|", TokenKind::Op { kind: OpKind::Pipe });
    stream_check("&", TokenKind::Op { kind: OpKind::And });
    stream_check("^", TokenKind::Op { kind: OpKind::Caret });
    stream_check("~", TokenKind::Op { kind: OpKind::Tilde });
    stream_check(">>", TokenKind::Op { kind: OpKind::ShiftR });
    stream_check("<<", TokenKind::Op { kind: OpKind::ShiftL });
    stream_check("+", TokenKind::Op { kind: OpKind::Plus });
    stream_check("-", TokenKind::Op { kind: OpKind::Minus });
    stream_check("*", TokenKind::Op { kind: OpKind::Star });
    stream_check("/", TokenKind::Op { kind: OpKind::FSlash });
    stream_check("%", TokenKind::Op { kind: OpKind::Percent });
}

//
#[test]
fn assignment_tokens() {
    stream_check("=", TokenKind::Eq);
    stream_check("|=", TokenKind::OpEq { kind: OpKind::Pipe });
    stream_check("&=", TokenKind::OpEq { kind: OpKind::And });
    stream_check("^=", TokenKind::OpEq { kind: OpKind::Caret });
    stream_check("~=", TokenKind::OpEq { kind: OpKind::Tilde });
    stream_check(">>=", TokenKind::OpEq { kind: OpKind::ShiftR });
    stream_check("<<=", TokenKind::OpEq { kind: OpKind::ShiftL });

    stream_check("+=", TokenKind::OpEq { kind: OpKind::Plus });
    stream_check("-=", TokenKind::OpEq { kind: OpKind::Minus });
    stream_check("*=", TokenKind::OpEq { kind: OpKind::Star });
    stream_check("/=", TokenKind::OpEq { kind: OpKind::FSlash });
    stream_check("%=", TokenKind::OpEq { kind: OpKind::Percent });
}
