use std::time::Duration;

use bevy::prelude::{Component, Event};

/// An event that clears the text of the Status Text Plugin.
#[derive(Event)]
pub struct ClearStatusTextEvent;

/// An event that sets the text of the Status Text Plugin.
#[derive(Event)]
pub struct SetStatusTextEvent {
    pub(crate) duration: Option<Duration>,
    pub(crate) new_text: String,
}

impl SetStatusTextEvent {
    //

    /// Creates a new SetStatusTextEvent instance.
    pub fn _new(new_text: impl Into<String>) -> Self {
        Self {
            duration: None,
            new_text: new_text.into(),
        }
    }

    /// Creates a new SetStatusTextEvent instance, specifying both the text and the display duration.
    pub fn new_with_duration(new_text: impl Into<String>, duration: Duration) -> Self {
        Self {
            duration: Some(duration),
            new_text: new_text.into(),
        }
    }
}

/// A marker to retrieve Status Text entities.
#[derive(Component)]
pub struct StatusTextComponent;
