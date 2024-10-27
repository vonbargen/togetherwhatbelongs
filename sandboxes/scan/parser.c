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

static bool parse_expression(void) {
    // Implement expression parsing
    return true;
}

static bool parse_array_type(void) {
    // Implement expression parsing
    return true;
}

static bool parse_record_type(void) {
    // Implement expression parsing
    return true;
}

static bool parse_pointer_type(void) {
    // Implement expression parsing
    return true;
}

static bool parse_procedure_type(void) {
    // Implement expression parsing
    return true;
}

static bool parse_const_expression(void) {
    // Implement constant expression parsing
    return parse_expression();
}

static bool parse_const_declaration(void) {
    if (!parse_ident()) return false;
    consume(T_EQU);
    if (!parse_const_expression()) return false;
    return true;
}

static bool parse_type(void) {
    // Implement type parsing
    return parse_ident() || parse_array_type() || parse_record_type() || parse_pointer_type() || parse_procedure_type();
}

static bool parse_type_declaration(void) {
    if (!parse_ident()) return false;
    consume(T_EQU);
    if (!parse_type()) return false;
    return true;
}

static bool parse_ident_list(void) {
    if (!parse_ident()) return false;
    while (current_token.type == T_COMMA) {
        consume(T_COMMA);
        if (!parse_ident()) return false;
    }
    return true;
}

static bool parse_variable_declaration(void) {
    if (!parse_ident_list()) return false;
    consume(T_COLON);
    if (!parse_type()) return false;
    return true;
}

static bool parse_formal_parameters(void) {
    consume(T_LPAREN);
    if (current_token.type != T_RPAREN) {
        do {
            if (current_token.type == T_VAR) {
                consume(T_VAR);
            }
            if (!parse_ident_list()) return false;
            consume(T_COLON);
            if (!parse_type()) return false;
        } while (current_token.type == T_SEMI);
    }
    consume(T_RPAREN);
    if (current_token.type == T_COLON) {
        consume(T_COLON);
        if (!parse_ident()) return false;
    }
    return true;
}

static bool parse_procedure_heading(void) {
    consume(T_PROCEDURE);
    if (!parse_ident()) return false;
    if (current_token.type == T_LPAREN) {
        if (!parse_formal_parameters()) return false;
    }
    return true;
}

static bool parse_procedure_body(void) {
    if (!parse_declaration_sequence()) return false;
    if (current_token.type == T_BEGIN) {
        consume(T_BEGIN);
        if (!parse_statement_sequence()) return false;
    }
    if (current_token.type == T_RETURN) {
        consume(T_RETURN);
        if (!parse_expression()) return false;
    }
    consume(T_END);
    return true;
}

static bool parse_procedure_declaration(void) {
    if (!parse_procedure_heading()) return false;
    consume(T_SEMI);
    if (!parse_procedure_body()) return false;
    if (!parse_ident()) return false;
    return true;
}

/*
static bool parse_declaration_sequence(void) {
    while (true) {
        switch (current_token.type) {
            case T_CONST:
                consume(T_CONST);
                while (current_token.type == T_ID) {
                    if (!parse_const_declaration()) return false;
                    consume(T_SEMI);
                }
                break;
            case T_TYPE:
                consume(T_TYPE);
                while (current_token.type == T_ID) {
                    if (!parse_type_declaration()) return false;
                    consume(T_SEMI);
                }
                break;
            case T_VAR:
                consume(T_VAR);
                while (current_token.type == T_ID) {
                    if (!parse_variable_declaration()) return false;
                    consume(T_SEMI);
                }
                break;
            case T_PROCEDURE:
                if (!parse_procedure_declaration()) return false;
                consume(T_SEMI);
                break;
            default:
                return true;
        }
    }
}
*/

static bool parse_assignment(void) {
    if (!parse_ident()) return false;
    consume(T_ASSIGN);
    if (!parse_expression()) return false;
    return true;
}

static bool parse_procedure_call(void) {
    if (!parse_ident()) return false;
    if (current_token.type == T_LPAREN) {
        consume(T_LPAREN);
        if (current_token.type != T_RPAREN) {
            do {
                if (!parse_expression()) return false;
            } while (current_token.type == T_COMMA);
        }
        consume(T_RPAREN);
    }
    return true;
}

static bool parse_if_statement(void) {
    consume(T_IF);
    if (!parse_expression()) return false;
    consume(T_THEN);
    if (!parse_statement_sequence()) return false;
    while (current_token.type == T_ELSIF) {
        consume(T_ELSIF);
        if (!parse_expression()) return false;
        consume(T_THEN);
        if (!parse_statement_sequence()) return false;
    }
    if (current_token.type == T_ELSE) {
        consume(T_ELSE);
        if (!parse_statement_sequence()) return false;
    }
    consume(T_END);
    return true;
}

static bool parse_case_statement(void) {
    consume(T_CASE);
    if (!parse_expression()) return false;
    consume(T_OF);
    do {
        // Parse case
        if (!parse_expression()) return false;
        consume(T_COLON);
        if (!parse_statement_sequence()) return false;
    } while (current_token.type == T_BAR);
    consume(T_END);
    return true;
}

static bool parse_while_statement(void) {
    consume(T_WHILE);
    if (!parse_expression()) return false;
    consume(T_DO);
    if (!parse_statement_sequence()) return false;
    while (current_token.type == T_ELSIF) {
        consume(T_ELSIF);
        if (!parse_expression()) return false;
        consume(T_DO);
        if (!parse_statement_sequence()) return false;
    }
    consume(T_END);
    return true;
}

static bool parse_repeat_statement(void) {
    consume(T_REPEAT);
    if (!parse_statement_sequence()) return false;
    consume(T_UNTIL);
    if (!parse_expression()) return false;
    return true;
}

static bool parse_for_statement(void) {
    consume(T_FOR);
    if (!parse_ident()) return false;
    consume(T_ASSIGN);
    if (!parse_expression()) return false;
    consume(T_TO);
    if (!parse_expression()) return false;
    if (current_token.type == T_BY) {
        consume(T_BY);
        if (!parse_const_expression()) return false;
    }
    consume(T_DO);
    if (!parse_statement_sequence()) return false;
    consume(T_END);
    return true;
}

static bool parse_statement(void) {
    switch (current_token.type) {
        case T_ID:
            if (scanner_peek_token().type == T_ASSIGN) {
                return parse_assignment();
            } else {
                return parse_procedure_call();
            }
        case T_IF:
            return parse_if_statement();
        case T_CASE:
            return parse_case_statement();
        case T_WHILE:
            return parse_while_statement();
        case T_REPEAT:
            return parse_repeat_statement();
        case T_FOR:
            return parse_for_statement();
        default:
            return true; // Empty statement
    }
}

/*
static bool parse_statement_sequence(void) {
    do {
        if (!parse_statement()) return false;
    } while (current_token.type == T_SEMI);
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
 */



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
