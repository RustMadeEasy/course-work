// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use chrono::{DateTime, Utc};
use log::debug;
use serde::Serialize;
use uuid::Uuid;

use crate::errors::GameError;
use crate::errors::GameError::BoardLocationAlreadyOccupied;
use crate::game_board::{BoardPosition, GameBoard, GamePiece};
use crate::game_state::GameState;
use crate::game_trait::GameTrait;
use crate::models::requests::{GameMode, GameTurnInfo, NewGameParams};
use crate::models::PlayerInfo;
use crate::play_status::PlayStatus;

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

    /// The list of Game States from the very first turn until the latest turn
    pub(super) play_history: Vec<GameState>,

    /// The list of Players engaged in the Game
    pub(crate) players: Vec<PlayerInfo>,
}

impl TicTacToeGame {
    //

    /// Determines whether the specified board location is occupied by a Game piece.
    fn is_location_occupied(game_board: &GameBoard, position: &BoardPosition) -> bool {
        if !GameState::is_valid_board_position(position) {
            return false;
        }
        game_board[position.row][position.column] != GamePiece::None
    }
}

impl GameTrait for TicTacToeGame {
    //

    /// Adds a Player to the Game.
    fn add_player(&mut self, display_name: impl Into<String> + Copy, is_automated: bool) -> Result<(), GameError> {
        //

        debug!("TicTacToeGame: Adding player {}", display_name.into());

        let game_piece: GamePiece;

        match self.players.len() {
            0 => {
                // Add Player One
                game_piece = GamePiece::X;
            }
            1 => {
                // Add Player Two
                game_piece = GamePiece::O;

                // Makes sure the display name of the Second Player is different from that of the First Player.
                if display_name.into().to_lowercase()
                    == self.players.first().unwrap().display_name.to_lowercase()
                {
                    return Err(GameError::DisplayNameAlreadyInUseInGame);
                }
            }
            _ => {
                // Tic-Tac-Toe is a 2-Player Game. No more Players can be added.
                return Err(GameError::MaximumPlayersAlreadyAdded);
            }
        }

        let player_info = PlayerInfo::new(display_name, &game_piece, is_automated);

        self.players.push(player_info);

        // Note Player One as the first to take their turn.
        self.current_player = Some(self.players.first().unwrap().clone());

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

    fn get_current_player(&self) -> Option<PlayerInfo> {
        self.current_player.clone()
    }

    fn get_game_mode(&self) -> GameMode {
        self.game_mode.clone()
    }

    fn get_players(&self) -> Vec<PlayerInfo> {
        self.players.clone()
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

    /// Creates a new Game instance.
    fn new(params: &NewGameParams) -> Result<Self, GameError> {
        //

        debug!("TicTacToeGame: Creating new game. Params: {:?}", params);

        let game = Self {
            current_player: None,
            game_mode: params.game_mode.clone(),
            id: Uuid::new_v4().to_string(),
            players: vec![],
            play_history: vec![],
        };

        Ok(game)
    }

    /// Make a Game move for the specified Player.
    fn take_turn(&mut self, game_turn_info: &GameTurnInfo) -> Result<GameState, GameError> {
        //

        debug!("TicTacToeGame: taking game turn. Params: {:?}", game_turn_info);

        let board_state = self.get_current_game_state();

        // Do not allow Game moves when the Game has already been completed.
        if board_state.has_ended() {
            return Err(GameError::GameHasAlreadyEnded);
        }

        // Get the Player - also validating that the correct IDs have been sent in.
        let player_taking_a_turn = self.get_player_info_by_id(&game_turn_info.player_id)?;

        // Ensure that the Player is not making a move out of turn.
        if player_taking_a_turn.player_id != self.current_player.clone().unwrap().player_id {
            return Err(GameError::WrongPlayerTakingTurn);
        }

        // Make sure that the target location is not already occupied.
        if Self::is_location_occupied(&board_state.get_game_board(), &game_turn_info.destination) {
            return Err(BoardLocationAlreadyOccupied);
        }

        // Load the other Player.
        let other_player =
            PlayerInfo::get_other_player_info_by_id(&game_turn_info.player_id, &self.players)?;

        // Take the turn and make a new Board State by adding the specified piece to the board of
        // the current Board State.
        let final_board_state = board_state.place_game_piece(
            &game_turn_info.destination,
            &player_taking_a_turn,
            &other_player,
        )?;

        // Add this move to our Game Play History
        self.play_history.push(final_board_state.clone());

        // Change Players
        self.current_player = Some(other_player);

        Ok(final_board_state.clone())
    }
}
