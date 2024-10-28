## How to build

cmake -G Xcode -B build

## Oberon
- https://github.com/oberon-lang/specification/blob/master/The_Programming_Language_Oberon+.adoc
- EBNF
  - https://oberon07.com/o7EBNF.xhtml
- Test

### Parser
```
This parser implementation follows the structure of the given EBNF and uses the provided scanner. Here are some key points about this implementation:
It uses recursive descent parsing, which is a top-down parsing technique.
The parse_module function is the entry point for parsing, following the module structure defined in the EBNF.
Helper functions like parse_ident, parse_number, and parse_string are used to parse basic elements.
The parse_import_list function handles the import list parsing as specified in the EBNF.
Placeholder functions parse_declaration_sequence and parse_statement_sequence are included. These should be expanded to handle the specific syntax of declarations and statements according to the full EBNF.
The parser uses the consume function to match and move past expected tokens, reporting errors if unexpected tokens are encountered.
Error handling is implemented through the error function, which reports parsing errors and exits the program.
The main function initializes the scanner, starts the parsing process, and handles cleanup.
To make this parser fully functional and compliant with the complete Oberon-07 EBNF, you would need to:
Expand the parse_declaration_sequence function to handle constant, type, variable, and procedure declarations in detail.
Implement the parse_statement_sequence function to handle all types of statements (assignments, procedure calls, control structures, etc.).
Add functions to parse expressions, types, and other language constructs as defined in the full EBNF.
Enhance error reporting and recovery mechanisms for a more robust parser.
Consider implementing a syntax tree or other intermediate representation to capture the structure of the parsed program.
This implementation provides a solid starting point that you can build upon to create a complete Oberon-07 parser.
```