// scanner.h

#ifndef SCANNER_H
#define SCANNER_H

#include <stdio.h>
#include <stdbool.h>
#include <stdint.h>

// Token types
typedef enum {
    TOKEN_EOF,
    TOKEN_IDENTIFIER,
    TOKEN_INTEGER,
    TOKEN_REAL,
    TOKEN_STRING,
    TOKEN_PLUS,
    TOKEN_MINUS,
    TOKEN_MULTIPLY,
    TOKEN_DIVIDE,
    TOKEN_ASSIGN,
    TOKEN_SEMICOLON,
    TOKEN_COLON,
    TOKEN_COMMA,
    TOKEN_LPAREN,
    TOKEN_RPAREN,
    TOKEN_LBRACKET,
    TOKEN_RBRACKET,
    TOKEN_DOT,
    TOKEN_DOTDOT,
    TOKEN_KEYWORD_BEGIN,
    TOKEN_KEYWORD_END,
    TOKEN_KEYWORD_IF,
    TOKEN_KEYWORD_THEN,
    TOKEN_KEYWORD_ELSE,
    TOKEN_KEYWORD_WHILE,
    TOKEN_KEYWORD_DO,
    TOKEN_KEYWORD_PROGRAM,
    TOKEN_KEYWORD_VAR,
    TOKEN_KEYWORD_PROCEDURE,
    TOKEN_KEYWORD_FUNCTION,
    // Add more token types as needed
} TokenType;

typedef struct {
    TokenType type;
    char *lexeme;
    int line;
    int column;
} Token;

// Initialize the scanner with a file
bool scanner_init(const char *filename);

// Get the next token
Token scanner_next_token();

// Peek at the next token without consuming it
Token scanner_peek_token();

// Clean up the scanner
void scanner_cleanup();

#endif // SCANNER_H