//
//  ContentView.swift
//  Tic Tac Toe
//
// © 2024 Rust Made Easy. All rights reserved.
// @author Info@RustMadeEasy.com
//

import SwiftUI
import OpenAPIClient

/// Presents the user with the Game state as well as Game manipulation controls.
struct GameView: View {

    @Environment(\.presentationMode) private var presentation

    static let buttonSide = 100.0
    static let separatorWidth = 2.0
    
    /// Wraps our Tic-Tac-Toe API calls and provides the local game state
    @StateObject private var gameInfoVM: GameInfoViewModel

    /// Invokes the 'game creation error' alert
    @State private var showGameCreationError = false

    /// Invokes the general purpose 'game comninuation error' alert
    @State private var showGenericGameError = false

    /// Invokes the 'name conflict' alert
    @State private var showNameConflictGameJoiningError = false

    /// Invokes an alert to indicate the 'wrong invitation code'
    @State private var showWrongCodeGameJoiningError = false

    /// Invokes an alert to indicate user is making a move out of turn
    @State private var showWrongTurnError = false

    /// Invokes an alert to indicate user is making a move to a location that is already occupied
    @State private var showLocationAlreadyOccupiedError = false

    /// The Game board.
    var board: some View {
        
        VStack {
            
            // Header
            HStack {

                VStack {
                    
                    invitationSection

                    playersSection
                }
                .padding()
            }
                .frame(maxWidth: .infinity)

            // Grid
            VStack {
                
                // Row One
                renderRow(row: 0)
                rowSeparator
                
                // Row Two
                renderRow(row: 1)
                rowSeparator
                
                // Row Three
                renderRow(row: 2)
            }
            .aspectRatio(1.0, contentMode: .fit)
            .padding(.bottom, 20)

        }
        .frame(maxWidth: .infinity, maxHeight: .infinity)
    }
    
    /// The view's body
    var body: some View {
        ZStack {
            self.board
        }
        .background(Color("MainColor").gradient)
        .overlay(content: {
            commonAlerts
        })
        .task {
            await createOrJoinGame()
        }
        .onDisappear() {
            Task {
                await gameInfoVM.endGame()
            }
        }
    }
    
    /// Vertical separator
    var colSeparator: some View {
        Rectangle()
            .frame(width: GameView.separatorWidth)
            .foregroundColor(Color("MainColor"))
            .padding(.vertical, -8)
    }

    /// All Game View level alerts
    var commonAlerts: some View {
        EmptyView()
            .alert(gameInfoVM.gameResults, isPresented: $gameInfoVM.gameEnded) {
                Button() {
                    self.presentation.wrappedValue.dismiss()
                } label: {
                    Text(String(localized: "Ok"))
                }
            }
            .alert(String(localized: "Failed to create new game. Please make sure there is a valid internet connection."), isPresented: $showGameCreationError) {
                Button() {
                    showGameCreationError = false
                    self.presentation.wrappedValue.dismiss()
                } label: {
                    okText
                }
            }
            .alert(String(localized: "Failed continue the game. Please make sure there is a valid internet connection."), isPresented: $showGenericGameError ) {
                Button() {
                    showGenericGameError = false
                } label: {
                    okText
                }
                Button() {
                    showGenericGameError = false
                    self.presentation.wrappedValue.dismiss()
                } label: {
                    Text(String(localized: "End Game"))
                }
            }
            .alert(String(localized: "Failed to join the game because Player One has already selected the name '\(gameInfoVM.localPlayerName)'. Please select a different Player name."), isPresented: $showNameConflictGameJoiningError ) {
                Button() {
                    showNameConflictGameJoiningError = false
                    self.presentation.wrappedValue.dismiss()
                } label: {
                    okText
                }
            }
            .alert(String(localized: "Failed to join the game because a game with the invitation code '\(self.gameInfoVM.invitationCode)' could not be found. Please make sure the invitation code is correct."), isPresented: $showWrongCodeGameJoiningError ) {
                Button() {
                    showWrongCodeGameJoiningError = false
                    self.presentation.wrappedValue.dismiss()
                } label: {
                    okText
                }
            }
            .alert(String(localized: "It is currently time for '\(self.gameInfoVM.otherPlayerName)' to take their turn."), isPresented: $showWrongTurnError ) {
                Button() {
                    showWrongTurnError = false
                } label: {
                    okText
                }
            }
            .alert(String(localized: "This location is already occupied. Please use another location."), isPresented: $showLocationAlreadyOccupiedError ) {
                Button() {
                    showLocationAlreadyOccupiedError = false
                } label: {
                    okText
                }
            }
    }

