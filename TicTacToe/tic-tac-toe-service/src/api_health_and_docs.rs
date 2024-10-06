// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::api_gaming::*;
use crate::game_board::BoardPosition;
use crate::game_board::GamePiece;
use crate::game_state::GameState;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::event_plane::EventPlaneTopicNames;
use crate::models::requests::AddPlayerParams;
use crate::models::requests::GameMode;
use crate::models::requests::GameTurnInfo;
use crate::models::requests::NewGameParams;
use crate::models::responses::GameCreationResult;
use crate::models::responses::GameInfo;
use crate::models::AutomaticPlayerSkillLevel;
use crate::models::PlayerInfo;
use crate::play_status::PlayStatus;
use actix_web::{get, web, HttpResponse};
use log::debug;
use utoipa::openapi::ContactBuilder;
use utoipa::OpenApi;

/**
 * Defines and implements the public Health and Docs contracts for this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

/// Generates the OpenAPI3 docs.
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        add_player,
        create_game,
        end_game,
        get_game_history,
        get_game_info,
        take_turn
    ),
    components(schemas(
        AddPlayerParams,
        BoardPosition,
        EventPlaneConfig,
        EventPlaneTopicNames,
        GameCreationResult,
        GameInfo,
        GameMode,
        GamePiece,
        GameState,
        GameTurnInfo,
        NewGameParams,
        PlayerInfo,
        PlayStatus,
        AutomaticPlayerSkillLevel,
    ))
)]
pub(crate) struct ApiDoc;

/// Responds with the OpenAPI specification of this Service. This can be used to create, for
/// instance, PostMan Collections, Client SDKs, etc.
#[get("/api-docs")]
pub(crate) async fn api_docs() -> actix_web::Result<String> {
    //

    debug!("HTTP GET to /api_docs");

    let mut doc = ApiDoc::openapi();
    doc.info.title = "Tic-Tac-Toe Service".to_string();
    doc.info.contact = Some(
        ContactBuilder::new()
            .name(Some("Support"))
            .url(Some("https://RustMadeEasy.com"))
            .email(Some("JoelDavisEngineering@Gmail.com"))
            .build(),
    );
    let json = doc.to_json()?.to_string();
    Ok(json)
}

/// Responds with the health of the Service.
#[get("/health")]
pub(crate) async fn health() -> HttpResponse {
    //

    debug!("HTTP GET to /health");

    HttpResponse::Ok().json(web::Json("Up"))
}
