/**
 * Defines Game State related structs and enums.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author Joel@RustMadeEasy.com
 */

use std::marker::PhantomData;

use log::error;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::GameError;
use crate::errors::GameError::{
    BoardLocationAlreadyOccupied, InvalidBoardPosition, WrongPlayerTakingTurn,
};
use crate::game_board::{
    BIN_FULL_BOARD, BIN_THREE_ACROSS_DIAGONAL_1, BIN_THREE_ACROSS_DIAGONAL_2, BIN_THREE_ACROSS_HORIZONTAL_BOTTOM, BIN_THREE_ACROSS_HORIZONTAL_MIDDLE,
    BIN_THREE_ACROSS_HORIZONTAL_TOP, BIN_THREE_ACROSS_VERTICAL_CENTER,
    BIN_THREE_ACROSS_VERTICAL_LEFT, BIN_THREE_ACROSS_VERTICAL_RIGHT,
    BoardPosition, GameBoard,
    GamePiece, MAX_BOARD_COLUMNS, MAX_BOARD_ROWS, THREE_ACROSS_DIAGONAL_1,
    THREE_ACROSS_DIAGONAL_2, THREE_ACROSS_HORIZONTAL_BOTTOM, THREE_ACROSS_HORIZONTAL_MIDDLE,
    THREE_ACROSS_HORIZONTAL_TOP, THREE_ACROSS_VERTICAL_CENTER, THREE_ACROSS_VERTICAL_LEFT,
    THREE_ACROSS_VERTICAL_RIGHT,
};
use crate::models::PlayerInfo;
use crate::play_outcome::PlayOutcome;
use crate::play_status::PlayStatus;

/// Models the state of a game at a particular move.
#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
pub(crate) struct GameState {
    /// ID of the Player who made this Move.
    id_of_player_who_made_move: String,
    pub(crate) game_board: [[GamePiece; MAX_BOARD_ROWS]; MAX_BOARD_COLUMNS],
    #[serde(skip)]
    _outside_instantiation_preventor: PhantomData<u8>,
    pub(crate) play_status: PlayStatus,
    pub(crate) winning_locations: Option<Vec<BoardPosition>>,
    pub(crate) winning_player_id: Option<String>,
}

impl GameState {
    //

    /// Generates binary representations of piece placements on a Game Board.
    ///
    /// Returns a tuple wherein the first element is the binary representation of placements for
    /// game_piece_one and the second element is the binary representation of placements for
    /// game_piece_two.
    fn binary_representation_for_piece_placement(
        grid: &GameBoard,
        game_piece_one: &GamePiece,
        game_piece_two: &GamePiece,
    ) -> (i16, i16) {
        //

        if grid.is_empty() {
            return (0, 0);
        }

        let mut binary_string_one: String = Default::default();
        let mut binary_string_two: String = Default::default();
        for col in grid {
            for game_piece in col {
                if game_piece != game_piece_one {
                    binary_string_one.push('0');
                } else {
                    binary_string_one.push('1');
                }
                if game_piece != game_piece_two {
                    binary_string_two.push('0');
                } else {
                    binary_string_two.push('1');
                }
            }
        }

        let binary_representation_one =
            i16::from_str_radix(&binary_string_one, 2).unwrap_or_else(|error| {
                error!("Failed to create binary view of board. Error: {}", error);
                0
            });
        let binary_representation_two =
            i16::from_str_radix(&binary_string_two, 2).unwrap_or_else(|error| {
                error!("Failed to create binary view of board. Error: {}", error);
                0
            });

        (binary_representation_one, binary_representation_two)
    }

