//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::prelude::States;

/// Indicates the operating mode of the app.
#[derive(Clone, Debug, Eq, Hash, PartialEq, States)]
pub(crate) enum AppMode {
    EnterInvitation,
    GamePlay,
    StartMenu,
}
