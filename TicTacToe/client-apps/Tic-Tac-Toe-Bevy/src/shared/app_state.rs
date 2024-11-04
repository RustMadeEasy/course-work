//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::prelude::Resource;
use tic_tac_toe_rust_client_sdk::models::{EventPlaneConfig, GamePiece, GamingSessionCreationResponse, PlayerInfo};

/// Houses the application state variables.
#[derive(Default, Resource)]
pub(crate) struct AppStateResource {
    //

    /// Specifies the configuration required for clients to subscribe to real-time Game state updates
    pub(crate) event_plane_config: EventPlaneConfig,

    /// ID of the Gaming Session.
    pub(crate) gaming_session_id: String,

    /// This is the code used to invite a new Player to the Game.
    pub(crate) invitation_code: String,

    // TODO: JD: we also need localPlayerInitiatedGame for when we support rematch within the same Gaming Session.
    /// Indicates that this client app instance is the one that started the Gaming Session.
    pub(crate) local_player_initiated_gaming_session: bool,

    /// The local Player, i.e. the Player using this app instance.
    pub(crate) local_player: PlayerInfo,

    /// The other Player, i.e. the local Player's opponent.
    pub(crate) other_player: Option<PlayerInfo>,
}

impl AppStateResource {
    //

    pub(crate) fn get_player_one(&self) -> Option<PlayerInfo> {
        // By convention, Player One uses X
        self.get_player_using_game_piece(GamePiece::X)
    }

    pub(crate) fn get_player_two(&self) -> Option<PlayerInfo> {
        // By convention, Player Two uses O
        self.get_player_using_game_piece(GamePiece::O)
    }

    fn get_player_using_game_piece(&self, game_piece: GamePiece) -> Option<PlayerInfo> {
        //

        if self.local_player.game_piece == game_piece {
            return Some(self.local_player.clone());
        }

        match self.other_player {
            Some(ref other_player) => {
                if other_player.game_piece == game_piece {
                    Some(other_player.clone())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Clears all fields of this instance.
    pub(crate) fn reset(&mut self) {
        *self = Self::default()
    }
}

impl From<GamingSessionCreationResponse> for AppStateResource {
    fn from(value: GamingSessionCreationResponse) -> Self {
        Self {
            event_plane_config: value.event_plane_config,
            gaming_session_id: value.session_id,
            invitation_code: value.invitation_code,
            local_player_initiated_gaming_session: false,
            local_player: value.initiating_player,
            other_player: value.other_player.unwrap_or_default(),
        }
    }
}
