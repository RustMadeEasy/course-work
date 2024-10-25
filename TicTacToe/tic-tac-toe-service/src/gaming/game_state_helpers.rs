// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/**
 * Defines Game State related structs and enums.
 */

use chrono::Utc;
use log::debug;


use crate::errors::GameError;
use crate::errors::GameError::{
    BoardLocationAlreadyOccupied, InvalidBoardPosition, WrongPlayerTakingTurn,
};
use crate::gaming::game_board;
use crate::gaming::game_board::{GameBoard, BIN_FULL_BOARD, BIN_THREE_ACROSS_DIAGONAL_1, BIN_THREE_ACROSS_DIAGONAL_2, BIN_THREE_ACROSS_HORIZONTAL_BOTTOM, BIN_THREE_ACROSS_HORIZONTAL_MIDDLE, BIN_THREE_ACROSS_HORIZONTAL_TOP, BIN_THREE_ACROSS_VERTICAL_CENTER, BIN_THREE_ACROSS_VERTICAL_LEFT, BIN_THREE_ACROSS_VERTICAL_RIGHT, MAX_BOARD_COLUMNS, MAX_BOARD_ROWS};
use crate::gaming::play_outcome::PlayOutcome;
use crate::models::board_position::BoardPosition;
use crate::models::game_piece::GamePiece;
use crate::models::game_state::GameState;
use crate::models::play_status::PlayStatus;
use crate::models::player_info::PlayerInfo;
use crate::models::responses::TurnResponse;

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
    ) -> Result<TurnResponse, GameError> {
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
            current_player,
            &current_player.game_piece,
            &other_player.game_piece,
        );

        // Return a new Game board state
        Ok(TurnResponse {
            current_player: Some(other_player.clone()), // switch Players
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

        let as_binary = game_board::binary_representation_for_piece_placement(
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
