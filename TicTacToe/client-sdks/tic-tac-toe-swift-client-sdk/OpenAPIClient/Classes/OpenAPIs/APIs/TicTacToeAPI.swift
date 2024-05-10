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
     * Defines and implements the public Gaming contract for this service.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Joel@RustMadeEasy.com
     
     - parameter addPlayerParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func addPlayer(addPlayerParams: AddPlayerParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GameInfo?, _ error: Error?) -> Void)) -> RequestTask {
        return addPlayerWithRequestBuilder(addPlayerParams: addPlayerParams).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     * Defines and implements the public Gaming contract for this service.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Joel@RustMadeEasy.com
     - POST /v1/games/players
     - * Defines and implements the public Gaming contract for this service.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Joel@RustMadeEasy.com Adds a Player to the Game. Returns the Game Info.
     - parameter addPlayerParams: (body)  
     - returns: RequestBuilder<GameInfo> 
     */
    open class func addPlayerWithRequestBuilder(addPlayerParams: AddPlayerParams) -> RequestBuilder<GameInfo> {
        let localVariablePath = "/v1/games/players"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: addPlayerParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<GameInfo>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Creates a new Game. Returns the Game Info.
     
     - parameter newGameParams: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func createGame(newGameParams: NewGameParams, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GameInfo?, _ error: Error?) -> Void)) -> RequestTask {
        return createGameWithRequestBuilder(newGameParams: newGameParams).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Creates a new Game. Returns the Game Info.
     - POST /v1/games
     - Creates a new Game. Returns the Game Info.
     - parameter newGameParams: (body)  
     - returns: RequestBuilder<GameInfo> 
     */
    open class func createGameWithRequestBuilder(newGameParams: NewGameParams) -> RequestBuilder<GameInfo> {
        let localVariablePath = "/v1/games"
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: newGameParams)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<GameInfo>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Closes down the specified Game.
     
     - parameter gameId: (path)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func endGame(gameId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Void?, _ error: Error?) -> Void)) -> RequestTask {
        return endGameWithRequestBuilder(gameId: gameId).execute(apiResponseQueue) { result in
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
     - Closes down the specified Game.
     - parameter gameId: (path)  
     - returns: RequestBuilder<Void> 
     */
    open class func endGameWithRequestBuilder(gameId: String) -> RequestBuilder<Void> {
        var localVariablePath = "/v1/games/{game_id}"
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

        let localVariableRequestBuilder: RequestBuilder<Void>.Type = OpenAPIClientAPI.requestBuilderFactory.getNonDecodableBuilder()

        return localVariableRequestBuilder.init(method: "DELETE", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Retrieves the history of the Game States from the initial creation to the current
     
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
     Retrieves the history of the Game States from the initial creation to the current
     - GET /v1/games/{game_id}/turns
     - Retrieves the history of the Game States from the initial creation to the current Game State. This can be used, for instance, for the client to provide an animation that shows a time-lapse of the game play.
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
     Retrieves the specified Game info.
     
     - parameter gameId: (path) Game ID 
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func getGameInfo(gameId: String, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: GameInfo?, _ error: Error?) -> Void)) -> RequestTask {
        return getGameInfoWithRequestBuilder(gameId: gameId).execute(apiResponseQueue) { result in
            switch result {
            case let .success(response):
                completion(response.body, nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Retrieves the specified Game info.
     - GET /v1/games/{game_id}
     - Retrieves the specified Game info.
     - parameter gameId: (path) Game ID 
     - returns: RequestBuilder<GameInfo> 
     */
    open class func getGameInfoWithRequestBuilder(gameId: String) -> RequestBuilder<GameInfo> {
        var localVariablePath = "/v1/games/{game_id}"
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

        let localVariableRequestBuilder: RequestBuilder<GameInfo>.Type = OpenAPIClientAPI.requestBuilderFactory.getBuilder()

        return localVariableRequestBuilder.init(method: "GET", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }

    /**
     Make a game move for the specified Player.
     
     - parameter gameId: (path)  
     - parameter gameTurnInfo: (body)  
     - parameter apiResponseQueue: The queue on which api response is dispatched.
     - parameter completion: completion handler to receive the data and the error objects
     */
    @discardableResult
    open class func takeTurn(gameId: String, gameTurnInfo: GameTurnInfo, apiResponseQueue: DispatchQueue = OpenAPIClientAPI.apiResponseQueue, completion: @escaping ((_ data: Void?, _ error: Error?) -> Void)) -> RequestTask {
        return takeTurnWithRequestBuilder(gameId: gameId, gameTurnInfo: gameTurnInfo).execute(apiResponseQueue) { result in
            switch result {
            case .success:
                completion((), nil)
            case let .failure(error):
                completion(nil, error)
            }
        }
    }

    /**
     Make a game move for the specified Player.
     - POST /v1/games/{game_id}/turns
     - Make a game move for the specified Player.
     - parameter gameId: (path)  
     - parameter gameTurnInfo: (body)  
     - returns: RequestBuilder<Void> 
     */
    open class func takeTurnWithRequestBuilder(gameId: String, gameTurnInfo: GameTurnInfo) -> RequestBuilder<Void> {
        var localVariablePath = "/v1/games/{game_id}/turns"
        let gameIdPreEscape = "\(APIHelper.mapValueToPathItem(gameId))"
        let gameIdPostEscape = gameIdPreEscape.addingPercentEncoding(withAllowedCharacters: .urlPathAllowed) ?? ""
        localVariablePath = localVariablePath.replacingOccurrences(of: "{game_id}", with: gameIdPostEscape, options: .literal, range: nil)
        let localVariableURLString = OpenAPIClientAPI.basePath + localVariablePath
        let localVariableParameters = JSONEncodingHelper.encodingParameters(forEncodableObject: gameTurnInfo)

        let localVariableUrlComponents = URLComponents(string: localVariableURLString)

        let localVariableNillableHeaders: [String: Any?] = [
            "Content-Type": "application/json",
        ]

        let localVariableHeaderParameters = APIHelper.rejectNilHeaders(localVariableNillableHeaders)

        let localVariableRequestBuilder: RequestBuilder<Void>.Type = OpenAPIClientAPI.requestBuilderFactory.getNonDecodableBuilder()

        return localVariableRequestBuilder.init(method: "POST", URLString: (localVariableUrlComponents?.string ?? localVariableURLString), parameters: localVariableParameters, headers: localVariableHeaderParameters, requiresAuthentication: false)
    }
}
