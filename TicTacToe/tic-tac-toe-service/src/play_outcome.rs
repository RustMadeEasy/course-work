// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::game_board::BoardPosition;
use crate::play_status::PlayStatus;

/// Models the outcome of a Game turn (play). See GameState::determine_outcome_of_play().
pub(crate) struct PlayOutcome {
    pub(crate) play_status: PlayStatus,
    pub(crate) winning_player_id: Option<String>,
    pub(crate) winning_position: Option<Vec<BoardPosition>>,
}

impl PlayOutcome {
    //

    /// Creates a new PlayOutcome instance.
    pub(crate) fn new(play_status: &PlayStatus) -> Self {
        Self {
            play_status: play_status.clone(),
            winning_player_id: None,
            winning_position: None,
        }
    }

    /// Creates a new PlayOutcome instance with details regarding the win.
    pub(crate) fn new_with_win_details(
        play_status: &PlayStatus,
        winning_position: &[BoardPosition],
        winning_player_id: &str,
    ) -> Self {
        Self {
            play_status: play_status.clone(),
            winning_player_id: Some(winning_player_id.to_string()),
            winning_position: Some(winning_position.to_vec()),
        }
    }
}
