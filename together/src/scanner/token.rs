use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Schlüsselwörter
    Array,
    Begin,
    By,
    Case,
    Const,
    Div,
    Do,
    Else,
    Elsif,
    End,
    False,
    For,
    If,
    Import,
    In,
    Is,
    Mod,
    Module,
    Nil,
    Of,
    Or,
    Pointer,
    Procedure,
    Record,
    Repeat,
    Return,
    Then,
    To,
    True,
    Type,
    Until,
    Var,
    While,

    // Literale
    Identifier(String),
    IntegerLiteral(i64),
    RealLiteral(f64),
    StringLiteral(String),

    // Operatoren
    Plus,           // +
    Minus,          // -
    Times,          // *
    Slash,          // /
    Ampersand,      // &
    Tilde,          // ~
    
    // Vergleichsoperatoren
    Equal,          // =
    NotEqual,       // #
    Less,           // <
    LessEqual,      // <=
    Greater,        // >
    GreaterEqual,   // >=

    // Trennzeichen
    Assign,         // :=
    Colon,          // :
    Semicolon,      // ;
    Comma,          // ,
    Period,         // .
    DotDot,         // ..
    Bar,            // |
    Caret,          // ^

    // Klammern
    LParen,         // (
    RParen,         // )
    LBracket,       // [
    RBracket,       // ]
    LBrace,         // {
    RBrace,         // }

    // Spezielle Token
    Eof,            // End of File
    Comment,        // Kommentar (optional, wenn behalten)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            lexeme,
            line,
            column,
        }
    }

    pub fn keyword(keyword: &str) -> Option<TokenType> {
        match keyword {
            "ARRAY" => Some(TokenType::Array),
            "BEGIN" => Some(TokenType::Begin),
            "BY" => Some(TokenType::By),
            "CASE" => Some(TokenType::Case),
            "CONST" => Some(TokenType::Const),
            "DIV" => Some(TokenType::Div),
            "DO" => Some(TokenType::Do),
            "ELSE" => Some(TokenType::Else),
            "ELSIF" => Some(TokenType::Elsif),
            "END" => Some(TokenType::End),
            "FALSE" => Some(TokenType::False),
            "FOR" => Some(TokenType::For),
            "IF" => Some(TokenType::If),
            "IMPORT" => Some(TokenType::Import),
            "IN" => Some(TokenType::In),
            "IS" => Some(TokenType::Is),
            "MOD" => Some(TokenType::Mod),
            "MODULE" => Some(TokenType::Module),
            "NIL" => Some(TokenType::Nil),
            "OF" => Some(TokenType::Of),
            "OR" => Some(TokenType::Or),
            "POINTER" => Some(TokenType::Pointer),
            "PROCEDURE" => Some(TokenType::Procedure),
            "RECORD" => Some(TokenType::Record),
            "REPEAT" => Some(TokenType::Repeat),
            "RETURN" => Some(TokenType::Return),
            "THEN" => Some(TokenType::Then),
            "TO" => Some(TokenType::To),
            "TRUE" => Some(TokenType::True),
            "TYPE" => Some(TokenType::Type),
            "UNTIL" => Some(TokenType::Until),
            "VAR" => Some(TokenType::Var),
            "WHILE" => Some(TokenType::While),
            _ => None,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{}:{}] {:?} '{}'",
            self.line, self.column, self.token_type, self.lexeme
        )
    }
}
