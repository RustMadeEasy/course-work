use bevy::prelude::Resource;

use crate::shared::local_models::local_player_info::LocalPlayerInfo;

/// Houses the application state variables.
#[derive(Default, Resource)]
pub(crate) struct AppStateResource {
    /// The local Player, i.e. the Player using this app instance.
    pub(crate) local_player: LocalPlayerInfo,

    /// Indicates that this client app instance is the one that started the Game
    pub(crate) local_player_initiated_game: bool,

    /// The other Player, i.e. the local Player's opponent.
    pub(crate) other_player: Option<LocalPlayerInfo>,
}

impl AppStateResource {
    //

    /// Clears all fields of this instance.
    pub(crate) fn reset(&mut self) {
        *self = Self::default()
    }

    /// Updates the transient information held by this struct.
    pub(crate) fn update(&mut self, players: &[LocalPlayerInfo]) {
        //

        if players.len() > 1 {
            self.other_player = if self.local_player_initiated_game {
                Some(players.last().unwrap().clone())
            } else {
                Some(players.first().unwrap().clone())
            };
        }

        self.local_player = if self.local_player_initiated_game {
            players.first().unwrap().clone().into()
        } else {
            players.last().unwrap().clone().into()
        };
    }
}
