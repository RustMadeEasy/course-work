use bevy::prelude::{Color, Commands, Component, Entity, Query, With};
use lazy_static::lazy_static;

pub(crate) mod app_mode;
pub(crate) mod app_state;
pub(crate) mod local_models;
pub(crate) mod local_service_client;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author Info@RustMadeEasy.com

lazy_static! {
    pub(crate) static ref BACKGROUND_COLOR: Color = Color::hex("521c93").unwrap_or_default();
    pub(crate) static ref FOREGROUND_COLOR: Color = Color::hex("ff7e79").unwrap_or_default();
    pub(crate) static ref BUTTON_COLOR_NORMAL: Color = Color::hex("875eb5").unwrap_or_default();
    pub(crate) static ref BUTTON_COLOR_HOVERED: Color = Color::hex("976ec5").unwrap_or_default();
    pub(crate) static ref BUTTON_COLOR_PRESSED: Color = Color::hex("774ea5").unwrap_or_default();
    pub(crate) static ref TEXT_COLOR: Color = Color::WHITE;
}

pub(crate) const FONT_SIZE: f32 = 15.0;

/// Helper function to despawn all components of the specified type.
pub(crate) fn despawn<T: Component>(components: Query<Entity, With<T>>, mut commands: Commands) {
    for component in &components {
        commands.entity(component).despawn();
    }
}
