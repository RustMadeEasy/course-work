// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::game_state::GameState;
use crate::game_trait::GameTrait;
use crate::games_manager::GamesManager;
use crate::models::requests::{AddPlayerParams, GameTurnInfo, NewGameParams, ID_LENGTH_MAX};
use crate::models::responses::{GameCreationResult, GameInfo};
use crate::tic_tac_toe_game::TicTacToeGame;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use log::debug;
use std::sync::Mutex;
use validator::Validate;

/**
 * Defines and implements the public Gaming contract for this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */


/// Adds a Player to the Game. Returns the result of the initial Game Creation.
#[utoipa::path(
    post,
    tag = "TicTacToe",
    path = "/v1/games/players",
    responses(
    (status = 200, description = "Player added to the Game", body = GameCreationResult, content_type = "application/json"),
    (status = 400, description = "Bad request - Malformed AddPlayerParams"),
    (status = 404, description = "No Game found for the specified Invitation"),
    (status = 500, description = "Internal server error")
,), )]
#[post("/games/players")]
pub(crate) async fn add_player(
    second_player_params: web::Json<AddPlayerParams>,
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> actix_web::Result<web::Json<GameCreationResult>> {
    debug!("HTTP POST to /games/players. Params: {:?}", second_player_params);

    // *** Validate input params ***
    if let Err(e) = second_player_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().unwrap();

    match games_manager.add_player(&second_player_params.into_inner()).await
    {
        Ok(game) => {
            let game_creation_result = GameCreationResult {
                game_info: GameInfo::from(game.clone()),
                event_plane_config: game.get_event_plane_config(),
                game_invitation_code: game.game_invitation_code.clone(),
            };

            Ok(web::Json(game_creation_result))
        }
        Err(error) => { Err(actix_web::error::ErrorInternalServerError(error.to_string())) }
    }
}

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
    debug!("HTTP POST to /games. Params: {:?}", new_game_params);

    // *** Validate input params ***
    if let Err(e) = new_game_params.validate() {
        return Err(actix_web::error::ErrorBadRequest(e.to_string()));
    }

    let mut games_manager = games_manager.lock().unwrap();

    match games_manager.create_game(&new_game_params).await {
        Ok(game) => {
            let new_game_info = GameCreationResult {
                game_info: GameInfo::from(game.clone()),
                event_plane_config: game.get_event_plane_config(),
                game_invitation_code: game.game_invitation_code,
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
    game_id: web::Path<String>,
    games_manager: web::Data<Mutex<GamesManager<TicTacToeGame>>>,
) -> HttpResponse {
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
    debug!("HTTP GET to /games/{}", game_id);

    // *** Validate input params ***
    validate_game_id(&game_id)?;

    match games_manager
        .lock()
        .unwrap()
        .get_game_instance(game_id.into_inner())
    {
        Ok(game) => match GameInfo::try_from(game) {
            Ok(game_info) => Ok(web::Json(game_info)),
            Err(error) => { Err(actix_web::error::ErrorNotFound(error.to_string())) }
        },
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
