// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::game_board::{MAX_BOARD_COLUMNS, MAX_BOARD_ROWS};
use crate::models::game_piece::GamePiece;
use crate::models::play_status::PlayStatus;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Models the state of a Game at a particular Move (turn).
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct GameState {
    //

    /// The date/time at which this Game State was created. This is used for cleanup of abandoned Games.
    #[serde(skip)]
    pub(crate) created_date: DateTime<Utc>,

    /// ID of the Player who made the Move that brought about this Game State.
    pub(crate) id_of_player_who_made_move: String,

    /// Specifies the layout of the Game Pieces for this particular Game State.
    pub(crate) game_board: [[GamePiece; MAX_BOARD_ROWS]; MAX_BOARD_COLUMNS],

    /// The current status of the Game.
    pub(crate) play_status: PlayStatus,
}

// Initialization
impl GameState {
    //

    /// Creates a new GameState instance.
    pub(crate) fn new() -> Self {
        Self {
            created_date: Utc::now(),
            id_of_player_who_made_move: "".to_string(),
            game_board: Default::default(),
            play_status: PlayStatus::NotStarted,
        }
    }

    /// Creates an initial GameState instance using a Player and a PlayStatus.
    pub(crate) fn new_with_initial_play_status(
        current_player_id: &str,
        play_status: &PlayStatus,
    ) -> Self {
        Self {
            created_date: Utc::now(),
            id_of_player_who_made_move: current_player_id.to_string(),
            game_board: Default::default(),
            play_status: play_status.clone(),
        }
    }
}