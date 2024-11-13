//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use crate::game_play_screen::Point;
use bevy::a11y::accesskit::Size;
use bevy::math::{Rect, Vec2};
use bevy::prelude::Component;
use tic_tac_toe_rust_client_sdk::models::BoardPosition;

/// Marker for tile UI entities.
#[derive(Component, Debug)]
pub(in crate::game_play_screen) struct TileDetailsComponent {
    pub(super) grid_position: BoardPosition,
    pub(super) window_rect: Rect,
}

impl TileDetailsComponent {
    //

    /// Creates a new TileDetailsComponent instance.
    pub(in crate::game_play_screen) fn new(
        grid_position: &BoardPosition,
        window_placement: &Point,
        size: &Size,
    ) -> Self {
        Self {
            grid_position: grid_position.clone(),
            window_rect: determine_bounding_box(window_placement, size),
        }
    }
}

/// Marker for the highlight element within a Tile.
#[derive(Component, Debug)]
pub(super) struct TileHighlightComponent {
    /// Specifies the position on-screen.
    pub(super) grid_position: BoardPosition,
}

impl TileHighlightComponent {
    /// Creates a new TileHighlightComponent instance.
    pub(super) fn new(grid_position: &BoardPosition) -> Self {
        Self {
            grid_position: grid_position.clone(),
        }
    }
}

/// Marker for the text element within a Tile.
#[derive(Component, Debug)]
pub(super) struct TileLabelComponent {
    /// Specifies the position on-screen.
    pub(super) grid_position: BoardPosition,
}
impl TileLabelComponent {
    /// Creates a new TileLabelComponent instance.
    pub(in crate::game_play_screen) fn new(grid_position: &BoardPosition) -> Self {
        Self {
            grid_position: grid_position.clone(),
        }
    }
}

/// Determines whether the specified location (point) is within the bounds of the
/// TileDetailsComponent instance.
pub(in crate::game_play_screen) fn hit_test(window_rect: &Rect, hit_location: &Point) -> bool {
    window_rect.contains((*hit_location).clone().into())
}

/// Determines the bounding box for a specified center-point and size.
fn determine_bounding_box(window_center: &Point, size: &Size) -> Rect {
    Rect::from_center_size(
        (*window_center).clone().into(),
        Vec2::new(size.width as f32, size.height as f32),
    )
}
