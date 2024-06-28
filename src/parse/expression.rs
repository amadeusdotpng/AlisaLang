use crate::parse::{Parser, ParseError, ParseResult};


use crate::ast::token::{T, TokenKind, LiteralKind, OpKind};
use crate::ast::{Statement, Expression};
use crate::ast::{ClosureExpression, IdentExpression, BlockExpression, CallExpression};
use crate::ast::{IfExpression, ElseExpression};
use crate::ast::{LitKind, LiteralExpression};
use crate::ast::{BinaryExpression, BinaryOperator};
use crate::ast::{UnaryExpression, UnaryOperator};
use crate::ast::{Tuple, List};


fn binop_tok_to_ast(op_kind: TokenKind) -> Option<BinaryOperator> {
    let op = match op_kind {
        T!("|") => BinaryOperator::BitOr,
        T!("&") => BinaryOperator::BitAnd,
        T!("^") => BinaryOperator::BitXor,
        T!(">>") => BinaryOperator::BitRight,
        T!("<<") => BinaryOperator::BitLeft,
        T!("+") => BinaryOperator::Add,
        T!("-") => BinaryOperator::Sub,
        T!("*") => BinaryOperator::Mul,
        T!("/") => BinaryOperator::Div,
        T!("%") => BinaryOperator::Mod,

        T!("||") => BinaryOperator::BoolOr,
        T!("&&")   => BinaryOperator::BoolAnd,

        T!("==")     => BinaryOperator::Eq,
        T!("!=")   => BinaryOperator::Ne,
        T!(">=")     => BinaryOperator::Ge,
        T!("<=")     => BinaryOperator::Le,
        T!(">")       => BinaryOperator::Gt,
        T!("<")       => BinaryOperator::Lt,

        T!("|>")   => BinaryOperator::Pipe,

        _ => return None,
    };
    Some(op)
}

fn unop_tok_to_ast(op_kind: TokenKind) -> Option<UnaryOperator> {
    let op = match op_kind {
        T!("+") => UnaryOperator::Plus,
        T!("-") => UnaryOperator::Minus,

        T!("!")  => UnaryOperator::BoolNot,
        T!("~") => UnaryOperator::BitNot,

        _ => return None,
    };
    Some(op)
}

fn prefix_binding_power(op: UnaryOperator) -> ((), u8) {
    match op {
        UnaryOperator::BoolNot => ((), 5),

        UnaryOperator::BitNot
        | UnaryOperator::Plus
        | UnaryOperator::Minus => ((), 25),
    }
}

fn infix_binding_power(op: BinaryOperator) -> (u8, u8) {
    match op {
        BinaryOperator::BoolOr  => (1, 2),
        BinaryOperator::BoolAnd => (3, 4),
                                 
        BinaryOperator::Eq
        | BinaryOperator::Ne
        | BinaryOperator::Ge
        | BinaryOperator::Le
        | BinaryOperator::Gt
        | BinaryOperator::Lt => (7, 8),

        BinaryOperator::Pipe => (9, 10),

        BinaryOperator::Mod  => (11, 12),

        BinaryOperator::BitOr     => (13, 14),
        BinaryOperator::BitAnd    => (15, 16),
        BinaryOperator::BitXor    => (17, 18),
        BinaryOperator::BitRight
        | BinaryOperator::BitLeft => (19, 20),

        BinaryOperator::Add
        | BinaryOperator::Sub => (21, 22),
        BinaryOperator::Mul
        | BinaryOperator::Div => (23, 24),
    }
}

