// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::errors::GameError;
use crate::gaming::game_trait::GameTrait;
use crate::models::game_mode::GameMode;
use crate::models::game_piece::GamePiece;
use crate::models::game_state::GameState;
use crate::models::play_status::PlayStatus;
use crate::models::player_info::PlayerInfo;
use crate::models::requests::GameTurnParams;
use crate::models::responses::TurnResponse;
use chrono::{DateTime, Utc};
use log::debug;
use serde::Serialize;
use uuid::Uuid;

/**
 * Provides Tic-Tac-Toe Game play functionality.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

/// Provides Tic-Tac-Toe Game play functionality.
#[derive(Clone, Serialize)]
pub(crate) struct TicTacToeGame {
    //

    /// The Player who can currently make a Game move
    pub(crate) current_player: Option<PlayerInfo>,

    /// Indicates whether this is a Single-Player or Two-Player Game.
    pub(crate) game_mode: GameMode,

    /// Unique ID of the Game
    pub(crate) id: String,

    /// Holds the results of the latest turn taken.
    pub(crate) latest_turn_result: Option<TurnResponse>,

    /// The list of Game States from the very first turn until the latest turn
    pub(crate) play_history: Vec<GameState>,

    /// The list of Players engaged in the Game
    pub(crate) players: Vec<PlayerInfo>,
}

impl TicTacToeGame {
    //

    /// Sets up the players for the first turn.
    fn begin(&mut self) {
        //

        debug!("TicTacToeGame::begin()");

        let mut player = self.players.first().unwrap().clone();
        let mut other_player = self.players.last().unwrap().clone();

        // Randomly assign the game piece for each player
        player.game_piece = GamePiece::new_with_random_choice();
        other_player.game_piece = GamePiece::new_as_opposite(&player.game_piece);

        // By convention, whoever has X starts first.
        let starting_player = if player.game_piece == GamePiece::X {
            player.clone()
        } else {
            other_player.clone()
        };

        // Update the list
        self.players.clear();
        self.players.push(player.clone());
        self.players.push(other_player.clone());

        self.current_player = Some(starting_player);
    }
}

impl GameTrait for TicTacToeGame {

    //

    fn add_player(&mut self, player: &PlayerInfo) -> Result<(), GameError> {
        //

        if self.players.len() >= 2 {
            return Err(GameError::GameHasMaximumNumberOfPlayers);
        }

        self.players.push(player.clone());

        if self.get_player_count() == 2 {
            self.begin();
        }

        Ok(())
    }

    /// Returns the current state of the Game Board.
    fn get_current_game_state(&self) -> GameState {
        //

        if !self.play_history.is_empty() {
            self.play_history.last().unwrap().clone()
        } else {
            //

            // This is a brand-new Game...

            // If we have both Players, then the Game has begun!
            if self.players.len() > 1 {
                GameState::new_with_initial_play_status(
                    &self.players.first().unwrap().player_id,
                    &PlayStatus::InProgress,
                )
            } else {
                GameState::new()
            }
        }
    }

    fn get_player_count(&self) -> i8 {
        self.players.len() as i8
    }

    fn get_current_player(&self) -> Option<PlayerInfo> {
        self.current_player.clone()
    }

    fn get_game_mode(&self) -> GameMode {
        self.game_mode.clone()
    }

    /// Returns the ID of this Game.
    fn get_id(&self) -> String {
        self.id.clone()
    }

    /// Returns the Game Play History.
    fn get_play_history(&self) -> Vec<GameState> {
        self.play_history.clone()
    }

    /// Returns the specified Player.
    fn get_player_info_by_id(&self, player_id: impl Into<String>) -> Result<PlayerInfo, GameError> {
        let player_id = player_id.into();
        match self.players.iter().find(|it| it.player_id == player_id) {
            None => Err(GameError::PlayerNotFound),
            Some(player) => Ok(player.clone()),
        }
    }

    fn get_time_of_latest_move(&self) -> Option<DateTime<Utc>> {
        self.play_history.last().map(|game_state| game_state.created_date)
    }

    /// Creates a new Game instance. Note that the begin() function must be called before game play can commence.
    fn new(game_mode: GameMode, player: &PlayerInfo, other_player: Option<PlayerInfo>, session_id: &str) -> Result<Self, GameError> {
        //

        debug!("TicTacToeGame::new() Creating new game. Session ID: {:?} with Player: {:?}", session_id, player);

        let mut game = Self {
            current_player: None,
            game_mode,
            id: Uuid::new_v4().to_string(),
            latest_turn_result: None,
            players: vec![player.clone()],
            play_history: vec![],
        };

        if let Some(other_player) = other_player {
            game.add_player(&other_player)?;
        }

        Ok(game)
    }

    /// Make a Game move for the specified Player.
    fn take_turn(&mut self, game_turn_info: &GameTurnParams) -> Result<TurnResponse, GameError> {
        //

        debug!("TicTacToeGame: taking game turn. Params: {:?}", game_turn_info);

        let board_state = self.get_current_game_state();

        // Do not allow Game moves when the Game has already been completed.
        if board_state.has_ended() {
            return Err(GameError::GameHasAlreadyEnded);
        }

        // Make sure the Game has begun.
        if self.current_player.is_none() {
            return Err(GameError::GameNotStarted);
        }

        // Make sure the position is valid.
        if !GameState::is_valid_board_position(&game_turn_info.destination) {
            return Err(GameError::InvalidBoardPosition);
        }

        // Make sure that the target location is not already occupied.
        if board_state.get_game_board()[game_turn_info.destination.row][game_turn_info.destination.column] != GamePiece::Unselected {
            return Err(GameError::BoardLocationAlreadyOccupied);
        }

        // Get the Player - also validating that the correct IDs have been sent in.
        let player_taking_a_turn = self.get_player_info_by_id(&game_turn_info.player_id)?;

        // Ensure that the Player is not making a move out of turn.
        if player_taking_a_turn.player_id != self.current_player.clone().unwrap().player_id {
            return Err(GameError::WrongPlayerTakingTurn);
        }

        // Load the other Player.
        let other_player = PlayerInfo::get_other_player_info(&game_turn_info.player_id, &self.players)?;

        // Take the turn and make a new Board State by adding the specified piece to the board of
        // the current Board State.
        let final_board_state = board_state.place_game_piece(
            &game_turn_info.destination,
            &player_taking_a_turn,
            &other_player,
        )?;

        // Add this move to our Game Play History
        self.play_history.push(final_board_state.new_game_state.clone());
        self.latest_turn_result = Some(final_board_state.clone());

        // Change Players
        self.current_player = Some(other_player);

        Ok(final_board_state.clone())
    }
}
