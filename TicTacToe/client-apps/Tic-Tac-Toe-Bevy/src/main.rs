use bevy::prelude::*;
use bevy::window::WindowResolution;
use helpers_for_bevy::status_text::status_text_plugin::StatusTextPlugin;

use crate::camera_plugin::CameraPlugin;
use crate::game_play_screen::GamePlayPluginGroup;
use crate::invitation_screen::invite_screen_plugin::InvitationScreenPlugin;
use crate::shared::app_mode::AppMode;
use crate::shared::app_state::AppStateResource;
use crate::shared::local_models::local_game_state::LocalGameStateResource;
use crate::start_screen::start_screen_plugin::StartScreenPlugin;

pub(crate) mod camera_plugin;
mod game_play_screen;
mod invitation_screen;
pub(crate) mod shared;
mod start_screen;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

/// App entry point.
fn main() {
    //

    // Set the window title and its initial size
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            title: "Tic-Tac-Toe".to_string(),
            resolution: WindowResolution::new(900.0, 700.0),
            ..Default::default()
        }),
        ..Default::default()
    };

    App::new()
        .insert_resource(AppStateResource::default())
        .insert_resource(LocalGameStateResource::default())
        .add_plugins(DefaultPlugins.set(window_plugin))
        .add_plugins(StatusTextPlugin::default())
        .add_plugins(GamePlayPluginGroup)
        .add_plugins(StartScreenPlugin)
        .add_plugins(InvitationScreenPlugin)
        .add_plugins(CameraPlugin)
        .add_systems(FixedUpdate, bevy::window::close_on_esc) // Close the app when the Escape button is pressed
        .insert_state(AppMode::StartMenu) // Set the initial state
        .run();
}
