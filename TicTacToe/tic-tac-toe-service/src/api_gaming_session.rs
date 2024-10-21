use crate::api_gaming::validate_id_string;
use crate::errors::GameError;
use crate::gaming_sessions_manager::GamingSessionsManager;
use crate::models::requests::{EndGamingSessionParams, JoinSessionParams, NewGamingSessionParams, ID_LENGTH_MAX};
use crate::models::responses::{GameCreationResult, GameInfo, GamingSessionCreationResult};
use crate::tic_tac_toe_game::TicTacToeGame;
use actix_web::{delete, get, post, put, web, Error, HttpResponse, Responder};
use log::debug;
use validator::Validate;

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
            Ok(web::Json(result))
        }
        Err(error) => Err(error.into()),
    }
}

/// Sets a Player as ready to Play.
#[utoipa::path(
    put,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/players/{player_id}/readiness",
    responses(
    (status = 200, description = "Notifies other Players that the newly added Player is ready to begin the Game"),
    (status = 400, description = "Bad request - Malformed Session ID or Player ID"),
    (status = 404, description = "No Gaming Session found"),
    (status = 500, description = "Internal server error")
,), )]
#[put("/gaming-sessions/{session_id}/players/{player_id}/readiness")]
pub(crate) async fn note_player_readiness(
    ids: web::Path<(String, String)>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> impl Responder {
    //

    debug!("HTTP POST to /gaming-sessions/{}/players/{}/readiness.", ids.0, ids.1);

    let (session_id, player_id) = ids.into_inner();

    // *** Validate input params ***
    validate_id_string(&session_id)?;
    validate_id_string(&player_id)?;

    let manager = manager.lock().await;

    match manager.note_player_readiness(&session_id, &player_id).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => Err(Error::from(error)),
    }
}

/// Retrieves the Games in a Gaming Session.
#[utoipa::path(
    get,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/{session_id}/current-game",
    params(("session_id" = String, Path, description = "Session ID"),),
    responses(
    (status = 200, description = "Gaming Session Game retrieved successfully", body = GameCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed Session ID"),
    (status = 404, description = "Session not found"),
    (status = 500, description = "Internal server error")
,), )]
#[get("/gaming-sessions/{session_id}/current-game")]
pub(crate) async fn get_session_current_game(
    session_id: web::Path<String>,
    manager: web::Data<tokio::sync::Mutex<GamingSessionsManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResult>> {
    //

    debug!("HTTP GET to /gaming-sessions/{}/current-game", session_id);

    // *** Validate input params ***
    crate::api_gaming::validate_id_string(&session_id)?;

    match manager
        .lock()
        .await
        .get_game_in_session(session_id.as_str()).await
    {
        Ok((session, game)) => {
            match session.participants.iter().find(|it| it.player_id != session.session_owner.player_id) {
                None => {
                    Err(Error::from(GameError::SessionHasTooFewPlayers))
                }
                Some(other_player) => {
                    let result = GameCreationResult {
                        game_info: GameInfo::from(game),
                        initiating_player: session.session_owner,
                        other_player: other_player.clone(),
                        session_id: session.session_id,
                    };
                    Ok(web::Json(result))
                }
            }
        }
        Err(error) => {
            Err(Error::from(error))
        }
    }
}

