//
//  HTMLPreview.swift
//  together
//
//  Created by thorsten on 19.11.24.
//

import SwiftUI
import WebKit

struct HTMLPreview: NSViewRepresentable {
    var htmlContent = """
    <html>
    <head>
        <style>body { font-family: -apple-system; }</style>
    </head>
    <body>
        <h1>Test HTML</h1>
        <p>Dies ist ein statischer Test der HTML-Vorschau.</p>
    </body>
    </html>
"""

    func makeNSView(context: Context) -> WKWebView {
        let webView = WKWebView()
        return webView
    }

    func updateNSView(_ nsView: WKWebView, context: Context) {
        print("Loading HTML into WebView") // Debug-Ausgabe
        
        nsView.loadHTMLString(htmlContent, baseURL: URL(string: "about:blank"))
    }
}