    /// Determines the status of the GameBoard relative to a particular Game Piece. Returns the
    /// status and, if the game has been won, returns the board positions that indicate the winning
    /// move.
    ///
    /// Returns the following tuple:
    ///     (Play Status, Optional Winning Board Positions, Optional Winning Player ID)
    fn determine_outcome_of_play(
        grid: &GameBoard,
        current_player_id: &str,
        current_player_game_piece: &GamePiece,
        other_player_game_piece: &GamePiece,
    ) -> PlayOutcome {
        //

        let as_binary = Self::binary_representation_for_piece_placement(
            grid,
            current_player_game_piece,
            other_player_game_piece,
        );

        // If there are no spaces filled, then the game has not started
        if as_binary.0 == 0 && as_binary.1 == 0 {
            return PlayOutcome::new(&PlayStatus::NotStarted);
        }

        // NOTE: The order in which we check the states is important!
        let current_player_binary_representation = as_binary.0;

        fn winning_board_positions_from_binary(
            binary_representation: i16,
        ) -> Option<Vec<BoardPosition>> {
            //

            let result = if (binary_representation & BIN_THREE_ACROSS_HORIZONTAL_TOP)
                == BIN_THREE_ACROSS_HORIZONTAL_TOP
            {
                THREE_ACROSS_HORIZONTAL_TOP
            } else if (binary_representation & BIN_THREE_ACROSS_HORIZONTAL_MIDDLE)
                == BIN_THREE_ACROSS_HORIZONTAL_MIDDLE
            {
                THREE_ACROSS_HORIZONTAL_MIDDLE
            } else if (binary_representation & BIN_THREE_ACROSS_HORIZONTAL_BOTTOM)
                == BIN_THREE_ACROSS_HORIZONTAL_BOTTOM
            {
                THREE_ACROSS_HORIZONTAL_BOTTOM
            } else if (binary_representation & BIN_THREE_ACROSS_VERTICAL_LEFT)
                == BIN_THREE_ACROSS_VERTICAL_LEFT
            {
                THREE_ACROSS_VERTICAL_LEFT
            } else if (binary_representation & BIN_THREE_ACROSS_VERTICAL_CENTER)
                == BIN_THREE_ACROSS_VERTICAL_CENTER
            {
                THREE_ACROSS_VERTICAL_CENTER
            } else if (binary_representation & BIN_THREE_ACROSS_VERTICAL_RIGHT)
                == BIN_THREE_ACROSS_VERTICAL_RIGHT
            {
                THREE_ACROSS_VERTICAL_RIGHT
            } else if (binary_representation & BIN_THREE_ACROSS_DIAGONAL_1)
                == BIN_THREE_ACROSS_DIAGONAL_1
            {
                THREE_ACROSS_DIAGONAL_1
            } else if (binary_representation & BIN_THREE_ACROSS_DIAGONAL_2)
                == BIN_THREE_ACROSS_DIAGONAL_2
            {
                THREE_ACROSS_DIAGONAL_2
            } else {
                return None;
            };

            Some(result.to_vec())
        }

        // Test for winning states
        if (current_player_binary_representation & BIN_THREE_ACROSS_HORIZONTAL_TOP)
            == BIN_THREE_ACROSS_HORIZONTAL_TOP
            || (current_player_binary_representation & BIN_THREE_ACROSS_HORIZONTAL_MIDDLE)
                == BIN_THREE_ACROSS_HORIZONTAL_MIDDLE
            || (current_player_binary_representation & BIN_THREE_ACROSS_HORIZONTAL_BOTTOM)
                == BIN_THREE_ACROSS_HORIZONTAL_BOTTOM
            || (current_player_binary_representation & BIN_THREE_ACROSS_VERTICAL_LEFT)
                == BIN_THREE_ACROSS_VERTICAL_LEFT
            || (current_player_binary_representation & BIN_THREE_ACROSS_VERTICAL_CENTER)
                == BIN_THREE_ACROSS_VERTICAL_CENTER
            || (current_player_binary_representation & BIN_THREE_ACROSS_VERTICAL_RIGHT)
                == BIN_THREE_ACROSS_VERTICAL_RIGHT
            || (current_player_binary_representation & BIN_THREE_ACROSS_DIAGONAL_1)
                == BIN_THREE_ACROSS_DIAGONAL_1
            || (current_player_binary_representation & BIN_THREE_ACROSS_DIAGONAL_2)
                == BIN_THREE_ACROSS_DIAGONAL_2
        {
            PlayOutcome::new_with_win_details(
                &PlayStatus::EndedInWin,
                &winning_board_positions_from_binary(current_player_binary_representation).unwrap(),
                current_player_id,
            )
        } else if (as_binary.0 | as_binary.1) == BIN_FULL_BOARD {
            PlayOutcome::new(&PlayStatus::EndedInStalemate)
        } else {
            PlayOutcome::new(&PlayStatus::InProgress)
        }
    }

