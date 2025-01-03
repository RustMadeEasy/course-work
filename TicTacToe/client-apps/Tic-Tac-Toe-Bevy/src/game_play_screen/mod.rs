//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use crate::game_play_screen::info_panel_plugin::InfoPanelPlugin;
use crate::game_play_screen::local_game_play_plugin::LocalGamePlayPlugin;
use crate::game_play_screen::tiles_plugin::TilesPlugin;
use bevy::app::PluginGroupBuilder;
use bevy::math::Vec2;
use bevy::prelude::{Component, Event, PluginGroup};
use tic_tac_toe_rust_client_sdk::models::BoardPosition;

pub(super) mod info_panel_plugin;
pub(super) mod local_game_play_plugin;
mod tile_component;
pub(super) mod tiles_plugin;

pub(super) const GRID_ROWS: usize = 3;
pub(super) const GRID_COLUMNS: usize = 3;

/// Provides the Game play UI and logic.
pub(super) struct GamePlayPluginGroup;

impl PluginGroup for GamePlayPluginGroup {
    /// Composes the plugin group.
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(LocalGamePlayPlugin)
            .add(InfoPanelPlugin)
            .add(TilesPlugin)
    }
}

/// Marker to indicate that an entity was spawned on the Game Play Screen.
#[derive(Component)]
struct OnGamePlayScreen;

/// Models a window location.
#[derive(Clone, Debug)]
pub(super) struct Point {
    pub(super) x: f32,
    pub(super) y: f32,
}

impl Point {
    pub(super) fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for Point {
    fn from(value: Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Vec2> for Point {
    fn into(self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
}

/// Event that is fired when a Tic-Tac-Toe tile is pressed.
#[derive(Event)]
pub(super) struct TilePressedEvent {
    /// Indicates the tile's location on the game board.
    pub(super) grid_position: BoardPosition,
}
