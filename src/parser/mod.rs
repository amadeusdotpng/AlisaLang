pub mod parse;
pub mod stream;


use stream::TokenStream;

use parse::Parser;
use crate::parser;

use crate::lexer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenKind {
    Identifier,
    Keyword { kind: KeywordKind },
    Literal { kind: LiteralKind },
    BooleanOp { kind: BooleanOpKind },
    ArithmeticOp { kind: ArithmeticOpKind },
    Assignment { kind: AssignmentKind },
    Punctuation { kind: PunctuationKind },
    EOF,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeywordKind {
    // `fn`
    Fn,
    // `struct`
    Struct,
    // `enum`
    Enum,
    // `let`
    Let,
    // `->`
    RArrow,

}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LiteralKind {
    Int,
    Float, 
    Str { terminated: bool },
    Char { terminated: bool },
    Bool
}

impl From<lexer::LiteralKind> for LiteralKind {
    fn from(value: lexer::LiteralKind) -> Self {
        match value {
            lexer::LiteralKind::Int => Self::Int,
            lexer::LiteralKind::Float => Self::Float,
            lexer::LiteralKind::Str { terminated } => Self::Str { terminated },
            lexer::LiteralKind::Char { terminated }=> Self::Char { terminated },
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
pub enum AssignmentKind {
    // `=`
    Assign,

    // `|=`
    BitOr,
    // `&=`
    BitAnd,
    // `^=`
    BitXor,
    // `~=`
    BitNot,
    // `>>=`
    BitRight,
    // `<<=`
    BitLeft,

    // `+=`
    Add,
    // `-=`
    Sub,
    // `*=`
    Mul,
    // `/=`
    Div,
    // `%=`
    Mod,
}

impl From<ArithmeticOpKind> for AssignmentKind {
    fn from(value: ArithmeticOpKind) -> Self {
        match value {
            ArithmeticOpKind::BitOr => AssignmentKind::BitOr,
            ArithmeticOpKind::BitXor => AssignmentKind::BitXor,
            ArithmeticOpKind::BitAnd => AssignmentKind::BitAnd,
            ArithmeticOpKind::BitNot => AssignmentKind::BitNot,
            ArithmeticOpKind::BitRight => AssignmentKind::BitRight,
            ArithmeticOpKind::BitLeft => AssignmentKind::BitLeft,

            ArithmeticOpKind::Add => AssignmentKind::Add,
            ArithmeticOpKind::Sub => AssignmentKind::Sub,
            ArithmeticOpKind::Mul => AssignmentKind::Mul,
            ArithmeticOpKind::Div => AssignmentKind::Div,
            ArithmeticOpKind::Mod => AssignmentKind::Mod,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PunctuationKind {
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
    CloseBrace
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Token {
    kind: TokenKind,
    start: usize,
    end: usize,
}

impl Token {
    pub fn new(kind: TokenKind, start: usize, end: usize) -> Token {
        Token { kind, start, end }
    }
}

impl<'src> TokenStream<'src> {

}

#[cfg(test)]
mod tests;
