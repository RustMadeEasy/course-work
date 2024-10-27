// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::models::game_piece::GamePiece;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

/// Models a Tic-Tac-Toe Game Player.
#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema, Validate)]
pub struct PlayerInfo {
    /// Name of the Player.
    pub display_name: String,
    /// The Game Piece assigned to the Player.
    pub game_piece: GamePiece,
    /// Indicates that this Player's moves are automated, i.e., guided by this service.
    pub is_automated: bool,
    /// Unique ID of the Player.
    pub player_id: String,
}

impl PlayerInfo {
    //

    /// Returns a Player from the list that is other than the specified Player - if any.
    pub fn get_other_player_info(
        player_id: impl Into<String>,
        players: &[PlayerInfo],
    ) -> Option<PlayerInfo> {
        let player_id = player_id.into();
        players.iter().find(|it| it.player_id != player_id).cloned()
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