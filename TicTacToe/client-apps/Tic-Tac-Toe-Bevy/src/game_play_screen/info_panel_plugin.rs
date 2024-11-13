//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::prelude::{
    in_state, App, Component, FixedUpdate, IntoSystemConfigs, OnEnter, Plugin, Update,
};

use crate::game_play_screen::info_panel_plugin::functionality::{
    button_interaction, set_status_text, update_info_panel_bottom, update_info_panel_top,
};
use crate::game_play_screen::info_panel_plugin::spawn_ui::{
    spawn_end_game_button, spawn_info_panel_entities,
};
use crate::shared::app_mode::AppMode;

/// Displays relevant Game info along the top of the screen.
pub(super) struct InfoPanelPlugin;

impl Plugin for InfoPanelPlugin {
    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .add_systems(
                OnEnter(AppMode::GamePlay),
                (
                    set_status_text,
                    spawn_end_game_button,
                    spawn_info_panel_entities,
                ),
            )
            .add_systems(
                Update,
                (button_interaction,).run_if(in_state(AppMode::GamePlay)),
            )
            .add_systems(
                FixedUpdate,
                (update_info_panel_bottom, update_info_panel_top)
                    .run_if(in_state(AppMode::GamePlay)),
            );
    }
}

const INFO_PANEL_FONT_SIZE_LABEL: f32 = 22.0;

/// Marker for entities on the panel's first row.
#[derive(Component)]
pub(super) struct InfoPanelUiComponentTop;

/// Marker for entities on the panel's second row.
#[derive(Component)]
pub(super) struct InfoPanelUiComponentBottom;

mod functionality {
    use std::time::Duration;

    use bevy::prelude::{BackgroundColor, Button, Changed, DetectChanges, EventWriter, Interaction, NextState, Query, Res, ResMut, Text, TextStyle, Window, With};
    use bevy::window::PrimaryWindow;
    use helpers_for_bevy::status_text::events::SetStatusTextEvent;

    use crate::game_play_screen::info_panel_plugin::{InfoPanelUiComponentBottom, InfoPanelUiComponentTop, INFO_PANEL_FONT_SIZE_LABEL};
    use crate::shared::api_helpers::GamePieceHelper;
    use crate::shared::app_mode::AppMode;
    use crate::shared::app_state_resource::AppStateResource;
    use crate::shared::{BUTTON_COLOR_HOVERED, BUTTON_COLOR_NORMAL, BUTTON_COLOR_PRESSED, FOREGROUND_COLOR};

    /// Provides button functionality, including state changes as well as response to presses.
    #[allow(clippy::type_complexity)] // The query is complex by necessity.
    pub(super) fn button_interaction(
        mut interactions: Query<
            (&Interaction, &mut BackgroundColor),
            (Changed<Interaction>, With<Button>),
        >,
        mut next_state: ResMut<NextState<AppMode>>,
    ) {
        for (interaction, mut color) in &mut interactions {
            match *interaction {
                Interaction::Hovered => {
                    *color = BackgroundColor(*BUTTON_COLOR_HOVERED);
                }
                Interaction::None => {
                    *color = BackgroundColor(*BUTTON_COLOR_NORMAL);
                }
                Interaction::Pressed => {
                    *color = BackgroundColor(*BUTTON_COLOR_PRESSED);
                    next_state.set(AppMode::StartMenu);
                }
            }
        }
    }

    /// Sets the Status text with instructions for sharing the Invitation Code.
    pub(super) fn set_status_text(
        mut event_writer: EventWriter<SetStatusTextEvent>,
        app_state: Res<AppStateResource>,
        _window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        if app_state.local_player_initiated_gaming_session && app_state.is_two_player_game {
            let event = SetStatusTextEvent::new_with_duration(
                "Please send the Invitation Code to another player so they can join the game...",
                Duration::from_secs(30),
            );
            event_writer.send(event);
        }
    }

    /// Sets the text of the info panel's top row, based on the current state of the Game.
    pub(super) fn update_info_panel_top(
        app_state: Res<AppStateResource>,
        mut top_text_query: Query<&mut Text, With<InfoPanelUiComponentTop>>,
    ) {
        if app_state.is_changed() {
            if let Ok(mut text_sections) = top_text_query.get_single_mut() {
                // Show the Invitation Code instructions until the Game has started
                if app_state.local_player_initiated_gaming_session && !app_state.has_game_started {
                    text_sections.sections[0].value = format!(
                        "Hi {}. Please send the Invitation Code: {} to another player so they can join...",
                        app_state.local_player.display_name, app_state.invitation_code.clone()
                    );
                } else {
                    text_sections.sections[0].value = format!("{} {}", app_state.local_player.display_name, GamePieceHelper::display_name(app_state.local_player.game_piece));
                    if app_state.has_game_started && app_state.current_player.is_some() {
                        if app_state.current_player.clone().unwrap().player_id != app_state.local_player.player_id {
                            text_sections.sections[0].style = TextStyle {
                                color: *BUTTON_COLOR_HOVERED,
                                font: Default::default(),
                                font_size: INFO_PANEL_FONT_SIZE_LABEL,
                            };
                        } else {
                            text_sections.sections[0].style = TextStyle {
                                color: *FOREGROUND_COLOR,
                                font: Default::default(),
                                font_size: INFO_PANEL_FONT_SIZE_LABEL,
                            };
                        }
                    }
                }
            }
        }
    }

