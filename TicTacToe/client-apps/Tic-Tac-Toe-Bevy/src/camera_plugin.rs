use bevy::app::{App, Startup};
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::{
    in_state, Camera, Camera2dBundle, ClearColor, Commands, OrthographicProjection, Plugin, Query,
    Res, Time, Transform, Update, Window, With,
};
use bevy::window::PrimaryWindow;

use crate::shared::app_mode::AppMode;
use crate::shared::BACKGROUND_COLOR;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

// Our set_camera_zoom() function will animate the zoom from 2.0 down to TARGET_SCALE.
const TARGET_SCALE: f32 = 0.52;

/// The 2D camera through which the Game is viewed.
pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    //

    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(ClearColor(*BACKGROUND_COLOR)) // Set the color to which everything erases.
            .add_systems(Startup, (spawn_camera, set_camera_zoom).chain())
            .add_systems(Update, zoom_in.run_if(in_state(AppMode::GamePlay)));
    }
}

/// Sets up and spawns a 2D camera from which the Game will be viewed.
fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    //

    if let Ok(window) = window_query.get_single() {
        //

        let x = window.width() / 2.0;
        let y = window.height() / 2.0;

        let camera_bundle = Camera2dBundle {
            transform: Transform::from_xyz(x, y, 100.0),
            ..Default::default()
        };

        commands.spawn(camera_bundle);
    }
}

/// Sets the initial zoom to be wide so that we can, subsequently, animate a zoom-in.
fn set_camera_zoom(mut projection_query: Query<&mut OrthographicProjection, With<Camera>>) {
    if let Ok(mut projection) = projection_query.get_single_mut() {
        projection.scale = 2.0;
    }
}

/// Zooms the camera in to the normal position for Game-play.
pub fn zoom_in(mut query: Query<&mut OrthographicProjection, With<Camera>>, time: Res<Time>) {
    for mut projection in query.iter_mut() {
        if projection.scale > TARGET_SCALE {
            projection.scale -= 0.5 * time.delta_seconds();
        }
    }
}
