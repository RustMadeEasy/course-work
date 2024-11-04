//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use crate::game_play_screen::tile_components::{
    TileDetailsComponent, TileHighlightComponent, TileLabelComponent,
};
use crate::game_play_screen::{OnGamePlayScreen, Point, TilePressedEvent, GRID_COLUMNS, GRID_ROWS};
use crate::shared::app_mode::AppMode;
use crate::shared::game_state_resource::GameStateResource;
use crate::shared::{BACKGROUND_COLOR, FOREGROUND_COLOR};
use bevy::a11y::accesskit::Size;
use bevy::app::{App, FixedUpdate};
use bevy::input::common_conditions::input_pressed;
use bevy::prelude::{
    in_state, Assets, ButtonInput, Camera, Color, ColorMaterial, Commands, DetectChanges,
    EventWriter, GlobalTransform, IntoSystemConfigs, Mesh, MouseButton, OnEnter, Plugin, Query,
    Rectangle, Res, ResMut, TextSection, TextStyle, Transform, Update, Visibility, Window, With,
};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy::text::{Text, Text2dBundle};
use bevy::utils::default;
use bevy::window::PrimaryWindow;
use tic_tac_toe_rust_client_sdk::models::BoardPosition;

const TILE_FONT_SIZE: f32 = 44_f32;
const TILE_SIDE: f32 = 100_f32;
const TILE_PADDING: f32 = 1.;

const TILE_HIGHLIGHT_Z_ORDER: f32 = 1_f32;
const TILE_TEXT_Z_ORDER: f32 = 3_f32;
const TILE_Z_ORDER: f32 = 2_f32;

/// Provides TicTacToe tile UI entities as well as functionality, e.g. hit-testing.
pub(super) struct TilesPlugin;

impl Plugin for TilesPlugin {
    //

    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .add_event::<TilePressedEvent>()
            .add_systems(OnEnter(AppMode::GamePlay), Self::spawn_tiles)
            .add_systems(
                Update,
                (Self::update_tiles, Self::highlight_winning_tiles)
                    .run_if(in_state(AppMode::GamePlay)),
            )
            .add_systems(
                FixedUpdate,
                Self::detect_tile_hit
                    .run_if(in_state(AppMode::GamePlay))
                    .run_if(input_pressed(MouseButton::Left)),
            );
    }
}

// Functionality
impl TilesPlugin {
    //

    /// Detects when any game Tile is hit, posting a Tile Pressed Event.
    fn detect_tile_hit(
        button: Res<ButtonInput<MouseButton>>,
        camera_query: Query<(&Camera, &GlobalTransform)>,
        mut event_writer: EventWriter<TilePressedEvent>,
        tiles_query: Query<&TileDetailsComponent>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        if !button.just_pressed(MouseButton::Left) {
            return;
        }

        if let Ok(window) = window_query.get_single() {
            //

            // Determine where the user pressed.
            if let Some(cursor_position) = window.cursor_position() {
                //

                // Convert the cursor position to real-world coordinates.
                let (camera, camera_transform) = camera_query.single();
                if let Some(cursor_position) =
                    camera.viewport_to_world_2d(camera_transform, cursor_position)
                {
                    // See if any Tile was pressed.
                    for tile in &tiles_query {
                        if tile.hit_test(&cursor_position.into()) {
                            event_writer.send(TilePressedEvent {
                                grid_position: tile.grid_position.clone(),
                            });
                            return;
                        }
                    }
                };
            }
        }
    }

    /// Provides a backing visual flare (highlight) to the Tiles that represent the TicTacToe
    /// 3-in-a-row winning combination.
    fn highlight_winning_tiles(
        local_game_state: Res<GameStateResource>,
        mut tile_highlights: Query<
            (&mut Visibility, &TileHighlightComponent),
            With<TileHighlightComponent>,
        >,
    ) {
        //

        if !local_game_state.is_changed() {
            return;
        }

        if let Some(winning_locations) = &local_game_state.winning_locations {
            for (mut visibility, tile_info) in tile_highlights.iter_mut() {
                if winning_locations.contains(&tile_info.grid_position) {
                    *visibility = Visibility::Visible;
                }
            }
        }
    }

    /// Updates the Tile's label with the visual representation of its Game Piece (if any).
    fn update_tiles(
        local_game_state: Res<GameStateResource>,
        mut label_query: Query<(&mut Text, &TileLabelComponent), With<TileLabelComponent>>,
    ) {
        //

        if !local_game_state.is_changed() {
            return;
        }

        for (mut label, tile_info) in label_query.iter_mut() {
            let game_piece = local_game_state.get_game_piece_at_placement(&tile_info.grid_position);
            label.sections[0].value = game_piece.to_string();
        }
    }
}

// UI
impl TilesPlugin {
    //

    /// Creates the Tile entities.
    fn spawn_tiles(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
        window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        if let Ok(window) = window_query.get_single() {
            //

            let offset_x = (window.width() / 2_f32) - (TILE_SIDE);
            let offset_y = (window.height() / 2_f32) - (TILE_SIDE);
            let tile_size = Size::new(TILE_SIDE as f64, TILE_SIDE as f64);

            for row in 0..GRID_ROWS {
                //

                for col in 0..GRID_COLUMNS {
                    //

                    let center_location = Point::new(
                        (col as f32 * TILE_SIDE) + offset_x,
                        (row as f32 * TILE_SIDE) + offset_y,
                    );

                    let pos = BoardPosition::new(col as i32, (2 - row) as i32);

                    // The Tile
                    let tile_bundle = MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                            TILE_SIDE - TILE_PADDING,
                            TILE_SIDE - TILE_PADDING,
                        ))),
                        material: materials.add(FOREGROUND_COLOR.with_a(0.8_f32)),
                        transform: Transform::from_xyz(
                            center_location.x,
                            center_location.y,
                            TILE_Z_ORDER,
                        ),
                        ..default()
                    };
                    let tile_component =
                        TileDetailsComponent::new(&pos, &center_location, &tile_size);
                    commands.spawn((tile_bundle, tile_component, OnGamePlayScreen));

                    // The Tile label (X or O)
                    let section = TextSection::new(
                        "",
                        TextStyle {
                            color: *BACKGROUND_COLOR,
                            font: Default::default(),
                            font_size: TILE_FONT_SIZE,
                        },
                    );
                    let text_bundle = Text2dBundle {
                        text: Text::from_sections([section]),
                        transform: Transform::from_xyz(
                            center_location.x,
                            center_location.y,
                            TILE_TEXT_Z_ORDER,
                        ),
                        ..default()
                    };
                    commands.spawn((text_bundle, TileLabelComponent::new(&pos), OnGamePlayScreen));

                    // The backing that is shown to highlight the winning locations
                    let highlight = MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                            TILE_SIDE - TILE_PADDING,
                            TILE_SIDE - TILE_PADDING,
                        ))),
                        material: materials.add(Color::WHITE.with_a(0.5)),
                        transform: Transform::from_xyz(
                            center_location.x,
                            center_location.y,
                            TILE_HIGHLIGHT_Z_ORDER,
                        ),
                        visibility: Visibility::Hidden,
                        ..default()
                    };
                    commands.spawn((
                        highlight,
                        TileHighlightComponent::new(&pos),
                        OnGamePlayScreen,
                    ));
                }
            }
        }
    }
}
