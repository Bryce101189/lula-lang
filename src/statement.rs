use crate::expr::Expr;

#[derive(Debug)]
pub enum Statement {
    Print(Expr),
    VarDecl(String, Option<Expr>),
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

            Statement::VarDecl(name, initializer) => {
                let initializer = match initializer {
                    Some(val) => Some(val.evaluate()),
                    None => None,
                };

                println!("Variable name: {}, initializer: {:?}", name, initializer);
            }

            _ => {}
        }

        true
    }
}
