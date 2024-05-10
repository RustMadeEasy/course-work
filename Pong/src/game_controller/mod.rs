use bevy::prelude::States;

pub(super) mod game_controller_plugin;

/// Specifies the state of the Game.
#[derive(Clone, Debug, Hash, Eq, PartialEq, States)]
pub(crate) enum GamePlayState {
    Playing,
    Paused,
}

/// Specifies whether Game sound is to play.
#[derive(Clone, Debug, Hash, Eq, PartialEq, States)]
pub(crate) enum SoundSetting {
    On,
    Off,
}
