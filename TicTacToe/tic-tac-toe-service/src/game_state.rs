// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/**
 * Defines Game State related structs and enums.
 */

use chrono::{DateTime, Utc};
use log::debug;


use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::GameError;
use crate::errors::GameError::{
    BoardLocationAlreadyOccupied, InvalidBoardPosition, WrongPlayerTakingTurn,
};
use crate::game_board::{
    BoardPosition, GameBoard, GamePiece, BIN_FULL_BOARD, BIN_THREE_ACROSS_DIAGONAL_1,
    BIN_THREE_ACROSS_DIAGONAL_2, BIN_THREE_ACROSS_HORIZONTAL_BOTTOM,
    BIN_THREE_ACROSS_HORIZONTAL_MIDDLE, BIN_THREE_ACROSS_HORIZONTAL_TOP,
    BIN_THREE_ACROSS_VERTICAL_CENTER, BIN_THREE_ACROSS_VERTICAL_LEFT,
    BIN_THREE_ACROSS_VERTICAL_RIGHT, MAX_BOARD_COLUMNS, MAX_BOARD_ROWS,
};
use crate::models::responses::TurnResult;
use crate::models::PlayerInfo;
use crate::play_outcome::PlayOutcome;
use crate::play_status::PlayStatus;

/// Models the state of a Game at a particular Move (turn).
#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub(crate) struct GameState {
    //

    /// The time at which this Game State was created. This is used for cleanup of abandoned Games.
    #[serde(skip)]
    pub(crate) created_date: DateTime<Utc>,

    /// ID of the Player who made this Move.
    id_of_player_who_made_move: String,

    /// The board on which the Game is played.
    pub(crate) game_board: [[GamePiece; MAX_BOARD_ROWS]; MAX_BOARD_COLUMNS],

    /// The current status of the Game.
    pub(crate) play_status: PlayStatus,
}

// Initialization
impl GameState {
    //

    /// Creates a new GameState instance.
    pub(crate) fn new() -> Self {
        Self {
            created_date: Utc::now(),
            id_of_player_who_made_move: "".to_string(),
            game_board: Default::default(),
            play_status: PlayStatus::NotStarted,
        }
    }

    /// Creates an initial GameState instance using a Player and a PlayStatus.
    pub(crate) fn new_with_initial_play_status(
        current_player_id: &str,
        play_status: &PlayStatus,
    ) -> Self {
        Self {
            created_date: Utc::now(),
            id_of_player_who_made_move: current_player_id.to_string(),
            game_board: Default::default(),
            play_status: play_status.clone(),
        }
    }
}

// Property accessors.
impl GameState {
    //

    /// Returns the ID of the Player who made the move at this Game State.
    #[allow(dead_code)]
    pub(crate) fn get_id_of_player_who_made_move(&self) -> String {
        self.id_of_player_who_made_move.clone()
    }

    /// Returns the Game Board.
    pub(crate) fn get_game_board(&self) -> GameBoard {
        self.game_board.clone()
    }

    /// Returns the Play Status.
    #[cfg(test)]
    pub(crate) fn get_play_status(&self) -> PlayStatus {
        self.play_status.clone()
    }

    /// Determines whether the Game has ended.
    pub(crate) fn has_ended(&self) -> bool {
        match self.play_status {
            PlayStatus::EndedInStalemate | PlayStatus::EndedInWin => true,
            PlayStatus::InProgress | PlayStatus::NotStarted => false
        }
    }
}

// Actions
impl GameState {
    //

