use crate::gaming::board_position::BoardPosition;
use crate::models::AutomaticPlayerSkillLevel;
use crate::models::INVITATION_CODE_LENGTH;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

pub const ID_LENGTH_MAX: u64 = 36;
const ID_LENGTH_MIN: u64 = 1;
const NAME_LENGTH_MAX: u64 = 40;
const NAME_LENGTH_MIN: u64 = 1;

/// Models info needed to end a Game.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct EndGameParams {
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub player_id: String,
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub session_id: String,
}

/// Models info needed to end a Gaming Session.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct EndGamingSessionParams {
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub player_id: String,
}

/// Models info needed to perform a Game turn.
#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
pub struct GameTurnInfo {
    pub destination: BoardPosition,
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub player_id: String,
    #[validate(length(min = "ID_LENGTH_MIN", max = "ID_LENGTH_MAX"))]
    pub session_id: String,
}

/// Models info needed to join a Gaming Session.
#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct JoinSessionParams {
    #[validate(length(min = "INVITATION_CODE_LENGTH", max = "INVITATION_CODE_LENGTH"))]
    pub game_invitation_code: String,
    #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
    pub player_display_name: String,
}

/// Models info needed to start a new Gaming Session.
#[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
pub struct NewGamingSessionParams {
    #[validate(length(min = "NAME_LENGTH_MIN", max = "NAME_LENGTH_MAX"))]
    pub session_owner_display_name: String,
}

/// Models info needed to start a new Single-Player Game.
#[derive(Clone, Debug, Deserialize, ToSchema, Validate)]
pub struct NewSinglePlayerGameParams {
    pub computer_skill_level: AutomaticPlayerSkillLevel,
}