pub mod parser;
pub mod stream;
pub mod nonterminal;

use parser::Parser;
use nonterminal::Nonterminal;

use crate::ast::*;
// Implement general parse function ?
// I will have to either change Nonterminal to Rule and add a "Terminal" variant that holds a
// token, or make a new enum Rule that has the variants Nonterminal and Terminal which respectively
// holds a Nonterminal and a TokenKind.
impl<'src> Parser<'src> {
    fn assign_or_id(&mut self) -> bool {
        todo!()
    }

    fn tuple_or_expr(&mut self) -> bool {
        todo!()
    }

    fn parse_nt(&mut self, nt: Nonterminal) -> bool {
        match nt {
            Nonterminal::Program => {
                loop { 
                    // Keep parsing statements
                    if !self.parse_nt(Nonterminal::Statement) { break }
                }
                
                // If it can no longer parse statements, it should be the end of the file.
                let res = self.expect(TokenKind::EOF).is_some();
                res
            }

            Nonterminal::Statement => {
                self.parse_nt(Nonterminal::VarDecl) ||
                self.parse_nt(Nonterminal::FuncDecl) ||
                self.parse_nt(Nonterminal::StructDecl) ||
                self.parse_nt(Nonterminal::EnumDecl) ||
               (self.parse_nt(Nonterminal::Expression) && self.expect(TokenKind::Semi).is_some())
            }

            Nonterminal::Expression => {
                // Assignment Expression or Lonesome Identifier
                if self.expect(TokenKind::Identifier).is_some() {
                    return self.assign_or_id();
                }

                // Tuple or Parenthesized Expression
                if self.expect(TokenKind::OpenParen).is_some() {
                    return self.tuple_or_expr();
                }

                self.parse_nt(Nonterminal::IfExpression) ||
                self.parse_nt(Nonterminal::BlockExpression) ||
                self.parse_nt(Nonterminal::ClosureExpression) ||
                self.parse_nt(Nonterminal::OperationExpression)
            }

            Nonterminal::Params => {
                // CAHNGE TO TYPE
                let param_rules = [TokenKind::Identifier, TokenKind::Colon, TokenKind::Identifier];
                if self.expect_n(&param_rules).is_some() {
                    let with_comma = [
                        TokenKind::Comma,
                        TokenKind::Identifier,
                        TokenKind::Colon,
                        TokenKind::Identifier
                    ];

                    loop { if !self.expect_n(&with_comma).is_some() { break } }

                    // OPTIONAL
                    self.expect(TokenKind::Comma);
                    return true;
                }
                false
            }

            Nonterminal::BlockExpression => {
                if self.expect(TokenKind::OpenBrace).is_some() {
                    loop { if !self.parse_nt(Nonterminal::Statement) { break } };
                    self.parse_nt(Nonterminal::Expression); // OPTIONAL
                    return self.expect(TokenKind::CloseBrace).is_some();
                }
                false
            }

            Nonterminal::VarDecl => {
                //CHANGE TO TYPE
                let rules = &[TokenKind::Let, TokenKind::Identifier, TokenKind::Colon, TokenKind::Identifier]; 
                self.expect_n(rules).is_some() && 
                self.parse_nt(Nonterminal::Expression) &&
                self.expect(TokenKind::Semi).is_some()
            }

            Nonterminal::FuncDecl => {
                let open_rules = [TokenKind::Fn, TokenKind::Identifier, TokenKind::OpenParen];
                if self.expect_n(&open_rules).is_some() {
                    // OPTIONAL
                    self.parse_nt(Nonterminal::Params);

                    // CHANGE TO TYPE
                    let close_rules = [TokenKind::CloseParen, TokenKind::RArrow, TokenKind::Identifier]; 
                    return self.expect_n(&close_rules).is_some() &&
                           self.parse_nt(Nonterminal::BlockExpression);
                }
                false
            }

            Nonterminal::StructDecl => {
                let open_rules = [TokenKind::Struct, TokenKind::Identifier, TokenKind::OpenBrace];
                if self.expect_n(&open_rules).is_some() {
                    self.parse_nt(Nonterminal::Params); // OPTIONAL
                    return self.expect(TokenKind::CloseBrace).is_some();
                }
                false
            }

            Nonterminal::EnumDecl => {
                let open_rules = [TokenKind::Enum, TokenKind::Identifier, TokenKind::OpenBrace];
                if self.expect_n(&open_rules).is_some() {
                    self.parse_nt(Nonterminal::Params); // OPTIONAL
                    return self.expect(TokenKind::CloseBrace).is_some();
                }
                false
            }

            Nonterminal::IfExpression => {
                let res = self.expect(TokenKind::If).is_some() &&
                self.parse_nt(Nonterminal::Expression) &&
                self.parse_nt(Nonterminal::BlockExpression);
                // Else Expression
                if res && self.expect(TokenKind::Else).is_some() {
                    return self.parse_nt(Nonterminal::IfExpression) ||
                           self.parse_nt(Nonterminal::BlockExpression);
                } 
                res
            }

            Nonterminal::ClosureExpression => {
                let open_rules = [TokenKind::BSlash, TokenKind::OpenParen];
                if self.expect_n(&open_rules).is_some() {
                    self.parse_nt(Nonterminal::Params);
                    // CHANGE TO TYPE
                    let close_rules = [TokenKind::CloseParen, TokenKind::RArrow, TokenKind::Identifier];
                    return self.expect_n(&close_rules).is_some() &&
                           self.parse_nt(Nonterminal::BlockExpression);
                }
                false
            }

            Nonterminal::OperationExpression => {
                self.parse_expr(0)
            }
            _ => todo!()
        }

    }

    fn parse_expr(&mut self, min_bp: u8) -> bool {
        todo!()
    }

    pub fn prefix_binding_power(op: TokenKind) -> u8 {
        0
    }

    pub fn parse(input: &'src str) -> bool {
        let mut parser = Parser::new(input);
        parser.parse_nt(Nonterminal::Program)
    }
}


#[cfg(test)]
mod tests;
