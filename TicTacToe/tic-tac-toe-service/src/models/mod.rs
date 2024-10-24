use crate::errors::GameError;
use crate::gaming::game_piece::GamePiece;
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

/// Specifies the type of Game - single player or two players.
#[derive(Debug, Deserialize, PartialEq, Serialize, ToSchema, Clone)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayers,
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