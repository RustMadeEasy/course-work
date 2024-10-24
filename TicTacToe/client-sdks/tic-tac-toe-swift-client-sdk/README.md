# Swift5 API client for OpenAPIClient

Tic-Tac-Toe Game Service

## Overview
This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://github.com/OAI/OpenAPI-Specification) from a remote server, you can easily generate an API client.

- API version: 0.4.0
- Package version: 
- Generator version: 7.9.0
- Build package: org.openapitools.codegen.languages.Swift5ClientCodegen
For more information, please visit [https://RustMadeEasy.com](https://RustMadeEasy.com)

## Installation

### Carthage

Run `carthage update`

### CocoaPods

Run `pod install`

## Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*TicTacToeAPI* | [**createGamingSession**](docs/TicTacToeAPI.md#creategamingsession) | **POST** /v1/gaming-sessions | Creates a new Gaming Session. Returns GamingSessionCreationResult.
*TicTacToeAPI* | [**createSinglePlayerGame**](docs/TicTacToeAPI.md#createsingleplayergame) | **POST** /v1/gaming-sessions/{session_id}/games | Creates a new Game. Returns Game Creation Result.
*TicTacToeAPI* | [**createTwoPlayerGame**](docs/TicTacToeAPI.md#createtwoplayergame) | **POST** /v1/gaming-session/{session_id}/two-player-games | Creates a new Two-Player Game. Returns Game Creation Result.
*TicTacToeAPI* | [**endGame**](docs/TicTacToeAPI.md#endgame) | **DELETE** /v1/games/{game_id} | Closes down the specified Game.
*TicTacToeAPI* | [**endGamingSession**](docs/TicTacToeAPI.md#endgamingsession) | **DELETE** /v1/gaming-sessions/{session_id} | Closes down the specified Gaming Session.
*TicTacToeAPI* | [**getGameHistory**](docs/TicTacToeAPI.md#getgamehistory) | **GET** /v1/games/{game_id}/turns | Retrieves the history of the Game States from the initial move (turn) to the latest
*TicTacToeAPI* | [**getLatestGameTurn**](docs/TicTacToeAPI.md#getlatestgameturn) | **GET** /v1/games/{game_id}/turns/latest | Retrieves details of the specified Game.
*TicTacToeAPI* | [**getSessionCurrentGame**](docs/TicTacToeAPI.md#getsessioncurrentgame) | **GET** /v1/gaming-sessions/{session_id}/current-game | Retrieves the Games in a Gaming Session.
*TicTacToeAPI* | [**joinGamingSession**](docs/TicTacToeAPI.md#joingamingsession) | **POST** /v1/gaming-sessions/players | Adds a Player to the Gaming Session.
*TicTacToeAPI* | [**notePlayerReadiness**](docs/TicTacToeAPI.md#noteplayerreadiness) | **PUT** /v1/gaming-sessions/{session_id}/players/{player_id}/readiness | Sets a Player as ready to Play.
*TicTacToeAPI* | [**takeTurn**](docs/TicTacToeAPI.md#taketurn) | **POST** /v1/games/{game_id}/turns | Make a Game move (turn) for the specified Player.


## Documentation For Models

 - [AutomaticPlayerSkillLevel](docs/AutomaticPlayerSkillLevel.md)
 - [BoardPosition](docs/BoardPosition.md)
 - [EndGameParams](docs/EndGameParams.md)
 - [EndGamingSessionParams](docs/EndGamingSessionParams.md)
 - [EventPlaneConfig](docs/EventPlaneConfig.md)
 - [EventPlaneTopicNames](docs/EventPlaneTopicNames.md)
 - [GameCreationResult](docs/GameCreationResult.md)
 - [GameInfo](docs/GameInfo.md)
 - [GameMode](docs/GameMode.md)
 - [GamePiece](docs/GamePiece.md)
 - [GameState](docs/GameState.md)
 - [GameTurnInfo](docs/GameTurnInfo.md)
 - [GamingSessionCreationResult](docs/GamingSessionCreationResult.md)
 - [JoinSessionParams](docs/JoinSessionParams.md)
 - [NewGamingSessionParams](docs/NewGamingSessionParams.md)
 - [NewSinglePlayerGameParams](docs/NewSinglePlayerGameParams.md)
 - [PlayStatus](docs/PlayStatus.md)
 - [PlayerInfo](docs/PlayerInfo.md)
 - [TurnResult](docs/TurnResult.md)


<a id="documentation-for-authorization"></a>
## Documentation For Authorization

Endpoints do not require authorization.


## Author

JoelDavisEngineering@Gmail.com

