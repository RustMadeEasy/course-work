//
//  Tic_Tac_ToeApp.swift
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com
//

import SwiftUI
import OpenAPIClient
import MQTTNIO

@main
struct Tic_Tac_ToeApp: App {
    
    init() {
        // TODO: HD: set this to the Cloud load-balancer address
        // OpenAPIClientAPI.basePath = "https://services.RustMadeEasy.com:40020"
        OpenAPIClientAPI.basePath = "http://127.0.0.1:50020"
    }

    var body: some Scene {
        WindowGroup {
            StartGameView()
        }
    }
}
