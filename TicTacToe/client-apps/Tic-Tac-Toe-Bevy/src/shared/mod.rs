//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::prelude::{Color, Commands, Component, Entity, Query, With};
use std::sync::LazyLock;

pub(crate) mod app_mode;
pub(crate) mod app_state_resource;
pub(crate) mod api_helpers;

pub(crate) static BACKGROUND_COLOR: LazyLock<Color> = LazyLock::new(|| { Color::hex("521c93").unwrap() });
pub(crate) static FOREGROUND_COLOR: LazyLock<Color> = LazyLock::new(|| { Color::hex("ff7e79").unwrap() });
pub(crate) static BUTTON_COLOR_NORMAL: LazyLock<Color> = LazyLock::new(|| { Color::hex("875eb5").unwrap() });
pub(crate) static BUTTON_COLOR_HOVERED: LazyLock<Color> = LazyLock::new(|| { Color::hex("976ec5").unwrap() });
pub(crate) static BUTTON_COLOR_PRESSED: LazyLock<Color> = LazyLock::new(|| { Color::hex("774ea5").unwrap() });
pub(crate) static TEXT_COLOR: LazyLock<Color> = LazyLock::new(|| { Color::WHITE });

pub(crate) const FONT_SIZE: f32 = 15.0;

/// Helper function to de-spawn all components of the specified type.
pub(crate) fn despawn<T: Component>(components: Query<Entity, With<T>>, mut commands: Commands) {
    for component in &components {
        commands.entity(component).despawn();
    }
}
