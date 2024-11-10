# TicTacToeAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createGamingSession**](TicTacToeAPI.md#creategamingsession) | **POST** /v1/gaming-sessions | Creates a new Gaming Session. Returns Gaming Session Creation Response.
[**createSinglePlayerGame**](TicTacToeAPI.md#createsingleplayergame) | **POST** /v1/gaming-sessions/{session_id}/games | Creates a new Single-Player Game. Returns Game Creation Response.
[**createTwoPlayerGame**](TicTacToeAPI.md#createtwoplayergame) | **POST** /v1/gaming-session/{session_id}/two-player-games | Creates a new Two-Player Game. Returns Game Creation Response.
[**endGame**](TicTacToeAPI.md#endgame) | **DELETE** /v1/games/{game_id} | Closes down the specified Game.
[**endGamingSession**](TicTacToeAPI.md#endgamingsession) | **DELETE** /v1/gaming-sessions/{session_id} | Closes down the specified Gaming Session.
[**getGameHistory**](TicTacToeAPI.md#getgamehistory) | **GET** /v1/games/{game_id}/turns | Retrieves the history of Game States from the initial move (turn) to the current Game State. This can be used, for instance, to create an animated time-lapse of the Game play.
[**getLatestGameTurn**](TicTacToeAPI.md#getlatestgameturn) | **GET** /v1/games/{game_id}/turns/latest | Retrieves the most recent Turn for the specified Game.
[**getPlayersReadiness**](TicTacToeAPI.md#getplayersreadiness) | **GET** /v1/games/{game_id}/players/readiness | Retrieves the readiness of the Game&#39;s Players, answering the questions: Have all Players been added to the Game and setup?
[**getSessionCurrentGame**](TicTacToeAPI.md#getsessioncurrentgame) | **GET** /v1/gaming-sessions/{session_id}/current-game | Retrieves the Gaming Session&#39;s current Game.
[**joinCurrentGame**](TicTacToeAPI.md#joincurrentgame) | **PUT** /v1/gaming-sessions/{session_id}/current_game/players/{player_id} | Adds a Player to the Session&#39;s Current Game.
[**joinGamingSession**](TicTacToeAPI.md#joingamingsession) | **POST** /v1/gaming-sessions/players | Adds a Player to the Gaming Session.
[**takeTurn**](TicTacToeAPI.md#taketurn) | **POST** /v1/games/{game_id}/turns | Make a Game move (turn) for the specified Player. Returns the Turn Response.


# **createGamingSession**
```swift
    open class func createGamingSession(newGamingSessionParams: NewGamingSessionParams, completion: @escaping (_ data: GamingSessionCreationResponse?, _ error: Error?) -> Void)
```

Creates a new Gaming Session. Returns Gaming Session Creation Response.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let newGamingSessionParams = NewGamingSessionParams(sessionOwnerDisplayName: "sessionOwnerDisplayName_example") // NewGamingSessionParams | 

// Creates a new Gaming Session. Returns Gaming Session Creation Response.
TicTacToeAPI.createGamingSession(newGamingSessionParams: newGamingSessionParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **newGamingSessionParams** | [**NewGamingSessionParams**](NewGamingSessionParams.md) |  | 

### Return type

[**GamingSessionCreationResponse**](GamingSessionCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createSinglePlayerGame**
```swift
    open class func createSinglePlayerGame(sessionId: String, newSinglePlayerGameParams: NewSinglePlayerGameParams, completion: @escaping (_ data: GameCreationResponse?, _ error: Error?) -> Void)
```

Creates a new Single-Player Game. Returns Game Creation Response.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | 
let newSinglePlayerGameParams = NewSinglePlayerGameParams(computerSkillLevel: AutomaticPlayerSkillLevel()) // NewSinglePlayerGameParams | 

// Creates a new Single-Player Game. Returns Game Creation Response.
TicTacToeAPI.createSinglePlayerGame(sessionId: sessionId, newSinglePlayerGameParams: newSinglePlayerGameParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String** |  | 
 **newSinglePlayerGameParams** | [**NewSinglePlayerGameParams**](NewSinglePlayerGameParams.md) |  | 

### Return type

[**GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTwoPlayerGame**
```swift
    open class func createTwoPlayerGame(sessionId: String, completion: @escaping (_ data: GameCreationResponse?, _ error: Error?) -> Void)
```

Creates a new Two-Player Game. Returns Game Creation Response.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | 

// Creates a new Two-Player Game. Returns Game Creation Response.
TicTacToeAPI.createTwoPlayerGame(sessionId: sessionId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String** |  | 

### Return type

[**GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **endGame**
```swift
    open class func endGame(gameId: String, endGameParams: EndGameParams, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```

Closes down the specified Game.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | 
let endGameParams = EndGameParams(playerId: "playerId_example", sessionId: "sessionId_example") // EndGameParams | 

// Closes down the specified Game.
TicTacToeAPI.endGame(gameId: gameId, endGameParams: endGameParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **gameId** | **String** |  | 
 **endGameParams** | [**EndGameParams**](EndGameParams.md) |  | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **endGamingSession**
```swift
    open class func endGamingSession(sessionId: String, endGamingSessionParams: EndGamingSessionParams, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```

Closes down the specified Gaming Session.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | 
let endGamingSessionParams = EndGamingSessionParams(playerId: "playerId_example") // EndGamingSessionParams | 

// Closes down the specified Gaming Session.
TicTacToeAPI.endGamingSession(sessionId: sessionId, endGamingSessionParams: endGamingSessionParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String** |  | 
 **endGamingSessionParams** | [**EndGamingSessionParams**](EndGamingSessionParams.md) |  | 

### Return type

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getGameHistory**
```swift
    open class func getGameHistory(gameId: String, completion: @escaping (_ data: [GameState]?, _ error: Error?) -> Void)
```

Retrieves the history of Game States from the initial move (turn) to the current Game State. This can be used, for instance, to create an animated time-lapse of the Game play.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | 

// Retrieves the history of Game States from the initial move (turn) to the current Game State. This can be used, for instance, to create an animated time-lapse of the Game play.
TicTacToeAPI.getGameHistory(gameId: gameId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **gameId** | **String** |  | 

### Return type

[**[GameState]**](GameState.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getLatestGameTurn**
```swift
    open class func getLatestGameTurn(gameId: String, completion: @escaping (_ data: TurnResponse?, _ error: Error?) -> Void)
```

Retrieves the most recent Turn for the specified Game.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | Game ID

// Retrieves the most recent Turn for the specified Game.
TicTacToeAPI.getLatestGameTurn(gameId: gameId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **gameId** | **String** | Game ID | 

### Return type

[**TurnResponse**](TurnResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getPlayersReadiness**
```swift
    open class func getPlayersReadiness(gameId: String, completion: @escaping (_ data: PlayersReadinessResponse?, _ error: Error?) -> Void)
```

Retrieves the readiness of the Game's Players, answering the questions: Have all Players been added to the Game and setup?

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | Game ID

// Retrieves the readiness of the Game's Players, answering the questions: Have all Players been added to the Game and setup?
TicTacToeAPI.getPlayersReadiness(gameId: gameId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **gameId** | **String** | Game ID | 

### Return type

[**PlayersReadinessResponse**](PlayersReadinessResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getSessionCurrentGame**
```swift
    open class func getSessionCurrentGame(sessionId: String, completion: @escaping (_ data: GameCreationResponse?, _ error: Error?) -> Void)
```

Retrieves the Gaming Session's current Game.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | Session ID

// Retrieves the Gaming Session's current Game.
TicTacToeAPI.getSessionCurrentGame(sessionId: sessionId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String** | Session ID | 

### Return type

[**GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **joinCurrentGame**
```swift
    open class func joinCurrentGame(sessionId: String, playerId: String, completion: @escaping (_ data: GameCreationResponse?, _ error: Error?) -> Void)
```

Adds a Player to the Session's Current Game.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | 
let playerId = "playerId_example" // String | 

// Adds a Player to the Session's Current Game.
TicTacToeAPI.joinCurrentGame(sessionId: sessionId, playerId: playerId) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sessionId** | **String** |  | 
 **playerId** | **String** |  | 

### Return type

[**GameCreationResponse**](GameCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **joinGamingSession**
```swift
    open class func joinGamingSession(joinSessionParams: JoinSessionParams, completion: @escaping (_ data: GamingSessionCreationResponse?, _ error: Error?) -> Void)
```

Adds a Player to the Gaming Session.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let joinSessionParams = JoinSessionParams(gameInvitationCode: "gameInvitationCode_example", playerDisplayName: "playerDisplayName_example") // JoinSessionParams | 

// Adds a Player to the Gaming Session.
TicTacToeAPI.joinGamingSession(joinSessionParams: joinSessionParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **joinSessionParams** | [**JoinSessionParams**](JoinSessionParams.md) |  | 

### Return type

[**GamingSessionCreationResponse**](GamingSessionCreationResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **takeTurn**
```swift
    open class func takeTurn(gameId: String, gameTurnParams: GameTurnParams, completion: @escaping (_ data: TurnResponse?, _ error: Error?) -> Void)
```

Make a Game move (turn) for the specified Player. Returns the Turn Response.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | 
let gameTurnParams = GameTurnParams(destination: BoardPosition(column: 123, row: 123), playerId: "playerId_example", sessionId: "sessionId_example") // GameTurnParams | 

// Make a Game move (turn) for the specified Player. Returns the Turn Response.
TicTacToeAPI.takeTurn(gameId: gameId, gameTurnParams: gameTurnParams) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **gameId** | **String** |  | 
 **gameTurnParams** | [**GameTurnParams**](GameTurnParams.md) |  | 

### Return type

[**TurnResponse**](TurnResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

