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
            // Peek current token and token ahead to check if it's the closing delimiter.
            let (peek_0, peek_1) = (self.peek(0).kind, self.peek(1).kind);
            if peek_0 == close || peek_1 == close { break }

            let err = if !first_param { 
                self.bump_expect(TokenKind::Comma)
            } else { first_param = false; Ok(()) };

            // After we check if there's a comma, we want to check if the next token is an
            // Identifier or a Comma since those two things are the only two things that should be
            // coming after the opening delimiter or a successful parameter parse.
            let peek = self.peek(0);
            if !matches!(peek.kind, TokenKind::Identifier | TokenKind::Comma) {
                // If it's not one of those two things, we know for sure parsing the parameter list
                // should be over. If there wasn't an error with checking the comma, we just say
                // that we're expecting the closing delimiter. If there was an issue, we say that
                // we expected a comma OR the closing delimiter.
                let err = match err.is_ok() {
                    true => ParseError::ExpectedSingle { expected: close, found: peek },
                    false => ParseError::ExpectedAlternatives { expected: Box::new([TokenKind::Comma, close]), found: peek },
                };
                return Err(err);
            }

            if let Err(err) = err {
                self.recover_error(err);
            }

            let parameter = match self.parse_param() {
                Ok(parameter) => parameter,
                Err(err) => {
                    self.bump_while(|kind| {
                        matches!(kind, TokenKind::Identifier | TokenKind::Colon)
                    });

                    let peek = self.peek(0);
                    if !matches!(peek.kind, TokenKind::Identifier | TokenKind::Comma) {
                        return Err(err);
                    }
                    self.recover_error(err);
                    continue
                }
            };

            parameters.push(parameter);
        }
        
        // Optional `,` after the last type_arg
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
                found: self.peek(0)
            }),
        }
    }

    // #[inline(always)]
    pub(super) fn parse_type_tuple(&mut self) -> Result<Type, ParseError> {
        let arguments = self.parse_type_args(TokenKind::OpenParen, TokenKind::CloseParen)?;
        Ok(Type::Tuple(TupleType(arguments)))
    }

    // #[inline(always)]
    pub(super) fn parse_type_list(&mut self) -> Result<Type, ParseError> {
        self.bump();

        let list_type = Box::new(self.parse_type()?);
        self.bump_recover(TokenKind::CloseBracket);

        Ok(Type::List(list_type))
    }

    // #[inline(always)]
    pub(super) fn parse_type_fn(&mut self) -> Result<Type, ParseError> {
        self.bump();

        let arguments = self.parse_type_args(TokenKind::OpenParen, TokenKind::CloseParen)?;

        self.bump_expect(TokenKind::RArrow)?;

        let return_type = Box::new(self.parse_type()?);

        Ok(Type::Fn { arguments, return_type })
    }

    // See parse_params()
    pub(super) fn parse_type_args(&mut self, open: TokenKind, close: TokenKind) -> Result<Vec<Type>, ParseError> {
        self.bump_expect(open)?;

        let mut first_type = true;
        let mut arguments = Vec::new();

        loop {
            let (peek_0, peek_1) = (self.peek(0).kind, self.peek(1).kind);
            if peek_0 == close || (peek_1 == close && !first_type) { break }

            let err = if !first_type { 
                self.bump_expect(TokenKind::Comma)
            } else { first_type = false; Ok(()) };

            let peek = self.peek(0);
            let valid = matches!(
                peek.kind,

                TokenKind::Comma
                | TokenKind::OpenParen
                | TokenKind::OpenBracket
                | TokenKind::Fn
                | TokenKind::Identifier
            );

            if !valid {
                let err = match err.is_ok() {
                    true => ParseError::ExpectedSingle { expected: close, found: peek },
                    false => ParseError::ExpectedAlternatives { expected: Box::new([TokenKind::Comma, close]), found: peek },
                };
                return Err(err);
            }

            if let Err(err) = err {
                self.recover_error(err);
            }

            let type_arg = match self.parse_type() {
                Ok(type_arg) => type_arg,
                Err(err) => {
                    self.bump_while(|kind| {
                        matches!(kind, TokenKind::Identifier)
                    });
                    let peek = self.peek(0);
                    let valid = matches!(
                        peek.kind,

                        TokenKind::Comma
                        | TokenKind::OpenParen
                        | TokenKind::OpenBracket
                        | TokenKind::Fn
                        | TokenKind::Identifier
                    );
                    if !valid {
                        return Err(err);
                    }
                    self.recover_error(err);
                    continue
                },
            };

            arguments.push(type_arg);
        }

        self.bump_check(TokenKind::Comma);
        self.bump_expect(close)?;
        Ok(arguments)
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
