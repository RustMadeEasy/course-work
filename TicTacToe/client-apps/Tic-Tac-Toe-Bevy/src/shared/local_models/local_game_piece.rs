/// Models a Game Piece with which the Tic-Tac-Toe game is played.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) enum LocalGamePiece {
    #[default]
    Unoccupied,
    O,
    X,
}

#[allow(clippy::from_over_into)]
impl Into<String> for LocalGamePiece {
    fn into(self) -> String {
        match self {
            LocalGamePiece::Unoccupied => "",
            LocalGamePiece::O => "O",
            LocalGamePiece::X => "X",
        }
            .to_string()
    }
}
