use crate::parse::Parser;
use crate::parse::ParseError;

use crate::ast::token::TokenKind;
use crate::ast::Statement;
use crate::ast::{FunctionStatement, StructStatement, EnumStatement, LetStatement};

impl<'src> Parser<'src> {
    pub(super) fn parse_statement(&mut self) -> Result<Statement, ParseError> {
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
                    Ok(Statement::Expression { 
                        expr,
                        has_semi: self.bump_check(TokenKind::Semi)
                    })
                },
                Err(err) => Err(err)
            }
        }
    }

    pub(super) fn parse_function(&mut self) -> Result<FunctionStatement, ParseError> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        let arguments = self.parse_params(TokenKind::OpenParen, TokenKind::CloseParen)?;

        self.bump_expect(TokenKind::RArrow)?;

        let return_type = self.parse_type()?;

        // CONVERT
        if let Some(block) = self.parse_block() {
            let name = self.get_lexeme(name);
            return Ok(FunctionStatement { name, return_type, arguments, block });
        }
        println!("ERROR: expected a block expression after function declaration");
        Err(ParseError::ExpectedSingle { expected: TokenKind::EOF, found: TokenKind::EOF })
    }

    pub(super) fn parse_struct(&mut self) -> Result<StructStatement, ParseError> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        let fields = self.parse_params(TokenKind::OpenBrace, TokenKind::CloseBrace)?;

        let name = self.get_lexeme(name);
        Ok(StructStatement { name, fields })
    }

    pub(super) fn parse_enum(&mut self) -> Result<EnumStatement, ParseError> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        self.bump_expect(TokenKind::OpenBrace)?;

        let mut variants = Vec::new();

        loop {
            match self.peek(0).kind {
                TokenKind::Identifier => {
                    let variant = self.take();
                    let variant = self.get_lexeme(variant);
                    variants.push(variant);
                },

                TokenKind::CloseBrace => {
                    self.bump();
                    break
                },

                _ => {
                    let err = ParseError::ExpectedAlternatives {
                        expected: [TokenKind::Identifier, TokenKind::CloseBrace].into(),
                        found: self.peek(0).kind,
                    };
                    return Err(err);
                }
            }
        }

        // There is no need to check for `TokenKind::CloseBrace` here since the only way to break
        // out of the loop is with the parser finding a `TokenKind::CloseBrace` in the loop which
        // then bumps it.
        
        let name = self.get_lexeme(name);
        Ok(EnumStatement { name, variants })
    }

    pub(super) fn parse_let(&mut self) -> Result<LetStatement, ParseError> {
        self.bump();

        let name = self.take_expect(TokenKind::Identifier)?;
        self.bump_expect(TokenKind::OpenBrace)?;
        let value = self.parse_expr(0)?;
        self.bump_expect(TokenKind::Semi)?;

        let name = self.get_lexeme(name);

        Ok(LetStatement { name, value })
    }
}