    /// Places the Player's Game piece at the specified board position.
    pub(crate) fn place_game_piece(
        self,
        position: &BoardPosition,
        current_player: &PlayerInfo,
        other_player: &PlayerInfo,
    ) -> Result<TurnResult, GameError> {
        //

        debug!("place_game_piece position: {:?}", position);

        // If the Game Pieces have not been chosen, then the Game has not started.
        if current_player.game_piece == GamePiece::Unselected || other_player.game_piece == GamePiece::Unselected {
            return Err(GameError::PlayerPieceNotSelected);
        }

        // *** Verify that a valid board location is being specified ***
        if !Self::is_valid_board_position(position) {
            return Err(InvalidBoardPosition);
        }

        // Make sure that two different Players are being sent in
        if current_player.player_id == other_player.player_id {
            return Err(WrongPlayerTakingTurn);
        }

        // Disallow any further changes once the Game has ended. Just forward our current state.
        match self.play_status {
            PlayStatus::EndedInStalemate | PlayStatus::EndedInWin => {
                return Err(GameError::GameHasAlreadyEnded);
            }
            PlayStatus::InProgress | PlayStatus::NotStarted => {}
        }

        // Grab the current piece placements
        let mut game_board: GameBoard = self.game_board;

        // Make sure the newly specified space is not already occupied
        if game_board[position.row][position.column] != GamePiece::Unselected {
            return Err(BoardLocationAlreadyOccupied);
        }

        // Set the location with the specified Game Piece
        game_board[position.row][position.column] = current_player.game_piece.clone();

        // Determine how this move impacts the Game.
        let outcome = Self::determine_outcome_of_play(
            &game_board,
            &current_player,
            &current_player.game_piece,
            &other_player.game_piece,
        );

        // Return a new Game board state
        Ok(TurnResult {
            new_game_state: Self {
                created_date: Utc::now(),
                id_of_player_who_made_move: current_player.player_id.clone(),
                game_board,
                play_status: outcome.play_status,
            },
            winning_locations: outcome.winning_position,
            winning_player: outcome.winning_player,
        })
    }
}

// Helper functions
impl GameState {
    //

    /// Generates binary representations of piece placements on a Game Board.
    ///
    /// Returns a tuple whose first element is the binary representation of placements for
    /// Game_piece_one and the second element is the binary representation of placements for
    /// Game_piece_two.
    pub(crate) fn binary_representation_for_piece_placement(
        grid: &GameBoard,
        game_piece_one: &GamePiece,
        game_piece_two: &GamePiece,
    ) -> (i16, i16) {
        //

        if grid.is_empty() {
            return (0, 0);
        }

        let mut position: i16 = 0b_100_000_000;
        let mut binary_representation_one: i16 = 0;
        let mut binary_representation_two: i16 = 0;

        for row in grid {
            for game_piece in row {
                if game_piece == game_piece_one {
                    binary_representation_one |= position;
                } else if game_piece == game_piece_two {
                    binary_representation_two |= position;
                }
                position >>= 1;
            }
        }

        (binary_representation_one, binary_representation_two)
    }

