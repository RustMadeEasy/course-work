use bevy::prelude::{Component, Resource};

use crate::game_play::{GRID_COLUMNS, GRID_ROWS};
use crate::shared::local_models::local_game_piece::LocalGamePiece;
use crate::shared::local_models::local_grid_position::LocalGridPosition;
use crate::shared::local_models::local_player_info::LocalPlayerInfo;
use crate::shared::local_models::local_player_status::LocalPlayStatus;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author Joel@RustMadeEasy.com

/// Holds the local rendition of the remote Game Play state.
#[derive(Clone, Default, Resource)]
pub(crate) struct LocalGameStateResource {
    //
    /// Specifies the Player currently taking their turn.
    pub current_player: Option<LocalPlayerInfo>,

    /// Specifies the locations of the Game pieces
    pub(crate) game_board: [[LocalGamePiece; GRID_ROWS]; GRID_COLUMNS],

    /// Indicates whether the Game has ended.
    pub(crate) game_ended: bool,

    /// Remembers the ID of the Game. This is used for calls to the GameInfoService.
    pub(crate) game_id: String,

    /// Indicates whether the Game has been started.
    pub(crate) game_has_started: bool,

    /// The code used to invite a new Player to the Game.
    pub(crate) invitation_code: String,

    pub(crate) play_status: LocalPlayStatus,

    pub(crate) id_of_player_who_made_the_last_move: String,

    /// If/when the Game has been won, winning_Player_name contains the name of the Player who won
    /// the Game.
    pub(crate) winning_player_name: Option<String>,

    /// If/when the Game has been won, winning_locations lists the locations of the winning
    /// Game pieces.
    pub(crate) winning_locations: Option<Vec<LocalGridPosition>>,
}

impl LocalGameStateResource {
    //

    pub(crate) fn get_game_id(&self) -> String {
        self.game_id.clone()
    }

    pub(crate) fn get_game_piece_at_placement(
        &self,
        position: &LocalGridPosition,
    ) -> LocalGamePiece {
        self.game_board[position.row][position.column].clone()
    }

    pub(crate) fn get_winning_location(&self) -> Option<Vec<LocalGridPosition>> {
        self.winning_locations.clone()
    }

    pub(crate) fn has_game_ended(&self) -> bool {
        self.game_ended
    }

    pub(crate) fn has_game_started(&self) -> bool {
        self.game_has_started
    }
}

impl LocalGameStateResource {
    //

    /// Generates Game completion text.
    pub(crate) fn generate_results(
        &self,
        local_game_state: &LocalGameStateResource,
        local_player_name: &String,
        winning_player_name: &Option<String>,
    ) -> String {
        // TODO: JD: localize the text
        match local_game_state.play_status {
            LocalPlayStatus::EndedInStalemate => "This game has ended in a stalemate.".to_string(),
            LocalPlayStatus::EndedInWin => {
                let winning_player_name = winning_player_name.clone().unwrap_or_default();
                if *local_player_name == winning_player_name {
                    "You won!".to_string()
                } else {
                    format!("{winning_player_name} won. Better luck next time.")
                }
            }
            _ => "".to_string(),
        }
    }

    /// Clears out all data fields.
    pub(crate) fn reset(&mut self) {
        *self = LocalGameStateResource::default();
    }
}

#[allow(clippy::from_over_into)]
impl Into<String> for LocalGamePiece {
    fn into(self) -> String {
        match self {
            LocalGamePiece::Unoccupied => "",
            LocalGamePiece::O => "O",
            LocalGamePiece::X => "X",
        }
        .to_string()
    }
}

#[derive(Component)]
pub(crate) struct Player {}
