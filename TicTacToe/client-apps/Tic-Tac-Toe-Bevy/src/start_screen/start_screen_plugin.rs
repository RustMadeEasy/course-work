use bevy::prelude::{in_state, App, Component, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::shared::app_mode::AppMode;
use crate::shared::despawn;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

/// Defines the purposes of the Start Screen buttons.
#[derive(Clone)]
enum ButtonPurpose {
    AcceptInvitation,
    StartTwoPlayerGame,
    StartSinglePlayerGame,
}

/// Marker to indicate that an entity was spawned on the Startup Screen.
#[derive(Component)]
struct OnStartScreen;

/// Companion component for Startup Screen Player name display text.
#[derive(Component)]
struct PlayerNameLabelComponent;

/// Provides UI and functionality for the Game's Startup Screen.
pub(crate) struct StartScreenPlugin;

impl Plugin for StartScreenPlugin {
    //

    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .add_systems(
                OnEnter(AppMode::StartMenu),
                (functionality::reset_game_play, ui::spawn_buttons).chain(),
            )
            .add_systems(
                Update,
                (functionality::button_interaction, functionality::text_input)
                    .run_if(in_state(AppMode::StartMenu)),
            )
            .add_systems(OnExit(AppMode::StartMenu), despawn::<OnStartScreen>);
    }
}

mod functionality {
    use std::time::Duration;

    use bevy::prelude::{
        BackgroundColor, Button, ButtonInput, Changed, EventReader, EventWriter, Interaction,
        KeyCode, NextState, Query, ReceivedCharacter, Res, ResMut, Text, With,
    };
    use helpers_for_bevy::entity_info_component::EntityInfoComponent;
    use helpers_for_bevy::status_text::events::SetStatusTextEvent;

    use crate::shared::app_mode::AppMode;
    use crate::shared::app_state::AppStateResource;
    use crate::shared::local_models::local_game_state::LocalGameStateResource;
    use crate::shared::{BUTTON_COLOR_HOVERED, BUTTON_COLOR_NORMAL, BUTTON_COLOR_PRESSED};
    use crate::start_screen::start_screen_plugin::{ButtonPurpose, PlayerNameLabelComponent};

    /// Provides button functionality, including state changes as well as response when clicked.
    #[allow(clippy::type_complexity)] // The query is complex by necessity.
    pub(super) fn button_interaction(
        mut app_state: ResMut<AppStateResource>,
        mut event_writer: EventWriter<SetStatusTextEvent>,
        mut interactions: Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &EntityInfoComponent<ButtonPurpose>,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        mut next_state: ResMut<NextState<AppMode>>,
    ) {
        for (interaction, mut color, button_info) in &mut interactions {
            match *interaction {
                Interaction::Hovered => {
                    *color = BackgroundColor(*BUTTON_COLOR_HOVERED);
                }
                Interaction::None => {
                    *color = BackgroundColor(*BUTTON_COLOR_NORMAL);
                }
                Interaction::Pressed => {
                    //

                    *color = BackgroundColor(*BUTTON_COLOR_PRESSED);

                    // Validate the Player Name entry...
                    if !app_state.local_player.display_name.trim().is_empty() {
                        // Which button was pressed?
                        match button_info.get_purpose() {
                            ButtonPurpose::AcceptInvitation => {
                                app_state.local_player_initiated_game = false;
                                next_state.set(AppMode::EnterInvitation);
                            }
                            ButtonPurpose::StartTwoPlayerGame => {
                                // Reflect the fact that this local Player is the one who initiated the Game.
                                app_state.local_player_initiated_game = true;
                                app_state.is_two_player_game = true;
                                next_state.set(AppMode::GamePlay);
                            }
                            ButtonPurpose::StartSinglePlayerGame => {
                                // Reflect the fact that this local Player is the one who initiated the Game.
                                app_state.local_player_initiated_game = true;
                                app_state.is_two_player_game = false;
                                next_state.set(AppMode::GamePlay);
                            }
                        }
                    } else {
                        // Ask the user to enter their display name.
                        let event = SetStatusTextEvent::new_with_duration(
                            "Please enter your name.",
                            Duration::from_secs(10),
                        );
                        event_writer.send(event);
                    }
                }
            }
        }
    }

    /// Clears the relevant values of the local Game state.
    pub(super) fn reset_game_play(
        mut app_state: ResMut<AppStateResource>,
        mut local_game_state: ResMut<LocalGameStateResource>,
    ) {
        // Forget everything except for the local Player's chosen name.
        let local_player_name = app_state.local_player.display_name.clone();
        app_state.reset();
        app_state.local_player.display_name = local_player_name;

        // TODO: JD: BUG: in some cases, the winning game slots remain highlighted when starting a
        // new game.

        local_game_state.reset();
    }

