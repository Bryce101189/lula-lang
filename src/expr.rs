use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Literal(Token),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Token, Box<Expr>, Token),
}
