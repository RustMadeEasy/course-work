// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::errors::GameError;
use crate::game_state::GameState;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::requests::{GameTurnInfo, NewGameParams};
use crate::models::PlayerInfo;
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

    // TODO: HD: generalize GameState, GameTurnInfo, NewGameParams, and PlayerInfo.

    /// Adds a Player to the Game.
    fn add_player(&mut self, display_name: impl Into<String> + Copy, is_automated: bool) -> Result<(), GameError>;

    /// Returns the current state of the Game Board.
    fn get_current_game_state(&self) -> GameState;

    /// Returns the Game ID.
    fn get_id(&self) -> String;

    /// Returns the configuration clients can use to subscribe to game change events.
    fn get_event_plane_config(&self) -> EventPlaneConfig;

    /// Returns the Game Invitation Code. This code is used to add a new client app to the Game.
    fn get_invitation_code(&self) -> String;

    /// Returns the Game Play History.
    fn get_play_history(&self) -> Vec<GameState>;

    /// Returns the specified Player.
    fn get_player_info_by_id(&self, player_id: impl Into<String>) -> Result<PlayerInfo, GameError>;

    /// Returns the date/time of the Game's latest move.
    fn get_time_of_latest_move(&self) -> Option<DateTime<Utc>>;

    /// Creates a new Game instance.
    fn new(params: &NewGameParams, invitation_code: impl Into<String>, broker_address: String, broker_port: u16) -> Result<Self, GameError>;

    /// Make a Game move for the specified Player.
    fn take_turn(&mut self, game_turn_info: &GameTurnInfo) -> Result<GameState, GameError>;
}
