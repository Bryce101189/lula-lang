use crate::expr::Expr;
use crate::token::{Position, Token, TokenKind};

pub struct Parser {
    source_path: String,
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(source_path: String, tokens: Vec<Token>) -> Parser {
        Parser {
            source_path,
            tokens,
            cursor: 0,
        }
    }

    fn display_error<S>(&self, message: S, position: Position)
    where
        S: Into<String>,
    {
        eprintln!(
            "Parsing error in file '{}', {}:\n    {}.",
            self.source_path,
            position,
            message.into()
        );
    }

    fn peek(&self) -> Token {
        self.tokens[self.cursor].clone()
    }

    fn advance(&mut self) -> Token {
        self.cursor += 1;
        self.tokens[self.cursor - 1].clone()
    }

    fn is_match(&self, kind: TokenKind) -> bool {
        self.peek().kind == kind
    }

    fn expect_closing(&mut self, kind: TokenKind) -> Option<Token> {
        let tok = self.advance();
        let expect = match kind {
            TokenKind::LeftParen => TokenKind::RightParen,
            TokenKind::LeftBrace => TokenKind::RightBrace,
            TokenKind::LeftBracket => TokenKind::RightBracket,

            _ => {
                self.display_error(
                    format!("Could not find complementary type for token {:?}", tok.kind),
                    tok.position,
                );
                return None;
            }
        };

        if tok.kind != expect {
            self.display_error(
                format!(
                    "Expected token of type {:?}; found token of type {:?} instead",
                    expect, tok.kind
                ),
                tok.position,
            );
            return None;
        }

        Some(tok)
    }

    fn parse_primary(&mut self) -> Option<Expr> {
        let tok = self.advance();

        match tok.kind {
            TokenKind::Literal(l) => Some(Expr::Literal(l)),

            TokenKind::LeftParen | TokenKind::LeftBrace | TokenKind::LeftBracket => {
                let expr = self.parse_expr()?;
                let rhs = self.expect_closing(tok.kind.clone())?;

                Some(Expr::Grouping(tok, Box::new(expr), rhs))
            }

            _ => None,
        }
    }

    fn parse_unary(&mut self) -> Option<Expr> {
        while self.is_match(TokenKind::Bang) || self.is_match(TokenKind::Minus) {
            let op = self.advance();
            let rhs = self.parse_unary()?;

            return Some(Expr::Unary(op, Box::new(rhs)));
        }

        self.parse_primary()
    }

    fn parse_factor(&mut self) -> Option<Expr> {
        let mut expr = self.parse_unary()?;

        while self.is_match(TokenKind::Star)
            || self.is_match(TokenKind::Slash)
            || self.is_match(TokenKind::Percent)
        {
            let op = self.advance();
            let rhs = self.parse_unary()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        Some(expr)
    }

    fn parse_term(&mut self) -> Option<Expr> {
        let mut expr = self.parse_factor()?;

        while self.is_match(TokenKind::Plus) || self.is_match(TokenKind::Minus) {
            let op = self.advance();
            let rhs = self.parse_factor()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        Some(expr)
    }

    fn parse_comparison(&mut self) -> Option<Expr> {
        let mut expr = self.parse_term()?;

        while self.is_match(TokenKind::Less)
            || self.is_match(TokenKind::LessEqual)
            || self.is_match(TokenKind::Greater)
            || self.is_match(TokenKind::GreaterEqual)
        {
            let op = self.advance();
            let rhs = self.parse_term()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        Some(expr)
    }

    fn parse_equality(&mut self) -> Option<Expr> {
        let mut expr = self.parse_comparison()?;

        while self.is_match(TokenKind::EqualEqual) || self.is_match(TokenKind::BangEqual) {
            let op = self.advance();
            let rhs = self.parse_comparison()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        Some(expr)
    }

    fn parse_and(&mut self) -> Option<Expr> {
        let mut expr = self.parse_equality()?;

        while self.is_match(TokenKind::And) {
            let op = self.advance();
            let rhs = self.parse_equality()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        Some(expr)
    }

    fn parse_or(&mut self) -> Option<Expr> {
        let mut expr = self.parse_and()?;

        while self.is_match(TokenKind::Or) {
            let op = self.advance();
            let rhs = self.parse_and()?;
            expr = Expr::Binary(Box::new(expr), op, Box::new(rhs));
        }

        Some(expr)
    }

    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_or()
    }
}
