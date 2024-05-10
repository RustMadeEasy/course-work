use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub(super) struct ScoreboardResource {
    pub(super) score: i64,
}
