// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::board_position::BoardPosition;
use crate::gaming::play_status::PlayStatus;
use crate::models::PlayerInfo;

/// Models the outcome of a Game turn (play). See GameState::determine_outcome_of_play().
pub struct PlayOutcome {
    pub play_status: PlayStatus,
    pub winning_player: Option<PlayerInfo>,
    pub winning_position: Option<Vec<BoardPosition>>,
}

impl PlayOutcome {
    //

    /// Creates a new PlayOutcome instance.
    pub(crate) fn new(play_status: &PlayStatus) -> Self {
        Self {
            play_status: play_status.clone(),
            winning_player: None,
            winning_position: None,
        }
    }

    /// Creates a new PlayOutcome instance with details regarding the win.
    pub(crate) fn new_with_win_details(
        play_status: &PlayStatus,
        winning_position: &[BoardPosition],
        winning_player: &PlayerInfo,
    ) -> Self {
        Self {
            play_status: play_status.clone(),
            winning_player: Some(winning_player.clone()),
            winning_position: Some(winning_position.to_vec()),
        }
    }
}
