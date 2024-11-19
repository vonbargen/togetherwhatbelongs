//
//  ContentView.swift
//  together
//
//  Created by thorsten on 19.11.24.
//

import SwiftUI

struct ContentView: View {
    @Binding var document: togetherDocument

    var body: some View {
        TextEditor(text: $document.text)
    }
}

#Preview {
    ContentView(document: .constant(togetherDocument()))
}
