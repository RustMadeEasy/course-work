// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::models::automatic_player_skill_level::AutomaticPlayerSkillLevel;
use crate::models::board_position::BoardPosition;
use crate::models::INVITATION_CODE_LENGTH;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub const ID_LENGTH_MAX: u64 = 36;
const ID_LENGTH_MIN: u64 = 1;
const NAME_LENGTH_MAX: u64 = 40;
const NAME_LENGTH_MIN: u64 = 1;

/// Models info needed to end a Game
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct EndGameParams {
    /// ID of one of the Players in the Gaming Session
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub player_id: String,
    /// ID of the Gaming Session in which the Game is being played
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub session_id: String,
}

/// Models info needed to end a Gaming Session
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct EndGamingSessionParams {
    /// ID of one of the Players in the Gaming Session
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub player_id: String,
}

/// Models info needed to perform a Game turn
#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct GameTurnParams {
    /// The location to which the Player's Game Piece is to be placed
    pub destination: BoardPosition,
    /// ID of the Player whose turn is being taken
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub player_id: String,
    /// ID of the Gaming Session
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub session_id: String,
}

/// Models info needed to join a Gaming Session
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct JoinSessionParams {
    /// Game Invitation Code
    #[validate(length(min = "INVITATION_CODE_LENGTH", max = "INVITATION_CODE_LENGTH"))]
    pub game_invitation_code: String,
    /// The proposed display name of the Player being added
    #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
    pub player_display_name: String,
}

/// Models info needed to start a new Gaming Session
#[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
pub struct NewGamingSessionParams {
    /// The proposed display name of the Player
    #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
    pub session_owner_display_name: String,
}

/// Models info needed to start a new Single-Player Game
#[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
pub struct NewSinglePlayerGameParams {
    /// The skill level at which the Automatic Player is to play the Game
    pub computer_skill_level: AutomaticPlayerSkillLevel,
}