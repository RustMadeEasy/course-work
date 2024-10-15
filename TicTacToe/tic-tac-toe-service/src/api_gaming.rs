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
use crate::game_state::GameState;
use crate::gaming_sessions_manager::GamingSessionsManager;
use crate::models::requests::{EndGameParams, EndGamingSessionParams, GameTurnInfo, JoinSessionParams, NewGamingSessionParams, NewSinglePlayerGameParams, ID_LENGTH_MAX};
use crate::models::responses::{GameCreationResult, GameInfo, GamingSessionCreationResult};
use crate::tic_tac_toe_game::TicTacToeGame;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::debug;
use validator::Validate;


/*

    Single Player session:
    
        Player A
            Creates a new Gaming Session
            Subscribes to MQTT
            Creates a new Single-Player Game
            Joins Game
            Plays
            Ends Game
            Ends Gaming Session


    Two Player session:

        Player A
            Creates a new Session
            Subscribes to MQTT
            Manually invites Player B
        
        Player B
            Joins the Session via the Invitation Code
            Subscribes to MQTT

        Player A
            Receives PlayerAddedToSession
            Creates a new Two-Player Game

        Player B
            Receives GameStarted
            Joins Game by Session ID
     
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

        Either Player:
            Exit Gaming Session
 */

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
    params: web::Json<NewGamingSessionParams>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResult>> {
    //

    debug!("HTTP POST to /gaming-sessions. Params: {:?}", params);

    // *** Validate input params ***
    if let Err(e) = params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut manager = manager.lock().await;

    match manager.create_new_session(&params.session_owner_display_name).await {
        Ok(session) => {
            let creation_result = GamingSessionCreationResult {
                current_game_id: Default::default(),
                event_plane_config: session.event_plane_config,
                invitation_code: session.invitation_code,
                session_id: session.session_id,
            };
            Ok(web::Json(creation_result))
        }
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Creates a new Game. Returns Game Creation Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/games",
    responses(
    (status = 200, description = "Single-Player Game created successfully", body = GameCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewSinglePlayerGameParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions/{session_id}/games")]
pub(crate) async fn create_single_player_game(
    new_game_params: web::Json<NewSinglePlayerGameParams>,
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResult>> {
    //

    debug!("HTTP POST to /gaming-sessions/{}/games. Params: {:?}", session_id, new_game_params);

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut manager = manager.lock().await;

    let new_game_params: NewSinglePlayerGameParams = new_game_params.0;

    let session = match manager.get_session_by_session_id(&session_id).await {
        None => return Err(actix_web::error::ErrorInternalServerError(GameError::SessionNotFound)),
        Some(session) => session,
    };

    match manager.create_new_single_player_game(session.session_id.as_str(), &new_game_params.computer_skill_level).await {
        Ok(game) => {
            let new_game_info = GameCreationResult {
                game_info: GameInfo::from(game.clone()),
                session_id: session.session_id.clone(),
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
    path = "/v1/gaming-session/two-player-games",
    responses(
    (status = 200, description = "Two-Player Game created successfully", body = GameCreationResult, content_type = "application/json"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-session/{session_id}/two-player-games")]
pub(crate) async fn create_two_player_game(
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResult>> {
    //

    debug!("HTTP POST to /gaming-session/two-player-games. Session-Id: {:?}", session_id);

    // *** Validate input params ***
    validate_id_string(&session_id)?;

    let mut manager = manager.lock().await;

    // Create a session if there is not one already.
    let session = match manager.get_session_by_session_id(&session_id).await {
        Some(session) => *session,
        None => {
            return Err(actix_web::error::ErrorInternalServerError(GameError::SessionNotFound))
        }
    };

    match manager.create_new_two_player_game(&session.session_id).await {
        Ok(game) => {
            let new_game_info = GameCreationResult {
                game_info: GameInfo::from(game.clone()),
                session_id: session.session_id.clone(),
            };
            Ok(web::Json(new_game_info))
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
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
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

    match manager.lock().await.end_game(&game_id, end_game_params.player_id.as_str(), end_game_params.session_id.as_str()).await {
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
    params: web::Json<EndGamingSessionParams>,
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> HttpResponse {
    //

    debug!("HTTP DELETE to /gaming-sessions/{}", session_id);

    // *** Validate input params ***
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("Gaming Session ID is empty");
    } else if session_id.len() as u64 > ID_LENGTH_MAX {
        return HttpResponse::BadRequest().body("Gaming Session ID exceeds maximum length");
    }

    match manager.lock().await.end_gaming_session(params.player_id.as_str(), &session_id).await {
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
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<Vec<GameState>>> {
    //

    debug!("HTTP GET to /games/{}/turns", game_id);

    // *** Validate input params ***
    validate_id_string(&game_id)?;

    match manager.lock().await.get_game_history(&game_id).await {
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
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameInfo>> {
    //

    debug!("HTTP GET to /games/{}", game_id);

    // *** Validate input params ***
    validate_id_string(&game_id)?;

    match manager
        .lock()
        .await
        .get_game_by_id(game_id.as_str()).await
    {
        Ok(game) => {
            Ok(web::Json(GameInfo::from(game)))
        }
        Err(error) => Err(actix_web::error::ErrorInternalServerError(error.to_string())),
    }
}

/// Retrieves the Games in a Gaming Session.
#[utoipa::path(
    get,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/current-games",
    params(("session_id" = String, Path, description = "Session ID"),),
    responses(
    (status = 200, description = "Gaming Session Games retrieved successfully", body = Vec<GameInfo>, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed Session ID"),
    (status = 404, description = "Session not found"),
    (status = 500, description = "Internal server error")
,), )]
#[get("/gaming-sessions/{session_id}/current-games")]
pub(crate) async fn get_session_current_games(
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<Vec<GameInfo>>> {
    //

    debug!("HTTP GET to /gaming-sessions/{}/current-games", session_id);

    // *** Validate input params ***
    validate_id_string(&session_id)?;

    match manager
        .lock()
        .await
        .get_games_in_session(session_id.as_str()).await
    {
        Ok(games) => {
            Ok(web::Json(games.into_iter().map(GameInfo::from).collect()))
        }
        Err(error) => {
            Err(actix_web::error::ErrorInternalServerError(error))
        }
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
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResult>> {
    //

    debug!("HTTP POST to /gaming-sessions/players. Params: {:?}", params);

    // *** Validate input params ***
    if let Err(e) = params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut manager = manager.lock().await;
    let params = params.into_inner();

    match manager.add_player_to_session(&params.game_invitation_code, &params.player_display_name).await {
        Ok(result) => {
            let game_session_addition_result = GamingSessionCreationResult {
                current_game_id: result.current_game_id,
                event_plane_config: result.event_plane_config,
                invitation_code: result.invitation_code,
                session_id: result.session_id,
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
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> impl Responder {
    //

    debug!("HTTP POST to /games/{}/turns", game_id);

    // *** Validate input params ***
    validate_id_string(&game_id)?;

    if let Err(e) = game_turn_info.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    match manager
        .lock()
        .await
        .take_turn(&game_id, &game_turn_info).await
    {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Verifies that the specified ID is of the correct length.
fn validate_id_string(id: &str) -> actix_web::Result<()> {
    if id.is_empty() {
        Err(actix_web::error::ErrorBadRequest("ID is empty"))
    } else if id.len() as u64 > ID_LENGTH_MAX {
        Err(actix_web::error::ErrorBadRequest("ID exceeds maximum length"))
    } else {
        Ok(())
    }
}
