use crate::scanner::{Token, TokenType};
pub enum Expression<'a> {
    Binary {
        l_expr: Box<Expression<'a>>,
        operator: Token<'a>,
        r_expr: Box<Expression<'a>>,
    },
    Grouping {
        expr: Box<Expression<'a>>,
    },
    Literal {
        token: Token<'a>,
    },
    Unary {
        operator: Token<'a>,
        r_expr: Box<Expression<'a>>,
    },
}

pub trait Visitor {
    type E;
    fn visit_expresssion(&self, expr: &Expression) -> Self::E;
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(&self, expr: &Expression) -> String {
        self.visit_expresssion(expr)
    }
}

impl Visitor for AstPrinter {
    type E = String;
    fn visit_expresssion(&self, e: &Expression) -> Self::E {
        match e {
            Expression::Binary {
                l_expr,
                operator,
                r_expr,
            } => format!(
                "(Binary {:?} {} {})",
                operator,
                self.visit_expresssion(l_expr),
                self.visit_expresssion(r_expr)
            ),
            Expression::Grouping { expr } => format!("(Grouping {})", self.visit_expresssion(expr)),
            Expression::Literal { token } => format!("(Literal {:?})", token),
            Expression::Unary { operator, r_expr } => {
                format!("(Unary {:?} {})", operator, self.visit_expresssion(r_expr))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ast() {
        let expr = Expression::Binary {
            l_expr: Box::new(Expression::Unary {
                operator: Token::new(TokenType::Minus, "-", 1),
                r_expr: Box::new(Expression::Literal {
                    token: Token::new(TokenType::Number { number: 123_f64 }, "123", 1),
                }),
            }),
            operator: Token::new(TokenType::Star, "*", 1),
            r_expr: Box::new(Expression::Grouping {
                expr: Box::new(Expression::Literal {
                    token: Token::new(TokenType::Number { number: 45.67 }, "45.67", 1),
                }),
            }),
        };
        println!("{}", AstPrinter {}.print(&expr));
    }
}
