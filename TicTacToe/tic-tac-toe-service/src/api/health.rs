// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use actix_web::{get, web, HttpResponse};
use log::debug;

/// Responds with the health of the Service. This intended for use in uptime monitoring.
#[get("/health")]
pub(crate) async fn get_health() -> HttpResponse {
    debug!("HTTP GET to /health");
    HttpResponse::Ok().json(web::Json("Up"))
}
