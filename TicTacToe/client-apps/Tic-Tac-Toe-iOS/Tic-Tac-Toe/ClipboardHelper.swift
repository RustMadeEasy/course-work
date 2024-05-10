//
//  ClipboardHelper.swift
//  Tic-Tac-Toe
//
// © 2024 Rust Made Easy. All rights reserved.
// @author Joel@RustMadeEasy.com
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
            NSPasteboard.general.setString(text, forType: NSPasteboard.PasteboardType.string)
        #endif
    }
}
