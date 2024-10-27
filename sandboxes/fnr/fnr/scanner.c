// scanner.c

#include "scanner.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <locale.h>
#include <wchar.h>
#include <wctype.h>

#define MAX_LEXEME_LENGTH 256

static FILE *file = NULL;
static wint_t current_char = WEOF;
static wint_t next_char = WEOF;
static int current_line = 1;
static int current_column = 0;
static Token peeked_token = {TOKEN_EOF, NULL, 0, 0};
static bool has_peeked = false;

static void advance(void) {
    if (current_char == L'\n') {
        current_line++;
        current_column = 0;
    } else {
        current_column++;
    }
    current_char = next_char;
    next_char = fgetwc(file);
}

bool scanner_init(const char *filename) {
    setlocale(LC_ALL, "en_US.UTF-8");
    file = fopen(filename, "r, ccs=UTF-8");
    if (!file) return false;
    
    current_char = fgetwc(file);
    next_char = fgetwc(file);
    return true;
}

static bool is_alpha(wint_t c) {
    return iswalpha(c) || c == L'_';
}

static bool is_alnum(wint_t c) {
    return iswalnum(c) || c == L'_';
}

static Token make_token(TokenType type, const wchar_t *lexeme) {
    Token token;
    token.type = type;
    size_t len = wcslen(lexeme);
    token.lexeme = malloc((len + 1) * sizeof(char));
    wcstombs(token.lexeme, lexeme, len + 1);
    token.line = current_line;
    token.column = current_column - (int)len;
    return token;
}

static Token scan_identifier(void) {
    wchar_t lexeme[MAX_LEXEME_LENGTH];
    int i = 0;
    
    while (is_alnum(current_char) && i < MAX_LEXEME_LENGTH - 1) {
        lexeme[i++] = current_char;
        advance();
    }
    lexeme[i] = L'\0';
    
    // Check for keywords
    if (wcscmp(lexeme, L"begin") == 0) return make_token(TOKEN_KEYWORD_BEGIN, lexeme);
    if (wcscmp(lexeme, L"end") == 0) return make_token(TOKEN_KEYWORD_END, lexeme);
    if (wcscmp(lexeme, L"if") == 0) return make_token(TOKEN_KEYWORD_IF, lexeme);
    if (wcscmp(lexeme, L"then") == 0) return make_token(TOKEN_KEYWORD_THEN, lexeme);
    if (wcscmp(lexeme, L"else") == 0) return make_token(TOKEN_KEYWORD_ELSE, lexeme);
    if (wcscmp(lexeme, L"while") == 0) return make_token(TOKEN_KEYWORD_WHILE, lexeme);
    if (wcscmp(lexeme, L"do") == 0) return make_token(TOKEN_KEYWORD_DO, lexeme);
    if (wcscmp(lexeme, L"program") == 0) return make_token(TOKEN_KEYWORD_PROGRAM, lexeme);
    if (wcscmp(lexeme, L"var") == 0) return make_token(TOKEN_KEYWORD_VAR, lexeme);
    if (wcscmp(lexeme, L"procedure") == 0) return make_token(TOKEN_KEYWORD_PROCEDURE, lexeme);
    if (wcscmp(lexeme, L"function") == 0) return make_token(TOKEN_KEYWORD_FUNCTION, lexeme);
    
    return make_token(TOKEN_IDENTIFIER, lexeme);
}

static Token scan_number(void) {
    wchar_t lexeme[MAX_LEXEME_LENGTH];
    int i = 0;
    bool is_real = false;
    
    while ((iswdigit(current_char) || current_char == L'.') && i < MAX_LEXEME_LENGTH - 1) {
        if (current_char == L'.') {
            if (is_real) break;
            is_real = true;
        }
        lexeme[i++] = current_char;
        advance();
    }
    lexeme[i] = L'\0';
    
    return make_token(is_real ? TOKEN_REAL : TOKEN_INTEGER, lexeme);
}

static Token scan_string(void) {
    wchar_t lexeme[MAX_LEXEME_LENGTH];
    int i = 0;
    
    advance(); // Skip opening quote
    while (current_char != L'\'' && current_char != WEOF && i < MAX_LEXEME_LENGTH - 1) {
        lexeme[i++] = current_char;
        advance();
    }
    lexeme[i] = L'\0';
    
    if (current_char == L'\'') {
        advance(); // Skip closing quote
        return make_token(TOKEN_STRING, lexeme);
    } else {
        return make_token(TOKEN_EOF, L"Unterminated string");
    }
}

Token scanner_next_token(void) {
    if (has_peeked) {
        has_peeked = false;
        return peeked_token;
    }
    
    while (iswspace(current_char)) {
        advance();
    }
    
    if (current_char == WEOF) {
        return make_token(TOKEN_EOF, L"EOF");
    }
    
    if (is_alpha(current_char)) {
        return scan_identifier();
    }
    
    if (iswdigit(current_char)) {
        return scan_number();
    }
    
    if (current_char == L'\'') {
        return scan_string();
    }
    
    wchar_t lexeme[3] = {current_char, L'\0', L'\0'}; // Increase size to 3
    advance();
    
    switch (lexeme[0]) {
        // ... (other cases remain the same)
        case L':':
            if (current_char == L'=') {
                lexeme[1] = current_char;
                lexeme[2] = L'\0';
                advance();
                return make_token(TOKEN_ASSIGN, lexeme);
            }
            return make_token(TOKEN_COLON, lexeme);
        // ... (other cases remain the same)
        case L'.':
            if (current_char == L'.') {
                lexeme[1] = current_char;
                lexeme[2] = L'\0';
                advance();
                return make_token(TOKEN_DOTDOT, lexeme);
            }
            return make_token(TOKEN_DOT, lexeme);
        default:
            return make_token(TOKEN_EOF, L"Unknown token");
    }
}

Token scanner_peek_token(void) {
    if (!has_peeked) {
        peeked_token = scanner_next_token();
        has_peeked = true;
    }
    return peeked_token;
}

void scanner_cleanup(void) {
    if (file) {
        fclose(file);
        file = NULL;
    }
    if (peeked_token.lexeme) {
        free(peeked_token.lexeme);
        peeked_token.lexeme = NULL;
    }
}