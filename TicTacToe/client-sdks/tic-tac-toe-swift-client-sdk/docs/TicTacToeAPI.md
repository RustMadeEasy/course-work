# TicTacToeAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createGamingSession**](TicTacToeAPI.md#creategamingsession) | **POST** /v1/gaming-sessions | Creates a new Gaming Session. Returns GamingSessionCreationResult.
[**createSinglePlayerGame**](TicTacToeAPI.md#createsingleplayergame) | **POST** /v1/gaming-sessions/{session_id}/games | Creates a new Game. Returns Game Creation Result.
[**createTwoPlayerGame**](TicTacToeAPI.md#createtwoplayergame) | **POST** /v1/gaming-session/{session_id}/two-player-games | Creates a new Two-Player Game. Returns Game Creation Result.
[**endGame**](TicTacToeAPI.md#endgame) | **DELETE** /v1/games/{game_id} | Closes down the specified Game.
[**endGamingSession**](TicTacToeAPI.md#endgamingsession) | **DELETE** /v1/gaming-sessions/{session_id} | Closes down the specified Gaming Session.
[**getGameHistory**](TicTacToeAPI.md#getgamehistory) | **GET** /v1/games/{game_id}/turns | Retrieves the history of the Game States from the initial move (turn) to the latest
[**getLatestGameTurn**](TicTacToeAPI.md#getlatestgameturn) | **GET** /v1/games/{game_id}/turns/latest | Retrieves details of the specified Game.
[**getSessionCurrentGames**](TicTacToeAPI.md#getsessioncurrentgames) | **GET** /v1/gaming-sessions/{session_id}/current-games | Retrieves the Games in a Gaming Session.
[**joinGamingSession**](TicTacToeAPI.md#joingamingsession) | **POST** /v1/gaming-sessions/players | Adds a Player to the Gaming Session.
[**notePlayerReadiness**](TicTacToeAPI.md#noteplayerreadiness) | **PUT** /v1/gaming-sessions/{session_id}/players/{player_id}/readiness | Sets a Player as ready to Play.
[**takeTurn**](TicTacToeAPI.md#taketurn) | **POST** /v1/games/{game_id}/turns | Make a Game move (turn) for the specified Player.


# **createGamingSession**
```swift
    open class func createGamingSession(newGamingSessionParams: NewGamingSessionParams, completion: @escaping (_ data: GamingSessionCreationResult?, _ error: Error?) -> Void)
```

Creates a new Gaming Session. Returns GamingSessionCreationResult.

Creates a new Gaming Session. Returns GamingSessionCreationResult.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let newGamingSessionParams = NewGamingSessionParams(sessionOwnerDisplayName: "sessionOwnerDisplayName_example") // NewGamingSessionParams | 

// Creates a new Gaming Session. Returns GamingSessionCreationResult.
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

[**GamingSessionCreationResult**](GamingSessionCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createSinglePlayerGame**
```swift
    open class func createSinglePlayerGame(sessionId: String, newSinglePlayerGameParams: NewSinglePlayerGameParams, completion: @escaping (_ data: GameCreationResult?, _ error: Error?) -> Void)
```

Creates a new Game. Returns Game Creation Result.

Creates a new Game. Returns Game Creation Result.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | 
let newSinglePlayerGameParams = NewSinglePlayerGameParams(computerSkillLevel: AutomaticPlayerSkillLevel()) // NewSinglePlayerGameParams | 

// Creates a new Game. Returns Game Creation Result.
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

[**GameCreationResult**](GameCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createTwoPlayerGame**
```swift
    open class func createTwoPlayerGame(sessionId: String, completion: @escaping (_ data: GameCreationResult?, _ error: Error?) -> Void)
```

Creates a new Two-Player Game. Returns Game Creation Result.

Creates a new Two-Player Game. Returns Game Creation Result.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | 

// Creates a new Two-Player Game. Returns Game Creation Result.
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

[**GameCreationResult**](GameCreationResult.md)

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

Retrieves the history of the Game States from the initial move (turn) to the latest

Retrieves the history of the Game States from the initial move (turn) to the latest Game State. This can be used, for instance, to create an animated time-lapse of the Game play.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | 

// Retrieves the history of the Game States from the initial move (turn) to the latest
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
    open class func getLatestGameTurn(gameId: String, completion: @escaping (_ data: TurnResult?, _ error: Error?) -> Void)
```

Retrieves details of the specified Game.

Retrieves details of the specified Game.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | Game ID

// Retrieves details of the specified Game.
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

[**TurnResult**](TurnResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getSessionCurrentGames**
```swift
    open class func getSessionCurrentGames(sessionId: String, completion: @escaping (_ data: [GameInfo]?, _ error: Error?) -> Void)
```

Retrieves the Games in a Gaming Session.

Retrieves the Games in a Gaming Session.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | Session ID

// Retrieves the Games in a Gaming Session.
TicTacToeAPI.getSessionCurrentGames(sessionId: sessionId) { (response, error) in
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

[**[GameInfo]**](GameInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **joinGamingSession**
```swift
    open class func joinGamingSession(joinSessionParams: JoinSessionParams, completion: @escaping (_ data: GamingSessionCreationResult?, _ error: Error?) -> Void)
```

Adds a Player to the Gaming Session.

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

[**GamingSessionCreationResult**](GamingSessionCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **notePlayerReadiness**
```swift
    open class func notePlayerReadiness(sessionId: String, playerId: String, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```

Sets a Player as ready to Play.

Sets a Player as ready to Play.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let sessionId = "sessionId_example" // String | 
let playerId = "playerId_example" // String | 

// Sets a Player as ready to Play.
TicTacToeAPI.notePlayerReadiness(sessionId: sessionId, playerId: playerId) { (response, error) in
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

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **takeTurn**
```swift
    open class func takeTurn(gameId: String, gameTurnInfo: GameTurnInfo, completion: @escaping (_ data: TurnResult?, _ error: Error?) -> Void)
```

Make a Game move (turn) for the specified Player.

Make a Game move (turn) for the specified Player.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | 
let gameTurnInfo = GameTurnInfo(destination: BoardPosition(column: 123, row: 123), playerId: "playerId_example", sessionId: "sessionId_example") // GameTurnInfo | 

// Make a Game move (turn) for the specified Player.
TicTacToeAPI.takeTurn(gameId: gameId, gameTurnInfo: gameTurnInfo) { (response, error) in
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
 **gameTurnInfo** | [**GameTurnInfo**](GameTurnInfo.md) |  | 

### Return type

[**TurnResult**](TurnResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