    /// Determines the status of the GameBoard relative to a particular Game Piece. Returns the
    /// status and, if the Game has been won, returns the board positions that indicate the winning
    /// move.
    ///
    /// Returns PlayOutcome.
    fn determine_outcome_of_play(
        game_board: &GameBoard,
        current_player: &PlayerInfo,
        current_player_game_piece: &GamePiece,
        other_player_game_piece: &GamePiece,
    ) -> PlayOutcome {
        //

        debug!("determine_outcome_of_play called for game board: {:?}", game_board);

        let as_binary = Self::binary_representation_for_piece_placement(
            game_board,
            current_player_game_piece,
            other_player_game_piece,
        );

        // If the Game Pieces have not been chosen, then the Game has not started.
        if *current_player_game_piece == GamePiece::Unselected || *other_player_game_piece == GamePiece::Unselected {
            return PlayOutcome::new(&PlayStatus::NotStarted);
        }

        // If there are no spaces filled, then the Game has not started.
        if as_binary.0 == 0 && as_binary.1 == 0 {
            return PlayOutcome::new(&PlayStatus::NotStarted);
        }

        let current_player_binary_representation = as_binary.0;

        // *** Test for winning states ***

        // NOTE: The order in which we check the states is important!

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
                &GameState::winning_board_positions_from_binary(current_player_binary_representation).unwrap(),
                current_player,
            )
        } else if (as_binary.0 | as_binary.1) == BIN_FULL_BOARD {
            PlayOutcome::new(&PlayStatus::EndedInStalemate)
        } else {
            PlayOutcome::new(&PlayStatus::InProgress)
        }
    }

    /// Determines whether the specified position is a valid for the Tic-Tac-Toe Game board.
    pub(crate) fn is_valid_board_position(position: &BoardPosition) -> bool {
        if position.column > (MAX_BOARD_COLUMNS - 1) || position.row > (MAX_BOARD_ROWS - 1) {
            return false;
        }
        true
    }

    /// If the specified binary representation connotes a winning board layout, this method converts
    /// the binary representation to a grid-based board layout.
    fn winning_board_positions_from_binary(
        binary_representation: i16,
    ) -> Option<Vec<BoardPosition>> {
        //

        let result = if (binary_representation & BIN_THREE_ACROSS_HORIZONTAL_TOP)
            == BIN_THREE_ACROSS_HORIZONTAL_TOP
        {
            &[
                BoardPosition { row: 0, column: 0 },
                BoardPosition { row: 0, column: 1 },
                BoardPosition { row: 0, column: 2 },
            ]
        } else if (binary_representation & BIN_THREE_ACROSS_HORIZONTAL_MIDDLE)
            == BIN_THREE_ACROSS_HORIZONTAL_MIDDLE
        {
            &[
                BoardPosition { row: 1, column: 0 },
                BoardPosition { row: 1, column: 1 },
                BoardPosition { row: 1, column: 2 },
            ]
        } else if (binary_representation & BIN_THREE_ACROSS_HORIZONTAL_BOTTOM)
            == BIN_THREE_ACROSS_HORIZONTAL_BOTTOM
        {
            &[
                BoardPosition { row: 2, column: 0 },
                BoardPosition { row: 2, column: 1 },
                BoardPosition { row: 2, column: 2 },
            ]
        } else if (binary_representation & BIN_THREE_ACROSS_VERTICAL_LEFT)
            == BIN_THREE_ACROSS_VERTICAL_LEFT
        {
            &[
                BoardPosition { row: 0, column: 0 },
                BoardPosition { row: 1, column: 0 },
                BoardPosition { row: 2, column: 0 },
            ]
        } else if (binary_representation & BIN_THREE_ACROSS_VERTICAL_CENTER)
            == BIN_THREE_ACROSS_VERTICAL_CENTER
        {
            &[
                BoardPosition { row: 0, column: 1 },
                BoardPosition { row: 1, column: 1 },
                BoardPosition { row: 2, column: 1 },
            ]
        } else if (binary_representation & BIN_THREE_ACROSS_VERTICAL_RIGHT)
            == BIN_THREE_ACROSS_VERTICAL_RIGHT
        {
            &[
                BoardPosition { row: 0, column: 2 },
                BoardPosition { row: 1, column: 2 },
                BoardPosition { row: 2, column: 2 },
            ]
        } else if (binary_representation & BIN_THREE_ACROSS_DIAGONAL_1)
            == BIN_THREE_ACROSS_DIAGONAL_1
        {
            &[
                BoardPosition { row: 0, column: 0 },
                BoardPosition { row: 1, column: 1 },
                BoardPosition { row: 2, column: 2 },
            ]
        } else if (binary_representation & BIN_THREE_ACROSS_DIAGONAL_2)
            == BIN_THREE_ACROSS_DIAGONAL_2
        {
            &[
                BoardPosition { row: 0, column: 2 },
                BoardPosition { row: 1, column: 1 },
                BoardPosition { row: 2, column: 0 },
            ]
        } else {
            return None;
        };

        Some(result.to_vec())
    }
}
