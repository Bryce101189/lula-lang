use crate::token::{Literal, Token};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Token, Box<Expr>, Token),
}
