use bevy::prelude::{
    default, AlignSelf, Color, JustifySelf, Resource, Style, TextStyle, Timer, TimerMode, UiRect,
    Val,
};
use std::time::Duration;

//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author Info@RustMadeEasy.com

/// Encapsulates the StatusTextPlugin style settings and text-clearing Timer.
#[derive(Clone, Resource)]
pub(super) struct StatusTextSettingsResource {
    pub(super) text_clearing_timer: Timer,
    pub(super) style: Style,
    pub(super) text_style: TextStyle,
}

impl StatusTextSettingsResource {
    //

    fn new_paused_timer() -> Timer {
        let mut timer = Timer::new(Duration::from_secs(0), TimerMode::Once);
        timer.pause();
        timer
    }

    /// Creates a new StatusTextSettingsResource instance.
    pub(super) fn new_with_style(text_style: TextStyle, style: Style) -> Self {
        Self {
            text_clearing_timer: Self::new_paused_timer(),
            style,
            text_style,
        }
    }
}

/// Provides default instantiation.
impl Default for StatusTextSettingsResource {
    fn default() -> Self {
        Self {
            text_clearing_timer: Self::new_paused_timer(),
            text_style: TextStyle {
                color: Color::GRAY,
                font: Default::default(),
                font_size: 18.0,
            },
            style: Style {
                align_self: AlignSelf::End,
                display: Default::default(),
                justify_self: JustifySelf::Center,
                margin: UiRect::all(Val::Px(10.)),
                ..default()
            },
        }
    }
}
