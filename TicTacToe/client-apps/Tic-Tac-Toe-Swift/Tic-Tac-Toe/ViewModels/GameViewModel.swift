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
class GameViewModel: ObservableObject {
    
    /// The Player who can take the next turn.
    @Published private var currentPlayer: PlayerInfo?

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
    
    /// Indicates that this is a Two-Player Game.
    @Published var isTwoPlayer: Bool = true
    
    // TODO: JD: we also need localPlayerInitiatedGame for when we support rematch within the same Gaming Session.
    /// Indicates that this client app instance is the one that started the Gaming Session.
    @Published var localPlayerInitiatedGamingSession: Bool = false
    
    /// ID of the local Player, i.e. the Player using this app instance.
    @Published var localPlayer: PlayerInfo

    /// ID of the local Player, i.e. the Player using this app instance.
    @Published var otherPlayer: PlayerInfo?

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
    
    /// Returns the first Player. By convention, Player One uses X.
    func getPlayerOne() -> PlayerInfo? {
        if self.localPlayer.gamePiece == .x {
            self.localPlayer
        } else if self.otherPlayer?.gamePiece == .x {
            self.otherPlayer
        } else {
            nil
        }
    }

    /// Returns the second Player. By convention, Player Two uses O.
    func getPlayerTwo() -> PlayerInfo? {
        if self.localPlayer.gamePiece == .o {
            self.localPlayer
        } else if self.otherPlayer?.gamePiece == .o {
            self.otherPlayer
        } else {
            nil
        }
    }
}

extension GameViewModel {
    
    func isPlayerOneCurrentPlayer() -> Bool {
        self.getPlayerOne()?.playerId == self.currentPlayer?.playerId
    }
    
