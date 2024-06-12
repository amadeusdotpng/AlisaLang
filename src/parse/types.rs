use crate::parse::Parser;
use crate::parse::ParseError;

use crate::ast::token::TokenKind;
use crate::ast::Parameter;
use crate::ast::{Type, IntKind, FloatKind, TupleType};


impl<'src> Parser<'src> {
    // #[inline]
    pub(super) fn parse_params(&mut self, open: TokenKind, close: TokenKind) -> Result<Vec<Parameter>, ParseError> {
        self.bump_expect(open)?;

        let mut parameters = Vec::new();
        let mut first_param = true;


        loop {
            if self.peek(0).kind == close { break }
            if !first_param { self.bump_recover(TokenKind::Comma); }
            first_param = false;

            let peek = self.peek(0).kind;
            let valid = matches!(peek, TokenKind::Identifier | TokenKind::Comma)
                || peek == close;

            if !valid {
                return Err(ParseError::ExpectedAlternatives {
                    expected: Box::new([TokenKind::Comma, close, TokenKind::Identifier]),
                    found: self.peek(0).kind,
                })
            }

            let parameter = match self.parse_param() {
                Ok(parameter) => parameter,
                Err(err) => {
                    self.recover_error(err);
                    self.bump_while(|kind| {
                        matches!(kind, TokenKind::Identifier | TokenKind::Colon)
                    });
                    continue
                }
            };

            parameters.push(parameter);
        }
        
        self.bump_check(TokenKind::Comma);
        self.bump_expect(close)?;

        Ok(parameters)
    }

    // #[inline]
    pub(super) fn parse_param(&mut self) -> Result<Parameter, ParseError> {
        let name = self.take_expect(TokenKind::Identifier)?;
        self.bump_expect(TokenKind::Colon)?;
        let param_type = self.parse_type()?;

        let name = self.get_lexeme(name);
        Ok(Parameter { name, param_type })
    }


    pub(super) fn parse_type(&mut self) -> Result<Type, ParseError> {
        match self.peek(0).kind {
            TokenKind::Identifier => {
                let name = self.take();
                let name = self.get_lexeme(name);
                Ok(Parser::parse_type_from_ident(&name))
            },

            TokenKind::OpenParen => self.parse_type_tuple(),
            TokenKind::OpenBracket => self.parse_type_list(),
            TokenKind::Fn => self.parse_type_fn(),
            _ => Err(ParseError::ExpectedNode {
                expected: "type".into(),
                found: self.peek(0).kind
            }),
        }
    }

    // #[inline(always)]
    pub(super) fn parse_type_tuple(&mut self) -> Result<Type, ParseError> {
        self.bump();

        let arguments = self.parse_type_args();
        self.bump_expect(TokenKind::CloseParen)?;
        Ok(Type::Tuple(TupleType(arguments)))
    }

    // #[inline(always)]
    pub(super) fn parse_type_list(&mut self) -> Result<Type, ParseError> {
        self.bump();

        let list_type = Box::new(self.parse_type()?);
        self.bump_expect(TokenKind::CloseBracket)?;

        Ok(Type::List(list_type))
    }

    // #[inline(always)]
    pub(super) fn parse_type_fn(&mut self) -> Result<Type, ParseError> {
        self.bump();

        self.bump_expect(TokenKind::OpenParen)?;
        let arguments = self.parse_type_args();
        self.bump_expect(TokenKind::CloseParen)?;
        self.bump_expect(TokenKind::RArrow)?;
        let return_type = Box::new(self.parse_type()?);

        Ok(Type::Fn { arguments, return_type })
    }

    pub(super) fn parse_type_args(&mut self) -> Vec<Type> {
        let mut first_type = true;
        let mut arguments = Vec::new();

        loop {
            if !first_type { self.bump_recover(TokenKind::Comma); }
            first_type = false;

            let type_arg = match self.parse_type() {
                Ok(type_arg) => type_arg,
                Err(_) => break,
            };

            arguments.push(type_arg);
        }

        // Optional `,` after the last type_arg
        self.bump_check(TokenKind::Comma);
        arguments
    }

    #[inline]
    pub(super) fn parse_type_from_ident(typename: &str) -> Type {
        match typename {
            "bool" => Type::Bool,
            "str" => Type::Str,
            "char" => Type::Char,

            "u8"  => Type::Int { sign: false, kind: IntKind::Bit8  },
            "u16" => Type::Int { sign: false, kind: IntKind::Bit16 },
            "u32" => Type::Int { sign: false, kind: IntKind::Bit32 },
            "u64" => Type::Int { sign: false, kind: IntKind::Bit64 },

            "i8"  => Type::Int { sign: true, kind: IntKind::Bit8  },
            "i16" => Type::Int { sign: true, kind: IntKind::Bit16 },
            "i32" => Type::Int { sign: true, kind: IntKind::Bit32 },
            "i64" => Type::Int { sign: true, kind: IntKind::Bit64 },

            "f32" => Type::Float { kind: FloatKind::Bit32 },
            "f64" => Type::Float { kind: FloatKind::Bit64 },

            "void" => Type::Void,

            _ => Type::UserDefined { name: String::from(typename) }
        }
    }
}
