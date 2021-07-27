use crate::expr::Expr;

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    Expr(Expr),
}

impl Statement {
    pub fn interpret(&self) -> bool {
        match self {
            Statement::Print(expr) => {
                match expr.evaluate() {
                    Some(val) => println!("{}", val),
                    None => return false,
                };
            }

            _ => {}
        }

        true
    }
}
