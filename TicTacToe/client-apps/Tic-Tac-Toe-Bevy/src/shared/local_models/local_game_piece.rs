/// Models a Game Piece with which the Tic-Tac-Toe game is played.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum LocalGamePiece {
    #[default]
    Unoccupied,
    O,
    X,
}
