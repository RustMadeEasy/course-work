// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::game_trait::GameTrait;
use crate::gaming::tic_tac_toe_game::TicTacToeGame;
use crate::models::board_position::BoardPosition;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::game_state::GameState;
use crate::models::player_info::PlayerInfo;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Models the results of a call to one of the Game creation endpoints
#[derive(Deserialize, Serialize, ToSchema)]
pub struct GameCreationResponse {
    /// The initial Game state
    pub game_info: GameInfoResponse,
    /// The Player who initiated the Gaming Session
    pub initiating_player: PlayerInfo,
    /// ID of the additional Player
    pub other_player: Option<PlayerInfo>,
    /// ID of the Gaming Session
    pub session_id: String,
}

/// Models the current view of a Game
#[derive(Deserialize, Serialize, ToSchema)]
pub struct GameInfoResponse {
    //

    /// Player who has an open turn
    pub current_player: Option<PlayerInfo>,

    /// Unique ID of the Game instance
    pub game_id: String,

    /// The current state the Game
    pub game_state: GameState,

    /// List of Players
    pub players: Vec<PlayerInfo>,
}

impl From<TicTacToeGame> for GameInfoResponse {
    fn from(game: TicTacToeGame) -> GameInfoResponse {
        let game_state = game.get_current_game_state();
        GameInfoResponse {
            current_player: game.current_player,
            game_state,
            game_id: game.id.clone(),
            players: game.players,
        }
    }
}

/// Models the results of a call to the Create Gaming Session endpoint
#[derive(Deserialize, Serialize, ToSchema)]
pub struct GamingSessionCreationResponse {
    /// Specifies the configuration required for clients to subscribe to real-time Game state updates
    pub event_plane_config: EventPlaneConfig,
    /// The Player who initiated the Gaming Session
    pub initiating_player: PlayerInfo,
    /// Unique Code that is used to invite other participants to the Gaming Session
    pub invitation_code: String,
    /// ID of the additional Player
    pub other_player: Option<PlayerInfo>,
    /// Identifies the Gaming Session. This also serves as the communication channel for MQTT notifications.
    pub session_id: String,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
/// Models the results of a call to the Get Players' Readiness end point
pub struct PlayersReadinessResponse {
    /// Indicates whether the Game's Players are ready.
    pub all_players_are_ready: bool,
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
/// Models the results of a call to the Get Latest Game Turn end point
pub struct TurnResponse {
    /// Player who will take the next turn
    pub current_player: Option<PlayerInfo>,
    /// The state of the Game after the turn has been taken
    pub new_game_state: GameState,
    /// If the Game has ended in a win, this contains the winning board positions
    pub winning_locations: Option<Vec<BoardPosition>>,
    /// If the Game has ended in a win, this indicates the winning Player
    pub winning_player: Option<PlayerInfo>,
}
