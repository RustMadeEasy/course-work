use std::sync::Mutex;

use actix_web::{delete, get, HttpResponse, post, web};

use crate::game_engine::GameEngine;
use crate::games_manager::GamesManager;
use crate::models::requests::{AddPlayerParams, GameTurnInfo, NewGameParams};
use crate::models::responses::GameInfo;

/**
 * Defines and implements the public Gaming contract for this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Joel@RustMadeEasy.com
 */

/// Adds a Player to the Game. Returns the Game Info.
#[utoipa::path(
post,
tag = "TicTacToe",
path = "/v1/games/players",
responses(
    (status = 200, description = "Player added to the Game", body = GameInfo, content_type = "application/json"),
    (status = 404, description = "No Game found for the specified Invitation")
,),)]
#[post("/games/players")]
pub(crate) async fn add_player(
    second_player_params: web::Json<AddPlayerParams>,
    games_manager: web::Data<Mutex<GamesManager<GameEngine>>>,
) -> HttpResponse {
    match games_manager
        .lock()
        .unwrap()
        .add_player(&second_player_params.into_inner())
    {
        Ok(game_engine) => match GameInfo::try_from(game_engine) {
            Ok(game_info) => HttpResponse::Ok().json(game_info),
            Err(error) => HttpResponse::from_error(error),
        },
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Creates a new Game. Returns the Game Info.
#[utoipa::path(
post,
tag = "TicTacToe",
path = "/v1/games",
responses(
    (status = 200, description = "Game created successfully", body = GameInfo, content_type = "application/json"),
    (status = 400, description = "Bad request")
,),)]
#[post("/games")]
pub(crate) async fn create_game(
    params: web::Json<NewGameParams>,
    games_manager: web::Data<Mutex<GamesManager<GameEngine>>>,
) -> HttpResponse {
    match games_manager.lock().unwrap().create_game_engine(&params) {
        Ok(game_engine) => match GameInfo::try_from(game_engine) {
            Ok(game_info) => HttpResponse::Ok().json(game_info),
            Err(error) => HttpResponse::from_error(error),
        },
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Closes down the specified Game.
#[utoipa::path(
delete,
tag = "TicTacToe",
path = "/v1/games/{game_id}",
responses(
    (status = 200, description = "Game ended successfully"),
    (status = 404, description = "Game not found")
,),)]
#[delete("/games/{game_id}")]
pub(crate) async fn end_game(
    game_id: web::Path<String>,
    games_manager: web::Data<Mutex<GamesManager<GameEngine>>>,
) -> HttpResponse {
    match games_manager.lock().unwrap().end_game(&game_id) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Retrieves the history of the Game States from the initial creation to the current
/// Game State. This can be used, for instance, for the client to provide an animation that
/// shows a time-lapse of the game play.
#[utoipa::path(
get,
tag = "TicTacToe",
path = "/v1/games/{game_id}/turns",
responses(
    (status = 200, description = "Game history retrieved successfully", body = Vec < GameState >, content_type = "application/json"),
    (status = 404, description = "Game not found")
,),)]
#[get("/games/{game_id}/turns")]
pub(crate) async fn get_game_history(
    game_id: web::Path<String>,
    games_manager: web::Data<Mutex<GamesManager<GameEngine>>>,
) -> HttpResponse {
    match games_manager.lock().unwrap().get_game_history(&game_id) {
        Ok(history) => HttpResponse::Ok().json(history),
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Retrieves the specified Game info.
#[utoipa::path(
get,
tag = "TicTacToe",
path = "/v1/games/{game_id}",
params(("game_id" = String, Path, description = "Game ID"),),
responses(
    (status = 200, description = "Game info retrieved successfully", body = GameInfo, content_type = "application/json"),
    (status = 404, description = "Game not found")
,),)]
#[get("/games/{game_id}")]
pub(crate) async fn get_game_info(
    game_id: web::Path<String>,
    games_manager: web::Data<Mutex<GamesManager<GameEngine>>>,
) -> HttpResponse {
    match games_manager
        .lock()
        .unwrap()
        .get_game_engine(game_id.into_inner())
    {
        Ok(game_engine) => match GameInfo::try_from(game_engine) {
            Ok(game_info) => HttpResponse::Ok().json(game_info),
            Err(error) => HttpResponse::from_error(error),
        },
        Err(error) => HttpResponse::from_error(error),
    }
}

/// Make a game move for the specified Player.
#[utoipa::path(
post,
tag = "TicTacToe",
path = "/v1/games/{game_id}/turns",
responses(
    (status = 200, description = "Game turn added successfully"),
    (status = 400, description = "Bad Request - Invalid Board Position"),
    (status = 404, description = "Not Found - Game Not Found"),
    (status = 405, description = "Method Not Allowed - Wrong Player Taking Turn"),
    (status = 406, description = "Not Acceptable - Game Has Already Ended"),
    (status = 409, description = "Conflict - Board Location Already Occupied"),
),)]
#[post("/games/{game_id}/turns")]
pub(crate) async fn take_turn(
    game_id: web::Path<String>,
    game_turn_info: web::Json<GameTurnInfo>,
    games_manager: web::Data<Mutex<GamesManager<GameEngine>>>,
) -> HttpResponse {
    match games_manager
        .lock()
        .unwrap()
        .take_turn(&game_id, &game_turn_info)
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(error) => HttpResponse::from_error(error),
    }
}
