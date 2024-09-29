use bevy::asset::Handle;
use bevy::audio::AudioSource;
use bevy::prelude::Resource;

#[derive(Resource)]
/// Holds preloaded audio resources.
pub(crate) struct CollisionSoundsResource {
    pub(crate) ball_missed: Handle<AudioSource>,
    pub(crate) paddle_hit: Handle<AudioSource>,
    pub(crate) ceiling_or_side_wall_hit: Handle<AudioSource>,
}