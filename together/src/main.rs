mod scanner;
mod parser;
mod semantic;
mod codegen;

use scanner::Lexer;
use parser::{Parser, PrettyPrinter};
use semantic::TypeChecker;
use codegen::CGenerator;
use std::fs;

fn main() {
    let source = r#"
    MODULE Example;

    CONST
        MaxSize = 100;
        Pi = 3.14159;

    TYPE
        IntArray = ARRAY MaxSize OF INTEGER;
        Point* = RECORD
            x*, y*: REAL;
        END;

    VAR
        count: INTEGER;
        points: ARRAY 10 OF Point;

    PROCEDURE Add*(a, b: INTEGER): INTEGER;
    BEGIN
        RETURN a + b
    END Add;

    PROCEDURE Init*;
    VAR
        i: INTEGER;
    BEGIN
        count := 0;
        FOR i := 0 TO 9 DO
            points[i].x := 0.0;
            points[i].y := 0.0
        END
    END Init;

    BEGIN
        Init;
        count := Add(5, 10);
    END Example.
    "#;

    println!("=== SCANNER ===\n");

    let mut lexer = Lexer::new(source);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => {
            println!("✓ Scanning erfolgreich: {} Tokens\n", tokens.len());
            tokens
        }
        Err(e) => {
            eprintln!("✗ Fehler beim Scannen: {}", e);
            return;
        }
    };

    println!("=== PARSER ===\n");

    let mut parser = Parser::new(tokens);
    let module = match parser.parse() {
        Ok(module) => {
            println!("✓ Parsing erfolgreich!\n");
            println!("Modul: {}", module.name);
            println!("  - {} Konstanten", module.declarations.constants.len());
            println!("  - {} Typen", module.declarations.types.len());
            println!("  - {} Variablen", module.declarations.variables.len());
            println!("  - {} Prozeduren", module.declarations.procedures.len());
            module
        }
        Err(e) => {
            eprintln!("✗ Fehler beim Parsen: {}", e);
            return;
        }
    };

    println!("\n=== SEMANTISCHE ANALYSE ===\n");

    let mut type_checker = TypeChecker::new();
    match type_checker.check_module(&module) {
        Ok(()) => {
            println!("✓ Semantische Analyse erfolgreich!");
            println!("  - Alle Typen korrekt");
            println!("  - Alle Symbole definiert");
        }
        Err(errors) => {
            eprintln!("✗ Semantische Fehler gefunden:\n");
            for (i, error) in errors.iter().enumerate() {
                eprintln!("  {}. {}", i + 1, error);
            }
            return;
        }
    }

    println!("\n=== CODE-GENERATOR ===\n");

    let mut generator = CGenerator::new();
    let c_code = generator.generate(&module);

    println!("✓ C-Code erfolgreich generiert!");
    println!("  - Ausgabe: output.c\n");

    // C-Code in Datei schreiben
    if let Err(e) = fs::write("output.c", &c_code) {
        eprintln!("✗ Fehler beim Schreiben der Ausgabedatei: {}", e);
        return;
    }

    println!("=== GENERIERTER C-CODE ===\n");
    println!("{}", c_code);

    println!("\n=== PRETTY PRINTER (Oberon) ===\n");

    let mut printer = PrettyPrinter::new();
    let formatted_code = printer.print_module(&module);
    println!("{}", formatted_code);

    println!("\n=== KOMPILIERUNG ===\n");
    println!("Um den generierten C-Code zu kompilieren:");
    println!("  gcc -o output output.c");
    println!("  ./output");
}