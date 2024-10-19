//
//  GameInfoViewModel.swift
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com
//

import Foundation
import SwiftUI
import OpenAPIClient

/// Models a position on the game board.
struct Position {
    var row: Int
    var column: Int
}

/// Provides control over a Tic-Tac-Toe Game.
class GameInfoViewModel: ObservableObject {
    
    /// Specifies the locations of the Game pieces
    @Published private var gameBoard: [[GamePiece]] = []
    
    /// Indicates whether the Game has ended.
    @Published var gameEnded: Bool = false
    
    /// Remembers the ID of the Game. This is used for subsequent cals to the GameInfoService.
    @Published private var gameId: String = ""
    
    /// Informs this instance when our Tic Tac Toe service has updated the game state.
    private var gameInfoReceiver: GameInfoReceiver?

    /// ID of the Gaming Session.
    @Published private var gamingSessionId: String = ""
    
    /// When the game has ended, gameResults contains localized messaging that details the result of the Game.
    @Published var gameResults: String = ""
    
    /// Indicates whether the Game has been started.
    @Published var hasGameStarted: Bool = false
    
    /// The code used to invite a new player to the Game.
    @Published var invitationCode: String = ""
    
    /// Indicates whether Player One is the current player.
    @Published var isPlayerOneCurrentPlayer: Bool = false
    
    /// Indicates whether Player Two is the current player.
    @Published var isPlayerTwoCurrentPlayer: Bool = false
    
    /// Name of the local Player, i.e. the Player using this app instance.
    @Published var isTwoPlayer: Bool = true
    
    /// Indicates that this client app instance is the one that started the Gaming Session.
    @Published var localPlayerInitiatedGamingSession: Bool = false
    
    /// ID of the local Player, i.e. the Player using this app instance.
    @Published private var localPlayerId: String = ""
    
    /// Name of the local Player, i.e. the Player using this app instance.
    @Published var localPlayerName: String = ""
    
    /// ID of the other Player, i.e. the local Player's opponent.
    @Published var otherPlayerId: String = ""
    
    /// Name of the other Player, i.e. the local Player's opponent.
    @Published var otherPlayerName: String = ""
    
    /// Display name of Player One.
    @Published var playerOneDisplayName: String = ""
    
    /// Display name of Player Two.
    @Published var playerTwoDisplayName: String = ""
    
    /// If/when the Game has been won, winningPlayerName contains the name of the player who won the Game.
    @Published private var winningPlayerName: String?
    
    /// If/when the Game has been won, winningLocations lists the locations of the winning Game pieces.
    @Published private var winningLocations: [Position]?
    
    init(localPlayerName: String, isTwoPlayer: Bool, invitationCode: String = "") {
        self.gameInfoReceiver = nil
        self._localPlayerName = Published(initialValue: localPlayerName)
        self._invitationCode = Published(initialValue: invitationCode)
        self._isTwoPlayer = Published(initialValue: isTwoPlayer)
    }
}

extension GameInfoViewModel {
    
    /// Clears all of the variables that must be cleared in order to start a new Game.
    private func prepareForNewGame() {
        gameBoard = []
        gameEnded = false
        gameId = ""
        gameResults = ""
        hasGameStarted = false
        localPlayerInitiatedGamingSession = false
        invitationCode = ""
        isPlayerOneCurrentPlayer = false
        isPlayerTwoCurrentPlayer = false
        localPlayerId = ""
        playerOneDisplayName = ""
        playerTwoDisplayName = ""
        winningPlayerName = nil
        winningLocations = nil
    }
    
    /// Gets the textual represenation for the board position.
    func textForGamePiece(pos: Position) -> String {
        
        if self.gameBoard.isEmpty {
            return ""
        }

        let gamePiece = self.gameBoard[pos.row][pos.column]
        return switch gamePiece {
        case .unselected:
            ""
        case .x:
            "X"
        case .o:
            "O"
        }
    }
}

extension GameInfoViewModel {
    
    func createSinglePlayerGame() async -> Error? {
        
        // TODO: JD: allow the UI to set the AutomaticPlayerSkillLevel
        let result = await GameInfoService.createSinglePlayerGame(computerSkillLevel: AutomaticPlayerSkillLevel.intermediate, sessionId: self.gamingSessionId, localPlayerName: self.localPlayerName)

        if let newGameInfo = result.newGameInfo {
            DispatchQueue.main.async {
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.id)
                self.update(turnResult: TurnResult(newGameState: newGameInfo.gameInfo.gameState))
                self._localPlayerId = Published(wrappedValue: newGameInfo.gameInfo.players.first!.playerId)
            }
        }
        
