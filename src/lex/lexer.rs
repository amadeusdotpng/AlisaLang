use std::str::Chars;

pub(crate) struct Lexer<'a> {
    length_remaining: usize,
    chars: Chars<'a>,
}

const EOF_CHAR: char = '\0';

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            length_remaining: input.len(),
            chars: input.chars(),
        }
    }

    pub fn peek_first(&mut self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    pub fn peek_second(&mut self) -> char {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next().unwrap_or(EOF_CHAR)
    }

    pub fn is_eof(&mut self) -> bool {
        self.chars.as_str().is_empty()
    }

    pub fn tok_length(&mut self) -> usize {
        self.length_remaining - self.chars.as_str().len()
    }

    pub fn set_length(&mut self) {
        self.length_remaining = self.chars.as_str().len();
    }

    pub fn take(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn take_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek_first()) && !self.is_eof() {
            self.take();
        }
    }
}
