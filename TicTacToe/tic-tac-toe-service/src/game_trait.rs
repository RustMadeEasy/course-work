use crate::errors::GameError;
use crate::game_state::GameState;
use crate::models::PlayerInfo;
use crate::models::requests::{GameTurnInfo, NewGameParams};

/**
 * Defines the behavior of a Game.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Info@RustMadeEasy.com
 */

/// Defines the behavior of a Game.
pub(crate) trait GameTrait: Sized {
    //

    // TODO: HD: generalize GameState, GameTurnInfo, NewGameParams, and PlayerInfo.

    /// Adds a Player to the Game.
    fn add_player(&mut self, display_name: impl Into<String> + Copy) -> Result<(), GameError>;

    /// Determines whether the specified Player can take a turn.
    fn _can_player_take_turn(&self, player: &PlayerInfo) -> bool;

    /// Returns the current state of the Game Board.
    fn get_current_game_state(&self) -> GameState;

    /// Returns the Event Channel ID
    fn get_event_channel_id(&self) -> String;

    /// Returns the Game ID
    fn get_id(&self) -> String;

    /// Returns the Game Invitation Code. This code is used to add a new client app to the Game.
    fn get_invitation_code(&self) -> String;

    /// Returns the Game Play History.
    fn get_play_history(&self) -> Vec<GameState>;

    /// Returns the specified Player.
    fn get_player_info_by_id(&self, player_id: impl Into<String>) -> Result<PlayerInfo, GameError>;

    /// Creates a new Game instance.
    fn new(params: &NewGameParams,
           mqtt_broker_address: impl Into<String>,
           mqtt_port: u16,
           invitation_code: impl Into<String>) -> Result<Self, GameError>;

    /// Make a game move for the specified Player.
    fn take_turn(&mut self, game_turn_info: &GameTurnInfo) -> Result<GameState, GameError>;
}
