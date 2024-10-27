// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/**
 * Defines and implements the public Gaming Session contract for this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

use crate::api::games::validate_id_string;
use crate::errors::GameError;
use crate::gaming::gaming_sessions_manager::GamingSessionsManager;
use crate::gaming::tic_tac_toe_game::TicTacToeGame;
use crate::models::player_info::PlayerInfo;
use crate::models::requests::{EndGamingSessionParams, JoinSessionParams, NewGamingSessionParams, ID_LENGTH_MAX};
use crate::models::responses::{GameCreationResponse, GameInfoResponse, GamingSessionCreationResponse};
use actix_web::{delete, get, post, put, web, Error, HttpResponse};
use log::debug;
use validator::Validate;


/// Creates a new Gaming Session. Returns GamingSessionCreationResult.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions",
    responses(
    (status = 200, description = "Gaming Session created successfully", body = GamingSessionCreationResponse, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewGamingSessionParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions")]
pub(crate) async fn create_gaming_session(
    params: web::Json<NewGamingSessionParams>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResponse>> {
    //

    // *** Validate input params ***
    if let Err(e) = params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    debug!("HTTP POST to /gaming-sessions. Params: {:?}", params);

    let mut manager = manager.lock().await;

    match manager.create_new_session(&params.session_owner_display_name).await {
        Ok(session) => {
            let creation_result = GamingSessionCreationResponse {
                event_plane_config: session.event_plane_config,
                initiating_player: session.session_owner,
                invitation_code: session.invitation_code,
                other_player: None,
                session_id: session.session_id,
            };
            Ok(web::Json(creation_result))
        }
        Err(error) => Err(error.into()),
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

    // *** Validate input params ***
    if session_id.is_empty() {
        return HttpResponse::BadRequest().body("Gaming Session ID is empty");
    } else if session_id.len() as u64 > ID_LENGTH_MAX {
        return HttpResponse::BadRequest().body("Gaming Session ID exceeds maximum length");
    }

    debug!("HTTP DELETE to /gaming-sessions/{}", session_id);

    match manager.lock().await.end_gaming_session(params.player_id.as_str(), &session_id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Retrieves the Gaming Session's current Game.
#[utoipa::path(
    get,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/current-game",
    params(("session_id" = String, Path, description = "Session ID"),),
    responses(
    (status = 200, description = "Gaming Session Game retrieved successfully", body = GameCreationResponse, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed Session ID"),
    (status = 404, description = "Session not found"),
    (status = 500, description = "Internal server error")
,), )]
#[get("/gaming-sessions/{session_id}/current-game")]
pub(crate) async fn get_session_current_game(
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResponse>> {
    //

    // *** Validate input params ***
    validate_id_string(&session_id)?;

    debug!("HTTP GET to /gaming-sessions/{}/current-game", session_id);

    match manager
        .lock()
        .await
        .get_game_in_session(session_id.as_str()).await
    {
        Ok((session, game)) => {
            let other_player = session.participants.iter().find(|it| it.player_id != session.session_owner.player_id).cloned();
            let result = GameCreationResponse {
                game_info: GameInfoResponse::from(game),
                initiating_player: session.session_owner,
                other_player,
                session_id: session.session_id,
            };
            Ok(web::Json(result))
        }
        Err(error) => {
            Err(Error::from(error))
        }
    }
}

/// Adds a Player to the Session's Current Game.
#[utoipa::path(
    put,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/current_game/players/{player_id}",
    responses(
    (status = 200, description = "Player joined Game successfully", body = GameCreationResponse, content_type = "application/json"),
    (status = 404, description = "Game not found"),
    (status = 404, description = "Player not found"),
    (status = 404, description = "Session not found"),
    (status = 500, description = "Internal server error")
,), )]
#[put("/gaming-sessions/{session_id}/current_game/players/{player_id}")]
pub(crate) async fn join_current_game(
    session_and_player: web::Path<(String, String)>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResponse>> {
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

    let session = match manager.get_session_by_id(&session_and_player.0).await {
        None => return Err(Error::from(GameError::GamingSessionNotFound)),
        Some(session) => session,
    };

    match manager.join_current_game(&session_and_player.0, &session_and_player.1).await {
        Ok(result) => {
            // Add the other Player if they are already part of the Gaming Session.
            let other_player = PlayerInfo::get_other_player_info(session.session_owner.player_id.clone(), &session.participants);
            let new_game_info = GameCreationResponse {
                game_info: GameInfoResponse::from(result.0.clone()),
                initiating_player: session.session_owner,
                other_player,
                session_id: session.session_id,
            };
            Ok(web::Json(new_game_info))
        }
        Err(error) => Err(error.into()),
    }
}

/// Adds a Player to the Gaming Session.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/players",
    responses(
    (status = 200, description = "Player added to the Gaming Session", body = GamingSessionCreationResponse, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed JoinSessionParams"),
    (status = 404, description = "No Game found for the specified Invitation"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions/players")]
pub(crate) async fn join_gaming_session(
    params: web::Json<JoinSessionParams>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResponse>> {
    //

    // *** Validate input params ***
    if let Err(e) = params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    debug!("HTTP POST to /gaming-sessions/players. Params: {:?}", params);

    let mut manager = manager.lock().await;
    let params = params.into_inner();

    match manager.add_player_to_session(&params.game_invitation_code, &params.player_display_name).await {
        Ok(result) => {
            Ok(web::Json(result))
        }
        Err(error) => Err(error.into()),
    }
}

