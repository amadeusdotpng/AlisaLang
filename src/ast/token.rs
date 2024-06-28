use crate::lex;

macro_rules! T {
    ("ID") => { TokenKind::Identifier };

    ("fn") => { TokenKind::Fn };
    ("struct") => { TokenKind::Struct };
    ("enum") => { TokenKind::Enum };
    ("let") => { TokenKind::Let };
    ("if") => { TokenKind::If };
    ("else") => { TokenKind::Else };

    // Punctuation
    ("->") => { TokenKind::RArrow };
    (";") => { TokenKind::Semi };
    (":") => { TokenKind::Colon };
    (",") => { TokenKind::Comma };
    (".") => { TokenKind::Dot };
    ("(") => { TokenKind::OpenParen };
    (")") => { TokenKind::CloseParen };
    ("{") => { TokenKind::OpenBrace };
    ("}") => { TokenKind::CloseBrace };
    ("[") => { TokenKind::OpenBracket };
    ("]") => { TokenKind::CloseBracket };
    ("\\") => { TokenKind::BSlash };

    // Operators
    ("|") => { TokenKind::Op { kind: OpKind::Pipe } };
    ("&") => { TokenKind::Op { kind: OpKind::And } };
    ("^") => { TokenKind::Op { kind: OpKind::Caret } };
    (">>") => { TokenKind::Op { kind: OpKind::ShiftR } };
    ("<<") => { TokenKind::Op { kind: OpKind::ShiftL } };
    ("+") => { TokenKind::Op { kind: OpKind::Plus } };
    ("-") => { TokenKind::Op { kind: OpKind::Minus } };
    ("*") => { TokenKind::Op { kind: OpKind::Star } };
    ("/") => { TokenKind::Op { kind: OpKind::FSlash } };
    ("%") => { TokenKind::Op { kind: OpKind::Percent } };

    ("|=") => { TokenKind::OpEq { kind: OpKind::Pipe } };
    ("&=") => { TokenKind::OpEq { kind: OpKind::And } };
    ("^=") => { TokenKind::OpEq { kind: OpKind::Caret } };
    (">>=") => { TokenKind::OpEq { kind: OpKind::ShiftR } };
    ("<<=") => { TokenKind::OpEq { kind: OpKind::ShiftL } };
    ("+=") => { TokenKind::OpEq { kind: OpKind::Plus } };
    ("-=") => { TokenKind::OpEq { kind: OpKind::Minus } };
    ("*=") => { TokenKind::OpEq { kind: OpKind::Star } };
    ("/=") => { TokenKind::OpEq { kind: OpKind::FSlash } };
    ("%=") => { TokenKind::OpEq { kind: OpKind::Percent } };

    ("!") => { TokenKind::Bang };
    ("~") => { TokenKind::Tilde };
    (">") => { TokenKind::Gt };
    ("<") => { TokenKind::Lt };
    ("=") => { TokenKind::Eq };
    ("||") => { TokenKind::PipePipe };
    ("&&") => { TokenKind::AndAnd };
    ("==") => { TokenKind::EqEq };
    ("!=") => { TokenKind::BangEq };
    (">=") => { TokenKind::GtEq };
    ("<=") => { TokenKind::LtEq };
    ("|>") => { TokenKind::PipeGt };

    ("bool") => { TokenKind::Literal { LiteralKind::Bool } };
    ("int") => { TokenKind::Literal { LiteralKind::Int } };
    ("float") => { TokenKind::Literal { LiteralKind::Float } };

    ("EOF") => { TokenKind::EOF };
}

pub(crate) use T;

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

    // Punctuation
    // `->`
    RArrow,
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
    // `~`
    Tilde,
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
pub enum OpKind {
    // `|`
    Pipe,
    // `&`
    And,
    // `^`
    Caret,
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
pub enum LiteralKind {
    Bool,
    Int,
    Float, 
    Str { terminated: bool },
    Char { terminated: bool },
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
