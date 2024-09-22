use bevy::prelude::*;
use lazy_static::lazy_static;

use physical_interactions::physical_interactions_plugin::PhysicalInteractionsPlugin;

use crate::ball::ball_plugin::BallPlugin;
use crate::camera::camera_plugin::PongCameraPlugin;
use crate::game_controller::game_controller_plugin::GameControllerPlugin;
use crate::game_controller::{GamePlayState, SoundSetting};
use crate::paddle::paddle_plugin::PaddlePlugin;
use crate::scoreboard::scoreboard_plugin::ScoreboardPlugin;
use crate::sound_player::sound_player_plugin::SoundPlayerPlugin;

mod ball;
mod camera;
mod game_controller;
mod paddle;
mod physical_interactions;
mod scoreboard;
mod sound_player;

// Pong Game Demo
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author Info@RustMadeEasy.com

lazy_static! {
    static ref BACKGROUND_COLOR: Color = Color::hex("6d2abc").unwrap();
}

fn main() {
    //

    // Set the window title and initial size.
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Pong".to_string(),
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .insert_resource(ClearColor(*BACKGROUND_COLOR))
        .add_plugins((
            DefaultPlugins.set(window_plugin),
            PongCameraPlugin,
            SoundPlayerPlugin,
            BallPlugin,
            PaddlePlugin,
            ScoreboardPlugin,
            PhysicalInteractionsPlugin,
            GameControllerPlugin,
        ))
        .add_systems(FixedUpdate, bevy::window::close_on_esc)
        .insert_state(SoundSetting::On)
        .insert_state(GamePlayState::Playing)
        .run()
}
