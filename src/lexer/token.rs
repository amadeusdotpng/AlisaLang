#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,

    Literal { kind: LiteralKind },

    // see is_whitespace() in mod.rs
    Whitespace,

    // `;`
    Semi,
    // `:`
    Colon,
    // `,`
    Comma,
    // `.`
    Dot,
    // `(`
    OpenParen,
    // `)`
    CloseParen,
    // `{`
    OpenBrace,
    // `}`
    CloseBrace,
    // `|`
    Or,
    // `&`
    And,
    // `^`
    Caret,
    // `%`
    Percent,
    // `+`
    Plus,
    // `-`
    Minus,
    // `*`
    Star,
    // `/`
    Slash,
    // `!`
    Bang,
    // `=`
    Eq,
    // `<`
    Lt,
    // `>`
    Gt,

    // Any other token not recognized by the lexer.
    Unknown,

    // End of Input
    EOF,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,

    Float,

    Str { terminated: bool },

    Char { terminated: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub length: usize,
}

impl Token {
    pub fn new(kind: TokenKind, length: usize) -> Token {
        Token { kind, length }
    }
}
