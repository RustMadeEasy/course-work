//
//  SetupView.swift
//  Tic Tac Toe
//
// © 2024 Rust Made Easy. All rights reserved.
// @author Joel@RustMadeEasy.com
//

import Foundation
import SwiftUI

/// Allows user to enter their Invitation Code to Join an existing Game.
public struct EnterInvitationCodeView: View {

    /// The name of the local player.
    @State private var localPlayerName: String
    
    /// The Game invitation code.
    @State private var invitationCode: String = ""

    /// Invokes navigation to the Game View (the main view of the app)
    @State private var navigateToGameView = false

    /// Invokes empty Invitation Code alert
    @State private var showEmptyInvitationAlert = false
    
    public var body: some View {

        ZStack {
            
            NavigationStack {
                
                VStack {
                    
                    Spacer()
                    Spacer()
                    
                    VStack {
                        
                        Text(String(localized: "Please enter the Invitation Code:"))

                        TextField(String(localized: "invitation code"), text: $invitationCode)
                            .frame(width: 140)
                            .border(.secondary)
                                                
                        Button {
                            if self.invitationCode.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty {
                                self.showEmptyInvitationAlert = true
                            } else {
                                self.navigateToGameView = true
                            }
                        } label: {
                            Text(String(localized: "Join Game"))
                                .padding()
                        }
                        .padding()

                        Spacer()
                    }
                    .navigationDestination(isPresented: $navigateToGameView, destination: {
                        GameView(localPlayerName: self.localPlayerName, invitationCode: self.invitationCode)
                    })
                    .alert(String(localized: "Please enter a Game Invitation from another Player."), isPresented: $showEmptyInvitationAlert) {
                        Button() {
                        } label: {
                            Text(String(localized: "Ok"))
                        }
                    }
                }
            }
        }
        .frame(maxWidth: .infinity)
        .background(Color("MainColor").gradient)
    }

    init(localPlayerName: String) {
        self._localPlayerName = State(initialValue: localPlayerName)
    }
}

#Preview {
    EnterInvitationCodeView(localPlayerName: "Player One")
}
