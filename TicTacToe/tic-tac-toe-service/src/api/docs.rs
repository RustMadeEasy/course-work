// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/**
 * Defines and implements the public Health and Docs contracts for this service.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

use crate::api::games::*;
use crate::api::gaming_session::*;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::event_plane::EventPlaneTopicNames;
use crate::models::requests::EndGameParams;
use crate::models::requests::EndGamingSessionParams;
use crate::models::requests::GameTurnParams;
use crate::models::requests::JoinSessionParams;
use crate::models::requests::NewGamingSessionParams;
use crate::models::requests::NewSinglePlayerGameParams;
use crate::models::responses::GameCreationResult;
use crate::models::responses::GameInfo;
use crate::models::responses::GamingSessionCreationResult;
use crate::models::responses::TurnResult;
use crate::models::AutomaticPlayerSkillLevel;
use crate::models::BoardPosition;
use crate::models::GameMode;
use crate::models::GamePiece;
use crate::models::GameState;
use crate::models::PlayStatus;
use crate::models::PlayerInfo;
use actix_web::get;
use log::debug;
use utoipa::openapi::{ContactBuilder, Tag};
use utoipa::OpenApi;


/// Generates the OpenAPI3 documentation.
#[derive(utoipa::OpenApi)]
#[openapi(
    paths(
        create_gaming_session,
        create_single_player_game,
        create_two_player_game,
        end_game,
        end_gaming_session,
        get_game_history,
        get_latest_game_turn,
        get_session_current_game,
        join_gaming_session,
        note_player_readiness,
        take_turn,
    ),
    components(schemas(
        AutomaticPlayerSkillLevel,
        BoardPosition,
        EndGameParams,
        EndGamingSessionParams,
        EventPlaneConfig,
        EventPlaneTopicNames,
        GameCreationResult,
        GameInfo,
        GameMode,
        GamePiece,
        GameState,
        GameTurnParams,
        GamingSessionCreationResult,
        JoinSessionParams,
        NewGamingSessionParams,
        NewSinglePlayerGameParams,
        PlayerInfo,
        PlayStatus,
        TurnResult,
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
    doc.info.contact = Some(
        ContactBuilder::new()
            .name(Some("Support"))
            .url(Some("https://RustMadeEasy.com"))
            .email(Some("JoelDavisEngineering@Gmail.com"))
            .build(),
    );
    doc.info.title = "Tic-Tac-Toe Service".to_string();
    doc.tags = Some(vec!(Tag::new("Tic-Tac-Toe Service")));
    let json = doc.to_json()?.to_string();
    Ok(json)
}
