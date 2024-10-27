// scanner.c

#include "scanner.h"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <wchar.h>

#define MAX_IDENTIFIER_LENGTH 255
#define MAX_STRING_LENGTH 1024

static FILE *file = NULL;
static int current_line = 1;
static int current_column = 0;
static Token peeked_token = {TOKEN_EOF, NULL, 0, 0};
static bool has_peeked = false;

static char32_t current_char = 0;
static char32_t next_char = 0;

// UTF-8 helper functions
static char32_t read_utf8_char(FILE *file) {
    char buffer[4];
    int bytes_read = 0;
    int c = fgetc(file);
    
    if (c == EOF) return 0;
    
    buffer[bytes_read++] = c;
    
    if ((c & 0x80) == 0) {
        return c;
    } else if ((c & 0xE0) == 0xC0) {
        bytes_read += fread(buffer + bytes_read, 1, 1, file);
    } else if ((c & 0xF0) == 0xE0) {
        bytes_read += fread(buffer + bytes_read, 1, 2, file);
    } else if ((c & 0xF8) == 0xF0) {
        bytes_read += fread(buffer + bytes_read, 1, 3, file);
    } else {
        return 0; // Invalid UTF-8 sequence
    }
    
    char32_t result;
    mbrtoc32(&result, buffer, bytes_read, NULL);
    return result;
}

static void advance() {
    if (current_char == '\n') {
        current_line++;
        current_column = 0;
    } else {
        current_column++;
    }
    current_char = next_char;
    next_char = read_utf8_char(file);
}

bool scanner_init(const char *filename) {
    file = fopen(filename, "r");
    if (!file) return false;
    
    current_char = read_utf8_char(file);
    next_char = read_utf8_char(file);
    return true;
}

static bool is_alpha(char32_t c) {
    return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

static bool is_digit(char32_t c) {
    return c >= '0' && c <= '9';
}

static bool is_alnum(char32_t c) {
    return is_alpha(c) || is_digit(c);
}

static Token make_token(TokenType type, const char *lexeme) {
    Token token;
    token.type = type;
    token.lexeme = strdup(lexeme);
    token.line = current_line;
    token.column = current_column - strlen(lexeme);
    return token;
}

static Token scan_identifier() {
    char lexeme[MAX_IDENTIFIER_LENGTH + 1];
    int length = 0;
    
    while (is_alnum(current_char) && length < MAX_IDENTIFIER_LENGTH) {
        lexeme[length++] = current_char;
        advance();
    }
    lexeme[length] = '\0';
    
    // Check for keywords
    if (strcmp(lexeme, "begin") == 0) return make_token(TOKEN_KEYWORD_BEGIN, lexeme);
    if (strcmp(lexeme, "end") == 0) return make_token(TOKEN_KEYWORD_END, lexeme);
    if (strcmp(lexeme, "if") == 0) return make_token(TOKEN_KEYWORD_IF, lexeme);
    if (strcmp(lexeme, "then") == 0) return make_token(TOKEN_KEYWORD_THEN, lexeme);
    if (strcmp(lexeme, "else") == 0) return make_token(TOKEN_KEYWORD_ELSE, lexeme);
    if (strcmp(lexeme, "while") == 0) return make_token(TOKEN_KEYWORD_WHILE, lexeme);
    if (strcmp(lexeme, "do") == 0) return make_token(TOKEN_KEYWORD_DO, lexeme);
    if (strcmp(lexeme, "program") == 0) return make_token(TOKEN_KEYWORD_PROGRAM, lexeme);
    if (strcmp(lexeme, "var") == 0) return make_token(TOKEN_KEYWORD_VAR, lexeme);
    if (strcmp(lexeme, "procedure") == 0) return make_token(TOKEN_KEYWORD_PROCEDURE, lexeme);
    if (strcmp(lexeme, "function") == 0) return make_token(TOKEN_KEYWORD_FUNCTION, lexeme);
    
    return make_token(TOKEN_IDENTIFIER, lexeme);
}

static Token scan_number() {
    char lexeme[MAX_IDENTIFIER_LENGTH + 1];
    int length = 0;
    bool is_real = false;
    
    while (is_digit(current_char) || current_char == '.') {
        if (current_char == '.') {
            if (is_real) break; // Second decimal point, exit loop
            is_real = true;
        }
        lexeme[length++] = current_char;
        advance();
    }
    lexeme[length] = '\0';
    
    return make_token(is_real ? TOKEN_REAL : TOKEN_INTEGER, lexeme);
}

static Token scan_string() {
    char lexeme[MAX_STRING_LENGTH + 1];
    int length = 0;
    
    advance(); // Skip opening quote
    while (current_char != '\'' && current_char != 0 && length < MAX_STRING_LENGTH) {
        lexeme[length++] = current_char;
        advance();
    }
    lexeme[length] = '\0';
    
    if (current_char == '\'') {
        advance(); // Skip closing quote
        return make_token(TOKEN_STRING, lexeme);
    } else {
        // Unterminated string
        return make_token(TOKEN_EOF, "Unterminated string");
    }
}

Token scanner_next_token() {
    if (has_peeked) {
        has_peeked = false;
        return peeked_token;
    }
    
    while (isspace(current_char)) {
        advance();
    }
    
    if (current_char == 0) {
        return make_token(TOKEN_EOF, "EOF");
    }
    
    if (is_alpha(current_char)) {
        return scan_identifier();
    }
    
    if (is_digit(current_char)) {
        return scan_number();
    }
    
    if (current_char == '\'') {
        return scan_string();
    }
    
    char lexeme[2] = {current_char, '\0'};
    advance();
    
    switch (lexeme[0]) {
        case '+': return make_token(TOKEN_PLUS, lexeme);
        case '-': return make_token(TOKEN_MINUS, lexeme);
        case '*': return make_token(TOKEN_MULTIPLY, lexeme);
        case '/': return make_token(TOKEN_DIVIDE, lexeme);
        case ':':
            if (current_char == '=') {
                lexeme[1] = current_char;
                lexeme[2] = '\0';
                advance();
                return make_token(TOKEN_ASSIGN, lexeme);
            }
            return make_token(TOKEN_COLON, lexeme);
        case ';': return make_token(TOKEN_SEMICOLON, lexeme);
        case ',': return make_token(TOKEN_COMMA, lexeme);
        case '(': return make_token(TOKEN_LPAREN, lexeme);
        case ')': return make_token(TOKEN_RPAREN, lexeme);
        case '[': return make_token(TOKEN_LBRACKET, lexeme);
        case ']': return make_token(TOKEN_RBRACKET, lexeme);
        case '.':
            if (current_char == '.') {
                lexeme[1] = current_char;
                lexeme[2] = '\0';
                advance();
                return make_token(TOKEN_DOTDOT, lexeme);
            }
            return make_token(TOKEN_DOT, lexeme);
        default:
            return make_token(TOKEN_EOF, "Unknown token");
    }
}

Token scanner_peek_token() {
    if (!has_peeked) {
        peeked_token = scanner_next_token();
        has_peeked = true;
    }
    return peeked_token;
}

void scanner_cleanup() {
    if (file) {
        fclose(file);
        file = NULL;
    }
    if (peeked_token.lexeme) {
        free(peeked_token.lexeme);
        peeked_token.lexeme = NULL;
    }
}