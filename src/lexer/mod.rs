// TODO: Make lexer emit error messages

pub mod lex;
pub mod token;

use lex::Lexer;
use token::{LiteralKind, Token, TokenKind};

// Most of this is straight out of the rust compiler.
pub fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        // Usual ASCII suspects
        '\u{0009}'   // \t
        | '\u{000A}' // \n
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{000D}' // \r
        | '\u{0020}' // space

        // NEXT LINE from latin1
        | '\u{0085}'

        // Bidi markers
        | '\u{200E}' // LEFT-TO-RIGHT MARK
        | '\u{200F}' // RIGHT-TO-LEFT MARK

        // Dedicated whitespace characters from Unicode
        | '\u{2028}' // LINE SEPARATOR
        | '\u{2029}' // PARAGRAPH SEPARATOR
    )
}

pub fn is_identifier_start(c: char) -> bool {
    c == '_' || c.is_ascii_alphabetic()
}

pub fn is_digit(c: char) -> bool {
    c == '_' || c.is_ascii_digit()
}

impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> Token {
        let c = match self.take() {
            Some(c) => c,
            None => {
                let (start, end) = self.get_pos();
                return Token::new(TokenKind::EOF, start, end)
            }
        };
        let kind = match c {
            // Skips through whitespace.
            c if is_whitespace(c) => self.whitespace(),

            // Identifiers can't start with a digit.
            c if is_identifier_start(c) => self.identifier(),

            // Dot or Float Literal
            '.' => {
                // If the next thing is a number, it's a float for sure.
                if self.peek_first().is_ascii_digit() {
                    // Keep taking until end of float.
                    self.take_while(is_digit);
                    TokenKind::Literal {
                        kind: LiteralKind::Float,
                    }
                } else {
                    TokenKind::Dot
                }
            }

            // Integer or Float Literal
            '0'..='9' => {
                // Skip through the rest of the digits.
                self.take_while(is_digit);

                // If the next is a dot, it might be a float or trying to access a field.
                if self.peek_first() == '.' && !is_identifier_start(self.peek_second()) {
                    // Skip the dot.
                    self.take();
                    self.take_while(is_digit);
                    TokenKind::Literal {
                        kind: LiteralKind::Float,
                    }
                // If there's no dot at all, it's just an int for sure.
                } else {
                    TokenKind::Literal {
                        kind: LiteralKind::Int,
                    }
                }
            }

            // String Literal
            '"' => {
                let terminated = self.literal_string();
                let litkind = LiteralKind::Str { terminated };
                TokenKind::Literal { kind: litkind }
            }

            // Single Character Literal
            '\'' => {
                let terminated = self.literal_char();
                let litkind = LiteralKind::Char { terminated };
                TokenKind::Literal { kind: litkind }
            }

            // Potential Double Character Tokens
            '|' => {
                match self.peek_first() {
                    '|' => { self.take(); TokenKind::BoolOr }
                    _ => TokenKind::BitOr
                }
            }
            '&' => {
                match self.peek_first() {
                    '&' => { self.take(); TokenKind::BoolAnd }
                    _ => TokenKind::BitAnd
                }
            }
            '=' => {
                match self.peek_first() {
                    '=' => { self.take(); TokenKind::Eq }
                    _ => TokenKind::Assign
                }
            }
            '<' => {
                match self.peek_first() {
                    '=' => { self.take(); TokenKind::Le }
                    _ => TokenKind::Lt
                }
                
            }
            '>' => {
                match self.peek_first() {
                    '=' => { self.take(); TokenKind::Ge }
                    _ => TokenKind::Gt
                }
            }

            // Single Character Tokens
            ';' => TokenKind::Semi,
            ':' => TokenKind::Colon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,

            '^' => TokenKind::BitXor,

            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Modulo,

            '!' => TokenKind::BoolNot,

            _ => TokenKind::Unknown,
        };
        let (start, end) = self.get_pos();
        let res = Token::new(kind, start, end);
        self.set_pos();
        res
    }

    pub fn whitespace(&mut self) -> TokenKind {
        self.take_while(is_whitespace);
        TokenKind::Whitespace
    }

    pub fn identifier(&mut self) -> TokenKind {
        self.take_while(|c: char| c == '_' || c.is_alphanumeric());
        TokenKind::Identifier
    }

    // goes through literal string and returns if it's terminated
    pub fn literal_string(&mut self) -> bool {
        while let Some(c) = self.take() {
            match c {
                '"' => return true,
                '\\' if self.peek_first() == '\\' || self.peek_first() == '"' => {
                    // skips the escape character
                    self.take();
                }
                _ => (),
            }
        }
        false
    }

    pub fn literal_char(&mut self) -> bool {
        match self.take() {
            // Empty character literals are illegal.
            // If self.take() is None, it doesn't terminate.
            Some('\'') | None => return false,

            Some(c) => {
                // Skip char after escape character if it's an escape sequence.
                if c == '\\' {
                    self.take();
                }
                if self.peek_first() == '\'' {
                    // skip "'"
                    self.take();
                    return true;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests;
