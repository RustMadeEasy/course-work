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
    
//    /// ID of the Gaming Session.
//    @Published var eventPlaneConfig: EventPlaneConfig? = nil
    
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
    
    /// Indicates that this client app instance is the one that started the Gaming Session.
    @Published private var initiatedGamingSession: Bool = false
    
    /// The code used to invite a new player to the Game.
    @Published var invitationCode: String = ""
    
    /// Indicates whether Player One is the current player.
    @Published var isPlayerOneCurrentPlayer: Bool = false
    
    /// Indicates whether Player Two is the current player.
    @Published var isPlayerTwoCurrentPlayer: Bool = false
    
    /// Name of the local Player, i.e. the Player using this app instance.
    @Published var isTwoPlayer: Bool = true
    
    /// ID of the local Player, i.e. the Player using this app instance.
    @Published private var localPlayerId: String = ""
    
    /// Name of the local Player, i.e. the Player using this app instance.
    @Published var localPlayerName: String = ""
    
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
        initiatedGamingSession = false
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
        case ._none:
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
        let result = await GameInfoService.createSinglePlayerGame(computerSkillLevel: AutomaticPlayerSkillLevel.intermediate, sessionId: self.gamingSessionId)
                
        if let newGameInfo = result.newGameInfo {
            
            DispatchQueue.main.async {
                self._initiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.id)
                self.update(gameInfo: newGameInfo.gameInfo)
                self._localPlayerId = Published(wrappedValue: newGameInfo.gameInfo.players.first!.playerId)
            }
        }
        
        return result.error
    }
    
    func createTwoPlayerGame() async -> Error? {
        
        let result = await GameInfoService.createTwoPlayerGame(sessionId: self.gamingSessionId)
                
        if let newGameInfo = result.newGameInfo {
            
            DispatchQueue.main.async {
                self._initiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.id)
                self.update(gameInfo: newGameInfo.gameInfo)
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
                self._initiatedGamingSession = Published(wrappedValue: true)
                self.invitationCode = result.newGamingSessionInfo?.invitationCode ?? ""
                self.gamingSessionId = result.newGamingSessionInfo?.sessionId ?? ""
                // Setup MQTT listener
                self.gameInfoReceiver = GameInfoReceiver(eventPlaneConfig: newGamingSessionInfo.eventPlaneConfig, delegate: self)
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
        if self.initiatedGamingSession {
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
        if result.error == nil && result.gameInfo != nil {
            self.update(gameInfo: result.gameInfo!)
        }
    }
    
    /// Generates the appropriate Game completion text.
    private func getGameResults(gameInfo: GameInfo, winningPlayerName: String) -> String {
        switch gameInfo.gameState.playStatus {
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
                self._initiatedGamingSession = Published(wrappedValue: false)
                self._gameId = Published(initialValue: newGamingSessionInfo.currentGameId)
                self._gamingSessionId = Published(initialValue: newGamingSessionInfo.sessionId)
                // Setup MQTT listener
                self.gameInfoReceiver = GameInfoReceiver(eventPlaneConfig: newGamingSessionInfo.eventPlaneConfig, delegate: self)
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
            if result.error == nil && result.gameInfo != nil {
                self.update(gameInfo: result.gameInfo!)
            }
        }
    }
    
    /// Performs a Game move for the specified Player.
    func takeTurn(pos: Position) async -> Error? {
        
        let result = await GameInfoService.takeTurn(gameId: self.gameId,
                                                    boardPosition: BoardPosition(column: pos.column, row: pos.row),
                                                    localPlayerId: self.localPlayerId, sessionId: self.gamingSessionId)
        
        if result.error == nil && result.gameInfo != nil {
            // Even though we update the Game Info periodically, let's take this
            //  opportunity to update it immediately.
            self.update(gameInfo: result.gameInfo!)
        }
        
        return result.error
    }
    
    /// Updates this instance with the values of the passed in GameInfo.
    private func update(gameInfo: GameInfo) {
        
        DispatchQueue.main.async {
            
            self.gameBoard = gameInfo.gameState.gameBoard
            
            self.gameEnded = gameInfo.gameState.playStatus == .endedInStalemate || gameInfo.gameState.playStatus == .endedInWin
            
            self.hasGameStarted = gameInfo.gameState.playStatus != .notStarted
            
            // isPlayerOneCurrentPlayer
            if gameInfo.gameState.playStatus == .inProgress {
                self.isPlayerOneCurrentPlayer = gameInfo.players.first?.playerId == gameInfo.currentPlayer?.playerId ?? ""
            } else {
                self.isPlayerOneCurrentPlayer = false
            }
            
            // isPlayerTwoCurrentPlayer
            if gameInfo.gameState.playStatus == .inProgress {
                self.isPlayerTwoCurrentPlayer = gameInfo.players.last?.playerId == gameInfo.currentPlayer?.playerId ?? ""
            } else {
                self.isPlayerTwoCurrentPlayer = false
            }
            
            // Player One Display Name
            self.playerOneDisplayName = gameInfo.players.first!.displayName
            
            // Player Two Display Name
            self.playerTwoDisplayName = gameInfo.players.count > 1 ? gameInfo.players.last!.displayName : ""
            
            // Other Player Display Name
            if gameInfo.players.count > 1 {
                if self.localPlayerId == gameInfo.players.first!.playerId {
                    self.otherPlayerName = self.playerTwoDisplayName
                } else {
                    self.otherPlayerName = self.playerOneDisplayName
                }
            } else {
                self.otherPlayerName = ""
            }
            
            // winningPlayerName and winningLocations
            if gameInfo.gameState.playStatus == .endedInWin {
                
                let winnerName = gameInfo.players.first(where: { it in
                    it.playerId == gameInfo.gameState.winningPlayerId
                })?.displayName ?? ""
                self.winningPlayerName = winnerName
                
                var locations: [Position] = []
                for loc in gameInfo.gameState.winningLocations! {
                    locations.append(Position(row: loc.row, column: loc.column))
                }
                self.winningLocations = locations
                
            } else {
                self.winningPlayerName = nil
            }
            
            self.gameResults = self.getGameResults(gameInfo: gameInfo, winningPlayerName: self.winningPlayerName ?? "")
        }
    }
}

/// GameInfoReceiverDelegate implementation
extension GameInfoViewModel: GameInfoReceiverDelegate {

    func onGameDeleted() {
        refreshGameInfo()
    }
    
    func onGameStarted() {
        refreshGameInfo()
    }
    
    func onPlayerAddedToSession() {
        if self.initiatedGamingSession {
            Task {
                await self.createTwoPlayerGame()
            }
        }
    }
    
    func onSessionDeleted() {
        refreshGameInfo()
    }
        
    func onGameEndedInStalemate() {

        refreshGameInfo()
        
        // TODO: JD: ask for a rematch
    }
    
    func onGameEndedInWin() {

        refreshGameInfo()

        // TODO: JD: ask for a rematch
    }
        
    func onTurnTaken() {
        refreshGameInfo()
    }
}
