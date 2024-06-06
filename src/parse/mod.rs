pub mod parser;
pub mod stream;

use parser::Parser;
use crate::ast::TokenKind;
use crate::ast::node::{Statement, Expression};
use crate::ast::node::{Parameter, Type, IntKind, FloatKind, TupleType};
use crate::ast::node::{FunctionStatement, StructStatement, EnumStatement, LetStatement};
use crate::ast::node::{ClosureExpression, CallExpression, BlockExpression, IfExpression};
use crate::ast::ASTree;

pub enum Nt {
    Statement,
    Expression,
    
    Function,
    Struct,
    Enum,
    Let,

    Closure,
    Block,
    If,
}

impl<'src> Parser<'src> {

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek(0).kind {
            TokenKind::Fn => {
                match self.parse_function() {
                    Some(item) => Some(Statement::Function(item)),
                    None => None,
                }
            }
            TokenKind::Struct => {
                match self.parse_struct() {
                    Some(item) => Some(Statement::Struct(item)),
                    None => None,
                }
            }
            TokenKind::Enum => {
                match self.parse_enum() {
                    Some(item) => Some(Statement::Enum(item)),
                    None => None,
                }
            }
            TokenKind::Let => {
                match self.parse_let() {
                    Some(item) => Some(Statement::Let(item)),
                    None => None
                }
            }

            _ => None
            /*
            _ => {
                match self.parse_expr() {
                    Some(expr) => {
                        // TODO: add actual error reporting.
                        if !self.bump_check(TokenKind::Semi) {
                            println!("ERROR: missing semicolon at {}", self.stream.pos)
                        }
                        Some(Statement::Expression(expr))
                    }
                    None => None
                }
            }
            */
        }
    }

    fn parse_function(&mut self) -> Option<FunctionStatement> {
        self.bump();
        if let Some(token) = self.take_check(TokenKind::Identifier) {

            // TODO: add actual error reporting.
            if !self.bump_check(TokenKind::OpenParen) {
                println!("ERROR: expected `(` in function declaration");
            }

            let arguments = match self.parse_params(TokenKind::CloseParen) {
                Some(args) => args,
                None => return None,
            };

            // TODO: add actual error reporting.
            if !self.bump_check(TokenKind::CloseParen) {
                println!("ERROR: expected `)` in function declaration");
            }

            // TODO: add actual error reporting.
            if !self.bump_check(TokenKind::RArrow) {
                println!("ERROR: expected `->` in function declaration");
                return None;
            }
    
            let return_type = match self.parse_type() {
                Some(return_type) => return_type,
                None => {
                    // TODO: add actual error reporting.
                    println!("ERROR: expected a return type");
                    return None;
                }
            };

            if let Some(block) = self.parse_block() {
                let (s, e) = (token.start, token.end);
                let name = String::from(&self.src[s..e]);

                return Some(FunctionStatement { name, return_type, arguments, block });
            }
            println!("ERROR: expected a block expression after function declaration");
        }

        println!("ERROR: expected `identifier` in function declaration");
        None
    }

    fn parse_struct(&mut self) -> Option<StructStatement> {
        self.bump();
        if let Some(token) = self.take_check(TokenKind::Identifier) {
            // TODO: add actual error reporting.
            if !self.bump_check(TokenKind::OpenBrace) {
                println!("ERROR: missing `{{` in struct declaration");
                return None;
            }
            let fields = match self.parse_params(TokenKind::CloseBrace) {
                Some(fields) => fields,
                None => return None,
            };

            // TODO: add actual error reporting.
            if !self.bump_check(TokenKind::CloseBrace) {
                println!("ERROR: missing `}}` in struct declaration");
                return None;
            }

            let (s, e) = (token.start, token.end);
            let name = String::from(&self.src[s..e]);
            return Some(StructStatement { name, fields });
        }
        None
    }

    fn parse_enum(&mut self) -> Option<EnumStatement> {
        self.bump();
        if let Some(token) = self.take_check(TokenKind::Identifier) {
            // TODO: add actual error reporting.
            if !self.bump_check(TokenKind::OpenBrace) {
                println!("ERROR: missing `{{` in enum declaration");
            }

            let mut variants = Vec::new();

            loop {
                if let Some(token) = self.take_check(TokenKind::Identifier) {
                    let (s, e) = (token.start, token.end);
                    let variant = String::from(&self.src[s..e]);
                    variants.push(variant);

                    // TODO: add actual error reporting.
                    if !self.bump_check(TokenKind::Comma) {
                        println!("ERROR: expected `,` after variant declaration");
                        return None;
                    }
                    continue
                } else if self.peek(0).kind == TokenKind::CloseBrace {
                    break
                } else {
                    println!("ERROR: expected identifier or `}}` in struct declaration");
                    return None;
                }
            }

            // TODO: add actual error reporting.
            if !self.bump_check(TokenKind::CloseBrace) {
                println!("ERROR: missing `}}` in struct declaration");
                return None;
            }

            let (s, e) = (token.start, token.end);
            let name = String::from(&self.src[s..e]);
            return Some(EnumStatement { name, variants });
        }
        None
    }

    fn parse_let(&mut self) -> Option<LetStatement> {
        self.bump();
        todo!("let statements")
    }



    fn parse_expr(&mut self) -> Option<Expression> {
        todo!("expressions")
    }

    fn parse_closure(&mut self) -> Option<ClosureExpression> {
        todo!("closures")
    }

    fn parse_call(&mut self) -> Option<CallExpression> {
        todo!("function calls")
    }

    fn parse_block(&mut self) -> Option<BlockExpression> {
        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::OpenBrace) {
            println!("ERROR: expected `{{ in a block expression`");
            return None
        }

        let mut statements = Vec::new();
        loop {
            match self.parse_statement() {
                Some(statement) => statements.push(statement),
                None => break,
            }
        }

        if !self.bump_check(TokenKind::CloseBrace) {
            println!("ERROR: expected `{{` to start a block expression, found `{:?}` instead", self.peek(0).kind);
            return None
        }
        Some(BlockExpression { statements, expression: None })
    }
    
    fn parse_if(&mut self) -> Option<IfExpression> {
        todo!("if expressions")
    }


    // #[inline]
    fn parse_params(&mut self, closing_delimiter: TokenKind) -> Option<Vec<Parameter>> {
        let mut parameters = Vec::new();
        let mut first_param = true;

        loop {
            if !first_param && !self.bump_check(TokenKind::Comma) {
                if self.peek(0).kind == closing_delimiter { break }
                println!("ERROR: expected `,` after parameter, found `{:?}` instead", self.peek(0).kind)
            }
            first_param = false;

            let parameter = match self.parse_param() {
                Some(parameter) => parameter,
                None => {
                    self.bump_while(|kind| kind != TokenKind::Comma && kind != closing_delimiter);
                    continue
                },
            };


            parameters.push(parameter);
        }

        // Optional `,` at the end.
        self.bump_check(TokenKind::Comma);
        Some(parameters)
    }

    // #[inline]
    fn parse_param(&mut self) -> Option<Parameter> {
        let token = match self.take_check(TokenKind::Identifier) {
            Some(token) => token,
            None => return None,
        };

        if !self.bump_check(TokenKind::Colon) {
            // TODO: add actual error reporting.
            println!("ERROR: expected `:` after identifier in parameter, found `{:?}` instead", self.peek(0).kind);
            return None;
        }

        let param_type = match self.parse_type() {
            Some(param_type) => param_type,
            None => {
                // TODO: add actual error reporting.
                println!("ERROR: missing argument type in parameter");
                return None;
            }
        };
    
        println!("inner {:?}", self.peek(0));
        let (s, e) = (token.start, token.end);
        let name = String::from(&self.src[s..e]);
        Some(Parameter { name, param_type })
    }


    fn parse_type(&mut self) -> Option<Type> {
        if let Some(token) = self.take_check(TokenKind::Identifier) {
            let (s, e) = (token.start, token.end);
            let lexeme = &self.src[s..e];
            return Some(Parser::parse_type_from_ident(lexeme))
        } else if self.bump_check(TokenKind::OpenParen) {
            self.parse_type_tuple();
        } else if self.bump_check(TokenKind::OpenBracket) {
            self.parse_type_list();
        } else if self.bump_check(TokenKind::Fn) {
            return self.parse_type_fn();
        }
        None
    }

    // #[inline(always)]
    fn parse_type_tuple(&mut self) -> Option<Type> {
        let arguments = self.parse_type_args();
        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::CloseParen) {
            println!("ERROR: expected `)` after type arguments in a function type, found `{:?}` instead", self.peek(0).kind);
            return None;
        }
        Some(Type::Tuple(TupleType(arguments)))
    }

    // #[inline(always)]
    fn parse_type_list(&mut self) -> Option<Type> {
        let list_type = match self.parse_type() {
            Some(list_type) => Box::new(list_type),
            None => return None,
        };
        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::CloseBracket) {
            println!("ERROR: expected `)` after type arguments in a function type, found `{:?}` instead", self.peek(0).kind);
            return None;
        }
        Some(Type::List(list_type))
    }

    #[inline(always)]
    fn parse_type_fn(&mut self) -> Option<Type> {
        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::OpenParen) {
            println!("ERROR: expected `(` after `fn` in function type");
            return None;
        }

        let arguments = self.parse_type_args();

        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::CloseParen) {
            println!("ERROR: expected `)` after type arguments in a function type");
            return None;
        }

        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::RArrow) {
            println!("ERROR: expected `->` after type arguments in a function type");
            return None;
        }

        match self.parse_type() {
            Some(return_type) => {
                let return_type = Box::new(return_type);
                Some(Type::Fn { arguments, return_type })
            }
            None => {
                // TODO: add actual error reporting.
                println!("ERROR: expected a return type in a function type");
                None
            }
        }
    }

    fn parse_type_args(&mut self) -> Vec<Type> {
        let mut first_type = true;
        let mut arguments = Vec::new();

        loop {
            // TODO: add actual error reporting.
            if !first_type && !self.bump_check(TokenKind::Comma) {
                println!("ERROR: expected `,` after a type argument in a function type")
            }

            let type_arg = match self.parse_type() {
                Some(type_arg) => type_arg,
                None => break,
            };

            arguments.push(type_arg);

            first_type = false;
        }

        // Optional `,` after the last type_arg
        self.bump_check(TokenKind::Comma);
        arguments
    }

    #[inline]
    fn parse_type_from_ident(typename: &str) -> Type {
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



    pub fn parse(input: &'src str) -> ASTree {
        let mut parser = Parser::new(input);
        let mut statements = Vec::new();
        loop {
            if let Some(statement) = parser.parse_statement() {
                statements.push(statement);
            } else {
                break
            }
        }

        // TODO: add actual error reporting.
        if !parser.bump_check(TokenKind::EOF) {
            println!("ERROR: parser did not reach end of file!");
        }

        ASTree::new(statements)
    }
}


#[cfg(test)]
mod tests;
