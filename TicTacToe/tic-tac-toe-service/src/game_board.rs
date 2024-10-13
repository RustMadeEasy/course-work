// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// This file defines Game Board related structs, types, enums, and constants.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/* Tic-Tac-Toe is played on a 3-by-3 grid. */
pub(crate) const MAX_BOARD_COLUMNS: usize = 3;
pub(crate) const MAX_BOARD_ROWS: usize = 3;

/// Specifies the locations of the Game pieces in Row/Column grid format.
pub(crate) type GameBoard = [[GamePiece; MAX_BOARD_ROWS]; MAX_BOARD_COLUMNS];

/* The following constants are various patterns on a Game board. These binary representations
    allow for very performant checks for winning moves, stalemate situations, etc.
*/
pub(crate) const BIN_FULL_BOARD: i16 = 0b_111_111_111;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_TOP: i16 = 0b_111_000_000;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_MIDDLE: i16 = 0b_000_111_000;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_BOTTOM: i16 = 0b_000_000_111;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_LEFT: i16 = 0b_100_100_100;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_CENTER: i16 = 0b_010_010_010;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_RIGHT: i16 = 0b_001_001_001;
pub(crate) const BIN_THREE_ACROSS_DIAGONAL_1: i16 = 0b_100_010_001;
pub(crate) const BIN_THREE_ACROSS_DIAGONAL_2: i16 = 0b_001_010_100;

/// Models a position on the Game board.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema, Validate)]
pub struct BoardPosition {
    #[validate(range(min = 0, max = 2))]
    pub(crate) row: usize,
    #[validate(range(min = 0, max = 2))]
    pub(crate) column: usize,
}

impl BoardPosition {
    /// Creates a new BoardPosition instance.
    pub(crate) fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

/// Models a Game Piece with which the Tic-Tac-Toe Game is played.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum GamePiece {
    #[default]
    None,
    X,
    O,
}

impl GamePiece {
    //

    /// Selects the opposite game piece. If self is X, then O is returned. If self is O, X is returned. If self is None, None is returned.
    pub(crate) fn opposite(&self) -> Self {
        match self {
            GamePiece::None => GamePiece::None,
            GamePiece::X => GamePiece::O,
            GamePiece::O => GamePiece::X,
        }
    }

    /// Makes a random selection between the X and O game pieces.
    pub fn random_choice() -> Self {
        match rand::random::<bool>() {
            true => Self::O,
            false => Self::X,
        }
    }
}
