use crate::lex::{self, lexer::Lexer};

use crate::ast;
use crate::ast::{LiteralKind, BooleanOpKind, ArithmeticOpKind};

#[derive(Debug)]
pub struct TokenStream{
    pub pos: usize,
    tokens: Vec<ast::Token>,
}

impl TokenStream {
    pub fn new(input: &str) -> TokenStream {
        let mut reader = StringReader::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = reader.take();
            tokens.push(tok);
            if tok.kind == ast::TokenKind::EOF { break }
        }

        Self {
            pos: 0,
            tokens,
        }
    }

    // Returns the token in the current position and advances.
    pub fn next_token(&mut self) -> ast::Token {
        match self.tokens.get(self.pos) {
            Some(&token) => {
                self.pos += 1;
                token
            }

            None => {
                match self.tokens.last() {
                    Some(&token) => token,
                    None => unreachable!("tokens should never be empty!"),
                }
            }
        }
    }

    pub fn mark(&mut self) -> usize {
        self.pos
    }

    pub fn reset(&mut self, pos: usize) {
        self.pos = pos;
    }
}

#[derive(Clone, Debug)]
struct StringReader<'a> {
    src: &'a str,
    lex: Lexer<'a>,
    pos: usize,

    // This is used for trying to convert double-character tokens
    // but it turns out it's not a double-character token
    reserved_lex_token: Option<lex::Token>,
}


impl<'a> StringReader<'a> {
    fn new(input: &'a str) -> StringReader {
        Self { 
            src: input,
            lex: Lexer::new(input),
            pos: 0,
            reserved_lex_token: None,
        }
    }

    fn take(&mut self) -> ast::Token {
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

                lex::TokenKind::Semi => ast::TokenKind::Semi,
                lex::TokenKind::Colon => ast::TokenKind::Colon,
                lex::TokenKind::Comma => ast::TokenKind::Comma,
                lex::TokenKind::Dot => ast::TokenKind::Dot,
                lex::TokenKind::OpenParen => ast::TokenKind::OpenParen,
                lex::TokenKind::CloseParen => ast::TokenKind::CloseParen,
                lex::TokenKind::OpenBrace => ast::TokenKind::OpenBrace,
                lex::TokenKind::CloseBrace => ast::TokenKind::CloseBrace,
                lex::TokenKind::OpenBracket => ast::TokenKind::OpenBracket,
                lex::TokenKind::CloseBracket => ast::TokenKind::CloseBracket,
                lex::TokenKind::BSlash => ast::TokenKind::BSlash,
                lex::TokenKind::EOF => ast::TokenKind::EOF,

                lex::TokenKind::Literal{ kind } => ast::TokenKind::Literal{ kind: kind.into() },

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
                    panic!("Found Unknown Token: {} at {} -> {}.", &self.src[start..end], start, end)
                }

            };
            let end = self.pos;
            return ast::Token::new(kind, start, end);
        }
    }

    fn identifier_or_other(&mut self, start: usize, length: usize) -> ast::TokenKind {
        let lexeme = &self.src[start..start+length];
        match lexeme {
            "fn" => ast::TokenKind::Fn,
            "struct" => ast::TokenKind::Struct,
            "enum" => ast::TokenKind::Enum,
            "let" => ast::TokenKind::Let,
            "if" => ast::TokenKind::If,
            "else" => ast::TokenKind::Else,

            "true" => ast::TokenKind::Literal { kind: LiteralKind::Bool },
            "false" => ast::TokenKind::Literal { kind: LiteralKind::Bool },

            _ => ast::TokenKind::Identifier
        }
    }

    fn operator(&mut self, op: lex::TokenKind) -> ast::TokenKind {
        let mut peek = self.lex.next_token();
        let op = match op {
            // If these guys pass, it implies that peek in this comment's scope has been consumed
            // successfully so we want to advance the parser's position.
            // We can also just early return on these guys since there's no other lex token that
            // can come after and still be a valid token.

            // `->`
            lex::TokenKind::Minus if peek.kind == lex::TokenKind::Gt => {
                self.pos += peek.length;
                return ast::TokenKind::RArrow;
            }

            // `||` 
            c @ lex::TokenKind::Pipe if peek.kind == c => {
                // We can just return here since there's no other lex token that can come after and
                // still be  a valid
                self.pos += peek.length;
                return ast::TokenKind::BooleanOp { kind: BooleanOpKind::Or };
            },

            // `&&`
            c @ lex::TokenKind::And if peek.kind == c => {
                self.pos += peek.length;
                return ast::TokenKind::BooleanOp { kind: BooleanOpKind::And };
            },

            // `==`
            c @ lex::TokenKind::Eq if peek.kind == c => {
                self.pos += peek.length;
                return ast::TokenKind::BooleanOp { kind: BooleanOpKind::Eq };
            },

            // `<<`
            // Since these guys have already consumed the first peek token, we get the next token
            // for later one when we check if it's a bit shift assignment.
            c @ lex::TokenKind::Lt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitLeft }
            }

            // `>>`
            c @ lex::TokenKind::Gt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitRight }
            }


            // Boolean 
            lex::TokenKind::Bang => ast::TokenKind::BooleanOp { kind: BooleanOpKind::Not },
            lex::TokenKind::Lt => ast::TokenKind::BooleanOp { kind: BooleanOpKind::Lt },
            lex::TokenKind::Gt => ast::TokenKind::BooleanOp { kind: BooleanOpKind::Gt },


            // Arithmetic Operators
            lex::TokenKind::Pipe => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitOr },
            lex::TokenKind::And => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitAnd },
            lex::TokenKind::Caret => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitXor },
            lex::TokenKind::Tilde => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::BitNot },

            lex::TokenKind::Plus => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Add },
            lex::TokenKind::Minus => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Sub },
            lex::TokenKind::Star => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mul },
            lex::TokenKind::FSlash => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Div },
            lex::TokenKind::Percent => ast::TokenKind::ArithmeticOp { kind: ArithmeticOpKind::Mod },

            // Normal Assign
            // We can early return here since there's nothing else after an `=` that can make a
            // valid token.
            lex::TokenKind::Eq => { return ast::TokenKind::Assignment },

            // Since this match statement addresses all the productions in the pattern from the
            // outer call, this should be unreachable.
            _ => unreachable!(),
        };

        if let ast::TokenKind::BooleanOp { kind } = op {
            if peek.kind == lex::TokenKind::Eq {
                self.pos += peek.length;
                return match kind {
                    BooleanOpKind::Not => ast::TokenKind::BooleanOp { kind: BooleanOpKind::Ne },
                    BooleanOpKind::Gt => ast::TokenKind::BooleanOp { kind: BooleanOpKind::Ge },
                    BooleanOpKind::Lt => ast::TokenKind::BooleanOp { kind: BooleanOpKind::Le },

                    // `||` and `&&` don't end in `=`, `==` is handled in the previous match
                    // statement, `!=, `>=`, and `<=` are what we're checking for.
                    // THESE PATTERNS SHOULD BE UNREACHABLE.
                    _ => unreachable!()
                }
            }
        } else if let ast::TokenKind::ArithmeticOp { kind } = op {
            // Check if it's a compound assignment
            if peek.kind == lex::TokenKind::Eq {
                self.pos += peek.length;
                return ast::TokenKind::CompoundAssign { kind };
            }
        }

        // Since this wasn't sucessfully consumed, we reserve it for the next time next_token()
        // gets called.
        self.reserved_lex_token = Some(peek);
        op
    }
}
