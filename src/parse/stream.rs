use crate::lex::{self, lexer::Lexer};

use crate::ast::token as ast_token;
use crate::ast::token::{LiteralKind, OpKind};

#[derive(Debug)]
pub struct TokenStream{
    pub pos: usize,
    pub tokens: Vec<ast_token::Token>,
}

impl TokenStream {
    pub fn new(input: &str) -> TokenStream {
        let mut reader = StringReader::new(input);
        let mut tokens = Vec::new();
        loop {
            let tok = reader.take();
            tokens.push(tok);
            if tok.kind == ast_token::TokenKind::EOF { break }
        }

        Self {
            pos: 0,
            tokens,
        }
    }

    // Returns the token in the current position and advances.
    pub fn next_token(&mut self) -> ast_token::Token {
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

    fn take(&mut self) -> ast_token::Token {
        loop {
            let lex_token = match self.reserved_lex_token {
                Some(tok) => {
                    self.reserved_lex_token = None;
                    tok
                },
                None => self.lex.next_token(),
            };
            let start = self.pos;
            self.pos += lex_token.length;

            let kind = match lex_token.kind {
                lex::TokenKind::Whitespace => {
                    // println!("{:?}: {}", lex_token, start); 
                    continue
                }

                lex::TokenKind::Identifier => self.identifier_or_other(start, lex_token.length),

                lex::TokenKind::Semi => ast_token::TokenKind::Semi,
                lex::TokenKind::Colon => ast_token::TokenKind::Colon,
                lex::TokenKind::Comma => ast_token::TokenKind::Comma,
                lex::TokenKind::Dot => ast_token::TokenKind::Dot,
                lex::TokenKind::OpenParen => ast_token::TokenKind::OpenParen,
                lex::TokenKind::CloseParen => ast_token::TokenKind::CloseParen,
                lex::TokenKind::OpenBrace => ast_token::TokenKind::OpenBrace,
                lex::TokenKind::CloseBrace => ast_token::TokenKind::CloseBrace,
                lex::TokenKind::OpenBracket => ast_token::TokenKind::OpenBracket,
                lex::TokenKind::CloseBracket => ast_token::TokenKind::CloseBracket,
                lex::TokenKind::BSlash => ast_token::TokenKind::BSlash,
                lex::TokenKind::EOF => ast_token::TokenKind::EOF,

                lex::TokenKind::Literal{ kind } => ast_token::TokenKind::Literal{ kind: kind.into() },

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
            // println!("{:?}: {}->{}", lex_token, start, end);
            return ast_token::Token::new(kind, start, end);
        }
    }

    fn identifier_or_other(&mut self, start: usize, length: usize) -> ast_token::TokenKind {
        let lexeme = &self.src[start..start+length];
        match lexeme {
            "fn" => ast_token::TokenKind::Fn,
            "struct" => ast_token::TokenKind::Struct,
            "enum" => ast_token::TokenKind::Enum,
            "let" => ast_token::TokenKind::Let,
            "if" => ast_token::TokenKind::If,
            "else" => ast_token::TokenKind::Else,

            "true" => ast_token::TokenKind::Literal { kind: LiteralKind::Bool },
            "false" => ast_token::TokenKind::Literal { kind: LiteralKind::Bool },

            _ => ast_token::TokenKind::Identifier
        }
    }

    fn operator(&mut self, op: lex::TokenKind) -> ast_token::TokenKind {
        let mut peek = self.lex.next_token();
        let op = match op {
            // If these guys pass, it implies that peek in this comment's scope has been consumed
            // successfully so we want to advance the parser's position.
            // We can also just early return on these guys since there's no other lex token that
            // can come after and still be a valid token.

            // `->`
            lex::TokenKind::Minus if peek.kind == lex::TokenKind::Gt => {
                self.pos += peek.length;
                return ast_token::TokenKind::RArrow;
            }

            // `||` 
            c @ lex::TokenKind::Pipe if peek.kind == c => {
                // We can just return here since there's no other lex token that can come after and
                // still be  a valid
                self.pos += peek.length;
                return ast_token::TokenKind::PipePipe
            },

            // `&&`
            c @ lex::TokenKind::And if peek.kind == c => {
                self.pos += peek.length;
                return ast_token::TokenKind::AndAnd
            },

            // `==`
            c @ lex::TokenKind::Eq if peek.kind == c => {
                self.pos += peek.length;
                return ast_token::TokenKind::EqEq
            },

            // `>=`
            lex::TokenKind::Gt if peek.kind == lex::TokenKind::Eq => {
                self.pos += peek.length;
                return ast_token::TokenKind::GtEq
            }

            // `<=`
            lex::TokenKind::Lt if peek.kind == lex::TokenKind::Eq => {
                self.pos += peek.length;
                return ast_token::TokenKind::LtEq
            }

            // `!=`
            lex::TokenKind::Bang if peek.kind == lex::TokenKind::Eq => {
                self.pos += peek.length;
                return ast_token::TokenKind::BangEq
            }

            // `<<`
            // Since these guys have already consumed the first peek token, we get the next token
            // for later one when we check if it's a bit shift assignment.
            c @ lex::TokenKind::Lt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                ast_token::TokenKind::Op { kind: OpKind::ShiftL }
            }

            // `>>`
            c @ lex::TokenKind::Gt if peek.kind == c => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                ast_token::TokenKind::Op { kind: OpKind::ShiftR }
            }

            // `|>`
            lex::TokenKind::Pipe if peek.kind == lex::TokenKind::Gt => {
                self.pos += peek.length;
                peek = self.lex.next_token();
                ast_token::TokenKind::PipeGt
            }

            // Boolean 
            lex::TokenKind::Bang => ast_token::TokenKind::Bang,
            lex::TokenKind::Lt => ast_token::TokenKind::Lt,
            lex::TokenKind::Gt => ast_token::TokenKind::Gt,


            // Arithmetic Operators
            lex::TokenKind::Pipe => ast_token::TokenKind::Op { kind: OpKind::Pipe },
            lex::TokenKind::And => ast_token::TokenKind::Op { kind: OpKind::And },
            lex::TokenKind::Caret => ast_token::TokenKind::Op { kind: OpKind::Caret },
            lex::TokenKind::Tilde => ast_token::TokenKind::Op { kind: OpKind::Tilde },

            lex::TokenKind::Plus => ast_token::TokenKind::Op { kind: OpKind::Plus },
            lex::TokenKind::Minus => ast_token::TokenKind::Op { kind: OpKind::Minus },
            lex::TokenKind::Star => ast_token::TokenKind::Op { kind: OpKind::Star },
            lex::TokenKind::FSlash => ast_token::TokenKind::Op { kind: OpKind::FSlash },
            lex::TokenKind::Percent => ast_token::TokenKind::Op { kind: OpKind::Percent },

            // Normal Assign
            // We can early return here since there's nothing else after an `=` that can make a
            // valid token.
            lex::TokenKind::Eq => {
                self.reserved_lex_token = Some(peek);
                return ast_token::TokenKind::Eq
            },

            // Since this match statement addresses all the productions in the pattern from the
            // outer call, this should be unreachable.
            _ => unreachable!(),
        };

        if let ast_token::TokenKind::Op { kind } = op {
            // Check if it's a compound assignment
            if peek.kind == lex::TokenKind::Eq {
                self.pos += peek.length;
                return ast_token::TokenKind::OpEq { kind };
            }
        }

        // Since this wasn't sucessfully consumed, we reserve it for the next time next_token()
        // gets called.
        self.reserved_lex_token = Some(peek);
        op
    }
}
