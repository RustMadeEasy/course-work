use tic_tac_toe_rust_client_sdk::models::{GameInfo, GamePiece, PlayStatus, PlayerInfo};

use crate::shared::local_models::local_game_piece::LocalGamePiece;
use crate::shared::local_models::local_game_state::LocalGameStateResource;
use crate::shared::local_models::local_grid_position::LocalGridPosition;
use crate::shared::local_models::local_player_info::LocalPlayerInfo;
use crate::shared::local_models::local_player_status::LocalPlayStatus;

/// Converts a list of remote (client SDK) Player models into a list of LocalPlayerInfo.
pub(super) fn remote_players_to_local_players(remote: &Vec<PlayerInfo>) -> Vec<LocalPlayerInfo> {
    let mut local_players: Vec<LocalPlayerInfo> = vec![];
    for player in remote {
        local_players.push(player.clone().into());
    }
    local_players
}

impl From<PlayerInfo> for LocalPlayerInfo {
    fn from(value: PlayerInfo) -> Self {
        let local_game_piece = match value.game_piece {
            GamePiece::None => LocalGamePiece::Unoccupied,
            GamePiece::X => LocalGamePiece::X,
            GamePiece::O => LocalGamePiece::O,
        };
        Self {
            display_name: value.display_name,
            game_piece: local_game_piece,
            player_id: value.player_id,
        }
    }
}

impl From<GameInfo> for LocalGameStateResource {
    //

    #[allow(clippy::field_reassign_with_default)]
    fn from(value: GameInfo) -> Self {
        //

        let mut local_game_state = LocalGameStateResource::default();

        if value.current_player.is_some() && value.current_player.clone().unwrap().is_some() {
            local_game_state.current_player = Some(value.current_player.unwrap().unwrap().into());
        }

        local_game_state.game_id = value.id.clone();

        if value.game_state.play_status != PlayStatus::NotStarted {
            for (row_index, row) in value.game_state.game_board.iter().enumerate() {
                for (col_index, game_piece) in row.iter().enumerate() {
                    local_game_state.game_board[row_index][col_index] = match game_piece {
                        GamePiece::None => LocalGamePiece::Unoccupied,
                        GamePiece::X => LocalGamePiece::X,
                        GamePiece::O => LocalGamePiece::O,
                    }
                }
            }
        }

        local_game_state.game_ended = match value.game_state.play_status {
            PlayStatus::EndedInStalemate | PlayStatus::EndedInWin => true,
            PlayStatus::InProgress | PlayStatus::NotStarted => false,
        };

        local_game_state.game_has_started = value.game_state.play_status != PlayStatus::NotStarted;

        local_game_state.play_status = match value.game_state.play_status {
            PlayStatus::EndedInStalemate => LocalPlayStatus::EndedInStalemate,
            PlayStatus::EndedInWin => LocalPlayStatus::EndedInWin,
            PlayStatus::InProgress => LocalPlayStatus::InProgress,
            PlayStatus::NotStarted => LocalPlayStatus::NotStarted,
        };

        let id_of_player_who_made_move = value.game_state.id_of_player_who_made_move.clone();
        local_game_state.id_of_player_who_made_the_last_move = id_of_player_who_made_move.clone();

        // winning_locations and winning_Player_name
        if value.game_state.play_status == PlayStatus::EndedInWin {
            let winning_player = value
                .players
                .iter()
                .find(|it| {
                    it.player_id == value.game_state.winning_player_id.clone().unwrap_or_default().unwrap_or_default()
                })
                .unwrap();
            local_game_state.winning_player_name = Some(winning_player.display_name.clone());

            let mut locations: Vec<LocalGridPosition> = vec![];
            for location in value
                .game_state
                .winning_locations
                .clone()
                .unwrap_or_default()
                .unwrap_or_default()
                .iter()
            {
                let grid_position =
                    LocalGridPosition::new(location.row as usize, location.column as usize);
                locations.push(grid_position);
            }
            local_game_state.winning_locations = Some(locations);
        } else {
            local_game_state.winning_locations = None;
            local_game_state.winning_player_name = None;
        }

        local_game_state
    }
}
