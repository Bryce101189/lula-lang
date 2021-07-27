use crate::expr::Expr;

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    Expr(Expr),
}
