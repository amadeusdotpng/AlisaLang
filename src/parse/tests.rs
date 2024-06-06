use super::*;

fn stream_check(s: &str, expected: TokenKind) {
    let mut stream = stream::TokenStream::new(s);
    assert_eq!(stream.next_token().kind, expected);
}
/*
#[test]
fn identifier_keyword_tokens() {
    stream_check("fn", TokenKind::Fn);
    stream_check("struct", TokenKind::Struct);
    stream_check("enum", TokenKind::Enum);
    stream_check("let", TokenKind::Let);
    stream_check("->", TokenKind::RArrow);
    stream_check("identifier", TokenKind::Identifier);
}

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
    stream_check("||", TokenKind::BooleanOp { kind: BooleanOpKind::Or });
    stream_check("&&", TokenKind::BooleanOp { kind: BooleanOpKind::And });
    stream_check("!", TokenKind::BooleanOp { kind: BooleanOpKind::Not });
    stream_check("==", TokenKind::BooleanOp { kind: BooleanOpKind::Eq });
    stream_check("!=", TokenKind::BooleanOp { kind: BooleanOpKind::Ne });
    stream_check(">=", TokenKind::BooleanOp { kind: BooleanOpKind::Ge });
    stream_check("<=", TokenKind::BooleanOp { kind: BooleanOpKind::Le });
    stream_check(">", TokenKind::BooleanOp { kind: BooleanOpKind::Gt });
    stream_check("<", TokenKind::BooleanOp { kind: BooleanOpKind::Lt });


    // Arithmetic Operators
    stream_check("|", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitOr });
    stream_check("&", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitAnd });
    stream_check("^", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitXor });
    stream_check("~", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitNot });
    stream_check(">>", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitRight });
    stream_check("<<", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitLeft });
    stream_check("+", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Add });
    stream_check("-", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Sub });
    stream_check("*", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mul });
    stream_check("/", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Div });
    stream_check("%", TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mod });
}

#[test]
fn assignment_tokens() {
    stream_check("=", TokenKind::Assignment { kind: AssignmentKind::Assign });
    stream_check("|=", TokenKind::Assignment { kind: AssignmentKind::BitOr });
    stream_check("&=", TokenKind::Assignment { kind: AssignmentKind::BitAnd });
    stream_check("^=", TokenKind::Assignment { kind: AssignmentKind::BitXor });
    stream_check("~=", TokenKind::Assignment { kind: AssignmentKind::BitNot });
    stream_check(">>=", TokenKind::Assignment { kind: AssignmentKind::BitRight });
    stream_check("<<=", TokenKind::Assignment { kind: AssignmentKind::BitLeft });

    stream_check("+=", TokenKind::Assignment { kind: AssignmentKind::Add });
    stream_check("-=", TokenKind::Assignment { kind: AssignmentKind::Sub });
    stream_check("*=", TokenKind::Assignment { kind: AssignmentKind::Mul });
    stream_check("/=", TokenKind::Assignment { kind: AssignmentKind::Div });
    stream_check("%=", TokenKind::Assignment { kind: AssignmentKind::Mod });
}
*/
