use crate::lexer::{self, lex::Lexer};
use crate::parser;
use crate::parser::KeywordKind;
use crate::parser::LiteralKind;
use crate::parser::BooleanOpKind;
use crate::parser::ArithmeticOpKind;
use crate::parser::AssignmentKind;
use crate::parser::PunctuationKind;

// Higher-level interface for the Lexer
pub struct TokenStream<'src> {
    src: &'src str,
    lex: Lexer<'src>,
    pos: usize,

    // This is used for trying to convert double-character tokens
    // but it turns out it's not a double-character token
    pub(super) reserved_lex_token: Option<lexer::Token>
}

impl<'src> TokenStream<'src> {
    pub fn new(input: &'src str) -> TokenStream {
        TokenStream { 
            src: input,
            lex: Lexer::new(input),
            pos: 0,
            reserved_lex_token: None,
        }
    }

    pub fn next_token(&mut self) -> parser::Token {
        loop {
            let lex_token = match self.reserved_lex_token {
                Some(tok) => tok,
                None => self.lex.next_token(),
            };
            let start = self.pos;
            self.pos += lex_token.length;

            let kind = match lex_token.kind {
                lexer::TokenKind::Whitespace => { continue }

                lexer::TokenKind::Identifier => self.identifier_or_other(start, lex_token.length),

                lexer::TokenKind::Literal{ kind } => parser::TokenKind::Literal{ kind: kind.into() },

                op @ (
                    lexer::TokenKind::Bang
                  | lexer::TokenKind::Lt
                  | lexer::TokenKind::Gt
                  | lexer::TokenKind::Pipe
                  | lexer::TokenKind::And
                  | lexer::TokenKind::Caret
                  | lexer::TokenKind::Tilde
                  | lexer::TokenKind::Eq
                  | lexer::TokenKind::Plus
                  | lexer::TokenKind::Minus
                  | lexer::TokenKind::Star
                  | lexer::TokenKind::Slash
                  | lexer::TokenKind::Percent) => self.operator(op),

                lexer::TokenKind::Semi => parser::TokenKind::Punctuation { kind: PunctuationKind::Semi },
                lexer::TokenKind::Colon => parser::TokenKind::Punctuation { kind: PunctuationKind::Colon },
                lexer::TokenKind::Comma => parser::TokenKind::Punctuation { kind: PunctuationKind::Comma },
                lexer::TokenKind::Dot => parser::TokenKind::Punctuation { kind: PunctuationKind::Dot },
                lexer::TokenKind::OpenParen => parser::TokenKind::Punctuation { kind: PunctuationKind::OpenParen },
                lexer::TokenKind::CloseParen => parser::TokenKind::Punctuation { kind: PunctuationKind::CloseParen },
                lexer::TokenKind::OpenBrace => parser::TokenKind::Punctuation { kind: PunctuationKind::OpenBrace },
                lexer::TokenKind::CloseBrace => parser::TokenKind::Punctuation { kind: PunctuationKind::CloseBrace },
                lexer::TokenKind::EOF => parser::TokenKind::EOF,

                c => {println!("{:?}", c); todo!();}
            };
            let end = self.pos;

            return parser::Token::new(kind, start, end);
        }
    }

    fn identifier_or_other(&mut self, start: usize, length: usize) -> parser::TokenKind {
        let lexeme = &self.src[start..start+length];
        match lexeme {
            "fn" => parser::TokenKind::Keyword { kind: KeywordKind::Fn },
            "struct" => parser::TokenKind::Keyword { kind: KeywordKind::Struct },
            "enum" => parser::TokenKind::Keyword { kind: KeywordKind::Enum },
            "let" => parser::TokenKind::Keyword { kind: KeywordKind::Let },

            "true" => parser::TokenKind::Literal { kind: LiteralKind::Bool },
            "false" => parser::TokenKind::Literal { kind: LiteralKind::Bool },

            _ => parser::TokenKind::Identifier
        }
    }

