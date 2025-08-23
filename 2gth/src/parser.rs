use crate::tokens::{Token, TokenType};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    /// Parse the token stream
    pub fn parse(&mut self) -> Result<(), String> {
        while !self.is_at_end() {
            self.parse_statement()?;
        }
        Ok(())
    }

    /// Parse a statement
    fn parse_statement(&mut self) -> Result<(), String> {
        match &self.peek().token_type {
            TokenType::Let => self.parse_let_statement(),
            TokenType::Fn => self.parse_function(),
            TokenType::Return => self.parse_return_statement(),
            TokenType::Comment(_) => {
                self.advance(); // consume comment
                Ok(())
            }
            TokenType::Eof => Ok(()),
            _ => {
                let token = self.advance();
                Err(format!(
                    "Unexpected token {:?} at line {}, column {}",
                    token.token_type, token.line, token.column
                ))
            }
        }
    }

    /// Parse let statement: let identifier = expression;
    fn parse_let_statement(&mut self) -> Result<(), String> {
        self.consume(TokenType::Let, "Expected 'let'")?;

        match &self.peek().token_type {
            TokenType::Identifier(_) => {
                self.advance(); // consume identifier
            }
            _ => return Err("Expected identifier after 'let'".to_string()),
        }

        self.consume(TokenType::Equal, "Expected '=' after identifier")?;

        // Parse expression (simplified - just handle numbers and strings)
        self.parse_expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;

        Ok(())
    }

    /// Parse function: fn identifier() { ... }
    fn parse_function(&mut self) -> Result<(), String> {
        self.consume(TokenType::Fn, "Expected 'fn'")?;

        match &self.peek().token_type {
            TokenType::Identifier(_) => {
                self.advance(); // consume function name
            }
            _ => return Err("Expected function name after 'fn'".to_string()),
        }

        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        self.consume(
            TokenType::LeftBrace,
            "Expected '{' after function signature",
        )?;

        // Parse function body
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            self.parse_statement()?;
        }

        self.consume(TokenType::RightBrace, "Expected '}' to close function body")?;

        Ok(())
    }

    /// Parse return statement: return expression;
    fn parse_return_statement(&mut self) -> Result<(), String> {
        self.consume(TokenType::Return, "Expected 'return'")?;

        self.parse_expression()?;

        self.consume(TokenType::Semicolon, "Expected ';' after return expression")?;

        Ok(())
    }

    /// Parse expression (simplified)
    fn parse_expression(&mut self) -> Result<(), String> {
        self.parse_addition()
    }

    /// Parse addition/subtraction
    fn parse_addition(&mut self) -> Result<(), String> {
        self.parse_multiplication()?;

        while self.match_token(&[TokenType::Plus, TokenType::Minus]) {
            self.parse_multiplication()?;
        }

        Ok(())
    }

    /// Parse multiplication/division
    fn parse_multiplication(&mut self) -> Result<(), String> {
        self.parse_primary()?;

        while self.match_token(&[TokenType::Star, TokenType::Slash]) {
            self.parse_primary()?;
        }

        Ok(())
    }

    /// Parse primary expression
    fn parse_primary(&mut self) -> Result<(), String> {
        match &self.peek().token_type {
            TokenType::Number(_) | TokenType::String(_) | TokenType::Identifier(_) => {
                self.advance();
                Ok(())
            }
            TokenType::LeftParen => {
                self.advance(); // consume '('
                self.parse_expression()?;
                self.consume(TokenType::RightParen, "Expected ')' after expression")?;
                Ok(())
            }
            _ => {
                let token = self.peek();
                Err(format!(
                    "Expected expression at line {}, column {}",
                    token.line, token.column
                ))
            }
        }
    }

    /// Utility methods
    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token_type) == std::mem::discriminant(token_type)
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().token_type, TokenType::Eof)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, String> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            let token = self.peek();
            Err(format!(
                "{} at line {}, column {} (found {:?})",
                message, token.line, token.column, token.token_type
            ))
        }
    }
}
