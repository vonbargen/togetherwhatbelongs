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
static Token peeked_token = {T_EOF, NULL, 0, 0};
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

// In scanner.c

static Token scan_identifier(void) {
    wchar_t lexeme[MAX_LEXEME_LENGTH];
    int i = 0;
    
    while (is_alnum(current_char) && i < MAX_LEXEME_LENGTH - 1) {
        lexeme[i++] = current_char;
        advance();
    }
    lexeme[i] = L'\0';
    
    // Check for keywords
    if (wcscmp(lexeme, L"ARRAY") == 0) return make_token(T_ARRAY, lexeme);
    if (wcscmp(lexeme, L"BEGIN") == 0) return make_token(T_BEGIN, lexeme);
    if (wcscmp(lexeme, L"BY") == 0) return make_token(T_BY, lexeme);
    if (wcscmp(lexeme, L"CASE") == 0) return make_token(T_CASE, lexeme);
    if (wcscmp(lexeme, L"CONST") == 0) return make_token(T_CONST, lexeme);
    if (wcscmp(lexeme, L"DIV") == 0) return make_token(T_DIV, lexeme);
    if (wcscmp(lexeme, L"DO") == 0) return make_token(T_DO, lexeme);
    if (wcscmp(lexeme, L"ELSE") == 0) return make_token(T_ELSE, lexeme);
    if (wcscmp(lexeme, L"ELSIF") == 0) return make_token(T_ELSIF, lexeme);
    if (wcscmp(lexeme, L"END") == 0) return make_token(T_END, lexeme);
    if (wcscmp(lexeme, L"EXIT") == 0) return make_token(T_EXIT, lexeme);
    if (wcscmp(lexeme, L"FOR") == 0) return make_token(T_FOR, lexeme);
    if (wcscmp(lexeme, L"IF") == 0) return make_token(T_IF, lexeme);
    if (wcscmp(lexeme, L"IMPORT") == 0) return make_token(T_IMPORT, lexeme);
    if (wcscmp(lexeme, L"IN") == 0) return make_token(T_IN, lexeme);
    if (wcscmp(lexeme, L"IS") == 0) return make_token(T_IS, lexeme);
    if (wcscmp(lexeme, L"LOOP") == 0) return make_token(T_LOOP, lexeme);
    if (wcscmp(lexeme, L"MOD") == 0) return make_token(T_MOD, lexeme);
    if (wcscmp(lexeme, L"MODULE") == 0) return make_token(T_MODULE, lexeme);
    if (wcscmp(lexeme, L"NIL") == 0) return make_token(T_NIL, lexeme);
    if (wcscmp(lexeme, L"OF") == 0) return make_token(T_OF, lexeme);
    if (wcscmp(lexeme, L"OR") == 0) return make_token(T_OR, lexeme);
    if (wcscmp(lexeme, L"POINTER") == 0) return make_token(T_POINTER, lexeme);
    if (wcscmp(lexeme, L"PROCEDURE") == 0) return make_token(T_PROCEDURE, lexeme);
    if (wcscmp(lexeme, L"RECORD") == 0) return make_token(T_RECORD, lexeme);
    if (wcscmp(lexeme, L"REPEAT") == 0) return make_token(T_REPEAT, lexeme);
    if (wcscmp(lexeme, L"RETURN") == 0) return make_token(T_RETURN, lexeme);
    if (wcscmp(lexeme, L"THEN") == 0) return make_token(T_THEN, lexeme);
    if (wcscmp(lexeme, L"TO") == 0) return make_token(T_TO, lexeme);
    if (wcscmp(lexeme, L"TYPE") == 0) return make_token(T_TYPE, lexeme);
    if (wcscmp(lexeme, L"UNTIL") == 0) return make_token(T_UNTIL, lexeme);
    if (wcscmp(lexeme, L"VAR") == 0) return make_token(T_VAR, lexeme);
    if (wcscmp(lexeme, L"WHILE") == 0) return make_token(T_WHILE, lexeme);
    if (wcscmp(lexeme, L"WITH") == 0) return make_token(T_WITH, lexeme);
    if (wcscmp(lexeme, L"BOOLEAN") == 0) return make_token(T_BOOLEAN, lexeme);
    if (wcscmp(lexeme, L"CHAR") == 0) return make_token(T_CHAR, lexeme);
    if (wcscmp(lexeme, L"FALSE") == 0) return make_token(T_FALSE, lexeme);
    if (wcscmp(lexeme, L"INTEGER") == 0) return make_token(T_INTEGER_KW, lexeme);
    if (wcscmp(lexeme, L"NEW") == 0) return make_token(T_NEW, lexeme);
    if (wcscmp(lexeme, L"REAL") == 0) return make_token(T_REAL_KW, lexeme);
    if (wcscmp(lexeme, L"TRUE") == 0) return make_token(T_TRUE, lexeme);
    
    return make_token(T_ID, lexeme);
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
    
    return make_token(is_real ? T_REAL : T_INTEGER, lexeme);
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
        return make_token(T_STRING, lexeme);
    } else {
        return make_token(T_EOF, L"Unterminated string");
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
        return make_token(T_EOF, L"EOF");
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
    
    wchar_t lexeme[3] = {current_char, L'\0', L'\0'};
    advance();
    
    switch (lexeme[0]) {
        case L'+': return make_token(T_PLUS, lexeme);
        case L'-': return make_token(T_MINUS, lexeme);
        case L'*': return make_token(T_STAR, lexeme);
        case L'/': return make_token(T_SLASH, lexeme);
        case L':':
            if (current_char == L'=') {
                lexeme[1] = current_char;
                lexeme[2] = L'\0';
                advance();
                return make_token(T_ASSIGN, lexeme);
            }
            return make_token(T_COLON, lexeme);
        case L';': return make_token(T_SEMI, lexeme);
        case L',': return make_token(T_COMMA, lexeme);
        case L'(': return make_token(T_LPAREN, lexeme);
        case L')': return make_token(T_RPAREN, lexeme);
        case L'[': return make_token(T_LBRACKET, lexeme);
        case L']': return make_token(T_RBRACKET, lexeme);
        case L'{': return make_token(T_LBRACE, lexeme);
        case L'}': return make_token(T_RBRACE, lexeme);
        case L'.':
            if (current_char == L'.') {
                lexeme[1] = current_char;
                lexeme[2] = L'\0';
                advance();
                return make_token(T_DOTDOT, lexeme);
            }
            return make_token(T_DOT, lexeme);
        case L'&': return make_token(T_AMPERSAND, lexeme);
        case L'^': return make_token(T_ARROW, lexeme);
        case L'|': return make_token(T_BAR, lexeme);
        case L'=': return make_token(T_EQU, lexeme);
        case L'>':
            if (current_char == L'=') {
                lexeme[1] = current_char;
                lexeme[2] = L'\0';
                advance();
                return make_token(T_GTE, lexeme);
            }
            return make_token(T_GT, lexeme);
        case L'<':
            if (current_char == L'=') {
                lexeme[1] = current_char;
                lexeme[2] = L'\0';
                advance();
                return make_token(T_LTE, lexeme);
            }
            return make_token(T_LT, lexeme);
        case L'#': return make_token(T_NEQ, lexeme);
        case L'~': return make_token(T_TILDE, lexeme);
        default:
            return make_token(T_EOF, L"Unknown token");
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