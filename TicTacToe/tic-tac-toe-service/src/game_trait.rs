// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::errors::GameError;
use crate::game_state::GameState;
use crate::models::requests::GameTurnInfo;
use crate::models::{GameMode, PlayerInfo};
use chrono::{DateTime, Utc};

/**
 * Defines the behavior of a Game.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

/// Defines the general behavior of a Game.
pub(crate) trait GameTrait: Sized {
    //

    /// Sets up the players for the first turn.
    fn begin(&mut self) -> Result<Self, GameError>;

    /// Returns the current state of the Game Board.
    fn get_current_game_state(&self) -> GameState;

    /// Returns the Player who can currently make a Game move.
    fn get_current_player(&self) -> Option<PlayerInfo>;

    /// Returns the Game Mode.
    fn get_game_mode(&self) -> GameMode;

    /// Returns the Game ID.
    fn get_id(&self) -> String;

    /// Returns the Game Play History.
    fn get_play_history(&self) -> Vec<GameState>;

    /// Returns the specified Player.
    fn get_player_info_by_id(&self, player_id: impl Into<String>) -> Result<PlayerInfo, GameError>;

    /// Returns the date/time of the Game's latest move.
    fn get_time_of_latest_move(&self) -> Option<DateTime<Utc>>;

    /// Creates a new Game instance.
    fn new(game_mode: GameMode,
           player: &PlayerInfo,
           other_player: &PlayerInfo,
           session_id: &str) -> Result<Self, GameError>;

    /// Make a Game move for the specified Player.
    fn take_turn(&mut self, game_turn_info: &GameTurnInfo) -> Result<GameState, GameError>;
}
