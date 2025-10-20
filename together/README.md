# Ohne LLVM (nur C-Backend)
cargo run

# Mit LLVM (wenn installiert)
brew install llvm@16  # auf macOS
cargo run --features llvm

# Finde den LLVM-Pfad
brew --prefix llvm@16

# Dann verwende den vollen Pfad:
$(brew --prefix llvm@16)/bin/llc output.ll -o output.s
$(brew --prefix llvm@16)/bin/clang output.s -o output_llvm

# Oder füge LLVM zum PATH hinzu:
export PATH="$(brew --prefix llvm@16)/bin:$PATH"
llc output.ll -o output.s
clang output.s -o output_llvm

# Dann ausführen:
./output_llvm

# Scriptsprache
Ich möchte eine Art Scriptsprache mit LLVM implementieren in einer einfachen Entwicklungumgebung. Dort möchte ich nicht das Anwender llc oder clang aufrufen müssen. Geht das auch unter der Haube?

Ja, es ist möglich, eine eigene Skriptsprache mit LLVM zu implementieren, ohne dass der Anwender explizit llc oder clang über die Kommandozeile aufrufen muss. LLVM ist modular aufgebaut und stellt nicht nur Kommandozeilen-Tools, sondern vor allem umfangreiche Programmierschnittstellen (APIs) bereit – in C, C++, und vielen Sprachen durch Bindings.
Vorgehensweise in einer IDE-ähnlichen Umgebung
•	Du kannst die LLVM-API direkt in deiner Anwendung nutzen und dort mit wenigen Funktionsaufrufen IR-Code generieren, optimieren und direkt in Maschinencode/JIT übersetzen.
•	Alle Schritte, für die man sonst llc oder clang verwenden würde, lassen sich über die Bibliotheken auch „unter der Haube“ als Funktionsaufrufe abbilden. Die API ist genauso mächtig wie die Compiler-Tools.
•	Viele eigene Sprachen nutzen dieses Vorgehen, ein klassisches Beispiel ist das bekannte „Kaleidoscope“-Beispiel-Tutorial im LLVM-Projekt.
Vorteile dieser Methode
•	Die Nutzer deiner Entwicklungsumgebung müssen keine externen Compiler aufrufen.
•	Die Übersetzung läuft direkt im Prozess deiner IDE, was Integration, Debugging und Fehlermeldungen vereinfachen kann.
Du kannst also direkt mit LLVM-Bibliotheken arbeiten, sowohl für Ahead-of-Time-Kompilierung (AOT) als auch für Just-In-Time-Execution (JIT), und alles in einer eigenen Benutzeroberfläche kapseln, genauso wie das bei modernen Editor- und IDE-Projekten üblich ist. Externe Tools wie clang oder llc sind nur Alternativen, aber keine Pflicht.Ja, das geht problemlos: Mit der LLVM-API kannst du sämtliche LLVM-Funktionen direkt aus deiner Entwicklungsumgebung heraus nutzen, ohne dass der Nutzer explizit llc oder clang aufrufen muss. Die LLVM-Bibliotheken erlauben das Erstellen, Optimieren und Kompilieren von IR bis hin zu Maschinencode programmatisch – also komplett im Hintergrund deiner Anwendung, sei es JIT (Just-In-Time) oder als AOT (Ahead-of-Time) Compilation.
Viele Beispiele, wie das berühmte “Kaleidoscope”-Tutorial im LLVM-Projekt, und auch andere Sprachen bauen eigene Compiler mit direkter Nutzung der LLVM-Bibliotheken und ohne externe Tools zu benötigen. Das Ganze läuft dann genauso wie in größeren IDEs: Der Anwender bekommt davon nichts mit, da alle Übersetzungsschritte unter der Haube mit der API ausgeführt werden.
Zusammengefasst: Externe Tools wie clang oder llc sind optional – alle Schritte können mit der API innerhalb deiner eigenen Anwendung automatisiert werden, sowohl unter Linux als auch im Docker-Container.

# Zusammenfassung
```sh
cargo run --features llvm
$(brew --prefix llvm@16)/bin/llc output.ll -o output.s
$(brew --prefix llvm@16)/bin/clang output.s -o output_llvm
./output_llvm
```