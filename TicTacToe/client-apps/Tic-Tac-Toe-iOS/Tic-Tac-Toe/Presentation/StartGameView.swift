//
//  SetupView.swift
//  Tic Tac Toe
//
// © 2024 Rust Made Easy. All rights reserved.
// @author Info@RustMadeEasy.com
//

import Foundation
import SwiftUI

/// Presents the end-user with Game startup options.
public struct StartGameView: View {

    /// Holds the name of the local player
    @State private var localPlayerName = ""

    /// Navigates to the view that allows user to Join an existing Game using an Invitiation Code
    @State private var navigateToEnterInvitationCodeView = false

    /// Invokes navigation to the Game View (the main view of the app)
    @State private var navigateToGameView = false

    /// Invokes the 'name missing' alert
    @State private var showingNameNeededAlert = false

    public var body: some View {
        
        NavigationStack {
            
            ZStack {
                                            
                VStack {
                    
                    Spacer()
                    
                    VStack {
                        
                        // Name section
                        Text(String(localized: "Please enter your name:"))
                        TextField(String(localized: "name"), text: $localPlayerName)
                            .frame(width: 200)
                            .border(.secondary)
                            .padding(.bottom)

                        // Start Options
                        HStack {
                            
                            // New Game
                            Button {
                                if self.localPlayerName.isEmpty {
                                    self.showingNameNeededAlert = true
                                    self.navigateToGameView = false
                                } else {
                                    self.showingNameNeededAlert = false
                                    self.navigateToGameView = true
                                }
                            } label: {
                                Text(String(localized: "Start New Game"))
                                    .padding()
                            }
                            
                            // Join a Game
                            Button {
                                if self.localPlayerName.isEmpty {
                                    self.showingNameNeededAlert = true
                                    self.navigateToEnterInvitationCodeView = false
                                } else {
                                    self.showingNameNeededAlert = false
                                    self.navigateToEnterInvitationCodeView = true
                                }
                            } label: {
                                Text(String(localized: "Accept An Invitation"))
                                    .padding()
                            }
                        }
                    }
                    .navigationDestination(isPresented: $navigateToGameView, destination: {
                        GameView(localPlayerName: self.localPlayerName)
                    })
                    .navigationDestination(isPresented: $navigateToEnterInvitationCodeView, destination: {
                        EnterInvitationCodeView(localPlayerName: localPlayerName)
                    })
                    .alert(String(localized: "Please choose a name to use."), isPresented: $showingNameNeededAlert) {
                    }

                    Spacer()                    
                }
                .frame(maxWidth: .infinity)
            }
            .background(Color("MainColor").gradient)
        }
    }
}

#Preview {
    StartGameView()
}