impl<'src> Parser<'src> {
    pub(super) fn parse_expr(&mut self, min_bp: u8) -> ParseResult<Expression> {
        let tok = self.peek(0);
        let mut lhs = match tok.kind {
            TokenKind::Literal { kind } => {
                self.bump();
                let lexeme = self.get_lexeme(tok);
                let literal = self.parse_literal(kind, lexeme);
                Expression::Literal(literal)
            }

            T!("\\") => {
                let closure = self.parse_closure()?;
                Expression::Closure(Box::new(closure))
            }

            T!("if") => {
                let if_expr = self.parse_if()?;
                Expression::If(Box::new(if_expr))
            }

            T!("(") => {
                self.bump();
                let lhs = self.parse_expr(0)?;

                const CLOSE: TokenKind = T!(")");

                if self.bump_check(T!(",")) {
                    let mut first_expr = true;
                    let mut expressions = Vec::new();
                    expressions.push(lhs);

                    loop {
                        let (peek_0, peek_1) = (self.peek(0).kind, self.peek(1).kind);
                        if peek_0 == CLOSE || (peek_1 == CLOSE && !first_expr) { break }

                        if !first_expr { 
                            self.bump_recover(T!(","));
                        } else { first_expr = false; }

                        let expression = match self.parse_expr(0) {
                            Ok(expression) => expression,
                            Err(err) => {
                                /*
                                self.recover_error(err);
                                break
                                */
                                return Err(err);
                            }
                        };

                        expressions.push(expression);
                    }

                    self.bump_expect(CLOSE)?;

                    let tuple_expr = LiteralExpression {
                        kind: LitKind::Tuple(Tuple(expressions))
                    };
                    Expression::Literal(tuple_expr)
                } else {
                    self.bump_expect(CLOSE)?;
                    lhs
                }
            }

            T!("{") => {
                let block_expr = Box::new(self.parse_block()?);
                Expression::Block(block_expr)
            }

            T!("{") => {
                self.bump();

                let mut first_expr = true;
                let mut expressions = Vec::new();

                const CLOSE: TokenKind = T!("}");

                loop {
                    let (peek_0, peek_1) = (self.peek(0).kind, self.peek(1).kind);
                    if peek_0 == CLOSE || (peek_1 == CLOSE && !first_expr) { break }

                    if !first_expr { 
                        self.bump_recover(T!(","));
                    } else { first_expr = false; }

                    let expression = match self.parse_expr(0) {
                        Ok(expression) => expression,
                        Err(err) => {
                            self.recover_error(err);
                            break
                        }
                    };

                    expressions.push(expression);
                }

                self.bump_expect(CLOSE)?;

                let list_expr = LiteralExpression {
                    kind: LitKind::List(List(expressions))
                };
                Expression::Literal(list_expr)
            }

            kind @
            ( TokenKind::Op { .. }
            | T!("!") 
            | T!("~")
            ) => {
                self.bump();
                let op = match unop_tok_to_ast(kind) {
                    Some(op) => op,
                    None => return Err(ParseError::ExpectedAlternatives {
                        expected: Box::new([
                            T!("!"), T!("~"),
                            T!("+"),
                            T!("-"),
                        ]),
                        found: tok,
                    })
                };

                let ((), r_bp) = prefix_binding_power(op);
                let rhs = self.parse_expr(r_bp)?;
                let un_expr = UnaryExpression { rhs, op };
                Expression::Unary(Box::new(un_expr))
            }

            T!("ID") => {
                self.bump();
                let name = self.get_lexeme(tok);
                let ident = IdentExpression { name: name.into() };
                Expression::Identifier(ident)
            }
            
            _ => return Err(ParseError::ExpectedNode {
                expected: "expression".into(),
                found: self.peek(0),
            })
        };

        loop {
            let tok = self.peek(0);
            let op = match binop_tok_to_ast(tok.kind) {
                Some(op) => op,
                None => break,
            };

            let (l_bp, r_bp) = infix_binding_power(op);
            
            if l_bp < min_bp {
                break
            }

            let mark = self.stream.mark();
            self.bump();
            let rhs = match self.parse_expr(r_bp) {
                Ok(rhs) => rhs,
                Err(err) => { 
                    self.stream.reset(mark);
                    return Err(err);
                }
            };

            let bin_expr = BinaryExpression { lhs, rhs, op };
            lhs = Expression::Binary(Box::new(bin_expr));
        }

        Ok(lhs)
    }

    pub(super) fn parse_literal(&self, kind: LiteralKind, lexeme: &str) -> LiteralExpression {
        let kind = match kind {
            LiteralKind::Bool => {
                let Ok(value) = lexeme.parse::<bool>() else {
                    unreachable!("could not parse boolean"); 
                };
                LitKind::Bool(value)
            }

            LiteralKind::Int => {
                let Ok(value) = lexeme.parse::<u128>() else { 
                    unreachable!("could not parse integer"); 
                };
                LitKind::Int(value)
            }

            LiteralKind::Float => {
                let Ok(value) = lexeme.parse::<f64>() else {
                    unreachable!("could not parse float");
                };
                LitKind::Float(value)
            }

            LiteralKind::Str { terminated: _ } => {
                let value = String::from(&self.src[1..lexeme.len()-1]);
                LitKind::Str(value)
            }

            LiteralKind::Char { terminated: _ } => {
                let lexeme = &self.src[1..lexeme.len()-1];
                let Ok(value) = lexeme.parse::<char>() else {
                    unreachable!("could not parse char");
                };
                LitKind::Char(value)
            }
        };
        LiteralExpression { kind }
    }

    pub(super) fn parse_closure(&mut self) -> ParseResult<ClosureExpression> {
        self.bump(); // `\`

        let arguments = self.parse_params(T!("("), T!(")"))?;

        self.bump_expect(T!("->"))?;

        let return_type = self.parse_type()?;

        let block = self.parse_block()?;

        Ok(ClosureExpression { arguments, block, return_type })
    }

    pub(super) fn parse_if(&mut self) -> ParseResult<IfExpression> {
        self.bump(); // `if`
        
        let condition = self.parse_expr(0)?;
        let body = self.parse_block()?;
        
        if self.bump_check(T!("else")) {
            let else_body = if self.bump_check(T!("if")) {
                let else_body = self.parse_if()?;
                ElseExpression::ElseIf(else_body)
            } else {
                let else_body = self.parse_block()?;
                ElseExpression::Else(else_body)
            };

            let else_body = Some(Box::new(else_body));
            return Ok(IfExpression { condition, body, else_body });
        }

        Ok(IfExpression { condition, body, else_body: None })
    }

    pub(super) fn parse_block(&mut self) -> ParseResult<BlockExpression> {
        self.bump_expect(T!("{"))?;

        let mut statements = Vec::new();
        loop {
            if self.peek(0).kind == T!("}") { break }
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(err) => {
                    self.recover_error(err);
                    break
                }
            }
        }

        self.bump_expect(T!("}"))?;

        if statements.is_empty() {
            return Ok(BlockExpression { statements, expression: None })
        }


        for statement in &statements[..statements.len()-1] {
            self.validate_statement(statement);
        }

        let expression = match statements.last() {
            Some(Statement::Expression { expr: _, end_token }) if end_token.kind != T!(";") => {
                let expr = statements.pop();
                let Some(Statement::Expression { expr, .. }) = expr else { unreachable!() };
                Some(expr)
            }
            _ => None,
        };

        Ok(BlockExpression { statements, expression })
    }

    fn validate_statement(&mut self, statement: &Statement) {
        if let Statement::Expression { expr: _, end_token } = statement {
            if end_token.kind == T!(";") {
                self.recover_error(ParseError::ExpectedSingle {
                    expected: T!(";"),
                    found: *end_token,
                })
            }
        }
    }
}

