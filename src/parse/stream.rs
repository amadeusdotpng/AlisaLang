use crate::lex::{self, lexer::Lexer};
use crate::parse;
use crate::parse::LiteralKind;
use crate::parse::BooleanOpKind;
use crate::parse::ArithmeticOpKind;

// Higher-level interface for the Lexer
pub struct TokenStream<'src> {
    src: &'src str,
    lex: Lexer<'src>,
    pos: usize,

    // This is used for trying to convert double-character tokens
    // but it turns out it's not a double-character token
    reserved_lex_token: Option<lex::Token>,
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

    pub fn next_token(&mut self) -> parse::Token {
        loop {
            let lex_token = match self.reserved_lex_token {
                Some(tok) => tok,
                None => self.lex.next_token(),
            };
            let start = self.pos;
            self.pos += lex_token.length;

            let kind = match lex_token.kind {
                lex::TokenKind::Whitespace => { continue }

                lex::TokenKind::Identifier => self.identifier_or_other(start, lex_token.length),

                lex::TokenKind::Semi => parse::TokenKind::Semi,
                lex::TokenKind::Colon => parse::TokenKind::Colon,
                lex::TokenKind::Comma => parse::TokenKind::Comma,
                lex::TokenKind::Dot => parse::TokenKind::Dot,
                lex::TokenKind::OpenParen => parse::TokenKind::OpenParen,
                lex::TokenKind::CloseParen => parse::TokenKind::CloseParen,
                lex::TokenKind::OpenBrace => parse::TokenKind::OpenBrace,
                lex::TokenKind::CloseBrace => parse::TokenKind::CloseBrace,
                lex::TokenKind::BSlash => parse::TokenKind::BSlash,
                lex::TokenKind::EOF => parse::TokenKind::EOF,

                lex::TokenKind::Literal{ kind } => parse::TokenKind::Literal{ kind: kind.into() },

                op @ (
                    lex::TokenKind::Bang
                  | lex::TokenKind::Lt
                  | lex::TokenKind::Gt
                  | lex::TokenKind::Pipe
                  | lex::TokenKind::And
                  | lex::TokenKind::Caret
                  | lex::TokenKind::Tilde
                  | lex::TokenKind::Eq
                  | lex::TokenKind::Plus
                  | lex::TokenKind::Minus
                  | lex::TokenKind::Star
                  | lex::TokenKind::FSlash
                  | lex::TokenKind::Percent) => self.operator(op),

                lex::TokenKind::Unknown => {
                    let end = start+lex_token.length;
                    panic!("Found Unknown Token: {} at {}:{}.", &self.src[start..end], start, end)
                }

            };
            let end = self.pos;

            return parse::Token::new(kind, start, end);
        }
    }

    fn identifier_or_other(&mut self, start: usize, length: usize) -> parse::TokenKind {
        let lexeme = &self.src[start..start+length];
        match lexeme {
            "fn" => parse::TokenKind::Fn,
            "struct" => parse::TokenKind::Struct,
            "enum" => parse::TokenKind::Enum,
            "let" => parse::TokenKind::Let,
            "if" => parse::TokenKind::If,
            "else" => parse::TokenKind::Else,

            "true" => parse::TokenKind::Literal { kind: LiteralKind::Bool },
            "false" => parse::TokenKind::Literal { kind: LiteralKind::Bool },

            _ => parse::TokenKind::Identifier
        }
    }

    fn operator(&mut self, op: lex::TokenKind) -> parse::TokenKind {
        let mut peek = self.lex.next_token();
        let op = match op {
            // If these guys pass, it implies that peek in this comment's scope has been consumed
            // successfully so we want to advance the parser's position.
            // We can also just early return on these guys since there's no other lex token that
            // can come after and still be a valid token.

            // `->`
            lex::TokenKind::Minus if peek.kind == lex::TokenKind::Gt => {
                self.pos += peek.length;
                return parse::TokenKind::RArrow;
            }

            // `||` 
            c @ lex::TokenKind::Pipe if peek.kind == c => {
                // We can just return here since there's no other lex token that can come after and
                // still be  a valid
                self.pos += peek.length;
                return parse::TokenKind::BooleanOp { kind: BooleanOpKind::Or };
            },

            // `&&`
            c @ lex::TokenKind::And if peek.kind == c => {
                self.pos += peek.length;
                return parse::TokenKind::BooleanOp { kind: BooleanOpKind::And };
            },

            // `==`
            c @ lex::TokenKind::Eq if peek.kind == c => {
                self.pos += peek.length;
                return parse::TokenKind::BooleanOp { kind: BooleanOpKind::Eq };
            },

            // `<<`
            // Since these guys have already consumed the first peek token, we get the next token
            // for later one when we check if it's a bit shift assignment.
            c @ lex::TokenKind::Lt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitLeft }
            }

            // `>>`
            c @ lex::TokenKind::Gt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitRight }
            }


            // Boolean 
            lex::TokenKind::Bang => parse::TokenKind::BooleanOp { kind: BooleanOpKind::Not },
            lex::TokenKind::Lt => parse::TokenKind::BooleanOp { kind: BooleanOpKind::Lt },
            lex::TokenKind::Gt => parse::TokenKind::BooleanOp { kind: BooleanOpKind::Gt },


            // Arithmetic Operators
            lex::TokenKind::Pipe => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitOr },
            lex::TokenKind::And => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitAnd },
            lex::TokenKind::Caret => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitXor },
            lex::TokenKind::Tilde => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitNot },

            lex::TokenKind::Plus => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Add },
            lex::TokenKind::Minus => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Sub },
            lex::TokenKind::Star => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mul },
            lex::TokenKind::FSlash => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Div },
            lex::TokenKind::Percent => parse::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mod },

            // Normal Assign
            // We can early return here since there's nothing else after an `=` that can make a
            // valid token.
            lex::TokenKind::Eq => { return parse::TokenKind::Assignment },

            // Since this match statement addresses all the productions in the pattern from the
            // outer call, this should be unreachable.
            _ => unreachable!(),
        };

        if let parse::TokenKind::BooleanOp { kind } = op {
            if peek.kind == lex::TokenKind::Eq {
                self.pos += peek.length;
                return match kind {
                    BooleanOpKind::Not => parse::TokenKind::BooleanOp { kind: BooleanOpKind::Ne },
                    BooleanOpKind::Gt => parse::TokenKind::BooleanOp { kind: BooleanOpKind::Ge },
                    BooleanOpKind::Lt => parse::TokenKind::BooleanOp { kind: BooleanOpKind::Le },

                    // `||` and `&&` don't end in `=`, `==` is handled in the previous match
                    // statement, `!=, `>=`, and `<=` are what we're checking for.
                    // THESE PATTERNS SHOULD BE UNREACHABLE.
                    _ => unreachable!()
                }
            }
        } else if let parse::TokenKind::ArithmeticOp { kind } = op {
            // Check if it's a compound assignment
            if peek.kind == lex::TokenKind::Eq {
                self.pos += peek.length;
                return parse::TokenKind::CompoundAssign { kind };
            }
        }

        // Since this wasn't sucessfully consumed, we reserve it for the next time next_token()
        // gets called.
        self.reserved_lex_token = Some(peek);
        op
    }

}
