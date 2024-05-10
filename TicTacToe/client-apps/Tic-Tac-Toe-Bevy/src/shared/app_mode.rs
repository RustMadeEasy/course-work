use bevy::prelude::States;

/// Indicates the operating mode of the app.
#[derive(Clone, Debug, Eq, Hash, PartialEq, States)]
pub(crate) enum AppMode {
    EnterInvitation,
    GamePlay,
    StartMenu,
}
