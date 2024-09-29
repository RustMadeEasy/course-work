// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::prelude::Resource;

/// Models the info needed to represent the Game score in the UI.
#[derive(Default, Resource)]
pub(super) struct ScoreboardResource {
    pub(super) score: i64,
}