    /// Provides keyboard input and rudimentary editing for the Player display name text field.
    pub(super) fn text_input(
        mut app_state: ResMut<AppStateResource>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut event_reader: EventReader<ReceivedCharacter>,
        mut player_name_label_text: Query<&mut Text, With<PlayerNameLabelComponent>>,
    ) {
        //

        if keyboard_input.just_pressed(KeyCode::Backspace)
            || keyboard_input.just_pressed(KeyCode::Delete)
        {
            // Character removal
            let len = app_state.local_player.display_name.len();
            if len > 0 {
                app_state.local_player.display_name.pop();
            }
        } else {
            //

            // Add the characters, ignoring non-alphanumeric characters.
            for received_char in event_reader.read() {
                let char = received_char
                    .char
                    .to_string()
                    .chars()
                    .next()
                    .unwrap_or_default();

                if char.is_alphanumeric() {
                    app_state.local_player.display_name.push(char);
                }
            }
        }

        // Update the UI with the latest edits.
        if let Ok(mut label) = player_name_label_text.get_single_mut() {
            label.sections[0].value = app_state.local_player.display_name.clone();
        }
    }
}

mod ui {
    use bevy::prelude::{
        default, AlignItems, BackgroundColor, BorderColor, BuildChildren, ButtonBundle, Color,
        Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, Style, TextBundle,
        TextSection, TextStyle, UiRect, Val,
    };
    use helpers_for_bevy::entity_info_component::EntityInfoComponent;

    use crate::shared::{BUTTON_COLOR_NORMAL, FONT_SIZE, TEXT_COLOR};
    use crate::start_screen::start_screen_plugin::ButtonPurpose::{AcceptInvitation, StartSinglePlayerGame, StartTwoPlayerGame};
    use crate::start_screen::start_screen_plugin::{OnStartScreen, PlayerNameLabelComponent};

    /// Sets up and then spawns the Start Screen UI widgets.
    pub(super) fn spawn_buttons(mut commands: Commands) {
        //

        // TODO: JD: localize this text
        let title_start_two_player = "Two-Player Game";
        let title_start_single_player = "Single-Player Game";
        let title_invitation = "Accept An Invitation";
        let title_instructions = "Please type your name:";

        let text_style = TextStyle {
            font: default(),
            font_size: FONT_SIZE,
            color: *TEXT_COLOR,
        };

        // Instructions Label
        let sections = [TextSection::new(title_instructions, text_style.clone())];
        let label_style = Style {
            display: Default::default(),
            justify_self: JustifySelf::Center,
            ..default()
        };
        let instructions_label_bundle =
            TextBundle::from_sections(sections.clone()).with_style(label_style);

        // Player Name
        let sections2 = [TextSection::new("", text_style.clone())];
        let label_style = Style {
            border: UiRect::all(Val::Px(2.)),
            display: Default::default(),
            justify_self: JustifySelf::Center,
            ..default()
        };
        let player_name_bundle =
            TextBundle::from_sections(sections2.clone()).with_style(label_style);

        // Button style
        let button_bundle = ButtonBundle {
            style: Style {
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.)),
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(16.)),
                ..default()
            },
            border_color: BorderColor(Color::GRAY),
            background_color: BackgroundColor(*BUTTON_COLOR_NORMAL),
            ..default()
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(10.),
                        flex_direction: FlexDirection::Column,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                OnStartScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                padding: UiRect::all(Val::Px(10.)),
                                row_gap: Val::Px(10.),
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        OnStartScreen,
                    ))
                    .with_children(|parent| {
                        parent.spawn((instructions_label_bundle, OnStartScreen));
                        parent.spawn((player_name_bundle, OnStartScreen, PlayerNameLabelComponent));
                    });
                parent
                    // The Buttons
                    .spawn((
                        NodeBundle {
                            style: Style {
                                align_items: AlignItems::Center,
                                column_gap: Val::Px(10.),
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                row_gap: Val::Px(10.),
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        OnStartScreen,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                button_bundle.clone(),
                                EntityInfoComponent::new(StartTwoPlayerGame),
                                OnStartScreen,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(title_start_two_player, text_style.clone()),
                                    OnStartScreen,
                                ));
                            });
                        parent
                            .spawn((
                                button_bundle.clone(),
                                EntityInfoComponent::new(StartSinglePlayerGame),
                                OnStartScreen,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(title_start_single_player, text_style.clone()),
                                    OnStartScreen,
                                ));
                            });
                        parent
                            .spawn((
                                button_bundle,
                                EntityInfoComponent::new(AcceptInvitation),
                                OnStartScreen,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(title_invitation, text_style.clone()),
                                    OnStartScreen,
                                ));
                            });
                    });
            });
    }
}
