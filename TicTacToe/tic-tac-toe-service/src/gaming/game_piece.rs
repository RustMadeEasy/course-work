use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Models a Game Piece with which the Tic-Tac-Toe Game is played.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum GamePiece {
    #[default]
    Unselected,
    X,
    O,
}

impl GamePiece {
    //

    /// Selects the opposite game piece. If self is X, then O is returned. If self is O, X is
    /// returned. If self is Unselected, Unselected is returned.
    pub(crate) fn opposite(&self) -> Self {
        match self {
            GamePiece::Unselected => GamePiece::Unselected,
            GamePiece::X => GamePiece::O,
            GamePiece::O => GamePiece::X,
        }
    }

    /// Makes a random selection between the X and O game pieces.
    pub(crate) fn random_choice() -> Self {
        match rand::random::<bool>() {
            true => Self::O,
            false => Self::X,
        }
    }
}