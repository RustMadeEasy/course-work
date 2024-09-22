use bevy::prelude::{in_state, App, Component, IntoSystemConfigs, OnEnter, OnExit, Plugin, Update};

use crate::invitation_screen::invite_screen_plugin::functionality::{
    button_interaction, text_input,
};
use crate::invitation_screen::invite_screen_plugin::ui::spawn_ui;
use crate::invitation_screen::OnInvitationScreen;
use crate::shared::app_mode::AppMode;
use crate::shared::despawn;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author Info@RustMadeEasy.com

const INVITATION_CODE_LENGTH: usize = 6;

/// Implements the Game's Invitation Screen.
pub(crate) struct InvitationScreenPlugin;

impl Plugin for InvitationScreenPlugin {
    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .add_systems(OnEnter(AppMode::EnterInvitation), spawn_ui)
            .add_systems(
                Update,
                (button_interaction, text_input).run_if(in_state(AppMode::EnterInvitation)),
            )
            .add_systems(
                OnExit(AppMode::EnterInvitation),
                despawn::<OnInvitationScreen>,
            );
    }
}

#[derive(Component)]
struct InvitationCodeLabelComponent;

mod functionality {
    use std::time::Duration;

    use bevy::prelude::{
        BackgroundColor, Button, ButtonInput, Changed, EventReader, EventWriter, Interaction,
        KeyCode, NextState, Query, ReceivedCharacter, Res, ResMut, Text, With,
    };
    use helpers_for_bevy::entity_info_component::EntityInfoComponent;
    use helpers_for_bevy::status_text::events::SetStatusTextEvent;

    use crate::invitation_screen::invite_screen_plugin::{
        InvitationCodeLabelComponent, INVITATION_CODE_LENGTH,
    };
    use crate::invitation_screen::ButtonPurpose;
    use crate::shared::app_mode::AppMode;
    use crate::shared::local_models::local_game_state::LocalGameStateResource;
    use crate::shared::{BUTTON_COLOR_HOVERED, BUTTON_COLOR_NORMAL, BUTTON_COLOR_PRESSED};

    /// Provides button functionality, including state changes as well as response when clicked.
    #[allow(clippy::type_complexity)] // The query is complex by necessity.
    pub(super) fn button_interaction(
        mut event_writer: EventWriter<SetStatusTextEvent>,
        local_game_state: ResMut<LocalGameStateResource>,
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

                    match button_info.get_purpose() {
                        ButtonPurpose::BackToStartScreen => next_state.set(AppMode::StartMenu),
                        ButtonPurpose::BeginGame => {
                            if local_game_state.invitation_code.trim().len()
                                == INVITATION_CODE_LENGTH
                            {
                                next_state.set(AppMode::GamePlay);
                            } else {
                                let event = SetStatusTextEvent::new_with_duration(
                                    "Please ask the other player for the 6-digit Invitation Code.",
                                    Duration::from_secs(10),
                                );
                                event_writer.send(event);
                            }
                        }
                    }
                }
            }
        }
    }

    pub(super) fn text_input(
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut evr_char: EventReader<ReceivedCharacter>,
        mut local_game_state: ResMut<LocalGameStateResource>,
        mut invitation_code_label_text: Query<&mut Text, With<InvitationCodeLabelComponent>>,
    ) {
        let mut skip_character_capture = false;

        // Use the back-space to delete from the end.
        if keyboard_input.just_pressed(KeyCode::Backspace)
            || keyboard_input.just_pressed(KeyCode::Delete)
        {
            let len = local_game_state.invitation_code.len();
            if len > 0 {
                local_game_state.invitation_code.pop();
                skip_character_capture = true;
            }
        }

        if !skip_character_capture {
            // Capture only numeric characters.
            for received_char in evr_char.read() {
                let received_char = received_char
                    .char
                    .to_string()
                    .chars()
                    .next()
                    .unwrap_or_default();
                if local_game_state.invitation_code.len() < INVITATION_CODE_LENGTH
                    && received_char.is_numeric()
                {
                    local_game_state.invitation_code.push(received_char);
                } else {
                    return;
                }
            }
        }

        // Update the UI
        if let Ok(mut label) = invitation_code_label_text.get_single_mut() {
            label.sections[0].value = local_game_state.invitation_code.clone();
        }
    }
}

