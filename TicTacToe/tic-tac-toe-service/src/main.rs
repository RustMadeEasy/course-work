extern crate core;

use std::net::Ipv4Addr;
use std::sync::Mutex;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use chrono::{Datelike, Utc};
use log::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api_gaming::{
    add_player, create_game, end_game, get_game_history, get_game_info, take_turn,
};
use crate::api_health_and_docs::{api_docs, ApiDoc, health};
use crate::games_manager::TicTacToeGamesManager;

mod api_gaming;
mod api_health_and_docs;
mod errors;
mod game_board;
mod game_engine;
mod game_state;
mod game_trait;
mod games_manager;
mod models;
mod play_outcome;
mod play_status;
mod player_info;
mod tests;

/// The HTTP port through which this service is accessed.
const PORT: u16 = 50020;

// Tic-Tac-Toe Service
//
// Provides 2-client game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author Joel@RustMadeEasy.com

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //

    env_logger::init();

    print_startup_banner();
    info!("Launched on port: {PORT}");

    // This is our global Games Manager instance
    let games_manager = Data::new(Mutex::new(TicTacToeGamesManager::new()));

    HttpServer::new(move || {
        App::new().app_data(games_manager.clone()).service(
            web::scope("/v1")
                // *** Gaming API ***
                .service(add_player)
                .service(create_game)
                .service(end_game)
                .service(get_game_history)
                .service(get_game_info)
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

fn print_startup_banner() {
    let now = Utc::now();
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
",
        now.year()
    );
}
