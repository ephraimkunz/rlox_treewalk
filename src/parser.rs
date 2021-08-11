use crate::ast::Expression;
use crate::scanner::{Token, TokenType};
use anyhow::anyhow;
use std::cell::Cell;
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct ParserError {
    message: String,
    line: usize,
    lexeme: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.lexeme, self.message
        )
    }
}

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current: Cell<usize>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser {
            tokens,
            current: Cell::new(0),
        }
    }

    pub fn parse(&self) -> anyhow::Result<Expression> {
        self.expression()
    }

    fn expression(&self) -> anyhow::Result<Expression> {
        self.equality()
    }

    fn equality(&self) -> anyhow::Result<Expression> {
        let mut expr = self.comparison()?;

        while let Some(t) = match self.peek().map(|t| &t.token_type) {
            Some(&TokenType::BangEqual | &TokenType::EqualEqual) => self.advance(),
            _ => None,
        } {
            let right = Box::new(self.comparison()?);
            expr = Expression::Binary {
                l_expr: Box::new(expr),
                operator: t.clone(),
                r_expr: right,
            };
        }

        Ok(expr)
    }

    fn comparison(&self) -> anyhow::Result<Expression> {
        let mut expr = self.term()?;

        while let Some(t) = match self.peek().map(|t| &t.token_type) {
            Some(
                &TokenType::GreaterEqual
                | &TokenType::Greater
                | &TokenType::LessEqual
                | &TokenType::Less,
            ) => self.advance(),
            _ => None,
        } {
            let right = Box::new(self.term()?);
            expr = Expression::Binary {
                l_expr: Box::new(expr),
                operator: t.clone(),
                r_expr: right,
            };
        }

        Ok(expr)
    }

    fn term(&self) -> anyhow::Result<Expression> {
        let mut expr = self.factor()?;

        while let Some(t) = match self.peek().map(|t| &t.token_type) {
            Some(&TokenType::Plus | &TokenType::Minus) => self.advance(),
            _ => None,
        } {
            let right = Box::new(self.factor()?);
            expr = Expression::Binary {
                l_expr: Box::new(expr),
                operator: t.clone(),
                r_expr: right,
            }
        }

        Ok(expr)
    }

    fn factor(&self) -> anyhow::Result<Expression> {
        let mut expr = self.unary()?;

        while let Some(t) = match self.peek().map(|t| &t.token_type) {
            Some(&TokenType::Slash | &TokenType::Star) => self.advance(),
            _ => None,
        } {
            let right = Box::new(self.unary()?);
            expr = Expression::Binary {
                l_expr: Box::new(expr),
                operator: t.clone(),
                r_expr: right,
            }
        }

        Ok(expr)
    }

    fn unary(&self) -> anyhow::Result<Expression> {
        if let Some(t) = match self.peek().map(|t| &t.token_type) {
            Some(&TokenType::Bang | &TokenType::Minus) => self.advance(),
            _ => None,
        } {
            let right = Box::new(self.unary()?);
            return Ok(Expression::Unary {
                operator: t.clone(),
                r_expr: right,
            });
        }

        self.primary()
    }

    fn primary(&self) -> anyhow::Result<Expression> {
        let next = self.peek();

        match next {
            Some(t) => match t.token_type {
                TokenType::False
                | TokenType::True
                | TokenType::Nil
                | TokenType::Number { .. }
                | TokenType::StringLiteral { .. } => {
                    self.advance();
                    Ok(Expression::Literal { token: t.clone() })
                }
                TokenType::LeftParen => {
                    self.advance();
                    let expr = Box::new(self.expression()?);
                    if let Some(t) = self.peek() {
                        if t.token_type == TokenType::RightParen {
                            self.advance();
                            Ok(Expression::Grouping { expr })
                        } else {
                            Err(ParserError {
                                message: "expect ')' after expression".to_string(),
                                lexeme: t.lexeme.to_string(),
                                line: t.line,
                            }
                            .into())
                        }
                    } else {
                        Err(anyhow!("expect ')' after expression"))
                    }
                }
                _ => Err(ParserError {
                    message: format!("unrecognized primary: {:?}", t),
                    lexeme: t.lexeme.to_string(),
                    line: t.line,
                }
                .into()),
            },
            _ => Err(anyhow!("expected expression")),
        }
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().map(|t| &t.token_type) == Some(t)
    }

    fn advance(&self) -> Option<&'a Token<'a>> {
        if !self.is_at_end() {
            self.current.set(self.current.get() + 1)
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().map(|t| &t.token_type) == Some(&TokenType::Eof)
    }

    fn peek(&self) -> Option<&'a Token<'a>> {
        self.tokens.get(self.current.get())
    }

    fn previous(&self) -> Option<&'a Token<'a>> {
        self.tokens.get(self.current.get() - 1)
    }

    fn synchronize(&self) {
        self.advance();
        while !self.is_at_end() {
            if let Some(t) = self.previous() {
                if t.token_type == TokenType::Semicolon {
                    return;
                }
            }

            match self.peek().map(|t| &t.token_type) {
                Some(
                    TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return,
                ) => return,
                _ => (),
            }

            self.advance();
        }
    }
}
