//
//  togetherApp.swift
//  together
//
//  Created by thorsten on 19.11.24.
//

import SwiftUI

@main
struct togetherApp: App {
    var body: some Scene {
        DocumentGroup(newDocument: togetherDocument()) { file in
            ContentView(document: file.$document)
        }
    }
}
