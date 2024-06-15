use crate::parse::{Parser, ParseError, ParseResult};

use crate::ast::token::TokenKind;
use crate::ast::Statement;
use crate::ast::{FunctionStatement, StructStatement, EnumStatement, LetStatement};

impl<'src> Parser<'src> {
    pub(super) fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.peek(0).kind {
            TokenKind::Fn => {
                let item = self.parse_function()?;
                Ok(Statement::Function(item))
            }

            TokenKind::Struct => {
                let item = self.parse_struct()?;
                Ok(Statement::Struct(item))
            }

            TokenKind::Enum => {
                let item = self.parse_enum()?;
                Ok(Statement::Enum(item))
            }
            
            TokenKind::Let => {
                let item = self.parse_let()?;
                Ok(Statement::Let(item))
            }

            TokenKind::EOF => Ok(Statement::EOF),

            _ => match self.parse_expr(0) {
                Ok(expr) => {
                    let end_token = self.peek(0);
                    if end_token.kind == TokenKind::Semi { self.bump() };
                    Ok(Statement::Expression { 
                        expr,
                        end_token,
                    })
                },
                Err(err) => Err(err)
            }
        }
    }

    pub(super) fn parse_function(&mut self) -> ParseResult<FunctionStatement> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        let arguments = self.parse_params(TokenKind::OpenParen, TokenKind::CloseParen)?;

        self.bump_expect(TokenKind::RArrow)?;

        let return_type = self.parse_type()?;

        let block = self.parse_block()?;
        let name = self.get_lexeme(name);

        Ok(FunctionStatement { name: name.into(), return_type, arguments, block })
    }

    pub(super) fn parse_struct(&mut self) -> ParseResult<StructStatement> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        let fields = self.parse_params(TokenKind::OpenBrace, TokenKind::CloseBrace)?;

        let name = self.get_lexeme(name);
        Ok(StructStatement { name: name.into(), fields })
    }

    pub(super) fn parse_enum(&mut self) -> ParseResult<EnumStatement> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        self.bump_expect(TokenKind::OpenBrace)?;

        let mut variants = Vec::new();

        loop {
            match self.peek(0).kind {
                TokenKind::Identifier => {
                    let variant = self.take();
                    let variant = String::from(self.get_lexeme(variant));
                    variants.push(variant);
                },

                TokenKind::CloseBrace => {
                    self.bump();
                    break
                },

                _ => {
                    let err = ParseError::ExpectedAlternatives {
                        expected: [TokenKind::Identifier, TokenKind::CloseBrace].into(),
                        found: self.peek(0),
                    };
                    return Err(err);
                }
            }
        }

        // There is no need to check for `TokenKind::CloseBrace` here since the only way to break
        // out of the loop is with the parser finding a `TokenKind::CloseBrace` in the loop which
        // then bumps it.
        let name = self.get_lexeme(name);
        Ok(EnumStatement { name: name.into(), variants })
    }

    pub(super) fn parse_let(&mut self) -> ParseResult<LetStatement> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        self.bump_expect(TokenKind::Eq)?;
        let value = self.parse_expr(0)?;
        self.bump_expect(TokenKind::Semi)?;

        let name = self.get_lexeme(name);

        Ok(LetStatement { name: name.into(), value })
    }
}
