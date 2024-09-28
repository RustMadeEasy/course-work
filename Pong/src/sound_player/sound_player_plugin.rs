// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::app::{App, FixedUpdate, Startup};
use bevy::asset::{AssetServer, Handle};
use bevy::audio::{AudioBundle, AudioSource, PlaybackSettings};
use bevy::prelude::{in_state, Commands, EventReader, IntoSystemConfigs, Plugin, Res, Resource};

use crate::game_controller::{GamePlayState, SoundSetting};
use crate::physical_interactions::collision_evaluator::CollisionEvaluator;
use crate::physical_interactions::collision_event::CollisionEvent;
use crate::physical_interactions::physical_interactions_actor::PhysicalInteractionActor::*;

const SOUND_BALL_MISSED: &str = "audio/impactBell_heavy_001.ogg";
const SOUND_PADDLE_HIT: &str = "audio/impactGlass_medium_000.ogg";
const SOUND_TOP_OR_SIDE_WALL_HIT: &str = "audio/impactMetal_medium_004.ogg";

/// Listens for specific events and plays the appropriate sound effect.
pub(crate) struct SoundPlayerPlugin;

impl Plugin for SoundPlayerPlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, preload_sound_effects)
            .add_systems(
                FixedUpdate,
                Self::handle_physical_interaction_events
                    .run_if(in_state(SoundSetting::On))
                    .run_if(in_state(GamePlayState::Playing)),
            );
    }
}

impl SoundPlayerPlugin {
    //

    /// Plays sound effects based on the incoming interaction events.
    fn handle_physical_interaction_events(
        collision_sounds_resource: Res<CollisionSoundsResource>,
        mut commands: Commands,
        mut event_reader: EventReader<CollisionEvent>,
    ) {
        //

        let mut spawn_audio = |handle: Handle<AudioSource>| {
            commands.spawn(AudioBundle {
                source: handle,
                settings: PlaybackSettings::DESPAWN,
            });
        };

        for event in event_reader.read() {
            let mut eval = CollisionEvaluator::new(event);
            if eval.did(Ball).collide_with(Ceiling).or(SideWall).evaluate() {
                spawn_audio(collision_sounds_resource.ceiling_or_side_wall_hit.clone());
            } else if eval.did(Ball).collide_with(Floor).evaluate() {
                spawn_audio(collision_sounds_resource.ball_missed.clone());
            } else if eval.did(Ball).collide_with(Paddle).evaluate() {
                spawn_audio(collision_sounds_resource.paddle_hit.clone());
            }
        }
    }
}

#[derive(Resource)]
struct CollisionSoundsResource {
    ball_missed: Handle<AudioSource>,
    paddle_hit: Handle<AudioSource>,
    ceiling_or_side_wall_hit: Handle<AudioSource>,
}

fn preload_sound_effects(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sounds = CollisionSoundsResource {
        ball_missed: asset_server.load(SOUND_BALL_MISSED),
        paddle_hit: asset_server.load(SOUND_PADDLE_HIT),
        ceiling_or_side_wall_hit: asset_server.load(SOUND_TOP_OR_SIDE_WALL_HIT),
    };
    commands.insert_resource(sounds);
}
