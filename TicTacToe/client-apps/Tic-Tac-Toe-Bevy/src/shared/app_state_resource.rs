//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::prelude::Resource;
use tic_tac_toe_rust_client_sdk::models;
use tic_tac_toe_rust_client_sdk::models::{BoardPosition, GamePiece, PlayStatus, PlayerInfo, TurnResponse};

/// Houses the application state variables.
#[derive(Default, Resource)]
pub(crate) struct AppStateResource {
    //

    /// ID of the Gaming Session.
    pub(crate) gaming_session_id: String,

    /// This is the code used to invite a new Player to the Game.
    pub(crate) invitation_code: String,

    // TODO: JD: we also need localPlayerInitiatedGame for when we support rematch within the same Gaming Session.
    /// Indicates that this client app instance is the one that started the Gaming Session.
    pub(crate) local_player_initiated_gaming_session: bool,

    /// The local Player, i.e. the Player using this app instance.
    pub(crate) local_player: PlayerInfo,

    /// The other Player, i.e. the local Player's opponent.
    pub(crate) other_player: Option<PlayerInfo>,

    /// The current state of the Game
    pub(crate) current_game_state: models::GameState,

    /// The Player who can take the next turn
    pub(crate) current_player: Option<PlayerInfo>,

    /// Remembers the ID of the Game. This is used for subsequent cals to the GameInfoService
    pub(crate) game_id: String,

    /// Indicates whether the Game has ended
    pub(crate) has_game_ended: bool,

    /// Indicates whether the Game has been started
    pub(crate) has_game_started: bool,

    /// Indicates that this is a Two-Player Game
    pub(crate) is_two_player_game: bool,

    /// If/when the Game has been won, winningLocations lists the locations of the winning Game pieces
    pub(crate) winning_locations: Option<Vec<BoardPosition>>,

}

impl AppStateResource {
    //

    pub(crate) fn get_game_piece_at_placement(
        &self,
        position: &BoardPosition,
    ) -> GamePiece {
        self.current_game_state.game_board[position.row as usize][position.column as usize]
    }

    /// Generates Game completion text.
    pub(crate) fn generate_results_text(
        &self,
        turn_info: &TurnResponse,
        local_player_name: &String,
        winning_player_name: &Option<String>,
    ) -> String {
        // TODO: JD: localize the text
        match turn_info.new_game_state.play_status {
            PlayStatus::EndedInStalemate => "This game has ended in a stalemate.".to_string(),
            PlayStatus::EndedInWin => {
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

    /// Clears all fields of this instance.
    pub(crate) fn reset(&mut self) {
        //

        // Grab the name
        let name = self.local_player.display_name.clone();

        *self = Self::default();

        // Set our board state to Not Started instead of the SDK default of EndedInStalemate.
        self.current_game_state.play_status = PlayStatus::NotStarted;

        // Preserve the name
        self.local_player.display_name = name;
    }
}
