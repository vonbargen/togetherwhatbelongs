// parser.c

#include "parser.h"
#include <stdio.h>
#include <stdlib.h>

static Token current_token;

static void error(const char *message) {
    fprintf(stderr, "Parser Error: %s at line %d, column %d\n",
            message, current_token.line, current_token.column);
    exit(1);
}

static void consume(TokenType expected_type) {
    if (current_token.type == expected_type) {
        current_token = scanner_next_token();
    } else {
        char error_message[100];
        snprintf(error_message, sizeof(error_message),
                 "Expected token %d, but found %d", expected_type, current_token.type);
        error(error_message);
    }
}

static bool parse_ident(void) {
    if (current_token.type == T_ID) {
        consume(T_ID);
        return true;
    }
    return false;
}

static bool parse_number(void) {
    if (current_token.type == T_INTEGER || current_token.type == T_REAL) {
        consume(current_token.type);
        return true;
    }
    return false;
}

static bool parse_string(void) {
    if (current_token.type == T_STRING) {
        consume(T_STRING);
        return true;
    }
    return false;
}

static bool parse_import(void) {
    if (!parse_ident()) return false;
    if (current_token.type == T_ASSIGN) {
        consume(T_ASSIGN);
        if (!parse_ident()) return false;
    }
    return true;
}

static bool parse_import_list(void) {
    consume(T_IMPORT);
    if (!parse_import()) return false;
    while (current_token.type == T_COMMA) {
        consume(T_COMMA);
        if (!parse_import()) return false;
    }
    consume(T_SEMI);
    return true;
}

static bool parse_declaration_sequence(void) {
    // This is a placeholder. You should implement the actual declaration sequence parsing here.
    // It should handle constant, type, variable, and procedure declarations.
    while (current_token.type == T_CONST || current_token.type == T_TYPE ||
           current_token.type == T_VAR || current_token.type == T_PROCEDURE) {
        // Parse each declaration type
        switch (current_token.type) {
            case T_CONST:
                consume(T_CONST);
                // Add constant declaration parsing
                break;
            case T_TYPE:
                consume(T_TYPE);
                // Add type declaration parsing
                break;
            case T_VAR:
                consume(T_VAR);
                // Add variable declaration parsing
                break;
            case T_PROCEDURE:
                consume(T_PROCEDURE);
                // Add procedure declaration parsing
                break;
            default:
                return false;
        }
    }
    return true;
}

static bool parse_statement_sequence(void) {
    // This is a placeholder. You should implement the actual statement sequence parsing here.
    // It should handle various types of statements (assignments, procedure calls, control structures, etc.)
    while (current_token.type != T_END && current_token.type != T_EOF) {
        // Parse each statement
        // Add specific statement parsing logic here
        consume(T_SEMI);
    }
    return true;
}

bool parse_module(void) {
    consume(T_MODULE);
    if (!parse_ident()) return false;
    consume(T_SEMI);

    if (current_token.type == T_IMPORT) {
        if (!parse_import_list()) return false;
    }

    if (!parse_declaration_sequence()) return false;

    if (current_token.type == T_BEGIN) {
        consume(T_BEGIN);
        if (!parse_statement_sequence()) return false;
    }

    consume(T_END);
    if (!parse_ident()) return false;
    consume(T_DOT);

    return true;
}


#include <unistd.h>

#define MAX_PATH 2048

int main(int argc, char *argv[]) {

    char cwd[MAX_PATH];

    // Get and print the current working directory
    if (getcwd(cwd, sizeof(cwd)) != NULL) {
        printf("Current working directory: %s\n", cwd);
    } else {
        perror("getcwd() error");
        return 1;
    }

    if (argc != 2) {
        fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
        return 1;
    }

    if (!scanner_init(argv[1])) {
        fprintf(stderr, "Failed to initialize scanner\n");
        return 1;
    }

    current_token = scanner_next_token();

    if (parse_module()) {
        printf("Parsing completed successfully.\n");
    } else {
        printf("Parsing failed.\n");
    }

    scanner_cleanup();
    return 0;
}
