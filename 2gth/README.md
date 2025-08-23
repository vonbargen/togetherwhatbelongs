https://www.perplexity.ai/search/c99d20c0-8f9e-401d-9f9a-5f1911a2c2d1?0=c

Projekt Beschreibung
Dieses Rust-Projekt implementiert einen schnellen Scanner (Lexer) mit Ein-Zeichen-Vorausschau-Funktionalität und einen Parser, der diesen Scanner verwendet. Das System unterstützt Java-Style Kommentare und bietet eine saubere Token-basierte Architektur.

Architektur Überblick
Scanner (Lexer)

Wandelt Eingabetext in Token um mit Ein-Zeichen-Vorausschau für effiziente Tokenisierung

Token Enumeration

Definiert alle verfügbaren Token-Typen in einer separaten Datei für modulare Architektur

Parser

Analysiert die Token-Liste und validiert die Syntax basierend auf definierten Grammatikregeln

Hauptfunktionen
Schnelle Tokenisierung: Optimierter Scanner mit minimaler Overhead
Ein-Zeichen-Vorausschau: Ermöglicht lookahead für komplexere Token-Erkennung
Java-Style Kommentare: Unterstützt sowohl // als auch /* */ Kommentare
Modulare Architektur: Separate Dateien für Token, Scanner und Parser
Fehlerbehandlung: Aussagekräftige Fehlermeldungen mit Zeilennummern
Projekt ausführen
# Projekt kompilieren
cargo build

# Projekt ausführen
cargo run

# Tests ausführen (falls vorhanden)
cargo test

Technische Dokumentation
Scanner Implementation

Der Scanner implementiert Ein-Zeichen-Vorausschau durch die Verwendung von Rusts Peekable<Chars> Iterator. Dies ermöglicht es, das nächste Zeichen zu inspizieren, ohne es zu konsumieren.

Lookahead Mechanismus

fn peek_char(&mut self) -> Option<char> {
    self.input.peek().copied()
}
Diese Funktion ermöglicht es dem Scanner, Entscheidungen basierend auf dem nächsten Zeichen zu treffen, ohne die aktuelle Position zu verändern.

Token Struktur

Token sind in einer separaten Enumeration definiert und enthalten Positionsinformationen:

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub lexeme: String,
}
Java-Style Kommentare

Das System unterstützt zwei Arten von Kommentaren:

Einzeilige Kommentare: // bis zum Zeilenende
Mehrzeilige Kommentare: /* */ verschachtelt nicht unterstützt
Kommentar-Scanning Algorithm

// Erkennung basiert auf Lookahead
'/' => {
    self.advance(); // konsumiere '/'
    match self.peek_char() {
        Some('/') => self.scan_line_comment(),
        Some('*') => self.scan_block_comment(),
        _ => Ok(Some(self.make_token(TokenType::Slash))),
    }
}
Parser Architektur

Der Parser implementiert einen rekursiven Abstiegsmechanismus mit folgender Grammatik-Hierarchie:

Statements: let, fn, return
Expressions: Addition → Multiplication → Primary
Primary: Zahlen, Strings, Identifiers, Gruppierungen
Fehlerbehandlung

Sowohl Scanner als auch Parser liefern detaillierte Fehlermeldungen mit Positionsinformationen:

Err(format!("Unexpected character '{}' at line {}, column {}",
           ch, self.line, self.column))
Performance Merkmale

O(n) Zeitkomplexität für das Scannen
Minimaler Memory-Overhead durch Iterator-basierte Implementierung
Zero-Copy String-Handling wo möglich
