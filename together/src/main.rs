mod scanner;
mod parser;
mod semantic;
mod codegen;

use scanner::Lexer;
use parser::{Parser, PrettyPrinter};
use semantic::TypeChecker;
use codegen::CGenerator;

#[cfg(feature = "llvm")]
use codegen::LLVMGenerator;
#[cfg(feature = "llvm")]
use inkwell::context::Context;

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

    PROCEDURE WriteInt*(n: INTEGER);
    BEGIN
        (* Wird später implementiert *)
    END WriteInt;

    PROCEDURE WriteLn*;
    BEGIN
        (* Wird später implementiert *)
    END WriteLn;

    BEGIN
        Init;
        count := Add(5, 10);
        WriteInt(count);
        WriteLn
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
        }
        Err(errors) => {
            eprintln!("✗ Semantische Fehler gefunden:\n");
            for (i, error) in errors.iter().enumerate() {
                eprintln!("  {}. {}", i + 1, error);
            }
            return;
        }
    }

    // LLVM Backend (optional, nur wenn Feature aktiviert)
    #[cfg(feature = "llvm")]
    {
        println!("\n=== LLVM CODE-GENERATOR ===\n");

        let context = Context::create();
        let mut llvm_gen = LLVMGenerator::new(&context, &module.name);

        match llvm_gen.generate(&module) {
            Ok(_llvm_ir) => {
                println!("✓ LLVM-IR erfolgreich generiert!");

                if let Err(e) = llvm_gen.write_to_file("output.ll") {
                    eprintln!("✗ Fehler beim Schreiben: {}", e);
                } else {
                    println!("  - Ausgabe: output.ll\n");
                }
            }
            Err(e) => {
                eprintln!("✗ LLVM-Fehler: {}", e);
            }
        }
    }

    // C Backend (immer verfügbar)
    println!("\n=== C CODE-GENERATOR ===\n");

    let mut c_gen = CGenerator::new();
    let c_code = c_gen.generate(&module);

    println!("✓ C-Code erfolgreich generiert!");
    println!("  - Ausgabe: output.c\n");

    if let Err(e) = fs::write("output.c", &c_code) {
        eprintln!("✗ Fehler beim Schreiben der Ausgabedatei: {}", e);
        return;
    }

    println!("=== PRETTY PRINTER (Oberon) ===\n");

    let mut printer = PrettyPrinter::new();
    let formatted_code = printer.print_module(&module);
    println!("{}", formatted_code);

    println!("\n=== KOMPILIERUNG ===\n");

    #[cfg(feature = "llvm")]
    {
        println!("LLVM (falls generiert):");
        println!("  llc output.ll -o output.s");
        println!("  gcc output.s -o output_llvm");
        println!();
    }

    println!("C:");
    println!("  gcc -o output output.c");
    println!("  ./output");

    #[cfg(not(feature = "llvm"))]
    {
        println!();
        println!("💡 Hinweis: LLVM-Backend nicht aktiviert.");
        println!("   Um LLVM zu nutzen:");
        println!("   1. Installiere LLVM 16: brew install llvm@16  (macOS)");
        println!("   2. Kompiliere mit: cargo run --features llvm");
    }
}