mod ui {
    use bevy::prelude::{
        default, AlignItems, BackgroundColor, BorderColor, BuildChildren, ButtonBundle, Color,
        Commands, FlexDirection, JustifyContent, JustifySelf, NodeBundle, ResMut, Style,
        TextBundle, TextSection, TextStyle, UiRect, Val,
    };
    use helpers_for_bevy::entity_info_component::EntityInfoComponent;

    use crate::invitation_screen::invite_screen_plugin::InvitationCodeLabelComponent;
    use crate::invitation_screen::{ButtonPurpose, OnInvitationScreen};
    use crate::shared::app_state::AppStateResource;
    use crate::shared::local_models::local_game_state::LocalGameStateResource;
    use crate::shared::{BUTTON_COLOR_NORMAL, FONT_SIZE, TEXT_COLOR};

    pub(super) fn spawn_ui(
        mut commands: Commands,
        mut app_state: ResMut<AppStateResource>,
        mut local_game_state: ResMut<LocalGameStateResource>,
    ) {
        //

        // Ensure a good state from which to set the Invitation Code
        app_state.local_player_initiated_game = false;
        local_game_state.invitation_code = "".to_string();

        // TODO: JD: localize the text
        let title_back = "Go Back";
        let title_start = "Join Game";
        let title_instructions = "Please enter the Invitation Code: ";

        let text_style = TextStyle {
            color: *TEXT_COLOR,
            font: Default::default(),
            font_size: FONT_SIZE,
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

        // Invitation Code
        let sections = [TextSection::new("", text_style.clone())];
        let label_style = Style {
            border: UiRect::all(Val::Px(2.)),
            display: Default::default(),
            justify_self: JustifySelf::Center,
            ..default()
        };
        let invitation_code_label_bundle =
            TextBundle::from_sections(sections.clone()).with_style(label_style);

        // Button template
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
        let button_text_style = TextStyle {
            font: default(),
            font_size: FONT_SIZE,
            color: *TEXT_COLOR,
        };

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        aspect_ratio: None,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        flex_grow: 0.0,
                        flex_shrink: 0.0,
                        row_gap: Val::Px(10.),
                        column_gap: Val::Px(10.),
                        ..default()
                    },
                    ..default()
                },
                OnInvitationScreen,
            ))
            .with_children(|parent| {
                //

                parent.spawn((instructions_label_bundle, OnInvitationScreen));

                parent.spawn((
                    invitation_code_label_bundle,
                    OnInvitationScreen,
                    InvitationCodeLabelComponent,
                ));

                parent
                    .spawn((
                        NodeBundle {
                            style: Style {
                                width: Val::Percent(100.0),
                                // height: Val::Percent(100.0),
                                aspect_ratio: None,
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::Row,
                                flex_grow: 0.0,
                                flex_shrink: 0.0,
                                row_gap: Val::Px(10.),
                                column_gap: Val::Px(10.),
                                ..default()
                            },
                            ..default()
                        },
                        OnInvitationScreen,
                    ))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                button_bundle.clone(),
                                EntityInfoComponent::new(ButtonPurpose::BeginGame),
                                OnInvitationScreen,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(
                                        title_start,
                                        button_text_style.clone(),
                                    ),
                                    OnInvitationScreen,
                                ));
                            });

                        parent
                            .spawn((
                                button_bundle.clone(),
                                EntityInfoComponent::new(ButtonPurpose::BackToStartScreen),
                                OnInvitationScreen,
                            ))
                            .with_children(|parent| {
                                parent.spawn((
                                    TextBundle::from_section(title_back, button_text_style.clone()),
                                    OnInvitationScreen,
                                ));
                            });
                    });
            });
    }
}
