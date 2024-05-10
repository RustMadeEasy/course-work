use bevy::a11y::accesskit::Size;
use bevy::math::{Rect, Vec2};
use bevy::prelude::Component;

use crate::game_play::Point;
use crate::shared::local_models::local_grid_position::LocalGridPosition;

/// Marker for UI tile entities.
#[derive(Component, Debug)]
pub(in crate::game_play) struct TileDetailsComponent {
    pub(super) grid_position: LocalGridPosition,
    window_rect: Rect,
}

impl TileDetailsComponent {
    //

    pub(in crate::game_play) fn hit_test(&self, hit_location: &Point) -> bool {
        self.window_rect.contains((*hit_location).clone().into())
    }

    fn calculate_bounding_box(window_center: &Point, size: &Size) -> Rect {
        Rect::from_center_size(
            (*window_center).clone().into(),
            Vec2::new(size.width as f32, size.height as f32),
        )
    }

    pub(in crate::game_play) fn new(
        grid_position: &LocalGridPosition,
        window_placement: &Point,
        size: &Size,
    ) -> Self {
        Self {
            grid_position: grid_position.clone(),
            window_rect: Self::calculate_bounding_box(window_placement, size),
        }
    }
}

/// Marker for the highlight element within a Tile.
#[derive(Component, Debug)]
pub(super) struct TileHighlightComponent {
    pub(super) grid_position: LocalGridPosition,
}
impl TileHighlightComponent {
    pub(super) fn new(grid_position: &LocalGridPosition) -> Self {
        Self {
            grid_position: grid_position.clone(),
        }
    }
}

/// Marker for the text element within a Tile.
#[derive(Component, Debug)]
pub(super) struct TileLabelComponent {
    pub(super) grid_position: LocalGridPosition,
}
impl TileLabelComponent {
    pub(super) fn new(grid_position: &LocalGridPosition) -> Self {
        Self {
            grid_position: grid_position.clone(),
        }
    }
}
