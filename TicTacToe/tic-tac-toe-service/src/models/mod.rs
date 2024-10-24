// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/**
 * Models used in API requests and responses and in MQTT notifications.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

use crate::errors::GameError;
use crate::gaming::game_board::{MAX_BOARD_COLUMNS, MAX_BOARD_ROWS};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;


pub(crate) mod requests;
pub(crate) mod responses;
pub(crate) mod event_plane;

const INVITATION_CODE_LENGTH: u64 = 6;

#[derive(Clone, Debug, Default, Deserialize, ToSchema)]
pub enum AutomaticPlayerSkillLevel {
    /// Performs random moves.
    #[default]
    Beginner,
    /// Takes best tactical move.
    Intermediate,
    /// Takes the best strategic moves, looking several moves into the future.
    Expert,
    /// Expands on the expert level by also making moves that can trick the other player into making
    /// wrong moves.
    Master,
}

/// Models a position on the Game board.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema, Validate)]
pub(crate) struct BoardPosition {
    #[validate(range(min = 0, max = 2))]
    pub(crate) row: usize,
    #[validate(range(min = 0, max = 2))]
    pub(crate) column: usize,
}

impl BoardPosition {
    /// Creates a new BoardPosition instance.
    pub(crate) fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

/// Specifies the type of Game - single player or two players.
#[derive(Debug, Deserialize, PartialEq, Serialize, ToSchema, Clone)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayers,
}

/// Models a Game Piece with which the Tic-Tac-Toe Game is played.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum GamePiece {
    #[default]
    Unselected,
    X,
    O,
}

impl GamePiece {
    //

    /// Selects the opposite game piece. If self is X, then O is returned. If self is O, X is
    /// returned. If self is Unselected, Unselected is returned.
    pub(crate) fn opposite(&self) -> Self {
        match self {
            GamePiece::Unselected => GamePiece::Unselected,
            GamePiece::X => GamePiece::O,
            GamePiece::O => GamePiece::X,
        }
    }

    /// Makes a random selection between the X and O game pieces.
    pub(crate) fn random_choice() -> Self {
        match rand::random::<bool>() {
            true => Self::O,
            false => Self::X,
        }
    }
}

/// Models the state of a Game at a particular Move (turn).
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct GameState {
    //

    /// The time at which this Game State was created. This is used for cleanup of abandoned Games.
    #[serde(skip)]
    pub(crate) created_date: DateTime<Utc>,

    /// ID of the Player who made this Move.
    pub(crate) id_of_player_who_made_move: String,

    /// The board on which the Game is played.
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

/// Models a Tic-Tac-Toe Game Player.
#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema, Validate)]
pub struct PlayerInfo {
    /// Name of the Player.
    pub display_name: String,
    /// The Game Piece with which the Tic-Tac-Toe Game is played.
    pub game_piece: GamePiece,
    /// Indicates that this Player's moves are automated, i.e., guided by this service.
    pub is_automated: bool,
    /// Unique ID of the Player.
    pub player_id: String,
}

impl PlayerInfo {
    //

    /// Returns the Player other than the specified Player.
    pub fn get_other_player_info_by_id(
        player_id: impl Into<String>,
        players: &[PlayerInfo],
    ) -> Result<PlayerInfo, GameError> {
        //

        if players.len() < 2 {
            return Err(GameError::PlayerNotFound);
        }

        let player_id = player_id.into();
        match players.iter().find(|it| it.player_id != player_id) {
            None => Err(GameError::PlayerNotFound),
            Some(player) => Ok(player.clone()),
        }
    }

    /// Creates a new PlayerInfo instance.
    pub fn new(display_name: impl Into<String>,
               is_automated: bool) -> Self {
        Self {
            display_name: display_name.into(),
            game_piece: GamePiece::Unselected,
            is_automated,
            player_id: Uuid::new_v4().to_string(),
        }
    }
}

/// Lists valid Game play statuses.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum PlayStatus {
    EndedInStalemate,
    EndedInWin,
    InProgress,
    #[default]
    NotStarted,
}