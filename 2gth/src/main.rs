mod parser;
mod scanner;
mod tokens;

use parser::Parser;
use scanner::Scanner;

fn main() {
    let input = r#"
    // This is a single line comment
    let x = 42;
    /* This is a
       multi-line comment */
    let name = "hello";
    fn test() {
        return x + 1;
    }
    "#;

    println!("=== Input Code ===");
    println!("{}", input);
    println!();

    // Create scanner and tokenize
    let mut scanner = Scanner::new(input);
    let tokens = scanner.scan_all().unwrap();

    println!("=== Tokens ===");
    for (i, token) in tokens.iter().enumerate() {
        println!("{:2}: {:?}", i, token);
    }
    println!();

    // Parse tokens
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(()) => println!("=== Parse Result ===\nParsing completed successfully!"),
        Err(e) => println!("=== Parse Error ===\n{}", e),
    }
}
