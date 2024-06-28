use crate::parse::{Parser, ParseError, ParseResult};

use crate::ast::token::T;
use crate::ast::token::TokenKind;
use crate::ast::Statement;
use crate::ast::{FunctionStatement, StructStatement, EnumStatement, LetStatement};

impl<'src> Parser<'src> {
    pub(super) fn parse_statement(&mut self) -> ParseResult<Statement> {
        match self.peek(0).kind {
            T!("fn") => {
                let item = self.parse_function()?;
                Ok(Statement::Function(item))
            }

            T!("struct") => {
                let item = self.parse_struct()?;
                Ok(Statement::Struct(item))
            }

            T!("enum") => {
                let item = self.parse_enum()?;
                Ok(Statement::Enum(item))
            }
            
            T!("let") => {
                let item = self.parse_let()?;
                Ok(Statement::Let(item))
            }

            T!("EOF") => Ok(Statement::EOF),

            _ => match self.parse_expr(0) {
                Ok(expr) => {
                    let end_token = self.peek(0);
                    if end_token.kind == T!(";") { self.bump() };
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

        let name = self.take_expect(T!("ID"))?;
        let arguments = self.parse_params(T!("("), T!(")"))?;

        self.bump_expect(T!("->"))?;

        let return_type = self.parse_type()?;

        let block = self.parse_block()?;
        let name = self.get_lexeme(name);

        Ok(FunctionStatement { name: name.into(), return_type, arguments, block })
    }

    pub(super) fn parse_struct(&mut self) -> ParseResult<StructStatement> {
        self.bump();

        let name = self.take_expect(T!("ID"))?;
        let fields = self.parse_params(T!("{"), T!("}"))?;

        let name = self.get_lexeme(name);
        Ok(StructStatement { name: name.into(), fields })
    }

    pub(super) fn parse_enum(&mut self) -> ParseResult<EnumStatement> {
        self.bump();

        let name = self.take_expect(T!("ID"))?;
        self.bump_expect(T!("{"))?;

        let mut variants = Vec::new();

        loop {
            match self.peek(0).kind {
                T!("ID") => {
                    let variant = self.take();
                    let variant = String::from(self.get_lexeme(variant));
                    variants.push(variant);
                },

                T!("}") => {
                    self.bump();
                    break
                },

                _ => {
                    let err = ParseError::ExpectedAlternatives {
                        expected: [T!("ID"), T!("}")].into(),
                        found: self.peek(0),
                    };
                    return Err(err);
                }
            }
        }

        // There is no need to check for `T!(CloseBrace)` here since the only way to break
        // out of the loop is with the parser finding a `T!(CloseBrace)` in the loop which
        // then bumps it.
        let name = self.get_lexeme(name);
        Ok(EnumStatement { name: name.into(), variants })
    }

    pub(super) fn parse_let(&mut self) -> ParseResult<LetStatement> {
        self.bump();

        let name = self.take_expect(T!("ID"))?;
        self.bump_expect(T!("="))?;
        let value = self.parse_expr(0)?;
        self.bump_expect(T!(";"))?;

        let name = self.get_lexeme(name);

        Ok(LetStatement { name: name.into(), value })
    }
}
