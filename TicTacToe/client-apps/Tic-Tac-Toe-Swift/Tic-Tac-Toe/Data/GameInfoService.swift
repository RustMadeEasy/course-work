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
    var newGameInfo: GameCreationResult?
    var newGamingSessionInfo: GamingSessionCreationResult?
    var gameInfo: GameInfo?
    var error: Error?
}

/// Enumerates the errors potentially returned by the GameInfoService methods.
enum GameInfoManagerError: Error {
    case emptyData
}

/// Performs all access to our Tic-Tac-Toe API.
class GameInfoService {

    /// Creates a new Gaming Session. Returns the Game Info.
    static func createGamingSession(sessionId: String, sessionOwnerDisplayName: String) async -> GameInfoServiceResult {

        let params = NewGamingSessionParams(sessionId: sessionId, sessionOwnerDisplayName: sessionOwnerDisplayName)
        
        do {

            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.createGamingSession(newGamingSessionParams: params, completion: { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(newGamingSessionInfo: data) )
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
            return GameInfoServiceResult(gameInfo: nil, error: error)
        }
    }
    
    /// Creates a new Single-Player Game. Returns GameCreationResult.
    static func createSinglePlayerGame(computerSkillLevel: AutomaticPlayerSkillLevel, sessionId: String) async -> GameInfoServiceResult {

        do {

            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                let params = NewSinglePlayerGameParams(computerSkillLevel: computerSkillLevel, sessionId: sessionId)
                TicTacToeAPI.createSinglePlayerGame(newSinglePlayerGameParams: params) { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(newGameInfo: data) )
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
            return GameInfoServiceResult(gameInfo: nil, error: error)
        }
    }

    /// Creates a new Single-Player Game. Returns GameCreationResult.
    static func createTwoPlayerGame(sessionId: String) async -> GameInfoServiceResult {

        do {

            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                let params = NewTwoPlayerGameParams(sessionId: sessionId)
                TicTacToeAPI.createTwoPlayerGame(newTwoPlayerGameParams: params) { data, error in
                    if error == nil {
                        if data != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(newGameInfo: data) )
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
            return GameInfoServiceResult(gameInfo: nil, error: error)
        }
    }

    /// Closes down the specified Game.
    static func endGame(gameId: String) async -> GameInfoServiceResult {
        
        do {
            
            let error: Error? = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.endGame(gameId: gameId) { data, error in
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
    
//    /// Joins an existing Game.
//    static func joinGame(invitationCode: String, playerName: String) async -> GameInfoServiceResult {
//        
//        do {
//            
//            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
//                let params = AddPlayerParams(gameInvitationCode: invitationCode, playerDisplayName: playerName)
//                TicTacToeAPI.addPlayer(addPlayerParams: params) { data, error in
//                    if error == nil {
//                        if data != nil {
//                            DispatchQueue.main.async {
//                                continuation.resume(returning: GameInfoServiceResult(newGameInfo: data) )
//                            }
//                        } else {
//                            let error = GameInfoManagerError.emptyData;
//                            print("joinGame() error: \(String(describing: error))")
//                            continuation.resume(returning: GameInfoServiceResult(error: error))
//                        }
//                    } else {
//                        print("joinGame() error: \(String(describing: error))")
//                        continuation.resume(returning: GameInfoServiceResult(error: error))
//                    }
//                }
//            }
//            return result
//            
//        } catch {
//            print("joinGame() error: \(String(describing: error))")
//            return GameInfoServiceResult(gameInfo: nil, error: error)
//        }
//    }

    /// Retrieves the specified Game info.
    static func retrieveGameInfo(gameId: String) async -> GameInfoServiceResult {
                
        do {
            
            let result: GameInfoServiceResult = try await withCheckedThrowingContinuation { continuation in
                TicTacToeAPI.getGameInfo(gameId: gameId) { gameInfo, error in
                    if error == nil {
                        if gameInfo != nil {
                            DispatchQueue.main.async {
                                continuation.resume(returning: GameInfoServiceResult(gameInfo: gameInfo!) )
                            }
                        } else {
                            let error = GameInfoManagerError.emptyData;
                            print("retrieveGameInfo() error: \(String(describing: error))")
                            continuation.resume(returning: GameInfoServiceResult(error: error))
                        }
                    } else {
                        print("retrieveGameInfo() error: \(String(describing: error))")
                        continuation.resume(returning: GameInfoServiceResult(error: error))
                    }
                }
            }
            return result
            
        } catch {
            print("retrieveGameInfo() error: \(String(describing: error))")
            return GameInfoServiceResult(gameInfo: nil, error: error)
        }
    }

    /// Performs a Game move for the specified Player.
    static func takeTurn(gameId: String, boardPosition: BoardPosition, localPlayerId: String) async -> GameInfoServiceResult {
        
        do {
            
            let error: Error? = try await withCheckedThrowingContinuation { continuation in
                let turnInfo = GameTurnInfo(destination: boardPosition, playerId: localPlayerId)
                TicTacToeAPI.takeTurn(gameId: gameId, gameTurnInfo: turnInfo) { data, error in
                    if error == nil {
                        DispatchQueue.main.async {
                            continuation.resume(returning: nil)
                        }
                    } else {
                        print("takeTurn() error: \(String(describing: error))")
                        continuation.resume(returning: error)
                    }
                }
            }
            return GameInfoServiceResult(error: error)
            
        } catch {
            print("takeTurn() error: \(String(describing: error))")
            return GameInfoServiceResult(error: error)
        }
    }
}
