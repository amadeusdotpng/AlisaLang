pub mod stream;
mod statement;
mod expression;
mod types;

use crate::parse::stream::TokenStream;

use crate::ast::token::{Token, TokenKind};
use crate::ast::Statement;
use crate::ast::ASTree;

pub struct Parser<'src> {
    pub src: &'src str,
    pub stream: TokenStream,
    token: Token,

    errors: Vec<ParseError>
}

impl<'src> Parser<'src> {
    pub(self) fn new(input: &'src str) -> Parser {
        let mut stream = TokenStream::new(input);
        let tok = stream.next_token();
        Self { 
            src: input,
            stream,
            token: tok,
            errors: Vec::new(),
        }
    }

    // Advances the token stream and returns the current token.
    pub(self) fn take(&mut self) -> Token {
        let tok = self.token;
        self.token = self.stream.next_token();
        tok
    }

    // Advances the token stream without returning anything.
    pub(self) fn bump(&mut self) {
        self.token = self.stream.next_token();
    }

    // Checks whether the current token matches `tok`.
    pub(self) fn check(&self, tok: TokenKind) -> bool {
        self.token.kind == tok
    }

    // Advances the token stream and returns the current token if the next token matches `tok`.
    pub(self) fn take_check(&mut self, tok: TokenKind) -> Option<Token> {
        if self.check(tok) {
            return Some(self.take());
        }
        None
    }

    // Advances the token stream and returns if the next token matches `tok`, otherwise returns a
    // ParseError.
    pub(self) fn take_expect(&mut self, tok: TokenKind) -> Result<Token, ParseError> {
        if self.check(tok) {
            return Ok(self.take());
        }
        let err = ParseError::ExpectedSingle { expected: tok, found: self.peek(0).kind };
        Err(err)
    }

    // Advances the token stream and returns if the next token matches `tok`.
    pub(self) fn bump_check(&mut self, tok: TokenKind) -> bool {
        if self.check(tok) {
            self.bump();
            return true;
        }
        false
    }

    // Advances the token stream if the next token matches `tok`, otherwise returns a ParseError.
    pub(self) fn bump_expect(&mut self, tok: TokenKind) -> Result<(), ParseError> {
        if self.check(tok) {
            self.bump();
            return Ok(());
        }
        let err = ParseError::ExpectedSingle { expected: tok, found: self.peek(0).kind };
        Err(err)
    }

    // Advances the token stream while the given predicate is true.
    pub(self) fn bump_while(&mut self, mut predicate: impl FnMut(TokenKind) -> bool) {
        while predicate(self.token.kind) {
            self.bump();
        }
    }

    //
    pub(self) fn bump_recover(&mut self, tok: TokenKind) {
        if let Err(err) = self.bump_expect(tok) {
            self.errors.push(err)
        }
    }

    // Peeks `dist` tokens ahead. If dist is zero, we just return the current token we're storing.
    pub(self) fn peek(&mut self, dist: usize) -> Token {
        if dist == 0 {
            return self.token
        }

        let mark = self.stream.mark();
        for _ in 0..(dist) {
            self.bump();
        }

        let tok = self.take();
        self.stream.reset(mark);
        tok
    }

    pub(self) fn get_lexeme(&mut self, tok: Token) -> String {
        String::from(&self.src[tok.start..tok.end])
    }

    pub(self) fn recover_error(&mut self, err: ParseError) {
        self.errors.push(err);
    }

    pub fn parse(input: &'src str) -> ASTree {
        let mut parser = Parser::new(input);
        let mut statements = Vec::new();
        loop {
            match parser.parse_statement() {
                Ok(Statement::EOF) => break,
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    parser.errors.push(err);
                    break
                }
            }
        }

        // TODO: add actual error reporting.
        for statement in &statements {
            let Statement::Expression { expr: _, has_semi } = statement else { continue };
            if !has_semi { 
                parser.errors.push(ParseError::OuterExpression);
                /*
                println!("ERROR: expected `;` at the end of expression\n\
                          \tnote: expressions are not allowed in the outer most block") 
                */
            }
        }

        for err in &parser.errors {
            println!("{:?}", err);
        }

        // TODO: add actual error reporting.
        if !parser.bump_check(TokenKind::EOF) {
            println!("ERROR: parser did not reach end of file!");
        }

        ASTree::new(statements)
    }
}

#[derive(Debug)]
pub(self) enum ParseError {
    ExpectedSingle{expected: TokenKind, found: TokenKind},
    ExpectedAlternatives{expected: Box<[TokenKind]>, found: TokenKind},
    ExpectedNode{expected: String, found: TokenKind},
    OuterExpression,
}

#[cfg(test)]
mod tests;
