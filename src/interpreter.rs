use std::fmt::{Display, Formatter};

use crate::ast::{Expression, Visitor};
use crate::scanner::TokenType;

#[derive(Clone, Debug)]
pub enum Types {
    Number(f64),
    ReturnString(String),
    Boolean(bool),
    Nil,
}

impl Display for Types {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Self::Number(n) => write!(f, "{}", n),
            Self::Boolean(b) => write!(f, "{}", b),
            Self::Nil => write!(f, "nil"),
            Self::ReturnString(s) => write!(f, "{}", s),
        }
    }
}

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, e: &Expression) -> anyhow::Result<()> {
        let t = self.visit_expression(e)?;
        println!("{}", t);

        Ok(())
    }
}

impl Visitor for Interpreter {
    type E = anyhow::Result<Types>;
    fn visit_expression(&self, e: &Expression) -> Self::E {
        match e {
            &Expression::Literal { ref token } => match token.token_type {
                TokenType::Number { number } => Ok(Types::Number(number)),
                TokenType::StringLiteral { literal } => {
                    Ok(Types::ReturnString(literal.to_string()))
                }
                TokenType::True => Ok(Types::Boolean(true)),
                TokenType::False => Ok(Types::Boolean(false)),
                TokenType::Nil => Ok(Types::Nil),
                _ => Err(anyhow::anyhow!("Unrecognized literal")),
            },
            &Expression::Grouping { ref expr } => self.visit_expression(expr),
            &Expression::Unary {
                ref operator,
                ref r_expr,
            } => {
                let right = self.visit_expression(r_expr)?;
                match (right, &operator.token_type) {
                    (Types::Number(n), TokenType::Minus) => Ok(Types::Number(-n)),
                    (Types::Boolean(false) | Types::Nil, TokenType::Bang) => {
                        Ok(Types::Boolean(true))
                    }
                    (_, TokenType::Bang) => Ok(Types::Boolean(false)),
                    _ => Err(anyhow::anyhow!("Unrecognized unary")),
                }
            }
            &Expression::Binary {
                ref l_expr,
                ref operator,
                ref r_expr,
            } => {
                let left = self.visit_expression(l_expr)?;
                let right = self.visit_expression(r_expr)?;

                match (left, right, &operator.token_type) {
                    (Types::Number(n_first), Types::Number(n_second), t) => match t {
                        &TokenType::Plus => Ok(Types::Number(n_first + n_second)),
                        &TokenType::Minus => Ok(Types::Number(n_first - n_second)),
                        &TokenType::Star => Ok(Types::Number(n_first * n_second)),
                        &TokenType::Slash => Ok(Types::Number(n_first / n_second)),
                        &TokenType::Greater => Ok(Types::Boolean(n_first > n_second)),
                        &TokenType::GreaterEqual => Ok(Types::Boolean(n_first >= n_second)),
                        &TokenType::Less => Ok(Types::Boolean(n_first < n_second)),
                        &TokenType::LessEqual => Ok(Types::Boolean(n_first <= n_second)),
                        &TokenType::EqualEqual => Ok(Types::Boolean(n_first == n_second)),
                        &TokenType::BangEqual => Ok(Types::Boolean(n_first != n_second)),
                        _ => Err(anyhow::anyhow!(
                            "Unrecognized binary operation to two numbers"
                        )),
                    },

                    (
                        Types::ReturnString(s_first),
                        Types::ReturnString(s_second),
                        TokenType::Plus,
                    ) => Ok(Types::ReturnString(s_first + &s_second)),

                    (Types::Nil, Types::Nil, TokenType::Equal) => Ok(Types::Boolean(true)),
                    (Types::Nil, Types::Nil, TokenType::BangEqual) => Ok(Types::Boolean(false)),

                    (Types::Boolean(b_first), Types::Boolean(b_second), TokenType::EqualEqual) => {
                        Ok(Types::Boolean(b_first == b_second))
                    }
                    (Types::Boolean(b_first), Types::Boolean(b_second), TokenType::BangEqual) => {
                        Ok(Types::Boolean(b_first != b_second))
                    }
                    _ => Err(anyhow::anyhow!("Unrecognized binary")),
                }
            }
        }
    }
}
