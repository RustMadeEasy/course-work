// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Models a Game Piece with which the Tic-Tac-Toe Game is played.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum GamePiece {
    /// Indicates that no Game Piece has been selected
    #[default]
    Unselected,
    /// Indicates the X in Tic-Tac-Toe
    X,
    /// Indicates the O in Tic-Tac-Toe
    O,
}

impl GamePiece {
    //

    /// Creates a new instance, selecting the piece at random
    pub(crate) fn new_with_random_choice() -> Self {
        Self::random_choice()
    }

    /// Creates a new instance whose piece is the opposite of the specified Game Piece
    pub(crate) fn new_as_opposite(opponent_game_piece: &Self) -> Self {
        opponent_game_piece.opposite()
    }

    /// Selects the opposite game piece. If self is X, then O is returned. If self is O, X is
    /// returned. If self is Unselected, Unselected is returned.
    fn opposite(&self) -> Self {
        match self {
            GamePiece::Unselected => GamePiece::Unselected,
            GamePiece::X => GamePiece::O,
            GamePiece::O => GamePiece::X,
        }
    }

    /// Makes a random selection between the X and O game pieces.
    fn random_choice() -> Self {
        match rand::random::<bool>() {
            true => Self::O,
            false => Self::X,
        }
    }
}