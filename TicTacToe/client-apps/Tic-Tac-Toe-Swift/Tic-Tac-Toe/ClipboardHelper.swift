//
//  ClipboardHelper.swift
//  Tic-Tac-Toe
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com
//

import Foundation

#if os(iOS)
    import SwiftUI
#else
    import AppKit
#endif

/// Provides cross-OS clipboard functionality.
public class ClipboardHelper {
    static public func copyTextToClipboard(text: String) {
        #if os(iOS)
            UIPasteboard.general.setValue(text, forPasteboardType: "public.plain-text")
        #else
            let pasteboard = NSPasteboard.general
            pasteboard.declareTypes([.string], owner: nil)
            pasteboard.setString(text, forType: .string)
        #endif
    }
}
