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
use crate::models::automatic_player_skill_level::AutomaticPlayerSkillLevel;
use crate::models::board_position::BoardPosition;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::event_plane::EventPlaneTopicNames;
use crate::models::game_mode::GameMode;
use crate::models::game_piece::GamePiece;
use crate::models::game_state::GameState;
use crate::models::play_status::PlayStatus;
use crate::models::player_info::PlayerInfo;
use crate::models::requests::EndGameParams;
use crate::models::requests::EndGamingSessionParams;
use crate::models::requests::GameTurnParams;
use crate::models::requests::JoinSessionParams;
use crate::models::requests::NewGamingSessionParams;
use crate::models::requests::NewSinglePlayerGameParams;
use crate::models::responses::GameCreationResponse;
use crate::models::responses::GameInfoResponse;
use crate::models::responses::GamingSessionCreationResponse;
use crate::models::responses::PlayersReadinessResponse;
use crate::models::responses::TurnResponse;
use actix_web::get;
use log::debug;
use utoipa::openapi::{ContactBuilder, LicenseBuilder, Tag};
use utoipa::OpenApi;


/// Generates OpenAPI3 documentation.
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
        get_players_readiness,
        get_session_current_game,
        join_current_game,
        join_gaming_session,
        take_turn,
    ),
    components(schemas(
        AutomaticPlayerSkillLevel,
        BoardPosition,
        EndGameParams,
        EndGamingSessionParams,
        EventPlaneConfig,
        EventPlaneTopicNames,
        GameCreationResponse,
        GameInfoResponse,
        GameMode,
        GamePiece,
        GameState,
        GameTurnParams,
        GamingSessionCreationResponse,
        JoinSessionParams,
        NewGamingSessionParams,
        NewSinglePlayerGameParams,
        PlayerInfo,
        PlayersReadinessResponse,
        PlayStatus,
        TurnResponse,
    ))
)]
pub(crate) struct ApiDoc;

/// Responds with the OpenAPI specification of this Service. This can be used to create 
/// PostMan Collections, Client SDKs, etc.
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
    doc.info.license = Some(LicenseBuilder::new()
        .identifier(Some("GPL3"))
        .build()
    );
    doc.info.title = "Tic-Tac-Toe Service".to_string();
    doc.tags = Some(vec!(Tag::new("Tic-Tac-Toe Service")));
    let json = doc.to_json()?.to_string();
    Ok(json)
}
