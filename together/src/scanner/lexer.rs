use super::token::{Token, TokenType};

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace();

            if self.is_at_end() {
                tokens.push(Token::new(
                    TokenType::Eof,
                    String::new(),
                    self.line,
                    self.column,
                ));
                break;
            }

            // Kommentare überspringen
            if self.peek() == '(' && self.peek_next() == Some('*') {
                self.skip_comment()?;
                continue;
            }

            let token = self.next_token()?;
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        let start_line = self.line;
        let start_column = self.column;
        let ch = self.advance();

        match ch {
            // Einzel-Zeichen-Operatoren
            '+' => Ok(Token::new(TokenType::Plus, "+".to_string(), start_line, start_column)),
            '-' => Ok(Token::new(TokenType::Minus, "-".to_string(), start_line, start_column)),
            '*' => Ok(Token::new(TokenType::Times, "*".to_string(), start_line, start_column)),
            '/' => Ok(Token::new(TokenType::Slash, "/".to_string(), start_line, start_column)),
            '&' => Ok(Token::new(TokenType::Ampersand, "&".to_string(), start_line, start_column)),
            '~' => Ok(Token::new(TokenType::Tilde, "~".to_string(), start_line, start_column)),
            '#' => Ok(Token::new(TokenType::NotEqual, "#".to_string(), start_line, start_column)),
            '^' => Ok(Token::new(TokenType::Caret, "^".to_string(), start_line, start_column)),
            ';' => Ok(Token::new(TokenType::Semicolon, ";".to_string(), start_line, start_column)),
            ',' => Ok(Token::new(TokenType::Comma, ",".to_string(), start_line, start_column)),
            '|' => Ok(Token::new(TokenType::Bar, "|".to_string(), start_line, start_column)),
            '(' => Ok(Token::new(TokenType::LParen, "(".to_string(), start_line, start_column)),
            ')' => Ok(Token::new(TokenType::RParen, ")".to_string(), start_line, start_column)),
            '[' => Ok(Token::new(TokenType::LBracket, "[".to_string(), start_line, start_column)),
            ']' => Ok(Token::new(TokenType::RBracket, "]".to_string(), start_line, start_column)),
            '{' => Ok(Token::new(TokenType::LBrace, "{".to_string(), start_line, start_column)),
            '}' => Ok(Token::new(TokenType::RBrace, "}".to_string(), start_line, start_column)),
            
            // Mehrdeutige Zeichen
            ':' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::new(TokenType::Assign, ":=".to_string(), start_line, start_column))
                } else {
                    Ok(Token::new(TokenType::Colon, ":".to_string(), start_line, start_column))
                }
            }
            '.' => {
                if self.peek() == '.' {
                    self.advance();
                    Ok(Token::new(TokenType::DotDot, "..".to_string(), start_line, start_column))
                } else {
                    Ok(Token::new(TokenType::Period, ".".to_string(), start_line, start_column))
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::new(TokenType::LessEqual, "<=".to_string(), start_line, start_column))
                } else {
                    Ok(Token::new(TokenType::Less, "<".to_string(), start_line, start_column))
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.advance();
                    Ok(Token::new(TokenType::GreaterEqual, ">=".to_string(), start_line, start_column))
                } else {
                    Ok(Token::new(TokenType::Greater, ">".to_string(), start_line, start_column))
                }
            }
            '=' => Ok(Token::new(TokenType::Equal, "=".to_string(), start_line, start_column)),

            // Strings
            '"' => self.scan_string(start_line, start_column),

            // Zahlen
            _ if ch.is_ascii_digit() => self.scan_number(ch, start_line, start_column),

            // Identifikatoren und Schlüsselwörter
            _ if ch.is_alphabetic() => self.scan_identifier(ch, start_line, start_column),

            _ => Err(format!(
                "Unerwartetes Zeichen '{}' in Zeile {}, Spalte {}",
                ch, start_line, start_column
            )),
        }
    }

    fn scan_identifier(&mut self, first: char, line: usize, column: usize) -> Result<Token, String> {
        let mut lexeme = String::new();
        lexeme.push(first);

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            lexeme.push(self.advance());
        }

        // Prüfe ob es ein Schlüsselwort ist
        let token_type = Token::keyword(&lexeme)
            .unwrap_or_else(|| TokenType::Identifier(lexeme.clone()));

        Ok(Token::new(token_type, lexeme, line, column))
    }

    fn scan_number(&mut self, first: char, line: usize, column: usize) -> Result<Token, String> {
        let mut lexeme = String::new();
        lexeme.push(first);

        // Lese alle Ziffern oder Hex-Ziffern
        while !self.is_at_end() && self.peek().is_ascii_hexdigit() {
            lexeme.push(self.advance());
        }

        // Prüfe auf Hexadezimal (endet mit 'H')
        if !self.is_at_end() && self.peek() == 'H' {
            lexeme.push(self.advance());
            let hex_str = &lexeme[..lexeme.len() - 1];
            match i64::from_str_radix(hex_str, 16) {
                Ok(val) => Ok(Token::new(TokenType::IntegerLiteral(val), lexeme, line, column)),
                Err(_) => Err(format!("Ungültige Hexadezimalzahl: {} in Zeile {}", lexeme, line)),
            }
        } 
        // Prüfe auf Fließkommazahl
        else if !self.is_at_end() && self.peek() == '.' && self.peek_ahead(1) != Some('.') {
            lexeme.push(self.advance());
            
            // Nachkommastellen
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                lexeme.push(self.advance());
            }

            // Optional: Skalierungsfaktor (E oder e)
            if !self.is_at_end() && (self.peek() == 'E' || self.peek() == 'e') {
                lexeme.push(self.advance());
                
                if !self.is_at_end() && (self.peek() == '+' || self.peek() == '-') {
                    lexeme.push(self.advance());
                }

                while !self.is_at_end() && self.peek().is_ascii_digit() {
                    lexeme.push(self.advance());
                }
            }

            match lexeme.parse::<f64>() {
                Ok(val) => Ok(Token::new(TokenType::RealLiteral(val), lexeme, line, column)),
                Err(_) => Err(format!("Ungültige Fließkommazahl: {} in Zeile {}", lexeme, line)),
            }
        }
        // Prüfe auf Zeichen-Literal (z.B. 0AH für newline, 22X für ")
        else if !self.is_at_end() && self.peek() == 'X' {
            lexeme.push(self.advance());
            let hex_str = &lexeme[..lexeme.len() - 1];
            match u32::from_str_radix(hex_str, 16) {
                Ok(val) => {
                    if let Some(ch) = char::from_u32(val) {
                        let string_val = ch.to_string();
                        Ok(Token::new(TokenType::StringLiteral(string_val), lexeme, line, column))
                    } else {
                        Err(format!("Ungültiger Zeichen-Code: {} in Zeile {}", lexeme, line))
                    }
                },
                Err(_) => Err(format!("Ungültiges Zeichen-Literal: {} in Zeile {}", lexeme, line)),
            }
        }
        // Normale Ganzzahl
        else {
            match lexeme.parse::<i64>() {
                Ok(val) => Ok(Token::new(TokenType::IntegerLiteral(val), lexeme, line, column)),
                Err(_) => Err(format!("Ungültige Ganzzahl: {} in Zeile {}", lexeme, line)),
            }
        }
    }

    fn scan_string(&mut self, line: usize, column: usize) -> Result<Token, String> {
        let mut value = String::new();
        let mut lexeme = String::from('"');

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                return Err(format!(
                    "Unerwartetes Zeilenende in String in Zeile {}",
                    line
                ));
            }
            let ch = self.advance();
            value.push(ch);
            lexeme.push(ch);
        }

        if self.is_at_end() {
            return Err(format!("Nicht geschlossener String in Zeile {}", line));
        }

        // Schließendes "
        self.advance();
        lexeme.push('"');

        Ok(Token::new(
            TokenType::StringLiteral(value),
            lexeme,
            line,
            column,
        ))
    }

    fn skip_comment(&mut self) -> Result<(), String> {
        let start_line = self.line;
        
        // Überspringe '(*'
        self.advance();
        self.advance();

        let mut depth = 1;

        while !self.is_at_end() && depth > 0 {
            if self.peek() == '(' && self.peek_next() == Some('*') {
                self.advance();
                self.advance();
                depth += 1;
            } else if self.peek() == '*' && self.peek_next() == Some(')') {
                self.advance();
                self.advance();
                depth -= 1;
            } else {
                self.advance();
            }
        }

        if depth > 0 {
            return Err(format!(
                "Nicht geschlossener Kommentar, begonnen in Zeile {}",
                start_line
            ));
        }

        Ok(())
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                }
                _ => break,
            }
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.input[self.position];
        self.position += 1;
        self.column += 1;
        ch
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input[self.position]
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.position + 1 >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + 1])
        }
    }

    fn peek_ahead(&self, offset: usize) -> Option<char> {
        if self.position + offset >= self.input.len() {
            None
        } else {
            Some(self.input[self.position + offset])
        }
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords() {
        let mut lexer = Lexer::new("MODULE BEGIN END");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Module);
        assert_eq!(tokens[1].token_type, TokenType::Begin);
        assert_eq!(tokens[2].token_type, TokenType::End);
    }

    #[test]
    fn test_identifiers() {
        let mut lexer = Lexer::new("myVar x123");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::Identifier(_)));
        assert!(matches!(tokens[1].token_type, TokenType::Identifier(_)));
    }

    #[test]
    fn test_numbers() {
        let mut lexer = Lexer::new("42 3.14 0FFH");
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(tokens[0].token_type, TokenType::IntegerLiteral(42)));
        assert!(matches!(tokens[1].token_type, TokenType::RealLiteral(_)));
        assert!(matches!(tokens[2].token_type, TokenType::IntegerLiteral(255)));
    }

    #[test]
    fn test_operators() {
        let mut lexer = Lexer::new(":= <= >= #");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Assign);
        assert_eq!(tokens[1].token_type, TokenType::LessEqual);
        assert_eq!(tokens[2].token_type, TokenType::GreaterEqual);
        assert_eq!(tokens[3].token_type, TokenType::NotEqual);
    }

    #[test]
    fn test_string() {
        let mut lexer = Lexer::new(r#""Hello World""#);
        let tokens = lexer.tokenize().unwrap();
        
        assert!(matches!(
            tokens[0].token_type,
            TokenType::StringLiteral(ref s) if s == "Hello World"
        ));
    }

    #[test]
    fn test_comment() {
        let mut lexer = Lexer::new("BEGIN (* Dies ist ein Kommentar *) END");
        let tokens = lexer.tokenize().unwrap();
        
        assert_eq!(tokens[0].token_type, TokenType::Begin);
        assert_eq!(tokens[1].token_type, TokenType::End);
        assert_eq!(tokens.len(), 3); // BEGIN, END, EOF
    }
}