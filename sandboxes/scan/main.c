#include "parser.h"
#include "scanner.h"

#include <stdlib.h>
#include <stdio.h>

/*
int main(int argc, char *argv[]) {

    // Print the number of arguments
    printf("Number of arguments (argc): %d\n", argc);

    // Print all arguments
    printf("Arguments (argv):\n");
    for (int i = 0; i < argc; i++) {
        printf("argv[%d]: %s\n", i, argv[i]);
    }

    // Check if we have the correct number of arguments
    if (argc != 2) {
        fprintf(stderr, "Usage: %s <filename>\n", argv[0]);
        return EXIT_FAILURE;
    }

    // Initialize the scanner with the provided filename
    if (!scanner_init(argv[1])) {
        fprintf(stderr, "Failed to open file: %s\n", argv[1]);
        return EXIT_FAILURE;
    }

    Token token;
    do {
        token = scanner_next_token();
        printf("Token: %d, Lexeme: %s, Line: %d, Column: %d\n",
               token.type, token.lexeme, token.line, token.column);
        free(token.lexeme);
    } while (token.type != T_EOF);

    scanner_cleanup();
    return EXIT_SUCCESS;
}
*/


