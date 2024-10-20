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
    @Published var localPlayer: PlayerInfo

    /// Indicates whether the local Player is the one who takes their turn firstly.
    @Published var playerOneId: String = ""
    
    /// ID of the local Player, i.e. the Player using this app instance.
    @Published var otherPlayer: PlayerInfo

    /// If/when the Game has been won, winningPlayerName contains the name of the player who won the Game.
    @Published private var winningPlayerName: String?
    
    /// If/when the Game has been won, winningLocations lists the locations of the winning Game pieces.
    @Published private var winningLocations: [Position]?
    
    init(localPlayerName: String, isTwoPlayer: Bool, invitationCode: String = "") {
        self.gameInfoReceiver = nil
        self._invitationCode = Published(initialValue: invitationCode)
        self._isTwoPlayer = Published(initialValue: isTwoPlayer)
        self.localPlayer = PlayerInfo.init(displayName: localPlayerName,
                                           gamePiece: GamePiece.unselected,
                                           isAutomated: false,
                                           playerId: "")
        self.otherPlayer = PlayerInfo(displayName: "",
                                      gamePiece: GamePiece.unselected,
                                      isAutomated: false,
                                      playerId: "")
    }
    
    /// Returns the first Player.
    func getPlayerOne() -> PlayerInfo {
        if self.playerOneId == self.localPlayer.playerId {
            self.localPlayer
        } else {
            self.otherPlayer
        }
    }

    /// Returns the second Player.
    func getPlayerTwo() -> PlayerInfo {
        if self.playerOneId != self.localPlayer.playerId {
            self.localPlayer
        } else {
            self.otherPlayer
        }
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
        let result = await GameInfoService.createSinglePlayerGame(computerSkillLevel: AutomaticPlayerSkillLevel.intermediate, sessionId: self.gamingSessionId, localPlayerName: self.localPlayer.displayName)

        if let newGameInfo = result.newGameInfo {
            
            DispatchQueue.main.async {
                
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.id)
                                
                self.localPlayer = newGameInfo.gameInfo.players.first(where: { it in
                    it.playerId == self.localPlayer.playerId
                })!

                self.otherPlayer = newGameInfo.gameInfo.players.first(where: { it in
                    it.playerId != self.localPlayer.playerId
                })!
                
                self._playerOneId = Published(wrappedValue: newGameInfo.gameInfo.players.first!.playerId)
                                                
                self._isTwoPlayer = Published(wrappedValue: false)
                
                self.updateGameInfo(turnResult: TurnResult(currentPlayer: newGameInfo.gameInfo.currentPlayer, newGameState: newGameInfo.gameInfo.gameState))
            }
        }
        
        return result.error
    }

    func createTwoPlayerGame() async -> Error? {
        
        let result = await GameInfoService.createTwoPlayerGame(sessionId: self.gamingSessionId,
                                                               localPlayerName: self.localPlayer.displayName)

        if let newGameInfo = result.newGameInfo {
            
            DispatchQueue.main.async {
            
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.id)
                
                self.localPlayer = newGameInfo.gameInfo.players.first(where: { it in
                    it.playerId == self.localPlayer.playerId
                })!
                
                self.otherPlayer = newGameInfo.gameInfo.players.first(where: { it in
                    it.playerId != self.localPlayer.playerId
                })!
                
                self._playerOneId = Published(wrappedValue: newGameInfo.gameInfo.players.first!.playerId)
                                
                self._isPlayerOneCurrentPlayer = Published(wrappedValue: true)
                self._isPlayerTwoCurrentPlayer = Published(wrappedValue: false)
                self._isTwoPlayer = Published(wrappedValue: true)
                
                self.updateGameInfo(turnResult: TurnResult(newGameState: newGameInfo.gameInfo.gameState))
            }
        }

        return result.error
    }
    
    /// Creates and starts a new Game. Note that localPlayerName must be set before calling this function.
    func createGamingSession(completion: @escaping ((_ succeeded: Bool, _ error: Error?) -> Void)) async {
        
        let result = await GameInfoService.createGamingSession(sessionOwnerDisplayName: self.localPlayer.displayName)
        
        if let newGamingSessionInfo = result.newGamingSessionInfo {
            DispatchQueue.main.async {
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self.updateGamingSessionInfo(info: newGamingSessionInfo)
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
            result = await GameInfoService.endGame(gameId: self.gameId,
                                                   playerId: self.localPlayer.playerId,
                                                   sessionId: self.gamingSessionId).error
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
            self.updateGameInfo(turnResult: TurnResult(newGameState: gameInfo.gameState))
        }
    }
    
    /// Generates the appropriate Game completion text.
    private func getGameResults(gameState: GameState, winningPlayerName: String) -> String {
        switch gameState.playStatus {
        case .endedInStalemate:
            return String(localized: "This game has ended in a stalemate.")
        case .endedInWin:
            if self.localPlayer.displayName == winningPlayerName {
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
        
        let result = await GameInfoService.joinGamingSession(invitationCode: self.invitationCode, playerName: self.localPlayer.displayName)

        if let newGamingSessionInfo = result.newGamingSessionInfo {
            
            DispatchQueue.main.async {
                
                self.updateGamingSessionInfo(info: newGamingSessionInfo)

                Task {
                    await GameInfoService.notePlayerReadiness(sessionId: self.gamingSessionId, playerId: self.localPlayer.playerId)
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
            let result = await GameInfoService.getLatestTurn(gameId: self.gameId)
            if let turnResult = result.turnResult {
                self.updateGameInfo(turnResult: turnResult)
            }
        }
    }
    
    /// Performs a Game move for the specified Player.
    func takeTurn(pos: Position) async -> Error? {
        
        let result = await GameInfoService.takeTurn(gameId: self.gameId,
                                                    boardPosition: BoardPosition(column: pos.column, row: pos.row),
                                                    localPlayerId: self.localPlayer.playerId, sessionId: self.gamingSessionId)
        
        if let gameInfo = result.gameInfo {
            self.updateGameInfo(turnResult: TurnResult(newGameState: gameInfo.gameState))
        }
        
        return result.error
    }
    
    /// Performs the inital setup of the Game parameters.
    private func updateGamingSessionInfo(info: GamingSessionCreationResult) {
        //
        
        DispatchQueue.main.async {
                        
            self._gamingSessionId = Published(wrappedValue: info.sessionId)
            
            self._invitationCode = Published(wrappedValue: info.invitationCode)

            if self.localPlayerInitiatedGamingSession {
                self.localPlayer.playerId = info.initiatingPlayer.playerId
            }

            // Setup MQTT listener
            self.gameInfoReceiver = GameInfoReceiver(eventPlaneConfig: info.eventPlaneConfig, delegate: self)
        }
    }

    /// Updates this instance with the values of the passed in TurnResult.
    private func updateGameInfo(turnResult: TurnResult) {
        
        DispatchQueue.main.async {
            
            self.gameBoard = turnResult.newGameState.gameBoard
            
            self.gameEnded = turnResult.newGameState.playStatus == .endedInStalemate || turnResult.newGameState.playStatus == .endedInWin
            
            self.hasGameStarted = turnResult.newGameState.playStatus != .notStarted
            
            self._isPlayerOneCurrentPlayer = Published(wrappedValue: turnResult.currentPlayer?.playerId == self.playerOneId)
            self._isPlayerTwoCurrentPlayer = Published(wrappedValue: turnResult.currentPlayer?.playerId != self.playerOneId)

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
        self.hasGameStarted = true
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
