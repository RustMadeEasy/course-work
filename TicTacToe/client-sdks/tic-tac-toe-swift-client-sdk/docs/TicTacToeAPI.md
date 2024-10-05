# TicTacToeAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**addPlayer**](TicTacToeAPI.md#addplayer) | **POST** /v1/games/players | Adds a Player to the Game. Returns the result of the initial Game Creation.
[**createGame**](TicTacToeAPI.md#creategame) | **POST** /v1/games | Creates a new Game. Returns Game Creation Result.
[**endGame**](TicTacToeAPI.md#endgame) | **DELETE** /v1/games/{game_id} | Closes down the specified Game.
[**getGameHistory**](TicTacToeAPI.md#getgamehistory) | **GET** /v1/games/{game_id}/turns | Retrieves the history of the Game States from the initial move (turn) to the latest
[**getGameInfo**](TicTacToeAPI.md#getgameinfo) | **GET** /v1/games/{game_id} | Retrieves details of the specified Game.
[**takeTurn**](TicTacToeAPI.md#taketurn) | **POST** /v1/games/{game_id}/turns | Make a Game move (turn) for the specified Player.


# **addPlayer**
```swift
    open class func addPlayer(addPlayerParams: AddPlayerParams, completion: @escaping (_ data: GameCreationResult?, _ error: Error?) -> Void)
```

Adds a Player to the Game. Returns the result of the initial Game Creation.

Adds a Player to the Game. Returns the result of the initial Game Creation.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let addPlayerParams = AddPlayerParams(gameInvitationCode: "gameInvitationCode_example", playerDisplayName: "playerDisplayName_example") // AddPlayerParams | 

// Adds a Player to the Game. Returns the result of the initial Game Creation.
TicTacToeAPI.addPlayer(addPlayerParams: addPlayerParams) { (response, error) in
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
 **addPlayerParams** | [**AddPlayerParams**](AddPlayerParams.md) |  | 

### Return type

[**GameCreationResult**](GameCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **createGame**
```swift
    open class func createGame(newGameParams: NewGameParams, completion: @escaping (_ data: GameCreationResult?, _ error: Error?) -> Void)
```

Creates a new Game. Returns Game Creation Result.

Creates a new Game. Returns Game Creation Result.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let newGameParams = NewGameParams(gameMode: GameMode(), playerOneDisplayName: "playerOneDisplayName_example", singlePlayerSkillLevel: SkillLevel()) // NewGameParams | 

// Creates a new Game. Returns Game Creation Result.
TicTacToeAPI.createGame(newGameParams: newGameParams) { (response, error) in
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
 **newGameParams** | [**NewGameParams**](NewGameParams.md) |  | 

### Return type

[**GameCreationResult**](GameCreationResult.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **endGame**
```swift
    open class func endGame(gameId: String, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```

Closes down the specified Game.

Closes down the specified Game.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | 

// Closes down the specified Game.
TicTacToeAPI.endGame(gameId: gameId) { (response, error) in
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

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
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

# **getGameInfo**
```swift
    open class func getGameInfo(gameId: String, completion: @escaping (_ data: GameInfo?, _ error: Error?) -> Void)
```

Retrieves details of the specified Game.

Retrieves details of the specified Game.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | Game ID

// Retrieves details of the specified Game.
TicTacToeAPI.getGameInfo(gameId: gameId) { (response, error) in
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

[**GameInfo**](GameInfo.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **takeTurn**
```swift
    open class func takeTurn(gameId: String, gameTurnInfo: GameTurnInfo, completion: @escaping (_ data: Void?, _ error: Error?) -> Void)
```

Make a Game move (turn) for the specified Player.

Make a Game move (turn) for the specified Player.

### Example
```swift
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
import OpenAPIClient

let gameId = "gameId_example" // String | 
let gameTurnInfo = GameTurnInfo(destination: BoardPosition(column: 123, row: 123), playerId: "playerId_example") // GameTurnInfo | 

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

Void (empty response body)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

