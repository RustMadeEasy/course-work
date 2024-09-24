use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/**
 * Defines Game Board related structs, types, enums, and constants.
 *
 * © 2024 Rust Made Easy. All rights reserved.
 * @author JoelDavisEngineering@Gmail.com
 */

/// Tic-Tac-Toe is played on a 3-by-3 grid.
pub(crate) const MAX_BOARD_COLUMNS: usize = 3;
pub(crate) const MAX_BOARD_ROWS: usize = 3;

/// Specifies the locations of the game pieces in Row/Column grid format.
pub(crate) type GameBoard = [[GamePiece; MAX_BOARD_ROWS]; MAX_BOARD_COLUMNS];

/// Binary representation of various patterns on a game board. This allows for very performant
/// checks for the winning move, stalemate, etc.
pub(crate) const BIN_FULL_BOARD: i16 = 0b_111_111_111;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_TOP: i16 = 0b_111_000_000;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_MIDDLE: i16 = 0b_000_111_000;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_BOTTOM: i16 = 0b_000_000_111;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_LEFT: i16 = 0b_100_100_100;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_CENTER: i16 = 0b_010_010_010;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_RIGHT: i16 = 0b_001_001_001;
pub(crate) const BIN_THREE_ACROSS_DIAGONAL_1: i16 = 0b_100_010_001;
pub(crate) const BIN_THREE_ACROSS_DIAGONAL_2: i16 = 0b_001_010_100;

/// Board Position representation of winning moves
pub(crate) const THREE_ACROSS_HORIZONTAL_TOP: &[BoardPosition] = &[
    BoardPosition { row: 0, column: 0 },
    BoardPosition { row: 0, column: 1 },
    BoardPosition { row: 0, column: 2 },
];
pub(crate) const THREE_ACROSS_HORIZONTAL_MIDDLE: &[BoardPosition] = &[
    BoardPosition { row: 1, column: 0 },
    BoardPosition { row: 1, column: 1 },
    BoardPosition { row: 1, column: 2 },
];
pub(crate) const THREE_ACROSS_HORIZONTAL_BOTTOM: &[BoardPosition] = &[
    BoardPosition { row: 2, column: 0 },
    BoardPosition { row: 2, column: 1 },
    BoardPosition { row: 2, column: 2 },
];
pub(crate) const THREE_ACROSS_VERTICAL_LEFT: &[BoardPosition] = &[
    BoardPosition { row: 0, column: 0 },
    BoardPosition { row: 1, column: 0 },
    BoardPosition { row: 2, column: 0 },
];
pub(crate) const THREE_ACROSS_VERTICAL_CENTER: &[BoardPosition] = &[
    BoardPosition { row: 0, column: 1 },
    BoardPosition { row: 1, column: 1 },
    BoardPosition { row: 2, column: 1 },
];
pub(crate) const THREE_ACROSS_VERTICAL_RIGHT: &[BoardPosition] = &[
    BoardPosition { row: 0, column: 2 },
    BoardPosition { row: 1, column: 2 },
    BoardPosition { row: 2, column: 2 },
];
pub(crate) const THREE_ACROSS_DIAGONAL_1: &[BoardPosition] = &[
    BoardPosition { row: 0, column: 0 },
    BoardPosition { row: 1, column: 1 },
    BoardPosition { row: 2, column: 2 },
];
pub(crate) const THREE_ACROSS_DIAGONAL_2: &[BoardPosition] = &[
    BoardPosition { row: 0, column: 2 },
    BoardPosition { row: 1, column: 1 },
    BoardPosition { row: 2, column: 0 },
];

/// Models a position on the game board.
#[derive(Clone, Default, Deserialize, Serialize, ToSchema, Validate)]
pub struct BoardPosition {
    #[validate(range(min = 0, max = 2))]
    pub(crate) row: usize,
    #[validate(range(min = 0, max = 2))]
    pub(crate) column: usize,
}

impl BoardPosition {
    /// Creates a new BoardPosition instance.
    #[cfg(test)]
    pub(crate) fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

/// Models a Game Piece with which the Tic-Tac-Toe game is played.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum GamePiece {
    #[default]
    None,
    X,
    O,
}
