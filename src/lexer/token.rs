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

    // `=`
    Assign,

    // `|`
    BitOr,
    // `&`
    BitAnd,
    // `^`
    BitXor,
    // `~`
    BitNot,

    // `+`
    Plus,
    // `-`
    Minus,
    // `*`
    Star,
    // `/`
    Slash,
    // `%`
    Modulo,

    // `!`
    BoolNot,
    // `&&`
    BoolAnd,
    // `||`
    BoolOr,

    // `==`
    Eq,
    // `<=`
    Le,
    // `>=`
    Ge,
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
    pub start: usize,
    pub end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Token {
        Token { kind, start, end }
    }
}
