// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::models::board_position::BoardPosition;
use crate::models::play_status::PlayStatus;
use crate::models::player_info::PlayerInfo;
use function_name::named;
use log::debug;

/// Models the outcome of a Game turn (play). See GameState::determine_outcome_of_play().
pub(crate) struct PlayOutcome {
    pub(crate) play_status: PlayStatus,
    pub(crate) winning_player: Option<PlayerInfo>,
    pub(crate) winning_position: Option<Vec<BoardPosition>>,
}

impl PlayOutcome {
    //

    /// Creates a new instance.
    #[named]
    pub(crate) fn new(play_status: &PlayStatus) -> Self {
        debug!("{} called", function_name!());
        Self {
            play_status: play_status.clone(),
            winning_player: None,
            winning_position: None,
        }
    }

    /// Creates a new instance with details regarding the win.
    #[named]
    pub(crate) fn new_with_win_details(
        play_status: &PlayStatus,
        winning_position: &[BoardPosition],
        winning_player: &PlayerInfo,
    ) -> Self {
        debug!("{} called", function_name!());
        Self {
            play_status: play_status.clone(),
            winning_player: Some(winning_player.clone()),
            winning_position: Some(winning_position.to_vec()),
        }
    }
}
