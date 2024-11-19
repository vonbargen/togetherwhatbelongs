//
//  HTMLPreview.swift
//  together
//
//  Created by thorsten on 19.11.24.
//

import SwiftUI
import WebKit

struct HTMLPreview: NSViewRepresentable {
    let htmlContent: String

    func makeNSView(context: Context) -> WKWebView {
        let webView = WKWebView()
        return webView
    }

    func updateNSView(_ nsView: WKWebView, context: Context) {
        print("Loading HTML into WebView") // Debug-Ausgabe
        nsView.loadHTMLString(htmlContent, baseURL: nil)
    }
}
