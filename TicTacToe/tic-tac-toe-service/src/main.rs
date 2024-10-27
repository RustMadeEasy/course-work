// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

extern crate core;

use std::net::Ipv4Addr;

use crate::api::docs::*;
use crate::api::games::*;
use crate::api::gaming_session::*;
use crate::api::health::get_health;
use crate::gaming::gaming_sessions_manager::GamingSessionsManager;
use crate::gaming::tic_tac_toe_game::TicTacToeGame;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use chrono::{Datelike, Utc};
use log::info;
use tokio::sync::Mutex;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod errors;
mod tests;
pub(crate) mod api;
mod gaming;
mod models;

/// The HTTP port through which this service is accessed.
const PORT: u16 = 50020;

/// This is the entry point for the Service
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
    let manager = Data::new(Mutex::new(GamingSessionsManager::<TicTacToeGame>::new()));

    HttpServer::new(move || {
        App::new().app_data(manager.clone()).service(
            web::scope("/v1")
                // *** Gaming API ***
                .service(create_gaming_session)
                .service(create_single_player_game)
                .service(create_two_player_game)
                .service(end_game)
                .service(end_gaming_session)
                .service(get_game_history)
                .service(get_latest_game_turn)
                .service(get_session_current_game)
                .service(join_current_game)
                .service(join_gaming_session)
                .service(take_turn)
                // *** Health & Docs API ***
                .service(api_docs)
                .service(get_health)
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
