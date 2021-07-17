use crate::token::{Literal, Position, Token, TokenKind, KEYWORDS};

pub struct Lexer {
    source_path: String,
    source: String,
    cursor: usize,
    position: Position,
}

impl Lexer {
    pub fn new(source_path: String, source: String) -> Lexer {
        Lexer {
            source_path,
            source,
            cursor: 0,
            position: Position(0, 0),
        }
    }

    fn display_error<S>(&self, message: S, position: Position)
    where
        S: Into<String>,
    {
        eprintln!(
            "Lexing error in file '{}', {}:\n    {}.",
            self.source_path,
            position,
            message.into()
        );
    }

    fn reached_end(&self) -> bool {
        self.source.len() <= self.cursor
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.cursor).unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        // Update position
        if self.peek() == '\n' {
            // Reset column and advance line by one
            self.position.1 = 0;
            self.position.0 += 1;
        } else {
            // Advance column by one
            self.position.1 += 1;
        }

        // Advance cursor and return previous character
        self.cursor += 1;
        self.source.chars().nth(self.cursor - 1).unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while !self.reached_end() && self.peek().is_ascii_whitespace() {
            self.advance();
        }
    }

    fn skip_line(&mut self) {
        while !self.reached_end() && self.peek() != '\n' {
            self.advance();
        }
    }

    fn collect_identifier(&mut self) -> Option<Token> {
        let mut lexemme = String::new();
        let start_pos = self.position;

        while !self.reached_end()
            && (self.peek().is_ascii_alphabetic()
                || self.peek().is_ascii_digit()
                || self.peek() == '_')
        {
            lexemme.push(self.advance());
        }

        // Get token kind
        let token_kind = match KEYWORDS.get(lexemme.as_str()) {
            Some(t) => t.clone(),
            None => TokenKind::Literal(Literal::Identifier(lexemme)),
        };

        Some(Token {
            kind: token_kind,
            position: start_pos,
        })
    }

    fn collect_number(&mut self) -> Option<Token> {
        let mut lexemme = String::new();
        let start_pos = self.position;

        let mut has_period = false;

        while !self.reached_end()
            && (self.peek().is_ascii_digit() || self.peek() == '.' && !has_period)
        {
            if self.peek() == '.' {
                has_period = true;
            }

            lexemme.push(self.advance());
        }

        // Parse lexemme as f64
        let value: f64 = match lexemme.parse() {
            Ok(v) => v,
            Err(..) => {
                self.display_error(format!("Failed to parse number '{}'", lexemme), start_pos);
                return None;
            }
        };

        Some(Token {
            kind: TokenKind::Literal(Literal::Number(value)),
            position: start_pos,
        })
    }

    fn collect_string(&mut self) -> Option<Token> {
        let start_pos = self.position;

        let mut lexemme = String::new();
        let mut esc_pos = start_pos;

        self.advance(); // Consume leading double-quote

        let mut escaped = false;

        while !self.reached_end() && self.peek() != '"' {
            let curr_pos = self.position;
            let mut c = self.advance(); // Get next char in string

            // Match escape sequences
            if escaped {
                c = match c {
                    '\\' => '\\',
                    '\n' => '\n',
                    '"' => '"',

                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',

                    '0' => '\0',

                    _ => {
                        self.display_error(
                            format!("Unrecognized escape sequence '\\{}'", c),
                            esc_pos,
                        );
                        return None;
                    }
                };

                escaped = false;
            } else if c == '\\' {
                escaped = true;
                esc_pos = curr_pos;
                continue; // Ignore escape char
            } else {
                // Disallow multi-line strings
                if c == '\n' {
                    self.display_error(
                        "Encountered unexpected newline character while scanning string literal",
                        curr_pos,
                    );
                    return None;
                }
            }

            lexemme.push(c);
        }

        if self.reached_end() {
            self.display_error(
                "Failed to locate closing double-quote for string literal",
                start_pos,
            );
            return None;
        }

        self.advance(); // Consume trailing double-quote

        Some(Token {
            kind: TokenKind::Literal(Literal::String(lexemme)),
            position: start_pos,
        })
    }

    fn collect_symbol(&mut self) -> Option<Token> {
        let start_pos = self.position;
        let c = self.advance();

        let token_kind = match c {
            // Single character tokens
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,

            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,

            // Single and double character tokens
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }

            // Control characters
            '\0' => TokenKind::Eof,

            // Unrecognized character
            _ => {
                self.display_error(format!("Encountered unrecognized symbol {}", c), start_pos);
                return None;
            }
        };

        Some(Token {
            kind: token_kind,
            position: start_pos,
        })
    }

    pub fn collect_tokens(&mut self) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.reached_end() {
            // Skip whitespace
            self.skip_whitespace();

            // Skip comment line
            if self.peek() == '#' {
                self.skip_line();
                self.skip_whitespace();
            }

            // Collect token by type
            let c = self.peek();
            let token = match c {
                'a'..='z' | 'A'..='Z' => self.collect_identifier(),
                '0'..='9' => self.collect_number(),
                '"' => self.collect_string(),
                _ => self.collect_symbol(),
            };

            match token {
                Some(t) => tokens.push(t),
                None => return None,
            }
        }

        Some(tokens)
    }
}
