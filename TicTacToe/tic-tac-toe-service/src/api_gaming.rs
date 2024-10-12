// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/**
 * Defines and implements the public Gaming contract for this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

use crate::game_state::GameState;
use crate::gaming_sessions_manager::GamingSessionsManager;
use crate::models::requests::{EndGameParams, GameTurnInfo, JoinSessionParams, NewGamingSessionParams, NewSinglePlayerGameParams, NewTwoPlayerGameParams, ID_LENGTH_MAX};
use crate::models::responses::{GameCreationResult, GameInfo, GamingSessionCreationResult};
use crate::tic_tac_toe_game::TicTacToeGame;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::debug;
use validator::Validate;


/*

    Single Player session:
    
        Player A
            Create a new Gaming Session
            Subscribe to MQTT
            Create a new Single-Player Game
            Play
            End Game
            End Gaming Session


    Two Player session:

        Player A
            Create a new Session
            Subscribe to MQTT
            Create a new Two-Player Game
            Manually invite Player B
        
        Player B
            Join the Session via Invitation Code
            Subscribe to MQTT

        Player A and Player B:
            Play
            Play
            Play
            
        Either Player:
            End Game
            
        Either Player:
            Create a new Two-Player Game

        Player A and Player B:
            Play
            Play
            Play

        Player B:        
            Exit Gaming Session

        Player A:        
            End Gaming Session
 */

