//
// TicTacToeAPI.swift
//
// Generated by openapi-generator
// https://openapi-generator.tech
//

import Foundation
#if canImport(AnyCodable)
import AnyCodable
#endif

open class TicTacToeAPI {

    /**
     Creates a new Gaming Session. Returns GamingSessionCreationResult.
     
     - parameter newGamingSessionParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func createGamingSession(newGamingSessionParams: NewGamingSessionParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GamingSessionCreationResponse?, _ error: Error?) -> Void)) -> RequestTask {
        return createGamingSessionWithRequestBuilder(newGamingSessionParams: newGamingSessionParams).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Creates a new Gaming Session. Returns GamingSessionCreationResult.
     - POST /v1/gaming-sessions
     - parameter newGamingSessionParams: (body)  
     - returns: RequestBuilder<GamingSessionCreationResponse> 
     */
    open class func createGamingSessionWithRequestBuilder(newGamingSessionParams: NewGamingSessionParams) -> RequestBuilder<GamingSessionCreationResponse> {
        let localVariablePath = "/v1/gaming-sessions"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: newGamingSessionParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<GamingSessionCreationResponse>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Creates a new Game. Returns Game Creation Result.
     
     - parameter sessionId: (path)  
     - parameter newSinglePlayerGameParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func createSinglePlayerGame(sessionId: String, newSinglePlayerGameParams: NewSinglePlayerGameParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GameCreationResponse?, _ error: Error?) -> Void)) -> RequestTask {
        return createSinglePlayerGameWithRequestBuilder(sessionId: sessionId, newSinglePlayerGameParams: newSinglePlayerGameParams).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Creates a new Game. Returns Game Creation Result.
     - POST /v1/gaming-sessions/{session_id}/games
     - parameter sessionId: (path)  
     - parameter newSinglePlayerGameParams: (body)  
     - returns: RequestBuilder<GameCreationResponse> 
     */
    open class func createSinglePlayerGameWithRequestBuilder(sessionId: String, newSinglePlayerGameParams: NewSinglePlayerGameParams) -> RequestBuilder<GameCreationResponse> {
        var localVariablePath = "/v1/gaming-sessions/{session_id}/games"
        let sessionIdPreEscape = "\(APIHelper.mapValueToPathItem(sessionId))"
        let sessionIdPostEscape = sessionIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{session_id}", with: sessionIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: newSinglePlayerGameParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<GameCreationResponse>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Creates a new Two-Player Game. Returns Game Creation Result.
     
     - parameter sessionId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func createTwoPlayerGame(sessionId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GameCreationResponse?, _ error: Error?) -> Void)) -> RequestTask {
        return createTwoPlayerGameWithRequestBuilder(sessionId: sessionId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Creates a new Two-Player Game. Returns Game Creation Result.
     - POST /v1/gaming-session/{session_id}/two-player-games
     - parameter sessionId: (path)  
     - returns: RequestBuilder<GameCreationResponse> 
     */
    open class func createTwoPlayerGameWithRequestBuilder(sessionId: String) -> RequestBuilder<GameCreationResponse> {
        var localVariablePath = "/v1/gaming-session/{session_id}/two-player-games"
        let sessionIdPreEscape = "\(APIHelper.mapValueToPathItem(sessionId))"
        let sessionIdPostEscape = sessionIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{session_id}", with: sessionIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<GameCreationResponse>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Closes down the specified Game.
     
     - parameter gameId: (path)  
     - parameter endGameParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func endGame(gameId: String, endGameParams: EndGameParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Void?, _ error: Error?) -> Void)) -> RequestTask {
        return endGameWithRequestBuilder(gameId: gameId, endGameParams: endGameParams).execute(apiResponseQueue) { result in
            switch result {
            case .success:
                completion((), nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Closes down the specified Game.
     - DELETE /v1/games/{game_id}
     - parameter gameId: (path)  
     - parameter endGameParams: (body)  
     - returns: RequestBuilder<Void> 
     */
    open class func endGameWithRequestBuilder(gameId: String, endGameParams: EndGameParams) -> RequestBuilder<Void> {
        var localVariablePath = "/v1/games/{game_id}"
        let gameIdPreEscape = "\(APIHelper.mapValueToPathItem(gameId))"
        let gameIdPostEscape = gameIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{game_id}", with: gameIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: endGameParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Void>.Type = OpenAPIClientAPI.requestBuilderFactory.getNonDecodableBuilder()

        return localVariableRequestBuilder.init(method: "DELETE", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Closes down the specified Gaming Session.
     
     - parameter sessionId: (path)  
     - parameter endGamingSessionParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func endGamingSession(sessionId: String, endGamingSessionParams: EndGamingSessionParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Void?, _ error: Error?) -> Void)) -> RequestTask {
        return endGamingSessionWithRequestBuilder(sessionId: sessionId, endGamingSessionParams: endGamingSessionParams).execute(apiResponseQueue) { result in
            switch result {
            case .success:
                completion((), nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Closes down the specified Gaming Session.
     - DELETE /v1/gaming-sessions/{session_id}
     - parameter sessionId: (path)  
     - parameter endGamingSessionParams: (body)  
     - returns: RequestBuilder<Void> 
     */
    open class func endGamingSessionWithRequestBuilder(sessionId: String, endGamingSessionParams: EndGamingSessionParams) -> RequestBuilder<Void> {
        var localVariablePath = "/v1/gaming-sessions/{session_id}"
        let sessionIdPreEscape = "\(APIHelper.mapValueToPathItem(sessionId))"
        let sessionIdPostEscape = sessionIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{session_id}", with: sessionIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: endGamingSessionParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Void>.Type = OpenAPIClientAPI.requestBuilderFactory.getNonDecodableBuilder()

        return localVariableRequestBuilder.init(method: "DELETE", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Retrieves the history of Game States from the initial move (turn) to the latest Game State. This can be used, for instance, to create an animated time-lapse of the Game play.
     
     - parameter gameId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getGameHistory(gameId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: [GameState]?, _ error: Error?) -> Void)) -> RequestTask {
        return getGameHistoryWithRequestBuilder(gameId: gameId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Retrieves the history of Game States from the initial move (turn) to the latest Game State. This can be used, for instance, to create an animated time-lapse of the Game play.
     - GET /v1/games/{game_id}/turns
     - parameter gameId: (path)  
     - returns: RequestBuilder<[GameState]> 
     */
    open class func getGameHistoryWithRequestBuilder(gameId: String) -> RequestBuilder<[GameState]> {
        var localVariablePath = "/v1/games/{game_id}/turns"
        let gameIdPreEscape = "\(APIHelper.mapValueToPathItem(gameId))"
        let gameIdPostEscape = gameIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{game_id}", with: gameIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<[GameState]>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Retrieves the most recent Turn Result for the specified Game.
     
     - parameter gameId: (path) Game ID 
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getLatestGameTurn(gameId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: TurnResponse?, _ error: Error?) -> Void)) -> RequestTask {
        return getLatestGameTurnWithRequestBuilder(gameId: gameId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Retrieves the most recent Turn Result for the specified Game.
     - GET /v1/games/{game_id}/turns/latest
     - parameter gameId: (path) Game ID 
     - returns: RequestBuilder<TurnResponse> 
     */
    open class func getLatestGameTurnWithRequestBuilder(gameId: String) -> RequestBuilder<TurnResponse> {
        var localVariablePath = "/v1/games/{game_id}/turns/latest"
        let gameIdPreEscape = "\(APIHelper.mapValueToPathItem(gameId))"
        let gameIdPostEscape = gameIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{game_id}", with: gameIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<TurnResponse>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Retrieves the Gaming Session's current Game.
     
     - parameter sessionId: (path) Session ID 
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getSessionCurrentGame(sessionId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GameCreationResponse?, _ error: Error?) -> Void)) -> RequestTask {
        return getSessionCurrentGameWithRequestBuilder(sessionId: sessionId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Retrieves the Gaming Session's current Game.
     - GET /v1/gaming-sessions/{session_id}/current-game
     - parameter sessionId: (path) Session ID 
     - returns: RequestBuilder<GameCreationResponse> 
     */
    open class func getSessionCurrentGameWithRequestBuilder(sessionId: String) -> RequestBuilder<GameCreationResponse> {
        var localVariablePath = "/v1/gaming-sessions/{session_id}/current-game"
        let sessionIdPreEscape = "\(APIHelper.mapValueToPathItem(sessionId))"
        let sessionIdPostEscape = sessionIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{session_id}", with: sessionIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<GameCreationResponse>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Adds a Player to the Session's Current Game.
     
     - parameter sessionId: (path)  
     - parameter playerId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func joinCurrentGame(sessionId: String, playerId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Void?, _ error: Error?) -> Void)) -> RequestTask {
        return joinCurrentGameWithRequestBuilder(sessionId: sessionId, playerId: playerId).execute(apiResponseQueue) { result in
            switch result {
            case .success:
                completion((), nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Adds a Player to the Session's Current Game.
     - PUT /v1/gaming-sessions/{session_id}/current_game/players/{player_id}
     - parameter sessionId: (path)  
     - parameter playerId: (path)  
     - returns: RequestBuilder<Void> 
     */
    open class func joinCurrentGameWithRequestBuilder(sessionId: String, playerId: String) -> RequestBuilder<Void> {
        var localVariablePath = "/v1/gaming-sessions/{session_id}/current_game/players/{player_id}"
        let sessionIdPreEscape = "\(APIHelper.mapValueToPathItem(sessionId))"
        let sessionIdPostEscape = sessionIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{session_id}", with: sessionIdPostEscape, options: .literal, range: nil)
        let playerIdPreEscape = "\(APIHelper.mapValueToPathItem(playerId))"
        let playerIdPostEscape = playerIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{player_id}", with: playerIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Void>.Type = OpenAPIClientAPI.requestBuilderFactory.getNonDecodableBuilder()

        return localVariableRequestBuilder.init(method: "PUT", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Adds a Player to the Gaming Session.
     
     - parameter joinSessionParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func joinGamingSession(joinSessionParams: JoinSessionParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GamingSessionCreationResponse?, _ error: Error?) -> Void)) -> RequestTask {
        return joinGamingSessionWithRequestBuilder(joinSessionParams: joinSessionParams).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Adds a Player to the Gaming Session.
     - POST /v1/gaming-sessions/players
     - parameter joinSessionParams: (body)  
     - returns: RequestBuilder<GamingSessionCreationResponse> 
     */
    open class func joinGamingSessionWithRequestBuilder(joinSessionParams: JoinSessionParams) -> RequestBuilder<GamingSessionCreationResponse> {
        let localVariablePath = "/v1/gaming-sessions/players"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: joinSessionParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<GamingSessionCreationResponse>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Called to indicate that a Player is ready to Play. This is required as part of the handshaking during new Game setup.
     
     - parameter sessionId: (path)  
     - parameter playerId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func notePlayerReadiness(sessionId: String, playerId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Void?, _ error: Error?) -> Void)) -> RequestTask {
        return notePlayerReadinessWithRequestBuilder(sessionId: sessionId, playerId: playerId).execute(apiResponseQueue) { result in
            switch result {
            case .success:
                completion((), nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Called to indicate that a Player is ready to Play. This is required as part of the handshaking during new Game setup.
     - PUT /v1/gaming-sessions/{session_id}/players/{player_id}/readiness
     - parameter sessionId: (path)  
     - parameter playerId: (path)  
     - returns: RequestBuilder<Void> 
     */
    open class func notePlayerReadinessWithRequestBuilder(sessionId: String, playerId: String) -> RequestBuilder<Void> {
        var localVariablePath = "/v1/gaming-sessions/{session_id}/players/{player_id}/readiness"
        let sessionIdPreEscape = "\(APIHelper.mapValueToPathItem(sessionId))"
        let sessionIdPostEscape = sessionIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{session_id}", with: sessionIdPostEscape, options: .literal, range: nil)
        let playerIdPreEscape = "\(APIHelper.mapValueToPathItem(playerId))"
        let playerIdPostEscape = playerIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{player_id}", with: playerIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters: [String: Any]? = nil

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            :
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Void>.Type = OpenAPIClientAPI.requestBuilderFactory.getNonDecodableBuilder()

        return localVariableRequestBuilder.init(method: "PUT", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Make a Game move (turn) for the specified Player. Returns the Turn Result.
     
     - parameter gameId: (path)  
     - parameter gameTurnParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func takeTurn(gameId: String, gameTurnParams: GameTurnParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: TurnResponse?, _ error: Error?) -> Void)) -> RequestTask {
        return takeTurnWithRequestBuilder(gameId: gameId, gameTurnParams: gameTurnParams).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Make a Game move (turn) for the specified Player. Returns the Turn Result.
     - POST /v1/games/{game_id}/turns
     - parameter gameId: (path)  
     - parameter gameTurnParams: (body)  
     - returns: RequestBuilder<TurnResponse> 
     */
    open class func takeTurnWithRequestBuilder(gameId: String, gameTurnParams: GameTurnParams) -> RequestBuilder<TurnResponse> {
        var localVariablePath = "/v1/games/{game_id}/turns"
        let gameIdPreEscape = "\(APIHelper.mapValueToPathItem(gameId))"
        let gameIdPostEscape = gameIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{game_id}", with: gameIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: gameTurnParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<TurnResponse>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }
}
