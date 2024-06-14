use crate::parse::Parser;
use crate::parse::ParseError;

use crate::ast::token::{TokenKind, LiteralKind, OpKind};
use crate::ast::{Statement, Expression};
use crate::ast::{ClosureExpression, CallExpression, BlockExpression, IfExpression};
use crate::ast::{LitKind, LiteralExpression, BinaryExpression, BinaryOperator};

impl<'src> Parser<'src> {
    pub(super) fn infix_binding_power(op: BinaryOperator) -> (u8, u8){
        match op {
            BinaryOperator::BoolOr  => (1, 2),
            BinaryOperator::BoolAnd => (3, 4),
                                     
            BinaryOperator::Eq
            | BinaryOperator::Ne
            | BinaryOperator::Ge
            | BinaryOperator::Le
            | BinaryOperator::Gt
            | BinaryOperator::Lt => (7, 8),

            BinaryOperator::BitOr     => (9, 10),
            BinaryOperator::BitAnd    => (11, 12),
            BinaryOperator::BitXor    => (13, 14),
            BinaryOperator::BitRight
            | BinaryOperator::BitLeft => (15, 16),

            BinaryOperator::Add
            | BinaryOperator::Sub => (17, 18),
            BinaryOperator::Mul
            | BinaryOperator::Div => (19, 20),
            BinaryOperator::Mod   => (21, 22),
        }
    }

    pub(super) fn parse_expr(&mut self, min_bp: u8) -> Result<Expression, ParseError> {
        let tok = self.peek(0);
        let mut lhs = match tok.kind {
            TokenKind::Literal { kind } => {
                self.bump();
                self.parse_literal(kind, (tok.start, tok.end))
            }
            
            _ => return Err(ParseError::ExpectedNode {
                expected: "expression".into(),
                found: self.peek(0),
            })
        };

        loop {
            let tok = self.peek(0);
            let op = match tok.kind {
                TokenKind::Op { kind: OpKind::Pipe    } => BinaryOperator::BitOr,
                TokenKind::Op { kind: OpKind::And     } => BinaryOperator::BitAnd,
                TokenKind::Op { kind: OpKind::Caret   } => BinaryOperator::BitXor,
                TokenKind::Op { kind: OpKind::ShiftR  } => BinaryOperator::BitRight,
                TokenKind::Op { kind: OpKind::ShiftL  } => BinaryOperator::BitLeft,
                TokenKind::Op { kind: OpKind::Plus    } => BinaryOperator::Add,
                TokenKind::Op { kind: OpKind::Minus   } => BinaryOperator::Sub,
                TokenKind::Op { kind: OpKind::Star    } => BinaryOperator::Mul,
                TokenKind::Op { kind: OpKind::FSlash  } => BinaryOperator::Div,
                TokenKind::Op { kind: OpKind::Percent } => BinaryOperator::Mod,

                TokenKind::PipePipe => BinaryOperator::BoolOr,
                TokenKind::AndAnd   => BinaryOperator::BoolAnd,

                TokenKind::EqEq     => BinaryOperator::Eq,
                TokenKind::BangEq   => BinaryOperator::Ne,
                TokenKind::GtEq     => BinaryOperator::Ge,
                TokenKind::LtEq     => BinaryOperator::Le,
                TokenKind::Gt       => BinaryOperator::Gt,
                TokenKind::Lt       => BinaryOperator::Lt,
                _ => break
            };

            let (l_bp, r_bp) = Self::infix_binding_power(op);
            
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

    pub(super) fn parse_literal(&mut self, kind: LiteralKind, pos: (usize, usize)) -> Expression {
        let (s, e) = pos;
        let kind = match kind {
            LiteralKind::Bool => {
                let lexeme = &self.src[s..e];
                let Ok(value) = lexeme.parse::<bool>() else {
                    unreachable!("could not parse boolean"); 
                };
                LitKind::Bool(value)
            }

            LiteralKind::Int => {
                let lexeme = &self.src[s..e];
                let Ok(value) = lexeme.parse::<u128>() else { 
                    unreachable!("could not parse integer"); 
                };
                LitKind::Int(value)
            }

            LiteralKind::Float => {
                let lexeme = &self.src[s..e];
                let Ok(value) = lexeme.parse::<f64>() else {
                    unreachable!("could not parse float");
                };
                LitKind::Float(value)
            }

            LiteralKind::Str { terminated: _ } => {
                let value = String::from(&self.src[s+1..e-1]);
                LitKind::Str(value)
            }

            LiteralKind::Char { terminated: _ } => {
                let lexeme = &self.src[s+1..e-1];
                let Ok(value) = lexeme.parse::<char>() else {
                    unreachable!("could not parse char");
                };
                LitKind::Char(value)
            }
        };
        Expression::Literal(LiteralExpression { kind })
    }

    pub(super) fn parse_closure(&mut self) -> Option<ClosureExpression> {
        todo!("closures")
    }

    pub(super) fn parse_call(&mut self) -> Option<CallExpression> {
        todo!("function calls")
    }

    pub(super) fn parse_block(&mut self) -> Option<BlockExpression> {
        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::OpenBrace) {
            println!("ERROR: expected `{{` in a block expression`");
            return None
        }

        let mut statements = Vec::new();
        loop {
            match self.parse_statement() {
                Ok(statement) => statements.push(statement),
                Err(_) => break,
            }
        }

        // TODO: add actual error reporting.
        if !statements.is_empty() {
            for statement in &statements[..statements.len()-1] {
                if let Statement::Expression { expr: _, has_semi } = statement {
                    if !has_semi { println!("ERROR: epected `;` at the end of expression") }
                }
            }
        }

        // TODO: add actual error reporting.
        if !self.bump_check(TokenKind::CloseBrace) {
            println!("ERROR: expected `}}` to end a block expression, found `{:?}` instead", self.peek(0).kind);
            return None
        }

        let expression = match statements.last() {
            Some(Statement::Expression { expr: _, has_semi }) if !has_semi => {
                let expr = statements.pop();
                let Some(Statement::Expression { expr, has_semi: _ }) = expr else { unreachable!() };
                Some(expr)
            }
            _ => None,
        };

        Some(BlockExpression { statements, expression })
    }
    
    pub(super) fn parse_if(&mut self) -> Option<IfExpression> {
        todo!("if expressions")
    }
}