    /// Returns the ID of the player who the move at this Game State.
    #[allow(dead_code)]
    pub(crate) fn get_id_of_player_who_made_move(&self) -> String {
        self.id_of_player_who_made_move.clone()
    }

    /// Returns the Game Board.
    #[allow(dead_code)]
    pub(crate) fn get_game_board(&self) -> GameBoard {
        self.game_board.clone()
    }

    /// Returns the Play Status.
    pub(crate) fn get_play_status(&self) -> PlayStatus {
        self.play_status.clone()
    }

    /// Determines whether the specified position is a valid for the Tic-Tac_Toe game board.
    pub(crate) fn is_valid_board_position(position: &BoardPosition) -> bool {
        if position.column > (MAX_BOARD_COLUMNS - 1) || position.row > (MAX_BOARD_ROWS - 1) {
            return false;
        }
        true
    }

    /// Creates a new GameState instance.
    #[allow(dead_code)]
    pub(crate) fn new() -> Self {
        Self {
            id_of_player_who_made_move: "".to_string(),
            game_board: Default::default(),
            _outside_instantiation_preventor: Default::default(),
            play_status: PlayStatus::NotStarted,
            winning_locations: None,
            winning_player_id: None,
        }
    }

    /// Creates an initial GameState instance using a PlayStatus.
    pub(crate) fn new_with_initial_play_status(
        current_player_id: &str,
        play_status: &PlayStatus,
    ) -> Self {
        Self {
            id_of_player_who_made_move: current_player_id.to_string(),
            game_board: Default::default(),
            _outside_instantiation_preventor: Default::default(),
            play_status: play_status.clone(),
            winning_locations: None,
            winning_player_id: None,
        }
    }

    /// Places the Player's game piece at the specified board position.
    pub(crate) fn place_game_piece(
        self,
        position: &BoardPosition,
        current_player: &PlayerInfo,
        other_player: &PlayerInfo,
    ) -> Result<GameState, GameError> {
        //

        // *** Verify that a valid board location is being specified ***
        if !Self::is_valid_board_position(position) {
            return Err(InvalidBoardPosition);
        }

        // Make sure that two different Players are being sent in
        if current_player.player_id == other_player.player_id {
            return Err(WrongPlayerTakingTurn);
        }

        // Disallow any further changes once the game has ended. Just forward our current state.
        match self.play_status {
            PlayStatus::EndedInStalemate | PlayStatus::EndedInWin => {
                return Ok(self.clone());
            }
            PlayStatus::InProgress | PlayStatus::NotStarted => {}
        }

        // Grab the current piece placements
        let mut game_board: GameBoard = self.game_board;

        // Make sure the newly specified space is not already occupied
        if game_board[position.row][position.column] != GamePiece::None {
            return Err(BoardLocationAlreadyOccupied);
        }

        // Set the location with the specified Game Piece
        game_board[position.row][position.column] = current_player.game_piece.clone();

        // Determine how this move impacts the Game.
        let outcome = Self::determine_outcome_of_play(
            &game_board,
            &current_player.player_id,
            &current_player.game_piece,
            &other_player.game_piece,
        );

        // Return a new game board state
        Ok(Self {
            id_of_player_who_made_move: current_player.player_id.clone(),
            game_board,
            _outside_instantiation_preventor: Default::default(),
            play_status: outcome.play_status,
            winning_locations: outcome.winning_position,
            winning_player_id: outcome.winning_player_id,
        })
    }
}
