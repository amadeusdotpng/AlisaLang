use crate::parse::stream::TokenStream;
use crate::ast::{Token, TokenKind};

pub(crate) struct Parser<'src> {
    pub src: &'src str,
    stream: TokenStream,
    token: Token,
}

impl<'src> Parser<'src> {
    pub(super) fn new(input: &'src str) -> Parser {
        let mut stream = TokenStream::new(input);
        let tok = stream.next_token();
        Self { 
            src: input,
            stream,
            token: tok,
        }
    }

    // Advances the token stream and returns the current token.
    pub(super) fn take(&mut self) -> Token {
        let tok = self.token;
        self.token = self.stream.next_token();
        tok
    }

    // Advances the token stream without returning anything.
    pub(super) fn bump(&mut self) {
        self.token = self.stream.next_token();
    }

    // Checks whether the current token matches `tok`.
    pub(super) fn check(&self, tok: TokenKind) -> bool {
        self.token.kind == tok
    }

    // Advances the token stream and returns the current token if the next token matches `tok`.
    pub(super) fn take_check(&mut self, tok: TokenKind) -> Option<Token> {
        if self.check(tok) {
            return Some(self.take());
        }
        None
    }

    // Advances the token stream and returns if the next token matches `tok`.
    pub(super) fn bump_check(&mut self, tok: TokenKind) -> bool {
        if self.check(tok) {
            self.bump();
            return true;
        }
        false
    }

    pub(super) fn bump_while(&mut self, mut predicate: impl FnMut(TokenKind) -> bool) {
        while predicate(self.token.kind) {
            self.bump();
        }
    }

    /*
    pub(super) fn bump_check_n(&mut self, tok: &[TokenKind]) -> bool {
        false
    }
    */

    // Peeks `dist` tokens ahead. If dist is zero, we just return the current token we're storing.
    pub(super) fn peek(&mut self, dist: usize) -> Token {
        if dist == 0 {
            return self.token
        }

        let mark = self.stream.mark();
        for _ in 0..(dist-1) {
            self.bump();
        }

        let tok = self.take();
        self.stream.reset(mark);
        tok
    }

    /*
    pub(super) fn expect_n(&'a mut self, kinds: &[TokenKind]) -> Option<Vec<Token>> {
        todo!()
    }
    */
}
