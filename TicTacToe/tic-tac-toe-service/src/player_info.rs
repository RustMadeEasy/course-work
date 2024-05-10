use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::errors::GameError;
use crate::game_board::GamePiece;

/**
 * Models a Tic-Tac-Toe game Player.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Joel@RustMadeEasy.com
 */

/// Models a Tic-Tac-Toe game Player.
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct PlayerInfo {
    pub(crate) display_name: String,
    pub(crate) game_piece: GamePiece,
    pub(crate) player_id: String,
    #[serde(skip)]
    _outside_instantiation_preventor: PhantomData<u8>,
}

impl PlayerInfo {
    //

    /// Returns the Player other than the specified Player.
    pub(crate) fn get_other_player_info_by_id(
        player_id: impl Into<String>,
        players: &[PlayerInfo],
    ) -> Result<PlayerInfo, GameError> {
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
    pub(crate) fn new(display_name: impl Into<String>, game_piece: &GamePiece) -> Self {
        Self {
            display_name: display_name.into(),
            player_id: Uuid::new_v4().to_string(),
            game_piece: game_piece.clone(),
            _outside_instantiation_preventor: Default::default(),
        }
    }
}
