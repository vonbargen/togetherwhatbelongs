use crate::tokens::{Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;

pub struct Scanner<'a> {
    input: Peekable<Chars<'a>>,
    line: usize,
    column: usize,
    current_lexeme: String,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Scanner {
            input: input.chars().peekable(),
            line: 1,
            column: 1,
            current_lexeme: String::new(),
        }
    }

    /// Scans all tokens from the input
    pub fn scan_all(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            match self.next_token()? {
                Some(token) => {
                    // Skip whitespace tokens for cleaner output
                    if !matches!(token.token_type, TokenType::Whitespace | TokenType::Newline) {
                        tokens.push(token);
                    }
                }
                None => {
                    // Add EOF token
                    tokens.push(Token::new(
                        TokenType::Eof,
                        self.line,
                        self.column,
                        String::new(),
                    ));
                    break;
                }
            }
        }

        Ok(tokens)
    }

    /// Gets the next token from input with one character lookahead
    pub fn next_token(&mut self) -> Result<Option<Token>, String> {
        self.current_lexeme.clear();

        while let Some(&ch) = self.input.peek() {
            match ch {
                // Whitespace
                ' ' | '\t' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::Whitespace)));
                }

                // Newlines
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                    return Ok(Some(self.make_token(TokenType::Newline)));
                }

                '\r' => {
                    self.advance();
                    // Handle \r\n
                    if self.peek_char() == Some('\n') {
                        self.advance();
                    }
                    self.line += 1;
                    self.column = 1;
                    return Ok(Some(self.make_token(TokenType::Newline)));
                }

                // Single character tokens
                '+' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::Plus)));
                }
                '-' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::Minus)));
                }
                '*' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::Star)));
                }
                '=' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::Equal)));
                }
                '(' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::LeftParen)));
                }
                ')' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::RightParen)));
                }
                '{' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::LeftBrace)));
                }
                '}' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::RightBrace)));
                }
                ';' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::Semicolon)));
                }
                ',' => {
                    self.advance();
                    return Ok(Some(self.make_token(TokenType::Comma)));
                }

                // String literals
                '"' => {
                    return self.scan_string();
                }

                // Comments or division
                '/' => {
                    self.advance(); // consume '/'

                    match self.peek_char() {
                        // Single line comment
                        Some('/') => {
                            self.advance(); // consume second '/'
                            return self.scan_line_comment();
                        }

                        // Multi-line comment
                        Some('*') => {
                            self.advance(); // consume '*'
                            return self.scan_block_comment();
                        }

                        // Division operator
                        _ => {
                            return Ok(Some(self.make_token(TokenType::Slash)));
                        }
                    }
                }

                // Numbers
                '0'..='9' => {
                    return self.scan_number();
                }

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    return self.scan_identifier();
                }

                // Unexpected character
                _ => {
                    let ch = self.advance();
                    return Err(format!(
                        "Unexpected character '{}' at line {}, column {}",
                        ch,
                        self.line,
                        self.column - 1
                    ));
                }
            }
        }

        Ok(None) // End of input
    }

    /// Advances to next character and updates position
    fn advance(&mut self) -> char {
        let ch = self.input.next().unwrap_or('\0');
        self.current_lexeme.push(ch);
        self.column += 1;
        ch
    }

    /// Peeks at next character without consuming it
    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    /// Creates a token with current position info
    fn make_token(&self, token_type: TokenType) -> Token {
        Token::new(
            token_type,
            self.line,
            self.column - self.current_lexeme.len(),
            self.current_lexeme.clone(),
        )
    }

    /// Scans a string literal
    fn scan_string(&mut self) -> Result<Option<Token>, String> {
        self.advance(); // consume opening quote
        let mut value = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch == '"' {
                self.advance(); // consume closing quote
                return Ok(Some(self.make_token(TokenType::String(value))));
            }

            if ch == '\n' || ch == '\r' {
                return Err(format!(
                    "Unterminated string at line {}, column {}",
                    self.line, self.column
                ));
            }

            // Handle escape sequences
            if ch == '\\' {
                self.advance(); // consume backslash
                match self.peek_char() {
                    Some('n') => {
                        self.advance();
                        value.push('\n');
                    }
                    Some('t') => {
                        self.advance();
                        value.push('\t');
                    }
                    Some('r') => {
                        self.advance();
                        value.push('\r');
                    }
                    Some('\\') => {
                        self.advance();
                        value.push('\\');
                    }
                    Some('"') => {
                        self.advance();
                        value.push('"');
                    }
                    Some(c) => {
                        self.advance();
                        value.push(c);
                    }
                    None => {
                        return Err("Unexpected end of input in string escape".to_string());
                    }
                }
            } else {
                value.push(self.advance());
            }
        }

        Err("Unterminated string literal".to_string())
    }

    /// Scans a single-line comment
    fn scan_line_comment(&mut self) -> Result<Option<Token>, String> {
        let mut comment = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch == '\n' || ch == '\r' {
                break;
            }
            comment.push(self.advance());
        }

        Ok(Some(self.make_token(TokenType::Comment(comment))))
    }

    /// Scans a multi-line comment
    fn scan_block_comment(&mut self) -> Result<Option<Token>, String> {
        let mut comment = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch == '*' {
                self.advance(); // consume '*'
                comment.push('*');

                if self.peek_char() == Some('/') {
                    self.advance(); // consume '/'
                    comment.push('/');
                    return Ok(Some(self.make_token(TokenType::Comment(comment))));
                }
            } else {
                if ch == '\n' {
                    self.line += 1;
                    self.column = 1;
                }
                comment.push(self.advance());
            }
        }

        Err("Unterminated block comment".to_string())
    }

    /// Scans a number literal
    fn scan_number(&mut self) -> Result<Option<Token>, String> {
        let mut value = 0i64;

        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() {
                let digit = (self.advance() as u8 - b'0') as i64;
                value = value.saturating_mul(10).saturating_add(digit);
            } else {
                break;
            }
        }

        Ok(Some(self.make_token(TokenType::Number(value))))
    }

    /// Scans an identifier or keyword
    fn scan_identifier(&mut self) -> Result<Option<Token>, String> {
        let mut identifier = String::new();

        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(self.advance());
            } else {
                break;
            }
        }

        // Check for keywords
        let token_type = match identifier.as_str() {
            "let" => TokenType::Let,
            "fn" => TokenType::Fn,
            "return" => TokenType::Return,
            _ => TokenType::Identifier(identifier.clone()),
        };

        Ok(Some(self.make_token(token_type)))
    }
}
