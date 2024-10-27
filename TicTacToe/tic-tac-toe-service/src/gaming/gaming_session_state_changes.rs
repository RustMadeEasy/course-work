// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

/// Defines the changes in state for a Gaming Session
#[derive(Debug)]
pub(crate) enum GamingSessionStateChanges {
    /// All Players are ready to play
    AllPlayersReady,
    /// The Game has been deleted
    GameDeleted,
    /// The new Game Turn has been taken
    GameTurnTaken,
    /// The Gaming Session has been deleted
    GamingSessionDeleted,
}
