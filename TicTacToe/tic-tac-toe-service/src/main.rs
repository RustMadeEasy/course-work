// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

extern crate core;

use std::net::Ipv4Addr;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use chrono::{Datelike, Utc};
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_gaming::{create_gaming_session, create_single_player_game, create_two_player_game, end_game, end_gaming_session, get_game_history, get_game_info, join_gaming_session, take_turn};
use crate::api_health_and_docs::{api_docs, health, ApiDoc};
use crate::games_manager::TicTacToeGamesManager;

mod api_gaming;
mod api_health_and_docs;
mod errors;
mod game_board;
mod tic_tac_toe_game;
mod game_state;
mod game_trait;
mod games_manager;
mod models;
mod play_outcome;
mod play_status;
mod tests;
mod auto_player;
mod game_observer_trait;
mod game_updates_publisher;
mod game_session;

/// The HTTP port through which this service is accessed.
const PORT: u16 = 50020;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //

    // Get the logger setup
    env_logger::init();

    // Nothing wrong with a fun startup banner
    print_startup_banner();

    info!("Launched on port: {PORT}");

    // This is our global Games Manager instance. Below, we add the Game Manager to the Actix app
    // data storage so that it is accessible to service methods.
    let games_manager = Data::new(tokio::sync::Mutex::new(TicTacToeGamesManager::new()));

    HttpServer::new(move || {
        App::new().app_data(games_manager.clone()).service(
            web::scope("/v1")
                // *** Gaming API ***
                .service(create_single_player_game)
                .service(create_two_player_game)
                .service(create_gaming_session)
                .service(end_game)
                .service(end_gaming_session)
                .service(get_game_history)
                .service(get_game_info)
                .service(join_gaming_session)
                .service(take_turn)
                // *** Health & Docs API ***
                .service(api_docs)
                .service(health)
                .service(
                    // Open up access to SwaggerUI
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/v1/api-docs", ApiDoc::openapi()),
                ),
        )
    })
        .bind((Ipv4Addr::UNSPECIFIED, PORT))?
        .run()
        .await
}

/// Prints a cool startup banner to the logging facility.
fn print_startup_banner() {
    info!(
        "
  _______ _   _______      _______                _____                 _
 |__   __(_) |__   __|    |__   __|              / ____|               (_)
    | |   _  ___| | __ _  ___| | ___   ___      | (___   ___ _ ____   ___  ___ ___
    | |  | |/ __| |/ _` |/ __| |/ _ \\ / _ \\      \\___ \\ / _ \\ '__\\ \\ / / |/ __/ _ \\
    | |  | | (__| | (_| | (__| | (_) |  __/      ____) |  __/ |   \\ V /| | (_|  __/
    |_|  |_|\\___|_|\\__,_|\\___|_|\\___/ \\___|     |_____/ \\___|_|    \\_/ |_|\\___\\___|\
    \r\n
    © {} Rust Made Easy. All rights reserved.\r\n
", Utc::now().year() );
}
