use crate::shared::local_models::local_game_piece::LocalGamePiece;

/// Models a Tic-Tac-Toe game Player.
#[derive(Clone, Default, Debug, PartialEq)]
pub(crate) struct LocalPlayerInfo {
    pub(crate) display_name: String,
    pub(crate) game_piece: LocalGamePiece,
    pub(crate) player_id: String,
}
