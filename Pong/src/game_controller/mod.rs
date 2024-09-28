// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::prelude::States;

pub(super) mod game_controller_plugin;

/// Specifies the state of the Game.
#[derive(Clone, Debug, Hash, Eq, PartialEq, States)]
pub(crate) enum GamePlayState {
    Playing,
    Paused,
}

/// Specifies whether Game sounds are on or off.
#[derive(Clone, Debug, Hash, Eq, PartialEq, States)]
pub(crate) enum SoundSetting {
    On,
    Off,
}
