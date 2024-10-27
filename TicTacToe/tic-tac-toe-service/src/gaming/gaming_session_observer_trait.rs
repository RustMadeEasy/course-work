// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::game_trait::GameTrait;
use crate::gaming::gaming_session::GamingSession;
use crate::gaming::gaming_session_state_changes::GamingSessionStateChanges;
use async_trait::async_trait;

/// Defines the behavior of a Gaming Session observer
#[async_trait]
pub(crate) trait GamingSessionObserverTrait<T: GameTrait + Clone + Send + Sync + 'static> {
    //

    /// Called when the specified Gaming Session has been updated
    async fn session_updated(&self, state_change: &GamingSessionStateChanges, session: &GamingSession<T>, game: Option<T>);

    /// Property accessor for the Observer's unique ID
    fn unique_id(&self) -> String;
}
