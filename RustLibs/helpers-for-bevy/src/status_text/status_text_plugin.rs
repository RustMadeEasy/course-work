use bevy::prelude::{default, App, FixedUpdate, Plugin};
use bevy::prelude::{Startup, Style, TextStyle};

use crate::status_text::events::{ClearStatusTextEvent, SetStatusTextEvent};
use crate::status_text::status_text_resource::StatusTextSettingsResource;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author Info@RustMadeEasy.com

/// StatusTextPlugin displays text for a specified (or indefinite) period of time. By default, the
/// text is centered along the bottom of the window. The text style and view style are configurable.
#[derive(Default)]
pub struct StatusTextPlugin {
    settings_resource: StatusTextSettingsResource,
}

impl Plugin for StatusTextPlugin {
    /// Composes the plugin.
    fn build(&self, app: &mut App) {
        app //
            .insert_resource(self.settings_resource.clone())
            .add_event::<ClearStatusTextEvent>()
            .add_event::<SetStatusTextEvent>()
            .add_systems(Startup, ui::spawn_ui)
            .add_systems(
                FixedUpdate,
                (
                    functionality::handle_auto_clear_timer,
                    functionality::handle_clear_text_events,
                    functionality::handle_set_text_events,
                ),
            );
    }
}

impl StatusTextPlugin {
    //

    /// Creates a new StatusTextPlugin instance with the specified TextStyle.
    pub fn new_with_text_style(text_style: TextStyle) -> Self {
        Self {
            settings_resource: StatusTextSettingsResource::new_with_style(text_style, default()),
        }
    }

    /// Creates a new StatusTextPlugin instance with the specified TextStyle and window Style.
    pub fn new_with_text_and_window_style(text_style: TextStyle, window_style: Style) -> Self {
        Self {
            settings_resource: StatusTextSettingsResource::new_with_style(text_style, window_style),
        }
    }
}

mod functionality {
    use bevy::prelude::{EventReader, Query, Res, ResMut, Text, Time, Timer, TimerMode, With};

    use crate::status_text::events::{
        ClearStatusTextEvent, SetStatusTextEvent, StatusTextComponent,
    };
    use crate::status_text::status_text_resource::StatusTextSettingsResource;

    /// Clears the text when the time arrives, if the text had been set using a Duration.
    pub(super) fn handle_auto_clear_timer(
        mut settings: ResMut<StatusTextSettingsResource>,
        mut text: Query<&mut Text, With<StatusTextComponent>>,
        time: Res<Time>,
    ) {
        //

        // Was the text set with a Duration?
        if settings.text_clearing_timer.paused() {
            return;
        }

        // Bevy does not yet provide an automatic timer. So, we have manually tick the Timer each
        // Game frame.
        settings.text_clearing_timer.tick(time.delta());

        // If the Duration has elapsed, clear the text and pause our Timer.
        if settings.text_clearing_timer.finished() {
            if let Ok(mut text) = text.get_single_mut() {
                text.sections[0].value = "".to_string();
                settings.text_clearing_timer.pause();
            }
        }
    }

    /// Sets the text to empty in response to a ClearStatusTextEvent.
    pub(super) fn handle_clear_text_events(
        mut event_reader: EventReader<ClearStatusTextEvent>,
        mut text: Query<&mut Text, With<StatusTextComponent>>,
    ) {
        if let Ok(mut text) = text.get_single_mut() {
            for _ in event_reader.read() {
                text.sections[0].value = "".to_string();
            }
        }
    }

    /// Sets the text in response to a SetStatusTextEvent.
    pub(super) fn handle_set_text_events(
        mut event_reader: EventReader<SetStatusTextEvent>,
        mut settings: ResMut<StatusTextSettingsResource>,
        mut text: Query<&mut Text, With<StatusTextComponent>>,
    ) {
        //

        if let Ok(mut text) = text.get_single_mut() {
            //

            for event in event_reader.read() {
                //

                text.sections[0].value = event.new_text.clone();

                // If the caller specified a Duration, set our Timer.
                if let Some(duration) = event.duration {
                    settings.text_clearing_timer = Timer::new(duration, TimerMode::Once);
                }
            }
        }
    }
}

mod ui {
    use bevy::prelude::JustifyText::Center;
    use bevy::prelude::{Commands, Query, Res, TextBundle, TextSection, Window, With};
    use bevy::window::PrimaryWindow;

    use crate::status_text::events::StatusTextComponent;
    use crate::status_text::status_text_resource::StatusTextSettingsResource;

    /// Styles and then spawns the text widgets.
    pub(super) fn spawn_ui(
        mut commands: Commands,
        settings: Res<StatusTextSettingsResource>,
        _window_query: Query<&Window, With<PrimaryWindow>>,
    ) {
        //

        let sections = [TextSection::new("", settings.text_style.clone())];

        let text_bundle = TextBundle::from_sections(sections)
            .with_text_justify(Center)
            .with_style(settings.style.clone());

        // specify StatusTextComponent so that we can later query for the text widget.
        commands.spawn((text_bundle, StatusTextComponent));
    }
}
