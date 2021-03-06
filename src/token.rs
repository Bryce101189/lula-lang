use core::fmt;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone)]
pub struct Token {
    pub position: Position,
    pub kind: TokenKind,
}

impl Token {}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Single character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    // Single and double character tokens
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    // Literals
    Literal(Literal),

    // Keywords
    If,
    Elif,
    Else,

    And,
    Or,

    Func,
    Let,

    Loop,
    Break,
    Continue,

    Print,

    // Control tokens
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let val = match self {
            Literal::Identifier(val) => val.clone(),
            Literal::String(val) => val.clone(),
            Literal::Number(val) => val.to_string(),
            Literal::Bool(val) => val.to_string(),
            Literal::Nil => String::from("nil"),
        };

        write!(f, "{}", val)
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenKind> = {
        let mut map = HashMap::new();

        map.insert("true", TokenKind::Literal(Literal::Bool(true)));
        map.insert("false", TokenKind::Literal(Literal::Bool(false)));
        map.insert("nil", TokenKind::Literal(Literal::Nil));

        map.insert("if", TokenKind::If);
        map.insert("elif", TokenKind::Elif);
        map.insert("else", TokenKind::Else);

        map.insert("and", TokenKind::And);
        map.insert("or", TokenKind::Or);

        map.insert("func", TokenKind::Func);
        map.insert("let", TokenKind::Let);

        map.insert("loop", TokenKind::Loop);
        map.insert("break", TokenKind::Break);
        map.insert("continue", TokenKind::Continue);

        map.insert("print", TokenKind::Print);

        map
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Position(pub usize, pub usize);

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}, column {}", self.0 + 1, self.1 + 1)
    }
}