    func isPlayerTwoCurrentPlayer() -> Bool {
        self.getPlayerTwo()?.playerId == self.currentPlayer?.playerId
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
    
    /// Clears all of the variables that must be cleared in order to start a new Game.
    private func prepareForNewGame() {
        gameBoard = []
        gameEnded = false
        gameId = ""
        gameResults = ""
        hasGameStarted = false
        localPlayerInitiatedGamingSession = false
        invitationCode = ""
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

extension GameViewModel {
    
    func createSinglePlayerGame(completion: @escaping ((_ succeeded: Bool, _ error: Error?) -> Void)) async {
        
        // TODO: JD: allow the UI to set the AutomaticPlayerSkillLevel
        let result = await GameInfoService.createSinglePlayerGame(computerSkillLevel: AutomaticPlayerSkillLevel.intermediate, sessionId: self.gamingSessionId, localPlayerName: self.localPlayer.displayName)

        if let newGameInfo = result.gameCreationResult {
            
            DispatchQueue.main.async {
                
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.gameId)
                
                completion(true, nil)
            }
        } else {
            completion(false, result.error)
        }
    }
    
    func createTwoPlayerGame(completion: @escaping ((_ succeeded: Bool, _ error: Error?) -> Void)) async {
        
        let result = await GameInfoService.createTwoPlayerGame(sessionId: self.gamingSessionId,
                                                               localPlayerName: self.localPlayer.displayName)

        if let newGameInfo = result.gameCreationResult {
            
            DispatchQueue.main.async {
            
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self._gameId = Published(wrappedValue: newGameInfo.gameInfo.gameId)
                
                completion(true, nil)
            }
        } else {
            completion(false, result.error)
        }
    }
    
    /// Creates and starts a new Gaming Session.
    func createGamingSession(completion: @escaping ((_ succeeded: Bool, _ error: Error?) -> Void)) async {
        
        let result = await GameInfoService.createGamingSession(sessionOwnerDisplayName: self.localPlayer.displayName)
        
        if let newGamingSessionInfo = result.gamingSessionCreationResult {
            DispatchQueue.main.async {
                self._localPlayerInitiatedGamingSession = Published(wrappedValue: true)
                self.updateGamingSessionInfo(info: newGamingSessionInfo) {
                    completion(true, nil)
                }
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
    
    /// Joins a Game.
    func joinCurrentGame(completion: @escaping ((_ succeeded: Bool, _ error: Error?) -> Void)) async {
        
        let result = await GameInfoService.joinCurrentGame(sessionId: self.gamingSessionId, localPlayerId: self.localPlayer.playerId)

        if let newGameInfo = result.gameCreationResult {
            
            DispatchQueue.main.async {
                
                self.gameId = newGameInfo.gameInfo.gameId
                
                self.setupPlayers(newGameInfo: newGameInfo)
                
                self.updateGameInfo(turnResult: TurnResponse(currentPlayer: newGameInfo.gameInfo.currentPlayer, newGameState: newGameInfo.gameInfo.gameState))
                
                Task {
                    completion(true, nil)
                }
            }
        } else {
            completion(false, result.error)
        }
    }
    
    /// Joins a Gaming Session.
    func joinGamingSession(invitationCode: String, completion: @escaping ((_ succeeded: Bool, _ error: Error?) -> Void)) async {
        
        let result = await GameInfoService.joinGamingSession(invitationCode: self.invitationCode, playerName: self.localPlayer.displayName)

        if let newGamingSessionInfo = result.gamingSessionCreationResult {
            DispatchQueue.main.async {
                self.updateGamingSessionInfo(info: newGamingSessionInfo) {
                    completion(true, nil)
                }
            }
        } else {
            completion(false, result.error)
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
    
    private func setupPlayers(newGameInfo: GameCreationResponse) {
        
        self._currentPlayer = Published(wrappedValue: newGameInfo.gameInfo.currentPlayer)

        if self.localPlayerInitiatedGamingSession {
            self._localPlayer = Published(wrappedValue: newGameInfo.initiatingPlayer)
            self._otherPlayer = Published(wrappedValue: newGameInfo.otherPlayer)
        } else {
            self._localPlayer = Published(wrappedValue: newGameInfo.otherPlayer!)
            self._otherPlayer = Published(wrappedValue: newGameInfo.initiatingPlayer)
        }
    }

    /// Performs a Game move for the specified Player.
    func takeTurn(pos: Position) async -> Error? {
        
        let result = await GameInfoService.takeTurn(gameId: self.gameId,
                                                    boardPosition: BoardPosition(column: pos.column, row: pos.row),
                                                    localPlayerId: self.localPlayer.playerId, sessionId: self.gamingSessionId)
        
        if let gameInfo = result.gameInfo {
            self.updateGameInfo(turnResult: TurnResponse(newGameState: gameInfo.gameState))
        }
        
        return result.error
    }
    
    /// Performs the inital setup of the Game parameters.
    private func updateGamingSessionInfo(info: GamingSessionCreationResponse, completion: @escaping (() -> Void)) {
        //
        
        DispatchQueue.main.async {
                        
            self._gamingSessionId = Published(wrappedValue: info.sessionId)
            
            self._invitationCode = Published(wrappedValue: info.invitationCode)

            if self.localPlayerInitiatedGamingSession {
                self._localPlayer = Published(wrappedValue: info.initiatingPlayer)
            } else if let otherPlayer = info.otherPlayer {
                self._localPlayer = Published(wrappedValue: otherPlayer)
            }

            // Setup MQTT listener
            self.gameInfoReceiver = GameInfoReceiver(eventPlaneConfig: info.eventPlaneConfig, delegate: self)
            
            Task {
                // Wait for MQTT to be connected. Otherwise, we miss important the onAllPlayersReady event.
                while self.gameInfoReceiver!.isConnecting {
                    // TODO: JD: provide a timeout
                }
                // Also, wait another 1/4 second for the MQTT client to settle
                try await Task.sleep(nanoseconds: 1000 * 1000 * 250)
                completion()
            }
        }
    }

    /// Updates this instance with the values of the passed in TurnResponse.
    private func updateGameInfo(turnResult: TurnResponse) {
        
        DispatchQueue.main.async {
            
            self.currentPlayer = turnResult.currentPlayer
            
            self.gameBoard = turnResult.newGameState.gameBoard
            
            self.gameEnded = turnResult.newGameState.playStatus == .endedInStalemate || turnResult.newGameState.playStatus == .endedInWin
            
            self.hasGameStarted = turnResult.newGameState.playStatus != .notStarted
            
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
extension GameViewModel: GameInfoReceiverDelegate {
        
    func onAllPlayersReady() {

        self._hasGameStarted = Published(wrappedValue: true)
        
        Task {
            let result = await GameInfoService.getSessionCurrentGame(sessionId: self.gamingSessionId)
            if let gameCreationResult = result.gameCreationResult {
                DispatchQueue.main.async {
                    self.setupPlayers(newGameInfo: gameCreationResult)
                    self.updateGameInfo(turnResult: TurnResponse(currentPlayer: gameCreationResult.gameInfo.currentPlayer, newGameState: gameCreationResult.gameInfo.gameState))
                }
            }
        }
    }

    func onGameDeleted() {
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

    func onSessionDeleted() {
        refreshGameInfo()
    }
        
    func onTurnTaken() {
        refreshGameInfo()
    }
}