        return result.error
    }
    
    func createTwoPlayerGame() async -> Error? {
        
        let result = await GameInfoService.createTwoPlayerGame(sessionId: self.gamingSessionId, localPlayerName: self.localPlayerName)
                
        if let newGameInfo = result.newGameInfo {            
            DispatchQueue.main.async {
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.id)
                self.update(turnResult: TurnResult(newGameState: newGameInfo.gameInfo.gameState))
                self._localPlayerId = Published(wrappedValue: newGameInfo.gameInfo.players.first!.playerId)
            }
        }
        
        return result.error
    }
    
    /// Creates and starts a new Game. Note that localPlayerName must be set before calling this function.
    func createGamingSession(completion: @escaping ((_ succeeded: Bool, _ error: Error?) -> Void)) async {
        
        let result = await GameInfoService.createGamingSession(sessionOwnerDisplayName: self.localPlayerName)
        
        if let newGamingSessionInfo = result.newGamingSessionInfo {
            DispatchQueue.main.async {
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self.initializeGame(info: newGamingSessionInfo)
                completion(true, nil)
            }
        } else {
            completion(false, result.error)
        }
    }
    
    /// Ends the current game and stops the auto updating of the game info.
    func endGame() async -> Error? {
        
        var result: Error? = nil
        
        // If this is the client that started the game, close it down on the server.
        if self.localPlayerInitiatedGamingSession {
            result = await GameInfoService.endGame(gameId: self.gameId, playerId: self.localPlayerId, sessionId: self.gamingSessionId).error
        }
        
        prepareForNewGame()
        
        return result
    }
    
    /// Ends the current Gaming Session.
    func endGamingSession() async -> Error? {
        
        // TODO: JD: finish
        return nil
    }
    
    /// Ends the current Gaming Session.
    func getSessionCurrentGame() async {
        let result = await GameInfoService.getSessionCurrentGame(sessionId: self.gamingSessionId)
        if let gameInfo = result.gameInfo {
            self._gameId = Published(initialValue: gameInfo.id)
            self.update(turnResult: TurnResult(newGameState: gameInfo.gameState))
        }
    }
    
    /// Generates the appropriate Game completion text.
    private func getGameResults(gameState: GameState, winningPlayerName: String) -> String {
        switch gameState.playStatus {
        case .endedInStalemate:
            return String(localized: "This game has ended in a stalemate.")
        case .endedInWin:
            if self.localPlayerName == winningPlayerName {
                return String(localized: "You won!")
            } else {
                return String(localized: "\(winningPlayerName) won. Better luck next time.")
            }
        default:
            return ""
        }
    }
    
    /// Joins a Gaming Session.
    func joinGamingSession(invitationCode: String) async -> Error? {        
        
        let result = await GameInfoService.joinGamingSession(invitationCode: self.invitationCode, playerName: self.localPlayerName)

        if let newGamingSessionInfo = result.newGamingSessionInfo {
            
            DispatchQueue.main.async {
                
                self.initializeGame(info: newGamingSessionInfo)

                Task {
                    await GameInfoService.notePlayerReadiness(sessionId: self.gamingSessionId, playerId: self.localPlayerId)
                }
            }
        }
        
        return result.error
    }
    
    /// When a Game has been won, this function determines whether the specified position (block) represents a position that won the Game.
    func isWinningPosition(pos: Position) -> Bool {
        if let winningLocations = self.winningLocations {
            return winningLocations.contains(where: { position in
                position.column == pos.column && position.row == pos.row
            })
        } else {
            return false
        }
    }
    
    /// Retrieves new game state info from our Tic Tac Toe service.
    func refreshGameInfo() {
        Task {
            let result = await GameInfoService.getGameInfo(gameId: self.gameId)
            if let gameInfo = result.gameInfo {
                self.update(turnResult: TurnResult(newGameState: gameInfo.gameState))
            }
        }
    }
    
    /// Performs a Game move for the specified Player.
    func takeTurn(pos: Position) async -> Error? {
        
        let result = await GameInfoService.takeTurn(gameId: self.gameId,
                                                    boardPosition: BoardPosition(column: pos.column, row: pos.row),
                                                    localPlayerId: self.localPlayerId, sessionId: self.gamingSessionId)
        
        if let gameInfo = result.gameInfo {
            // Even though we update the Game Info periodically, let's take this
            //  opportunity to update it immediately.
            self.update(turnResult: TurnResult(newGameState: gameInfo.gameState))
        }
        
        return result.error
    }
    
    /// Performs the inital setup of the Game parameters.
    private func initializeGame(info: GamingSessionCreationResult) {
        //
        
        DispatchQueue.main.async {
                        
            self._gamingSessionId = Published(wrappedValue: info.sessionId)
            
            self._invitationCode = Published(wrappedValue: info.invitationCode)

            if self.localPlayerInitiatedGamingSession {
                self._localPlayerId = Published(wrappedValue: info.initiatingPlayer.playerId)
                self._localPlayerName = Published(wrappedValue: info.initiatingPlayer.displayName)
            } else {
                self._localPlayerId = Published(wrappedValue: info.otherPlayer.playerId)
                self._localPlayerName = Published(wrappedValue: info.otherPlayer.displayName)
            }

            // Setup MQTT listener
            self.gameInfoReceiver = GameInfoReceiver(eventPlaneConfig: info.eventPlaneConfig, delegate: self)
        }
    }

    /// Updates this instance with the values of the passed in TurnResult.
    private func update(turnResult: TurnResult) {
        
        DispatchQueue.main.async {
            
            self.gameBoard = turnResult.newGameState.gameBoard
            
            self.gameEnded = turnResult.newGameState.playStatus == .endedInStalemate || turnResult.newGameState.playStatus == .endedInWin
            
            self.hasGameStarted = turnResult.newGameState.playStatus != .notStarted
            
//            // isPlayerOneCurrentPlayer
//            
//            if turnResult.newGameState.playStatus == .inProgress {
//                isPlayerOneCurrentPlayer = turnResult.newGameState.idOfPlayerWhoMadeMove != self.id
//            } else {
//                self.isPlayerOneCurrentPlayer = false
//                self.isPlayerTwoCurrentPlayer = false
//            }
                        
//            // Player One Display Name
//            self.playerOneDisplayName = turnResult.newGameState.players.first!.displayName
            
//            // Player Two Display Name
//            self.playerTwoDisplayName = turnResult.newGameState.players.count > 1 ? turnResult.newGameState.players.last!.displayName : ""
            
//            // Other Player Display Name
//            if turnResult.newGameState.players.count > 1 {
//                if self.localPlayerId == turnResult.newGameState.players.first!.playerId {
//                    self.otherPlayerName = self.playerTwoDisplayName
//                } else {
//                    self.otherPlayerName = self.playerOneDisplayName
//                }
//            } else {
//                self.otherPlayerName = ""
//            }
            
            // winningPlayerName and winningLocations
            if turnResult.newGameState.playStatus == .endedInWin {
                
                self.winningPlayerName = turnResult.winningPlayer?.displayName ?? ""
                
                var locations: [Position] = []
                for loc in turnResult.winningLocations! {
                    locations.append(Position(row: loc.row, column: loc.column))
                }
                self.winningLocations = locations
                
            } else {
                self.winningPlayerName = nil
            }
            
            self.gameResults = self.getGameResults(gameState: turnResult.newGameState, winningPlayerName: self.winningPlayerName ?? "")
        }
    }
}

/// GameInfoReceiverDelegate implementation
extension GameInfoViewModel: GameInfoReceiverDelegate {
        
    func onGameDeleted() {
        refreshGameInfo()
    }
    
    func onGameStarted() {
        if !self.localPlayerInitiatedGamingSession {
            Task {
                await self.getSessionCurrentGame()
            }
        } else {
            refreshGameInfo()
        }
    }
    
    func onGameEndedInStalemate() {

        refreshGameInfo()
        
        // TODO: JD: ask for a rematch
    }
    
    func onGameEndedInWin() {

        refreshGameInfo()

        // TODO: JD: ask for a rematch
    }

    func onPlayerReady() {
        if self.localPlayerInitiatedGamingSession && self.isTwoPlayer {
            Task {
                await self.createTwoPlayerGame()
            }
        }
    }

    func onSessionDeleted() {
        refreshGameInfo()
    }
        
    func onTurnTaken() {
        refreshGameInfo()
    }
}