    /// Sets the text of the info panel's bottom row, based on the current state of the Game.
    pub(super) fn update_info_panel_bottom(
        app_state: Res<AppStateResource>,
        mut bottom_text_query: Query<&mut Text, With<InfoPanelUiComponentBottom>>,
    ) {
        if app_state.is_changed() {
            if let Ok(mut text_sections) = bottom_text_query.get_single_mut() {
                // Hide the other Player's section until the Game has started
                if app_state.has_game_started && app_state.current_player.is_some() {
                    let other_player = app_state.other_player.clone().unwrap_or_default();
                    text_sections.sections[0].value = format!("{} {}", other_player.display_name, GamePieceHelper::display_name(other_player.game_piece));
                    if app_state.current_player.clone().unwrap().player_id != other_player.player_id {
                        text_sections.sections[0].style = TextStyle {
                            color: *BUTTON_COLOR_HOVERED,
                            font: Default::default(),
                            font_size: INFO_PANEL_FONT_SIZE_LABEL,
                        };
                    } else {
                        text_sections.sections[0].style = TextStyle {
                            color: *FOREGROUND_COLOR,
                            font: Default::default(),
                            font_size: INFO_PANEL_FONT_SIZE_LABEL,
                        };
                    }
                }
            }
        }
    }
}

mod spawn_ui {
    use bevy::hierarchy::BuildChildren;
    use bevy::prelude::{
        AlignItems, BackgroundColor, BorderColor, ButtonBundle, Color, Commands, FlexDirection,
        JustifyContent, JustifySelf, NodeBundle, Query, TextBundle, TextSection, Window, With,
    };
    use bevy::text::TextStyle;
    use bevy::ui::{Style, UiRect, Val};
    use bevy::utils::default;
    use bevy::window::PrimaryWindow;

    use crate::game_play_screen::info_panel_plugin::{
        InfoPanelUiComponentBottom, InfoPanelUiComponentTop, INFO_PANEL_FONT_SIZE_LABEL,
    };
    use crate::game_play_screen::OnGamePlayScreen;
    use crate::shared::{BUTTON_COLOR_NORMAL, FONT_SIZE, FOREGROUND_COLOR, TEXT_COLOR};

    /// Sets up and creates the UI entities in the information panel area.
    pub(super) fn spawn_info_panel_entities(
        mut commands: Commands,
        _window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        let style = Style {
            display: Default::default(),
            justify_self: JustifySelf::Center,
            margin: UiRect::vertical(Val::Px(4.)),
            ..default()
        };

        // Local Player and Invitation instructions
        let sections = [TextSection::new(
            "",
            TextStyle {
                color: *FOREGROUND_COLOR,
                font: Default::default(),
                font_size: INFO_PANEL_FONT_SIZE_LABEL,
            },
        )];
        let text_bundle1 = TextBundle::from_sections(sections).with_style(style.clone());

        // Other Player
        let sections = [TextSection::new(
            "",
            TextStyle {
                color: *FOREGROUND_COLOR,
                font: Default::default(),
                font_size: INFO_PANEL_FONT_SIZE_LABEL,
            },
        )];
        let text_bundle2 = TextBundle::from_sections(sections).with_style(style);

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        padding: UiRect::horizontal(Val::Percent(20.0)),
                        width: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                OnGamePlayScreen,
            ))
            .with_children(|parent| {
                parent.spawn((text_bundle1, InfoPanelUiComponentTop, OnGamePlayScreen));
                parent.spawn((text_bundle2, InfoPanelUiComponentBottom, OnGamePlayScreen));
            });
    }

    /// Sets up and creates the End Game button.
    pub(super) fn spawn_end_game_button(
        mut commands: Commands,
        _window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        // Start Over Button
        let title_end_game = "End Game";
        let button_bundle = ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                height: Val::Px(32.),
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(16.)),
                ..default()
            },
            border_color: BorderColor(Color::GRAY),
            background_color: BackgroundColor(*BUTTON_COLOR_NORMAL),
            ..default()
        };
        let button_text_style = TextStyle {
            font: default(),
            font_size: FONT_SIZE,
            color: *TEXT_COLOR,
        };
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        padding: UiRect::all(Val::Px(12.)),
                        ..default()
                    },
                    ..default()
                },
                OnGamePlayScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((button_bundle, OnGamePlayScreen))
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(title_end_game, button_text_style),
                            OnGamePlayScreen,
                        ));
                    });
            });
    }
}