/// Creates a new Game. Returns Game Creation Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/single-player-games",
    responses(
    (status = 200, description = "Single-Player Game created successfully", body = GameCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewSinglePlayerGameParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/single-player-games")]
pub(crate) async fn create_single_player_game(
    new_game_params: web::Json<NewSinglePlayerGameParams>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResult>> {
    //

    debug!("HTTP POST to /single-player-games. Params: {:?}", new_game_params);

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().await;

    let new_game_params: NewSinglePlayerGameParams = new_game_params.0;

    match games_manager.create_new_single_player_game(&new_game_params.clone().session_id,
                                                      &new_game_params.computer_skill_level).await {
        Ok(game) => {
            let new_game_info = GameCreationResult {
                game_info: GameInfo::from(game.clone()),
            };
            Ok(web::Json(new_game_info))
        }
        Err(error) => {
            Err(actix_web::error::ErrorInternalServerError(error.to_string()))
        }
    }
}

/// Creates a new Two-Player Game. Returns Game Creation Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/two-player-games",
    responses(
    (status = 200, description = "Two-Player Game created successfully", body = GameCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewTwoPlayerGameParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/two-player-games")]
pub(crate) async fn create_two_player_game(
    new_game_params: web::Json<NewTwoPlayerGameParams>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResult>> {
    //

    debug!("HTTP POST to /two-player-games. Params: {:?}", new_game_params);

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().await;

    let new_game_params = new_game_params.0;

    match games_manager.create_new_two_player_game(&new_game_params.session_id).await {
        Ok(game) => {
            let new_game_info = GameCreationResult {
                game_info: GameInfo::from(game.clone()),
            };
            Ok(web::Json(new_game_info))
        }
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Creates a new Gaming Session. Returns GamingSessionCreationResult.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions",
    responses(
    (status = 200, description = "Gaming Session created successfully", body = GamingSessionCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewGamingSessionParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions")]
pub(crate) async fn create_gaming_session(
    new_game_params: web::Json<NewGamingSessionParams>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResult>> {
    //

    debug!("HTTP POST to /gaming-sessions. Params: {:?}", new_game_params);

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().await;

    match games_manager.create_new_session(&new_game_params).await {
        Ok(session) => {
            let creation_result = GamingSessionCreationResult {
                event_plane_config: session.event_plane_config,
                invitation_code: session.invitation_code,
                session_id: session.session_id,
            };
            Ok(web::Json(creation_result))
        }
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Closes down the specified Game.
#[utoipa::path(
    delete,
    tag = "TicTacToe",
    path = "/v1/games/{game_id}",
    responses(
    (status = 200, description = "Game ended successfully"),
    (status = 400, description = "Bad request - Malformed Game ID"),
    (status = 403, description = "Unauthorized"),
    (status = 404, description = "Game not found")
,), )]
#[delete("/games/{game_id}")]
pub(crate) async fn end_game(
    end_game_params: web::Json<EndGameParams>,
    game_id: web::Path<String>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> HttpResponse {
    //

    debug!("HTTP DELETE to /games/{}", game_id);

    // *** Validate input params ***
    if let Err(e) = end_game_params.validate() {
        return actix_web::error::ErrorBadRequest(e.to_string()).into();
    }
    if game_id.is_empty() {
        return HttpResponse::BadRequest().body("Game ID is empty");
    } else if game_id.len() as u64 > ID_LENGTH_MAX {
        return HttpResponse::BadRequest().body("Game ID exceeds maximum length");
    }

    match games_manager.lock().await.end_game(&game_id, end_game_params.player_id.as_str(), end_game_params.session_id.as_str()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Closes down the specified Gaming Session.
#[utoipa::path(
    delete,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}",
    responses(
    (status = 200, description = "Gaming Session ended successfully"),
    (status = 400, description = "Bad request - Malformed Gaming Session ID"),
    (status = 403, description = "Unauthorized"),
    (status = 404, description = "Gaming Session not found")
,), )]
#[delete("/gaming-sessions/{session_id}")]
pub(crate) async fn end_gaming_session(
    session_id: web::Path<String>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> HttpResponse {
    //

    debug!("HTTP DELETE to /gaming-sessions/{}", session_id);

    // *** Validate input params ***
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("Gaming Session ID is empty");
    } else if session_id.len() as u64 > ID_LENGTH_MAX {
        return HttpResponse::BadRequest().body("Gaming Session ID exceeds maximum length");
    }

    // TODO: JD: only allow Players who owns the session to end the session.

    match games_manager.lock().await.end_gaming_session(&session_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Retrieves the history of the Game States from the initial move (turn) to the latest
/// Game State. This can be used, for instance, to create an animated time-lapse of the Game play.
#[utoipa::path(
    get,
    tag = "TicTacToe",
    path = "/v1/games/{game_id}/turns",
    responses(
    (status = 200, description = "Game history retrieved successfully", body = Vec < GameState >, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed Game ID"),
    (status = 404, description = "Game not found"),
    (status = 500, description = "Internal server error")
,), )]
#[get("/games/{game_id}/turns")]
pub(crate) async fn get_game_history(
    game_id: web::Path<String>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<Vec<GameState>>> {
    //

    debug!("HTTP GET to /games/{}/turns", game_id);

    // *** Validate input params ***
    validate_game_id(&game_id)?;

    match games_manager.lock().await.get_game_history(&game_id).await {
        Ok(history) => Ok(web::Json(history)),
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Retrieves details of the specified Game.
#[utoipa::path(
    get,
    tag = "TicTacToe",
    path = "/v1/games/{game_id}",
    params(("game_id" = String, Path, description = "Game ID"),),
    responses(
    (status = 200, description = "Game info retrieved successfully", body = GameInfo, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed Game ID"),
    (status = 404, description = "Game not found"),
    (status = 500, description = "Internal server error")
,), )]
#[get("/games/{game_id}")]
pub(crate) async fn get_game_info(
    game_id: web::Path<String>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameInfo>> {
    //

    debug!("HTTP GET to /games/{}", game_id);

    // *** Validate input params ***
    validate_game_id(&game_id)?;

    match games_manager
        .lock()
        .await
        .get_game_by_id(game_id.as_str()).await
    {
        Ok(game) => Ok(web::Json(GameInfo::from(game))),
        Err(error) => Err(actix_web::error::ErrorInternalServerError(error.to_string())),
    }
}

/// Adds a Player to the Gaming Session.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/players",
    responses(
    (status = 200, description = "Player added to the Gaming Session", body = GamingSessionCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed JoinSessionParams"),
    (status = 404, description = "No Game found for the specified Invitation"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions/players")]
pub(crate) async fn join_gaming_session(
    params: web::Json<JoinSessionParams>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResult>> {
    //

    debug!("HTTP POST to /gaming-sessions/players. Params: {:?}", params);

    // *** Validate input params ***
    if let Err(e) = params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().await;
    let second_player_params = params.into_inner();

    match games_manager.add_player_to_session(&second_player_params).await {
        Ok(session) => {
            let game_session_addition_result = GamingSessionCreationResult {
                event_plane_config: session.event_plane_config,
                invitation_code: session.invitation_code,
                session_id: session.session_id,
            };
            Ok(web::Json(game_session_addition_result))
        }
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Make a Game move (turn) for the specified Player.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/games/{game_id}/turns",
    responses(
    (status = 200, description = "Game turn added successfully"),
    (status = 400, description = "Bad Request - Malformed Game ID, Invalid Board Position"),
    (status = 404, description = "Not Found - Game Not Found"),
    (status = 405, description = "Method Not Allowed - Wrong Player Taking Turn"),
    (status = 406, description = "Not Acceptable - Game Has Already Ended"),
    (status = 409, description = "Conflict - Board Location Already Occupied"),
    ), )]
#[post("/games/{game_id}/turns")]
pub(crate) async fn take_turn(
    game_id: web::Path<String>,
    game_turn_info: web::Json<GameTurnInfo>,
    games_manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> impl Responder {
    //

    debug!("HTTP POST to /games/{}/turns", game_id);

    // *** Validate input params ***
    validate_game_id(&game_id)?;

    if let Err(e) = game_turn_info.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    match games_manager
        .lock()
        .await
        .take_turn(&game_id, &game_turn_info).await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Verifies that the specified Game ID is formatted properly.
fn validate_game_id(game_id: &str) -> actix_web::Result<()> {
    if game_id.is_empty() {
        Err(actix_web::error::ErrorBadRequest("Game ID is empty"))
    } else if game_id.len() as u64 > ID_LENGTH_MAX {
        Err(actix_web::error::ErrorBadRequest("Game ID exceeds maximum length"))
    } else {
        Ok(())
    }
}
