// Pong Game
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{Camera2dBundle, Commands, Query, Transform, Window, With};
use bevy::window::PrimaryWindow;

pub(crate) struct PongCameraPlugin;

impl Plugin for PongCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::spawn_camera);
    }
}

impl PongCameraPlugin {
    //

    /// Creates the camera through which the Game is viewed.
    fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
        //

        if let Ok(window) = window_query.get_single() {
            //

            let x = window.width() / 2.0;
            let y = window.height() / 2.0;

            let camera_bundle = Camera2dBundle {
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            };

            commands.spawn(camera_bundle);
        }
    }
}
