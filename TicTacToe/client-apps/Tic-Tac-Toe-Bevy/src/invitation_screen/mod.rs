//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use bevy::prelude::Component;

pub(crate) mod invite_screen_plugin;

/// Marker to indicate that an entity was spawned on the Invitation Screen.
#[derive(Component)]
struct OnInvitationScreen;

/// Defines the purposes of the Invitation Screen buttons.
#[derive(Clone)]
pub(super) enum ButtonPurpose {
    BackToStartScreen,
    BeginGame,
}
