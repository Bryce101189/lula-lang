use crate::error::display_general_error;
use crate::token::{Literal, Token, TokenKind};

#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping(Token, Box<Expr>, Token),
}

impl Expr {
    pub fn evaluate(&self) -> Option<Literal> {
        match self {
            Expr::Literal(..) => self.evaluate_literal(),
            Expr::Unary(..) => self.evaluate_unary(),
            Expr::Binary(..) => self.evaluate_binary(),
            Expr::Grouping(..) => self.evaluate_grouping(),
        }
    }

    fn evaluate_literal(&self) -> Option<Literal> {
        match self {
            Expr::Literal(l) => Some(l.clone()),
            _ => None,
        }
    }

    fn evaluate_unary(&self) -> Option<Literal> {
        let (op, expr) = match self {
            Expr::Unary(o, e) => (o, e),
            _ => unreachable!(),
        };

        // Evaluate inner expresion
        let lit = match expr.evaluate() {
            Some(e) => e,
            None => return None,
        };

        // Apply operations
        match op.kind {
            TokenKind::Minus => match lit {
                Literal::Number(val) => Some(Literal::Number(-val)),
                _ => {
                    display_general_error(
                        "Type",
                        format!("Could not apply operation {:?} on type {:?}", op.kind, lit),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::Bang => match lit {
                Literal::Bool(val) => Some(Literal::Bool(!val)),
                _ => {
                    display_general_error(
                        "Type",
                        format!("Could not apply operation {:?} on type {:?}", op.kind, lit),
                        op.position,
                    );
                    None
                }
            },
            _ => unreachable!(),
        }
    }

    fn evaluate_binary(&self) -> Option<Literal> {
        let (lhs, op, rhs) = match self {
            Expr::Binary(l, o, r) => (l, o, r),
            _ => unreachable!(),
        };

        // Evaluate outer expressions
        let left_lit = match lhs.evaluate() {
            Some(e) => e,
            None => return None,
        };

        let right_lit = match rhs.evaluate() {
            Some(e) => e,
            None => return None,
        };

        // Apply operations
        match op.kind {
            // Mathematical operations
            TokenKind::Plus => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Number(left_val + right_val))
                }
                (Literal::String(left_val), Literal::String(right_val)) => {
                    Some(Literal::String(left_val + right_val.as_str()))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::Minus => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Number(left_val - right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::Star => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Number(left_val * right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::Slash => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Number(left_val / right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::Percent => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Number(left_val % right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },

            // Numeric comparisons
            TokenKind::Less => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Bool(left_val < right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::LessEqual => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Bool(left_val <= right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::Greater => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Bool(left_val > right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::GreaterEqual => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Bool(left_val >= right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },

            // Comparsions
            TokenKind::EqualEqual => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Bool(left_val == right_val))
                }
                (Literal::Bool(left_val), Literal::Bool(right_val)) => {
                    Some(Literal::Bool(left_val == right_val))
                }
                (Literal::String(left_val), Literal::String(right_val)) => {
                    Some(Literal::Bool(left_val == right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            TokenKind::BangEqual => match (left_lit.clone(), right_lit.clone()) {
                (Literal::Number(left_val), Literal::Number(right_val)) => {
                    Some(Literal::Bool(left_val != right_val))
                }
                (Literal::Bool(left_val), Literal::Bool(right_val)) => {
                    Some(Literal::Bool(left_val != right_val))
                }
                (Literal::String(left_val), Literal::String(right_val)) => {
                    Some(Literal::Bool(left_val != right_val))
                }
                _ => {
                    display_general_error(
                        "Type",
                        format!(
                            "Could not apply operation {:?} on types {:?} and {:?}",
                            op.kind, left_lit, right_lit
                        ),
                        op.position,
                    );
                    None
                }
            },
            _ => unreachable!(),
        }
    }

    fn evaluate_grouping(&self) -> Option<Literal> {
        let (_, expr, _) = match self {
            Expr::Grouping(l, e, r) => (l, e, r),
            _ => unreachable!(),
        };

        expr.evaluate()
    }
}
