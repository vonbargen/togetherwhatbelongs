/// Token types for the lexer
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Literals
    Number(i64),
    String(String),
    Identifier(String),

    // Keywords
    Let,
    Fn,
    Return,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Equal,

    // Punctuation
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    Comma,

    // Special
    Whitespace,
    Newline,
    Comment(String),

    // End of file
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize, lexeme: String) -> Self {
        Token {
            token_type,
            line,
            column,
            lexeme,
        }
    }
}
