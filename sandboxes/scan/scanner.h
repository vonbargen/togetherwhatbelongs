// scanner.h

#ifndef SCANNER_H
#define SCANNER_H

#include <stdbool.h>
#include <stdint.h>

typedef enum {
    T_EOF = 0,
    T_ID,
    T_INTEGER,
    T_REAL,
    T_STRING,
    T_PLUS = 5,
    T_MINUS,
    T_STAR,
    T_SLASH,
    T_ASSIGN,
    T_SEMI = 10,
    T_COLON,
    T_COMMA,
    T_LPAREN,
    T_RPAREN,
    T_LBRACKET = 15,
    T_RBRACKET,
    T_DOT,
    T_DOTDOT,
    T_ARRAY,
    T_BEGIN = 20,
    T_BY,
    T_CASE,
    T_CONST,
    T_DIV,
    T_DO = 25,
    T_ELSE,
    T_ELSIF,
    T_END,
    T_EXIT,
    T_FOR = 30,
    T_IF,
    T_IMPORT,
    T_IN,
    T_IS,
    T_LOOP = 35,
    T_MOD,
    T_MODULE,
    T_NIL,
    T_OF,
    T_OR = 40,
    T_POINTER,
    T_PROCEDURE,
    T_RECORD,
    T_REPEAT,
    T_RETURN = 45,
    T_THEN,
    T_TO,
    T_TYPE,
    T_UNTIL,
    T_VAR = 50,
    T_WHILE,
    T_WITH,
    T_BOOLEAN,
    T_CHAR,
    T_FALSE = 55,
    T_INTEGER_KW,
    T_NEW,
    T_REAL_KW,
    T_TRUE,
    T_AMPERSAND = 60,
    T_ARROW,
    T_BAR,
    T_EQU,
    T_GT,
    T_GTE = 65,
    T_LBRACE,
    T_LT,
    T_LTE,
    T_NEQ,
    T_RBRACE = 70,
    T_TILDE
    // Add any other tokens you need
} TokenType;

typedef struct {
    TokenType type;
    char *lexeme;
    int line;
    int column;
} Token;

bool scanner_init(const char *filename);
Token scanner_next_token(void);
Token scanner_peek_token(void);
void scanner_cleanup(void);

#endif // SCANNER_H
