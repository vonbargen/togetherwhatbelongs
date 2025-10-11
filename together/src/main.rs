mod scanner;

use scanner::Lexer;

fn main() {
    let source = r#"
    MODULE Example;
    
    CONST
        MaxSize = 100;
    
    VAR
        x, y: INTEGER;
        name: ARRAY 20 OF CHAR;
    
    BEGIN
        x := 42;
        y := x * 2;
    END Example.
    "#;

    let mut lexer = Lexer::new(source);
    
    match lexer.tokenize() {
        Ok(tokens) => {
            println!("Tokens erfolgreich gescannt:\n");
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(e) => {
            eprintln!("Fehler beim Scannen: {}", e);
        }
    }
}
