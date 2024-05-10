# Tic-Tac-Toe Game Service

## Description

Provides 2-client game-play of Tic-Tac-Toe.

NOTE: This is sample code, part of the RustMadeEasy.com course: Intro to Rust. This code is not suitable for
production. For example, the end-points are not secured and the game state is not persisted in a centralized fashion.
Lack of central persistence means that only a single instance can be run (preventing resilience and
scalability). Future courses will enhance the service so that it goes from sample-quality code to being highly secure 
and scalable.

Roadmap:

1. Validation for request parameters.
2. Auth.
3. Sockets for Game state changes so that the clients do not have to poll.
4. Notion of Game User-pair Session so that an invitation is still required to initially connect Users, but, a new invitation is no longer required for rematches within a Gaming Session.
5. Central persistence, e.g. Surreal DB.
6. Automatic cleanup of older or abandoned Games.
7. AI model so that users can play against the service.

## Supported End-Points

| Name             | Method | Path                      | Description                        |
|------------------|--------|---------------------------|------------------------------------|
| Health           | GET    | /v1/health                | Provides the status of the Service |
| Open API Spec    | GET    | /v1/api-docs              | Generates an OpenAPI and document  |
|                  |        |                           |                                    |
| Add Player       | POST   | /v1/games/players         | Adds a player to a Game            |
| Create Game      | POST   | /v1/games                 | Creates a new Game                 |
| End Game         | DELETE | /v1/games/{game_id}       | Ends a Game                        |
| Get Game History | GET    | /v1/games/{game_id}/turns | Retrieves the history of a Game    |
| Get Game Info    | GET    | /v1/games/{game_id}       | Gets the info for a Game           |
| Take Turn        | PUT    | /v1/games/{game_id}/turns | Performs a new Game move           |

## Usage

![](./Tic-Tac-Toe_Call_Sequence.png)

1. The first client (Player One) starts a new Game by posting to Create Game (POST /v1/games).
2. The first client begins to gently poll the board state by calling Get Game Info (GET /v1/games/{game_id}), updating
   the UI rendering and the state of the client app.
3. Player One invites Player Two to the game by sharing the Game Invitation Code with Player Two. The invitation code is
   in the response returned by Get Game Info.
4. The second client (Player Two) joins the Game by using the Game Invitation Code and the Second Player's info to post
   to Add Player (POST /v1/games/players). The Add Player responds with Game Info which contains the Game ID required
   for all
   subsequent calls.
5. The second client begins to gently poll the board state by calling Get Game Info (GET /v1/games/{game_id}), updating
   the UI rendering and the state of the client app.
6. Each client takes turns on behalf of its Player by calling Take Turn (PUT /v1/games/{game_id}/turns).
7. When the board state response from the Take Turn call indicates that the Game is won or stalemated, the clients show
   this visually and disallow further game play.
8. The first client calls End Game (DELETE /v1/games/{game_id}).

## Generating a Client SDK

### Kotlin
`openapi-generator generate -i "api-docs.txt" -g kotlin -o ./sdks/kotlin/tic-tac-toe-kotlin-client-sdk`

### Rust
`openapi-generator generate -i "api-docs.txt" -g rust -o ./sdks/tic-tac-toe-rust-client-sdk --package-name tic_tac_toe_rust_client_sdk --additional-properties=avoidBoxedModels=true`
`openapi-generator generate -i "api-docs.txt" -g rust -o ./sdks/tic-tac-toe-rust-client-sdk --package-name tic_tac_toe_rust_client_sdk --additional-properties=avoidBoxedModels=true,supportAsync=false`

### Swift
`openapi-generator generate -i "api-docs.txt" -g swift5 -o ./sdks/tic-tac-toe-swift-client-sdk`

## Port

50020

## API Documentation

https://github.com/RustMadeEasy/course-work.git
