// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// This file defines Game Board related structs, types, enums, and constants.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::models::GamePiece;

/* Tic-Tac-Toe is played on a 3-by-3 grid. */
pub(crate) const MAX_BOARD_COLUMNS: usize = 3;
pub(crate) const MAX_BOARD_ROWS: usize = 3;

/// Specifies the locations of the Game pieces in Row/Column grid format.
pub(crate) type GameBoard = [[GamePiece; MAX_BOARD_ROWS]; MAX_BOARD_COLUMNS];

// Binary representation of a completely full Tic-Tac-Toe board.
pub(crate) const BIN_FULL_BOARD: i16 = 0b_111_111_111;

/* The following constants represent the 3-across winning move patterns on a Tic-tac-Toe Game board.
    As they are binary, they allow for very performant checks for winning moves. */
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_TOP: i16 = 0b_111_000_000;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_MIDDLE: i16 = 0b_000_111_000;
pub(crate) const BIN_THREE_ACROSS_HORIZONTAL_BOTTOM: i16 = 0b_000_000_111;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_LEFT: i16 = 0b_100_100_100;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_CENTER: i16 = 0b_010_010_010;
pub(crate) const BIN_THREE_ACROSS_VERTICAL_RIGHT: i16 = 0b_001_001_001;
pub(crate) const BIN_THREE_ACROSS_DIAGONAL_1: i16 = 0b_100_010_001;
pub(crate) const BIN_THREE_ACROSS_DIAGONAL_2: i16 = 0b_001_010_100;