    fn operator(&mut self, op: lexer::TokenKind) -> parser::TokenKind {
        let mut peek = self.lex.next_token();
        let op = match op {
            // If these guys pass, it implies that peek in this comment's scope has been consumed
            // successfully so we want to advance the parser's position.
            // We can also just early return on these guys since there's no other lex token that
            // can come after and still be a valid token.

            // `->`
            lexer::TokenKind::Minus if peek.kind == lexer::TokenKind::Gt => {
                self.pos += peek.length;
                return parser::TokenKind::Keyword { kind: KeywordKind::RArrow }
            }

            // `||` 
            c @ lexer::TokenKind::Pipe if peek.kind == c => {
                // We can just return here since there's no other lex token that can come after and
                // still be  a valid
                self.pos += peek.length;
                return parser::TokenKind::BooleanOp { kind: BooleanOpKind::Or };
            },

            // `&&`
            c @ lexer::TokenKind::And if peek.kind == c => {
                self.pos += peek.length;
                return parser::TokenKind::BooleanOp { kind: BooleanOpKind::And };
            },

            // `==`
            c @ lexer::TokenKind::Eq if peek.kind == c => {
                self.pos += peek.length;
                return parser::TokenKind::BooleanOp { kind: BooleanOpKind::Eq };
            },

            // `<<`
            // Since these guys have already consumed the first peek token, we get the next token
            // for later one when we check if it's a bit shift assignment.
            c @ lexer::TokenKind::Lt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitLeft }
            }

            // `>>`
            c @ lexer::TokenKind::Gt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitRight }
            }


            // Boolean 
            lexer::TokenKind::Bang => parser::TokenKind::BooleanOp { kind: BooleanOpKind::Not },
            lexer::TokenKind::Lt => parser::TokenKind::BooleanOp { kind: BooleanOpKind::Lt },
            lexer::TokenKind::Gt => parser::TokenKind::BooleanOp { kind: BooleanOpKind::Gt },


            // Arithmetic Operators
            lexer::TokenKind::Pipe => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitOr },
            lexer::TokenKind::And => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitAnd },
            lexer::TokenKind::Caret => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitXor },
            lexer::TokenKind::Tilde => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitNot },

            lexer::TokenKind::Plus => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Add },
            lexer::TokenKind::Minus => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Sub },
            lexer::TokenKind::Star => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mul },
            lexer::TokenKind::Slash => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Div },
            lexer::TokenKind::Percent => parser::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mod },

            // Normal Assign
            // We can early return here since there's nothing else after an `=` that can make a
            // valid token.
            lexer::TokenKind::Eq => { return parser::TokenKind::Assignment { kind: AssignmentKind::Assign }; },

            // Since this match statement addresses all the productions in the pattern from the
            // outer call, this should be unreachable.
            _ => unreachable!(),
        };

        if let parser::TokenKind::BooleanOp { kind } = op {
            if peek.kind == lexer::TokenKind::Eq {
                self.pos += peek.length;
                return match kind {
                    BooleanOpKind::Not => parser::TokenKind::BooleanOp { kind: BooleanOpKind::Ne },
                    BooleanOpKind::Gt => parser::TokenKind::BooleanOp { kind: BooleanOpKind::Ge },
                    BooleanOpKind::Lt => parser::TokenKind::BooleanOp { kind: BooleanOpKind::Le },

                    // `||` and `&&` don't end in `=`, `==` is handled in the previous match
                    // statement, `!=, `>=`, and `<=` are what we're checking for.
                    // THESE PATTERNS SHOULD BE UNREACHABLE.
                    _ => unreachable!()
                }
            }
        } else if let parser::TokenKind::ArithmeticOp { kind } = op {
            // Check if it's a compound assignment
            if peek.kind == lexer::TokenKind::Eq {
                self.pos += peek.length;
                return parser::TokenKind::Assignment { kind: kind.into() };
            }
        }

        // Since this wasn't sucessfully consumed, we reserve it for the next time next_token()
        // gets called.
        self.reserved_lex_token = Some(peek);
        op
    }

}
