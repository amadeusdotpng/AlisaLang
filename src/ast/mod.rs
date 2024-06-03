use crate::lex;

pub mod nodes;
use nodes::*;

pub struct ASTree {
    root: Vec<Statement>
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
    // `\`
    BSlash,

    Literal { kind: LiteralKind },
    BooleanOp { kind: BooleanOpKind },
    ArithmeticOp { kind: ArithmeticOpKind },
    CompoundAssign { kind: ArithmeticOpKind },
    Assignment,

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
pub enum BooleanOpKind {
    // Boolean Logic Operators
    // `||`
    Or,
    // `&&`
    And,
    // `!`
    Not,

    // Comparison Operators
    // `==`
    Eq,
    // `!=`
    Ne,
    // `>=`
    Ge,
    // `<=`
    Le,
    // `>`
    Gt,
    // `<`
    Lt,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArithmeticOpKind {
    // `|`
    BitOr,
    // `&`
    BitAnd,
    // `^`
    BitXor,
    // `~`
    BitNot,
    // `>>`
    BitRight,
    // `<<
    BitLeft,

    // `+`
    Add,
    // `-`
    Sub,
    // `*`
    Mul,
    // `/`
    Div,
    // `%`
    Mod,
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
