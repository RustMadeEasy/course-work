use bevy::app::{App, FixedUpdate, Plugin, Startup};

/// Provides the UI and functionality for the on-screen Game Controls.
pub(crate) struct GameControllerPlugin;

impl Plugin for GameControllerPlugin {
    //

    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .add_systems(Startup, ui::spawn_buttons)
            .add_systems(FixedUpdate, functionality::button_interaction)
            .add_systems(FixedUpdate, functionality::update_button_image);
    }
}

mod functionality {
    use bevy::asset::AssetServer;
    use bevy::prelude::{
        Button, Changed, DetectChanges, Interaction, NextState, Query, Res, ResMut, State, UiImage,
        With,
    };
    use helpers_for_bevy::entity_info_component::EntityInfoComponent;

    use crate::game_controller::game_controller_plugin::ButtonPurpose;
    use crate::game_controller::{GamePlayState, SoundSetting};

    /// Responds to Button clicks.
    #[allow(clippy::type_complexity)] // The query is complex by necessity.
    pub(super) fn button_interaction(
        mut interactions: Query<
            (&Interaction, &EntityInfoComponent<ButtonPurpose>),
            (Changed<Interaction>, With<Button>),
        >,
        state_game_play: Res<State<GamePlayState>>,
        mut next_game_play: ResMut<NextState<GamePlayState>>,
        state_sound: Res<State<SoundSetting>>,
        mut next_state_sound: ResMut<NextState<SoundSetting>>,
    ) {
        for (interaction, button_info) in &mut interactions {
            if *interaction == Interaction::Pressed {
                //
                // Which button was pressed?
                match button_info.get_purpose() {
                    ButtonPurpose::TogglePlay => match state_game_play.get() {
                        GamePlayState::Playing => next_game_play.set(GamePlayState::Paused),
                        GamePlayState::Paused => next_game_play.set(GamePlayState::Playing),
                    },
                    ButtonPurpose::ToggleSound => match state_sound.get() {
                        SoundSetting::On => next_state_sound.set(SoundSetting::Off),
                        SoundSetting::Off => next_state_sound.set(SoundSetting::On),
                    },
                }
            }
        }
    }

    /// Places the appropriate image Button into the Button according to the current state.
    pub(super) fn update_button_image(
        asset_server: Res<AssetServer>,
        mut query: Query<(&mut UiImage, &EntityInfoComponent<ButtonPurpose>), With<Button>>,
        state_game_play: Res<State<GamePlayState>>,
        state_sound: Res<State<SoundSetting>>,
    ) {
        //

        if state_game_play.is_changed() {
            for (mut button_image, button_info) in &mut query {
                //
                // Which button was pressed?
                if button_info.get_purpose() == ButtonPurpose::TogglePlay {
                    match state_game_play.get() {
                        GamePlayState::Playing => {
                            *button_image = UiImage::new(asset_server.load("sprites/pause.png"));
                        }
                        GamePlayState::Paused => {
                            *button_image = UiImage::new(asset_server.load("sprites/play.png"));
                        }
                    }
                }
            }
        }

        if state_sound.is_changed() {
            for (mut button_image, button_info) in &mut query {
                //
                // Which button was pressed?
                if button_info.get_purpose() == ButtonPurpose::ToggleSound {
                    match state_sound.get() {
                        SoundSetting::On => {
                            *button_image = UiImage::new(asset_server.load("sprites/sound_on.png"));
                        }
                        SoundSetting::Off => {
                            *button_image =
                                UiImage::new(asset_server.load("sprites/sound_off.png"));
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, PartialEq)]
enum ButtonPurpose {
    TogglePlay,
    ToggleSound,
}

mod ui {
    use bevy::asset::AssetServer;
    use bevy::prelude::{
        default, AlignItems, BuildChildren, ButtonBundle, Commands, FlexDirection, JustifyContent,
        NodeBundle, Res, Style, UiRect, Val,
    };
    use bevy::ui::UiImage;
    use helpers_for_bevy::entity_info_component::EntityInfoComponent;

    use crate::game_controller::game_controller_plugin::ButtonPurpose;

    /// Sets up and then spawns the UI widgets.
    pub(super) fn spawn_buttons(mut commands: Commands, asset_server: Res<AssetServer>) {
        //

        // Button style
        let button_style = Style {
            left: Val::Px(0.),
            top: Val::Px(0.),
            align_items: AlignItems::Center,
            border: UiRect::all(Val::Px(1.)),
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(16.)),
            ..default()
        };

        let play_pause_button_bundle = ButtonBundle {
            style: button_style.clone(),
            image: UiImage::new(asset_server.load("sprites/play.png")),
            ..default()
        };

        let sound_toggle_button_bundle = ButtonBundle {
            style: button_style.clone(),
            image: UiImage::new(asset_server.load("sprites/sound_off.png")),
            ..default()
        };

        commands
            .spawn((NodeBundle {
                style: Style {
                    align_items: AlignItems::Start,
                    column_gap: Val::Px(10.),
                    flex_direction: FlexDirection::Row,
                    height: Val::Percent(10.0),
                    justify_content: JustifyContent::End,
                    padding: UiRect::all(Val::Px(40.)),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn((
                    play_pause_button_bundle,
                    EntityInfoComponent::new(ButtonPurpose::TogglePlay),
                ));
                parent.spawn((
                    sound_toggle_button_bundle,
                    EntityInfoComponent::new(ButtonPurpose::ToggleSound),
                ));
            });
    }
}
