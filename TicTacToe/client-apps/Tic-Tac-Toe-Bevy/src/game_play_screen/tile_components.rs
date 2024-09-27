//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::a11y::accesskit::Size;
use bevy::math::{Rect, Vec2};
use bevy::prelude::Component;

use crate::game_play_screen::Point;
use crate::shared::local_models::local_grid_position::LocalGridPosition;

/// Marker for tile UI entities.
#[derive(Component, Debug)]
pub(in crate::game_play_screen) struct TileDetailsComponent {
    pub(super) grid_position: LocalGridPosition,
    window_rect: Rect,
}

impl TileDetailsComponent {
    //

    /// Determines whether the specified location (point) is within the bounds of the
    /// TileDetailsComponent instance.
    pub(in crate::game_play_screen) fn hit_test(&self, hit_location: &Point) -> bool {
        self.window_rect.contains((*hit_location).clone().into())
    }

    /// Determines the bounding box for a specified center-point and size.
    fn determine_bounding_box(window_center: &Point, size: &Size) -> Rect {
        Rect::from_center_size(
            (*window_center).clone().into(),
            Vec2::new(size.width as f32, size.height as f32),
        )
    }

    /// Creates a new TileDetailsComponent instance.
    pub(in crate::game_play_screen) fn new(
        grid_position: &LocalGridPosition,
        window_placement: &Point,
        size: &Size,
    ) -> Self {
        Self {
            grid_position: grid_position.clone(),
            window_rect: Self::determine_bounding_box(window_placement, size),
        }
    }
}

/// Marker for the highlight element within a Tile.
#[derive(Component, Debug)]
pub(super) struct TileHighlightComponent {
    /// Specifies the position on-screen.
    pub(super) grid_position: LocalGridPosition,
}

impl TileHighlightComponent {
    /// Creates a new TileHighlightComponent instance.
    pub(super) fn new(grid_position: &LocalGridPosition) -> Self {
        Self {
            grid_position: grid_position.clone(),
        }
    }
}

/// Marker for the text element within a Tile.
#[derive(Component, Debug)]
pub(super) struct TileLabelComponent {
    /// Specifies the position on-screen.
    pub(super) grid_position: LocalGridPosition,
}
impl TileLabelComponent {
    /// Creates a new TileLabelComponent instance.
    pub(super) fn new(grid_position: &LocalGridPosition) -> Self {
        Self {
            grid_position: grid_position.clone(),
        }
    }
}
