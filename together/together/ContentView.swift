import SwiftUI

struct ContentView: View {
    @Binding var document: togetherDocument
    
    var body: some View {
        TabView {
            // Tab 1: Markdown Editor
            TextEditor(text: $document.text)
                .padding()
                .tabItem {
                    Label("Editor", systemImage: "pencil")
                }
            
            // Tab 2: Formularansicht
            Form {
                Section(header: Text("Formularansicht")) {
                    Text("Hier könnte ein Formular entstehen.")
                }
            }
            .tabItem {
                Label("Formular", systemImage: "list.bullet.rectangle")
            }
            
            // Tab 3: Vorschau
            HTMLPreview(htmlContent: renderHTML(from: document.text))
                .tabItem {
                    Label("Vorschau", systemImage: "eye")
                }
        }
    }
    
    private func renderHTML(from markdown: String) -> String {
        let html = """
        <html>
        <head><style>body { font-family: -apple-system; padding: 20px; }</style></head>
        <body>
        \(markdown)
        </body>
        </html>
        """
        print("Generated HTML: \(html)") // Debug-Ausgabe
        return html
    }}

#Preview {
    ContentView(document: .constant(togetherDocument()))
}
