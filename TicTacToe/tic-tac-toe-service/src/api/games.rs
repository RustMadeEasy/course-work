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

use crate::errors::GameError;
use crate::gaming::gaming_sessions_manager::GamingSessionsManager;
use crate::gaming::tic_tac_toe_game::TicTacToeGame;
use crate::models::game_state::GameState;
use crate::models::player_info::PlayerInfo;
use crate::models::requests::{EndGameParams, GameTurnParams, NewSinglePlayerGameParams, ID_LENGTH_MAX, ID_LENGTH_MIN};
use crate::models::responses::{GameCreationResponse, GameInfoResponse, TurnResponse};
use actix_web::{delete, get, post, put, web, Error, HttpResponse, ResponseError};
use log::debug;
use validator::Validate;


/// Creates a new Game. Returns Game Creation Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/games",
    responses(
    (status = 200, description = "Single-Player Game created successfully", body = GameCreationResponse, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewSinglePlayerGameParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions/{session_id}/games")]
pub(crate) async fn create_single_player_game(
    new_game_params: web::Json<NewSinglePlayerGameParams>,
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResponse>> {
    //

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    debug!("HTTP POST to /gaming-sessions/{}/games. Params: {:?}", session_id, new_game_params);

    let mut manager = manager.lock().await;

    let new_game_params: NewSinglePlayerGameParams = new_game_params.0;

    let session = match manager.get_session_by_id(&session_id).await {
        None => return Err(Error::from(GameError::GamingSessionNotFound)),
        Some(session) => session,
    };

    match manager.create_new_single_player_game(session.session_id.as_str(), &new_game_params.computer_skill_level).await {
        Ok(game) => {
            let other_player = PlayerInfo::get_other_player_info(session.session_owner.player_id.clone(), &game.players)?;
            let new_game_info = GameCreationResponse {
                game_info: GameInfoResponse::from(game.clone()),
                initiating_player: session.session_owner,
                other_player: Some(other_player),
                session_id: session.session_id.clone(),
            };
            Ok(web::Json(new_game_info))
        }
        Err(error) => Err(error.into()),
    }
}

/// Creates a new Two-Player Game. Returns Game Creation Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-session/{session_id}/two-player-games",
    responses(
    (status = 200, description = "Two-Player Game created successfully", body = GameCreationResponse, content_type = "application/json"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-session/{session_id}/two-player-games")]
pub(crate) async fn create_two_player_game(
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResponse>> {
    //

    // *** Validate input params ***
    validate_id_string(&session_id)?;

    debug!("HTTP POST to /gaming-session/two-player-games. Session-Id: {:?}", session_id);

    let mut manager = manager.lock().await;

    let session = match manager.get_session_by_id(&session_id).await {
        None => return Err(Error::from(GameError::GamingSessionNotFound)),
        Some(session) => session,
    };

    match manager.create_new_two_player_game(&session_id).await {
        Ok(result) => {
            // Add the other Player if they are already part of the Gaming Session.
            let other_player = match PlayerInfo::get_other_player_info(session.session_owner.player_id.clone(), &session.participants) {
                Ok(other_player) => Some(other_player),
                Err(_) => None,
            };
            let new_game_info = GameCreationResponse {
                game_info: GameInfoResponse::from(result.0.clone()),
                initiating_player: session.session_owner,
                other_player,
                session_id: session_id.clone(),
            };
            Ok(web::Json(new_game_info))
        }
        Err(error) => Err(error.into()),
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
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> HttpResponse {
    //

    // *** Validate input params ***
    if let Err(e) = end_game_params.validate() {
        return actix_web::error::ErrorBadRequest(e.to_string()).into();
    }
    if game_id.is_empty() {
        return HttpResponse::BadRequest().body("Game ID is empty");
    } else if game_id.len() as u64 > ID_LENGTH_MAX {
        return HttpResponse::BadRequest().body("Game ID exceeds maximum length");
    }

    debug!("HTTP DELETE to /games/{}", game_id);

    match manager.lock().await.end_game(&game_id, end_game_params.player_id.as_str(), end_game_params.session_id.as_str()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Retrieves the history of Game States from the initial move (turn) to the latest Game State. 
/// This can be used, for instance, to create an animated time-lapse of the Game play.
#[utoipa::path(
    get,
    tag = "TicTacToe",
    path = "/v1/games/{game_id}/turns",
    responses(
    (status = 200, description = "Game history retrieved successfully", body = Vec<GameState>, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed Game ID"),
    (status = 404, description = "Game not found"),
    (status = 500, description = "Internal server error")
,), )]
#[get("/games/{game_id}/turns")]
pub(crate) async fn get_game_history(
    game_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<Vec<GameState>>> {
    //

    // *** Validate input params ***
    validate_id_string(&game_id)?;

    debug!("HTTP GET to /games/{}/turns", game_id);

    match manager.lock().await.get_game_history(&game_id).await {
        Ok(history) => Ok(web::Json(history)),
        Err(error) => Err(error.into()),
    }
}

/// Retrieves the most recent Turn Result for the specified Game.
#[utoipa::path(
    get,
    tag = "TicTacToe",
    path = "/v1/games/{game_id}/turns/latest",
    params(("game_id" = String, Path, description = "Game ID"),),
    responses(
    (status = 200, description = "Latest Game Turn info retrieved successfully", body = TurnResponse, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed Game ID"),
    (status = 404, description = "Game not found"),
    (status = 500, description = "Internal server error")
,), )]
#[get("/games/{game_id}/turns/latest")]
pub(crate) async fn get_latest_game_turn(
    game_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<TurnResponse>> {
    //

    // *** Validate input params ***
    validate_id_string(&game_id)?;

    debug!("HTTP GET to /games/{}/turns/latest", game_id);

    match manager
        .lock()
        .await
        .get_game_by_id(game_id.as_str()).await
    {
        Ok(game) => {
            match game.latest_turn_result {
                Some(result) => Ok(web::Json(result)),
                None => Err(Error::from(GameError::GameNotStarted)),
            }
        }
        Err(error) => Err(error.into()),
    }
}

/// Adds a Player to the Session's Current Game.
#[utoipa::path(
    put,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/current_game/players/{player_id}",
    responses(
    (status = 200, description = "Player joined Game successfully"),
    (status = 404, description = "Game not found"),
    (status = 404, description = "Player not found"),
    (status = 404, description = "Session not found"),
    (status = 500, description = "Internal server error")
,), )]
#[put("/gaming-sessions/{session_id}/current_game/players/{player_id}")]
pub(crate) async fn join_current_game(
    session_and_player: web::Path<(String, String)>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> HttpResponse {
    //

    // *** Validate input params ***
    match validate_id_string(&session_and_player.0) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => error.into(),
    };
    match validate_id_string(&session_and_player.1) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => error.into(),
    };

    debug!("HTTP PUT to /gaming-sessions/{}/current_game/players/{}.",
        session_and_player.0,
        session_and_player.1);

    let mut manager = manager.lock().await;

    match manager.join_current_game(&session_and_player.0, &session_and_player.1).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(error) => error.error_response(),
    }
}

/// Make a Game move (turn) for the specified Player. Returns the Turn Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/games/{game_id}/turns",
    responses(
    (status = 200, description = "Game turn added successfully", body = TurnResponse, content_type = "application/json"),
    (status = 400, description = "Bad Request - Malformed Game ID, Invalid Board Position"),
    (status = 404, description = "Not Found - Game Not Found"),
    (status = 405, description = "Method Not Allowed - Wrong Player Taking Turn"),
    (status = 406, description = "Not Acceptable - Game Has Already Ended"),
    (status = 409, description = "Conflict - Board Location Already Occupied"),
    ), )]
#[post("/games/{game_id}/turns")]
pub(crate) async fn take_turn(
    game_id: web::Path<String>,
    game_turn_info: web::Json<GameTurnParams>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<TurnResponse>> {
    //

    // *** Validate input params ***
    validate_id_string(&game_id)?;

    debug!("HTTP POST to /games/{}/turns", game_id);

    if let Err(e) = game_turn_info.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    match manager
        .lock()
        .await
        .take_turn(&game_id, &game_turn_info).await
    {
        Ok(turn_info) => Ok(web::Json(turn_info)),
        Err(error) => Err(error.into()),
    }
}

/// Verifies that the specified ID is of the correct length.
pub(crate) fn validate_id_string(id: &str) -> actix_web::Result<()> {
    if id.is_empty() {
        Err(actix_web::error::ErrorBadRequest("ID is empty"))
    } else if (id.len() as u64) < ID_LENGTH_MIN {
        Err(actix_web::error::ErrorBadRequest("ID length is smaller than the minimum length allowed"))
    } else if (id.len() as u64) > ID_LENGTH_MAX {
        Err(actix_web::error::ErrorBadRequest("ID exceeds maximum length"))
    } else {
        Ok(())
    }
}
