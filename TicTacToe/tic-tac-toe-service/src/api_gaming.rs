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

/*

    Client A
        * Create Session
        * Create Game
        * Use Session ID to Setup MQTT
        * Share Invitation ID
    
    Client B
        Join session via invitation code
        Use Session ID to Setup MQTT
        Join Session's current game
    
    Both
        Play Game

 */

use crate::game_state::GameState;
use crate::games_manager::GamesManager;
use crate::models::requests::{GameTurnInfo, JoinGameParams, JoinSessionParams, NewGameParams, NewGamingSessionParams, ID_LENGTH_MAX};
use crate::models::responses::{GameCreationResult, GameInfo, GamingSessionCreationResult};
use crate::tic_tac_toe_game::TicTacToeGame;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::debug;
use std::sync::Mutex;
use validator::Validate;


/// Creates a new Game. Returns Game Creation Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/games",
    responses(
    (status = 200, description = "Game created successfully", body = GameCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewGameParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/games")]
pub(crate) async fn create_game(
    new_game_params: web::Json<NewGameParams>,
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResult>> {
    //

    debug!("HTTP POST to /games. Params: {:?}", new_game_params);

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().unwrap();

    match games_manager.create_new_game(&new_game_params).await {
        Ok(game) => {
            let new_game_info = GameCreationResult {
                game_info: GameInfo::from(game.clone()),
            };
            Ok(web::Json(new_game_info))
        }
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Creates a new Game. Returns Game Creation Result.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions",
    responses(
    (status = 200, description = "Gaming Session created successfully", body = NewGamingSessionParams, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed NewGamingSessionParams"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions")]
pub(crate) async fn create_session(
    new_game_params: web::Json<NewGamingSessionParams>,
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResult>> {
    //

    debug!("HTTP POST to /gaming-sessions. Params: {:?}", new_game_params);

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().unwrap();

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
    game_id: web::Path<String>,
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> HttpResponse {
    //

    debug!("HTTP DELETE to /games/game/{}", game_id);

    // *** Validate input params ***
    if game_id.is_empty() {
        return HttpResponse::BadRequest().body("Game ID is empty");
    } else if game_id.len() as u64 > ID_LENGTH_MAX {
        return HttpResponse::BadRequest().body("Game ID exceeds maximum length");
    }

    // TODO: JD: only allow Players who is part of the Game to end the Game.

    match games_manager.lock().unwrap().end_game(&game_id) {
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
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<Vec<GameState>>> {
    //

    debug!("HTTP GET to /games/{}/turns", game_id);

    // *** Validate input params ***
    validate_game_id(&game_id)?;

    match games_manager.lock().unwrap().get_game_history(&game_id) {
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
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameInfo>> {
    //

    debug!("HTTP GET to /games/{}", game_id);

    // *** Validate input params ***
    validate_game_id(&game_id)?;

    match games_manager
        .lock()
        .unwrap()
        .get_game_by_id(game_id.as_str())
    {
        Ok(game) => match GameInfo::try_from(game) {
            Ok(game_info) => Ok(web::Json(game_info)),
            Err(error) => { Err(actix_web::error::ErrorNotFound(error.to_string())) }
        },
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

/// Adds a Player to the Gaming Session.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/game/players",
    responses(
    (status = 200, description = "Player added to the Game"),
    (status = 400, description = "Bad request - Malformed AddPlayerParams"),
    (status = 404, description = "No Game found for the specified Invitation"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/game/players")]
pub(crate) async fn join_game(
    params: web::Json<JoinGameParams>,
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> impl Responder {
    //

    debug!("HTTP POST to /game/players. Params: {:?}", params);

    // *** Validate input params ***
    if let Err(e) = params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().unwrap();
    let params = params.into_inner();

    match games_manager.add_player_to_game(&params).await {
        Ok(_) => Ok(HttpResponse::Ok().finish()),
        Err(error) => { Err(error.into()) }
    }
}

/// Adds a Player to the Gaming Session.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/gaming-sessions/players",
    responses(
    (status = 200, description = "Player added to the Gaming Session", body = GamingSessionAdditionResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed AddPlayerParams"),
    (status = 404, description = "No Game found for the specified Invitation"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/gaming-sessions/players")]
pub(crate) async fn join_session(
    params: web::Json<JoinSessionParams>,
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GamingSessionCreationResult>> {
    //

    debug!("HTTP POST to /gaming-sessions/players. Params: {:?}", params);

    // *** Validate input params ***
    if let Err(e) = params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().unwrap();
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
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
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
        .unwrap()
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
