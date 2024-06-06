use crate::lex;

pub mod node;
use node::*;

#[derive(Debug)]
pub struct ASTree {
    root: Vec<Statement>
}

impl ASTree {
    pub fn new(root: Vec<Statement>) -> Self {
        Self { root }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,

    // Keywords
    // `fn`
    Fn,
    // `struct`
    Struct,
    // `enum`
    Enum,
    // `let`
    Let,
    // `if`
    If,
    // `else`
    Else,
    // `->`
    RArrow,

    // Punctuation
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
    // `[`
    OpenBracket,
    // `]`
    CloseBracket,
    // `\`
    BSlash,

    // Operators
    Op { kind: OpKind },
    OpEq { kind: OpKind },

    // `!`
    Bang,
    // `>`
    Gt,
    // `<`
    Lt,
    // `=`
    Eq,
    // `||`
    PipePipe,
    // `&&`
    AndAnd,
    // `==`
    EqEq,
    // `!=`
    BangEq,
    // `>=`
    GtEq,
    // `<=`
    LtEq,
    // `|>`
    PipeGt,

    Literal { kind: LiteralKind },

    EOF,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
    Float, 
    Str { terminated: bool },
    Char { terminated: bool },
    Bool
}

impl From<lex::LiteralKind> for LiteralKind {
    fn from(value: lex::LiteralKind) -> Self {
        match value {
            lex::LiteralKind::Int => Self::Int,
            lex::LiteralKind::Float => Self::Float,
            lex::LiteralKind::Str { terminated } => Self::Str { terminated },
            lex::LiteralKind::Char { terminated }=> Self::Char { terminated },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpKind {
    // `|`
    Pipe,
    // `&`
    And,
    // `^`
    Caret,
    // `~`
    Tilde,
    // `>>`
    ShiftR,
    // `<<
    ShiftL,

    // `+`
    Plus,
    // `-`
    Minus,
    // `*`
    Star,
    // `/`
    FSlash,
    // `%`
    Percent,
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
