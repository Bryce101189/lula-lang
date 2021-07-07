use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Token {
    pub position: Position,
    pub kind: TokenKind,
}

impl Token {}

#[derive(Debug, Clone)]
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
    Identifier(String),
    Bool(bool),
    Number(f64),
    String(String),

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
    Eof,
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenKind> = {
        let mut map = HashMap::new();
        map.insert("true", TokenKind::Bool(true));
        map.insert("false", TokenKind::Bool(false));

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

impl Position {
    pub fn as_readable_position(&self) -> (usize, usize) {
        (self.0 + 1, self.1 + 1)
    }
}
