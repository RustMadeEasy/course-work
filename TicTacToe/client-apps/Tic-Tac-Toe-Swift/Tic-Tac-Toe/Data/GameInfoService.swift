//
//  GameInfoService.swift
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com
//

import Foundation
import OpenAPIClient

/// The data set returned by the GameInfoService methods.
struct GameInfoServiceResult {
    var error: Error? = nil
    var gameInfo: GameInfoResponse? = nil
    var gameCreationResult: GameCreationResponse? = nil
    var gamingSessionCreationResult: GamingSessionCreationResponse? = nil
    var turnResult: TurnResponse? = nil
}

/// Enumerates the errors potentially returned by the GameInfoService methods.
enum GameInfoManagerError: Error {
    case emptyData
}

/// Performs all access to our Tic-Tac-Toe API.
class GameInfoService {

    /// Creates a new Gaming Session. Returns the Session Creation Result.
    static func createGamingSession(sessionOwnerDisplayName: String) async -> GameInfoServiceResult {

        let params = NewGamingSessionParams(sessionOwnerDisplayName: sessionOwnerDisplayName)
        
        do {

            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.createGamingSession(newGamingSessionParams: params, completion: { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(gamingSessionCreationResult: data) )
                            }
                        } else {
                            let error = GameInfoManagerError.emptyData;
                            print("createGamingSession() error: \(String(describing: error))")
                            continuation.resume(returning: GameInfoServiceResult(error: error))
                        }
                    } else {
                        print("createGamingSession() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                })
            }
            return result
            
        } catch {
            print("createGamingSession() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }
    
    /// Creates a new Single-Player Game. Returns GameCreationResult.
    static func createSinglePlayerGame(computerSkillLevel: AutomaticPlayerSkillLevel, sessionId: String, localPlayerName: String) async -> GameInfoServiceResult {

        do {

            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                let params = NewSinglePlayerGameParams(computerSkillLevel: computerSkillLevel)
                TicTacToeAPI.createSinglePlayerGame(sessionId: sessionId, newSinglePlayerGameParams: params) { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(gameCreationResult: data) )
                            }
                        } else {
                            let error = GameInfoManagerError.emptyData;
                            print("createSinglePlayerGame() error: \(String(describing: error))")
                            continuation.resume(returning: GameInfoServiceResult(error: error))
                        }
                    } else {
                        print("createSinglePlayerGame() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("createSinglePlayerGame() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }

    /// Creates a new Single-Player Game. Returns GameCreationResult.
    static func createTwoPlayerGame(sessionId: String, localPlayerName: String) async -> GameInfoServiceResult {

        do {

            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.createTwoPlayerGame(sessionId: sessionId) { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(gameCreationResult: data) )
                            }
                        } else {
                            let error = GameInfoManagerError.emptyData;
                            print("createSinglePlayerGame() error: \(String(describing: error))")
                            continuation.resume(returning: GameInfoServiceResult(error: error))
                        }
                    } else {
                        print("createSinglePlayerGame() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("createSinglePlayerGame() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }

    /// Closes down the specified Game.
    static func endGame(gameId: String, playerId: String, sessionId: String) async -> GameInfoServiceResult {
        
        do {
            
            let error: Error? = try await withCheckedThrowingContinuation { continuation in
                let params = EndGameParams.init(playerId: playerId, sessionId: sessionId)
                TicTacToeAPI.endGame(gameId: gameId, endGameParams: params) { data, error in
                    if error == nil {
                        DispatchQueue.main.async {
                            continuation.resume(returning: nil )
                        }
                    } else {
                        print("endGame() error: \(String(describing: error))")
                        continuation.resume(returning: error)
                    }
                }
            }
            return GameInfoServiceResult(error: error)
            
        } catch {
            print("endGame() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }
    
    /// Closes down the specified Gaming Session.
    static func endGamingSession(playerId: String, sessionId: String) async -> GameInfoServiceResult {
        
        do {
            
            let error: Error? = try await withCheckedThrowingContinuation { continuation in
                let params = EndGamingSessionParams.init(playerId: playerId)
                TicTacToeAPI.endGamingSession(sessionId: sessionId, endGamingSessionParams: params) { data, error in
                    if error == nil {
                        DispatchQueue.main.async {
                            continuation.resume(returning: nil )
                        }
                    } else {
                        print("endGamingSession() error: \(String(describing: error))")
                        continuation.resume(returning: error)
                    }
                }
            }
            return GameInfoServiceResult(error: error)
            
        } catch {
            print("endGamingSession() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }
    
    /// Retrieves the latest turn info from the specified Game.
    static func getLatestTurn(gameId: String) async -> GameInfoServiceResult {
                
        do {
            
            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.getLatestGameTurn(gameId: gameId) { turnResult, error in
                    if error == nil {
                        if turnResult != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(turnResult: turnResult) )
                            }
                        } else {
                            let error = GameInfoManagerError.emptyData;
                            print("getLatestTurn() error: \(String(describing: error))")
                            continuation.resume(returning: GameInfoServiceResult(error: error))
                        }
                    } else {
                        print("getLatestTurn() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("getLatestTurn() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }

    /// Creates a new Single-Player Game. Returns GameCreationResult.
    static func getSessionCurrentGame(sessionId: String) async -> GameInfoServiceResult {

        do {

            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.getSessionCurrentGame(sessionId: sessionId) { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(gameCreationResult: data) )
                            }
                        } else {
                            let error = GameInfoManagerError.emptyData;
                            print("getSessionCurrentGame() error: \(String(describing: error))")
                            continuation.resume(returning: GameInfoServiceResult(error: error))
                        }
                    } else {
                        print("getSessionCurrentGame() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("getSessionCurrentGame() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }

    /// Joins the Gaming Session's current Game.
    static func joinCurrentGame(sessionId: String, localPlayerId: String) async -> GameInfoServiceResult {
        
        do {
            
            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.joinCurrentGame(sessionId: sessionId, playerId: localPlayerId) { data, error in
                    if error == nil {
                        continuation.resume(returning: GameInfoServiceResult(gameCreationResult: data))
                    } else {
                        print("joinCurrentGame() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("joinCurrentGame() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }

    /// Joins a Gaming Session.
    static func joinGamingSession(invitationCode: String, playerName: String) async -> GameInfoServiceResult {
        
        do {
            
            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                let params = JoinSessionParams.init(gameInvitationCode: invitationCode, playerDisplayName: playerName)
                TicTacToeAPI.joinGamingSession(joinSessionParams: params) { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(gamingSessionCreationResult: data) )
                            }
                        } else {
                            let error = GameInfoManagerError.emptyData;
                            print("joinGamingSession() error: \(String(describing: error))")
                            continuation.resume(returning: GameInfoServiceResult(error: error))
                        }
                    } else {
                        print("joinGamingSession() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("joinGamingSession() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }

    /// Performs a Game move for the specified Player.
    static func takeTurn(gameId: String, boardPosition: BoardPosition, localPlayerId: String, sessionId: String) async -> GameInfoServiceResult {
        
        do {
            
            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                let turnInfo = GameTurnParams(destination: boardPosition, playerId: localPlayerId, sessionId: sessionId)
                TicTacToeAPI.takeTurn(gameId: gameId, gameTurnParams: turnInfo) { data, error in
                    if let data = data {
                        DispatchQueue.main.async {
                            continuation.resume(returning: GameInfoServiceResult(turnResult: data))
                        }
                    } else {
                        print("takeTurn() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("takeTurn() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }
}
