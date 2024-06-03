use std::collections::VecDeque;

use crate::parse::{Token, TokenKind, stream::TokenStream};

pub(crate) struct Parser<'a> {
    stream: TokenStream<'a>,
    lookahead: VecDeque<Token>,
    // lookahead: Option<Token>,
}

impl<'a> Parser<'a> {
    pub(super) fn new(input: &'a str) -> Parser {
        Parser { 
            stream: TokenStream::new(input),
            lookahead: VecDeque::new(),
        }
    }

    pub(super) fn next(&mut self) -> Token {
        match self.lookahead.pop_front() {
            Some(token) => token,
            None => self.stream.next_token()
        }
    }

    pub(super) fn peek(&mut self) -> Token {
        match self.lookahead.front() {
            Some(token) => *token,
            None => {
                let token = self.stream.next_token();
                self.lookahead.push_back(token);
                token
            }
        }
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> Option<Token> {
        if self.peek().kind == kind {
            Some(self.next())
        } else { 
            None
        }
    }

    pub(super) fn peek_n(&mut self, n: usize) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(n);
        for i in 0..n {
            let token = match self.lookahead.get(i) {
                Some(&token) => token,
                None => {
                    let token = self.stream.next_token();
                    self.lookahead.push_back(token);
                    token
                }
            };
            tokens.push(token)
        }
        tokens
    }

    pub(super) fn expect_n(&mut self, kinds: &[TokenKind]) -> Option<Vec<Token>> {
        let n = kinds.len();
        let mut tokens = Vec::with_capacity(n);
        for (tok, &kind) in self.peek_n(n).into_iter().zip(kinds.iter()) {
            if tok.kind != kind {
                return None
            }
        }

        // Since it's over, we can go ahead and iterate over these guys that we've peeked since we
        // know that it's all good.
        for _ in 0..n { 
            tokens.push(self.next());
        }

        Some(tokens)
    }
}
