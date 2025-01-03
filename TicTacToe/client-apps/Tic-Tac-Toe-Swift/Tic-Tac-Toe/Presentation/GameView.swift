//
//  ContentView.swift
//  Tic Tac Toe
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com
//

import SwiftUI
import OpenAPIClient

/// Presents the user with the Game state as well as Game manipulation controls.
struct GameView: View {
    
    @Environment(\.presentationMode) private var presentation

    static let buttonSide = 100.0
    static let separatorWidth = 2.0
    
    /// Wraps our Tic-Tac-Toe API calls and provides the local game state
    @StateObject private var gameInfoVM: GameViewModel

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
    
    // MARK: Initialization

    /// Creates a new instance. The invitation code must be provided when joining an existing Game.
    public init(localPlayerName: String, isTwoPlayer: Bool, invitationCode: String = "") {
        self._gameInfoVM = StateObject(wrappedValue: GameViewModel(localPlayerName: localPlayerName, isTwoPlayer: isTwoPlayer, invitationCode: invitationCode))
    }

    // MARK: Layout
    
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
            if !self.gameInfoVM.invitationCode.isEmpty {
                await self.joinGamingSession()
            } else if self.gameInfoVM.isTwoPlayer {
                await self.createTwoPlayerGame()
            } else {
                await self.createSinglePlayerGame()
            }
        }
        .onDisappear() {
            Task {
                let _ = await gameInfoVM.endGame()
                let _ = await gameInfoVM.endGamingSession()
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
            .alert(String(localized: "Failed to join the game because Player One has already selected the name '\(gameInfoVM.localPlayer.displayName)'. Please select a different Player name."), isPresented: $showNameConflictGameJoiningError ) {
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
            .alert(String(localized: "It is currently time for '\(self.gameInfoVM.otherPlayer?.displayName ?? "")' to take their turn."), isPresented: $showWrongTurnError ) {
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
            let playerOne = String(localized: "Player One\(self.gamePieceToText(gamePiece: gameInfoVM.getPlayerOne()?.gamePiece ?? .unselected)): \(gameInfoVM.getPlayerOne()?.displayName ?? "")")
            Text(playerOne)
                .foregroundStyle(Color("AlternateColor").gradient)
                .bold(gameInfoVM.isPlayerOneCurrentPlayer())
            let playerTwo = String(localized: "Player Two\(self.gamePieceToText(gamePiece: gameInfoVM.getPlayerTwo()?.gamePiece ?? .unselected)): \(gameInfoVM.getPlayerTwo()?.displayName ?? "")")
            Text(playerTwo)
                .foregroundStyle(Color("AlternateColor").gradient)
                .bold(gameInfoVM.isPlayerTwoCurrentPlayer())
        }.padding()
    }
    
    /// Horizontal separator.
    var rowSeparator: some View {
        Rectangle()
            .frame(height: GameView.separatorWidth)
            .foregroundColor(Color("MainColor"))
    }

    // MARK: Funtionality

    /// Begins a new Single-Player Game.
    private func createSinglePlayerGame() async {
        
        await self.gameInfoVM.createGamingSession { succeeded, _ in
            if succeeded {
                self.gameInfoVM.localPlayerInitiatedGamingSession = true
                self.gameInfoVM.isTwoPlayer = false
                Task {
                    await self.gameInfoVM.createSinglePlayerGame { succeeded2, _ in
                        if succeeded2 {
                            Task {
                                await self.gameInfoVM.joinCurrentGame { succeeded3, _ in
                                    if !succeeded3 {
                                        DispatchQueue.main.async {
                                            self.showGameCreationError = true
                                        }
                                    }
                                }
                            }
                        } else {
                            DispatchQueue.main.async {
                                self.showGameCreationError = true
                            }
                        }
                    }
                }
            } else {
                DispatchQueue.main.async {
                    self.gameInfoVM.localPlayerInitiatedGamingSession = false
                    self.gameInfoVM.isTwoPlayer = false
                    self.showGameCreationError = true
                }
            }
        }
    }
        
    /// Begins a Two-Player Game.
    private func createTwoPlayerGame() async {
        
        await self.gameInfoVM.createGamingSession { succeeded, _ in
            if succeeded {
                self.gameInfoVM.localPlayerInitiatedGamingSession = true
                self.gameInfoVM.isTwoPlayer = true
                Task {
                    await self.gameInfoVM.createTwoPlayerGame { succeeded2, _ in
                        if succeeded2 {
                            Task {
                                await self.gameInfoVM.joinCurrentGame { succeeded3, _ in
                                    if !succeeded3 {
                                        DispatchQueue.main.async {
                                            self.showGameCreationError = true
                                        }
                                    }
                                }
                            }
                        } else {
                            DispatchQueue.main.async {
                                self.showGameCreationError = true
                            }
                        }
                    }
                }
            } else {
                DispatchQueue.main.async {
                    self.gameInfoVM.localPlayerInitiatedGamingSession = false
                    self.gameInfoVM.isTwoPlayer = false
                    self.showGameCreationError = true
                }
            }
        }
    }
        
    private func gamePieceToText(gamePiece: GamePiece) -> String {
        switch gamePiece {
        case .o, .x:
            return " (\(gamePiece.rawValue.uppercased()))"
        default:
            return ""
        }
    }
 
    /// Joins an existing Gaming Session.
    private func joinGamingSession() async {
        
        await self.gameInfoVM.joinGamingSession(invitationCode: self.gameInfoVM.invitationCode) { succeeded, error in
            Task {
                await self.gameInfoVM.joinCurrentGame { succeeded, error in
                    if !succeeded {
                        DispatchQueue.main.async {
                            self.showGameCreationError = true
                        }
                    }
                }
            }
        }
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

// MARK: Preview generation

#Preview {
    GameView(localPlayerName: "Player One", isTwoPlayer: false)
}
