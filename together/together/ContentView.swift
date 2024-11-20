import SwiftUI
import Maaku

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
    
    public func renderHTML(from markdown: String) -> String {
        do {
            
            let testmd = """
    # Heading Linz
    ## Heading 2
    Simple paragraph
    with two lines
    
    Another Paragraph
    """
            
        // Erstelle ein Maaku-Dokument mit dem richtigen Argumentlabel
            let document = try CMDocument(text: markdown,
                                      options: .default,
                                      extensions: .all)
        let html = try document.renderHtml()
        return html
            
        } catch {
            print("Fehler beim Rendern des mardowns: \(error)")
            return "<html><body><p>Fehler beim Rendern des mardowns: \(error)</p></body></html>"
        }
    }}

#Preview {
    ContentView(document: .constant(togetherDocument()))
}