    /// Creates a new Game or joins an existing Game. If an inviation code is present, an existing Game is joined. Otherwise, a new Game is created.
    private func createOrJoinGame() async {
        
        if self.gameInfoVM.invitationCode.isEmpty {
            if await gameInfoVM.createGame() != nil {
                showGameCreationError = true
            }
        } else {
            let error = await gameInfoVM.joinGame(invitationCode: self.gameInfoVM.invitationCode)
            if let error = error {
                switch error {
                case ErrorResponse.error(let code, _, _, _):
                    switch code {
                    case 404:
                        showWrongCodeGameJoiningError = true
                    case 409:
                        showNameConflictGameJoiningError = true
                    default:
                        showGenericGameError = true
                    }
                default:
                    showGenericGameError = true
                }
            }
        }
    }

    /// Creates a new GameInfoViewModel instance. The invitation code must be provided when joining an existing Game.
    public init(localPlayerName: String, invitationCode: String = "") {
        self._gameInfoVM = StateObject(wrappedValue: GameInfoViewModel(localPlayerName: localPlayerName, invitationCode: invitationCode))
    }

    /// Section of the View that shows the Game invitation.
    var invitationSection: AnyView {
        
        // Show the invitation code until the game has begun
        if !gameInfoVM.hasGameStarted {
            AnyView (Button {
                ClipboardHelper.copyTextToClipboard(text: gameInfoVM.invitationCode)
            } label: {
                let text = String(localized: "Invitation Code: \(gameInfoVM.invitationCode)")
                Text(text)
                    .foregroundStyle(Color("AlternateColor"))
                Image(systemName: "doc.on.doc")
                    .foregroundStyle(Color("AlternateColor"))
                    .padding(.leading, 8)
            }
            .padding(.top))
        } else {
            AnyView(EmptyView())
        }
    }

    /// Localized OK text.
    private var okText: some View {
        Text(String(localized: "Ok"))
    }

    /// Section of the view holding the Players.
    var playersSection: some View {
     
        VStack {
            let playerOne = String(localized: "Player One (X): \(gameInfoVM.playerOneDisplayName)")
            Text(playerOne)
                .foregroundStyle(Color("AlternateColor").gradient)
                .bold(gameInfoVM.isPlayerOneCurrentPlayer)
            let playerTwo = String(localized: "Player Two (O): \(gameInfoVM.playerTwoDisplayName)")
            Text(playerTwo)
                .foregroundStyle(Color("AlternateColor").gradient)
                .bold(gameInfoVM.isPlayerTwoCurrentPlayer)
        }.padding()
    }

    /// Renders a Game block (game piece holder).
    func renderBlock(position: Position) -> some View {
        let isWinningPosition = gameInfoVM.isWinningPosition(pos: position)
        return Button {
            takeTurn(position: position)
        } label: {
            Text(gameInfoVM.textForGamePiece(pos: position))
                .bold(isWinningPosition)
                .foregroundStyle(Color("MainColor").gradient)
                .font(.largeTitle)
                .scaleEffect(2.0)
                .frame(maxHeight: .infinity)
                .frame(maxWidth: .infinity)
        }
        .buttonStyle(BorderlessButtonStyle())
        .background(Color("AlternateColor").gradient)
        .border(Color("HighlightColor"), width: isWinningPosition ? 3 : 0) // higlight winning blocks
        .padding(-8)
    }

    /// Renders a row of the Tic-Tac-Toe game board.
    func renderRow(row: Int) -> some View {

        HStack {

            Spacer()
            renderBlock(position: Position(row: row, column: 0))
            Spacer()

            colSeparator

            Spacer()
            renderBlock(position: Position(row: row, column: 1))
            Spacer()

            colSeparator

            Spacer()
            renderBlock(position: Position(row: row, column: 2))
            Spacer()

        }.background(.white)
    }
    
    /// Horizontal separator.
    var rowSeparator: some View {
        Rectangle()
            .frame(height: GameView.separatorWidth)
            .foregroundColor(Color("MainColor"))
    }
    
    /// Performs a Game move for the specified Player.
    private func takeTurn(position: Position) {
        Task {
            let error = await gameInfoVM.takeTurn(pos: position)
            if let error = error {
                switch error {
                case ErrorResponse.error(let code, _, _, _):
                    switch code {
                    case 405:
                        showWrongTurnError = true
                    case 409:
                        showLocationAlreadyOccupiedError = true
                    default:
                        showGenericGameError = true
                    }
                default:
                    showGenericGameError = true
                }
            }
        }
    }
}

#Preview {
    GameView(localPlayerName: "")
}
