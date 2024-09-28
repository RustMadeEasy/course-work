// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub(super) struct ScoreboardResource {
    pub(super) score: i64,
}
