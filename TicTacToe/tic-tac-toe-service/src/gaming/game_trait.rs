// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::errors::GameError;
use crate::models::game_mode::GameMode;
use crate::models::game_state::GameState;
use crate::models::player_info::PlayerInfo;
use crate::models::requests::GameTurnParams;
use crate::models::responses::TurnResponse;
use chrono::{DateTime, Utc};

/// Defines the general behavior of a Game.
pub(crate) trait GameTrait: Sized {
    //

    /// Adds a Player to the Game.
    fn add_player(&mut self, player: &PlayerInfo) -> Result<(), GameError>;

    /// Property accessor for the current state of the Game.
    fn get_current_game_state(&self) -> GameState;

    /// Returns the number of Players.
    fn get_player_count(&self) -> i8;

    /// Returns the Player who can currently make a Game move.
    fn get_current_player(&self) -> Option<PlayerInfo>;

    /// Property accessor for the Game Mode.
    fn get_game_mode(&self) -> GameMode;

    /// Property accessor for the Game ID.
    fn get_id(&self) -> String;

    /// Property accessor for the Game Play History.
    fn get_play_history(&self) -> Vec<GameState>;

    /// Property accessor for the specified Player.
    fn get_player_info_by_id(&self, player_id: impl Into<String>) -> Result<PlayerInfo, GameError>;

    /// Property accessor for the date/time of the Game's latest move.
    fn get_time_of_latest_move(&self) -> Option<DateTime<Utc>>;

    /// Creates a new Game instance.
    fn new(game_mode: GameMode,
           initial_player: &PlayerInfo,
           other_player: Option<PlayerInfo>,
           session_id: &str) -> Result<Self, GameError>;

    /// Make a Game move for the specified Player.
    fn take_turn(&mut self, game_turn_info: &GameTurnParams) -> Result<TurnResponse, GameError>;
}
