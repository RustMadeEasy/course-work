// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::game_trait::GameTrait;
use crate::gaming::gaming_session::GamingSession;
use async_trait::async_trait;

/// Defines the behavior of a Gaming Session observer.
#[async_trait]
pub(crate) trait GamingSessionObserverTrait<T: GameTrait + Clone + Send + Sync + 'static> {
    async fn session_updated(&self, state_change: &GamingSessionStateChanges, session: &GamingSession<T>, game: Option<&T>);
    fn unique_id(&self) -> String;
}

/// Defines the changes in state for a Gaming Session. 
#[derive(Debug)]
pub(crate) enum GamingSessionStateChanges {
    GameDeleted,
    GameStarted,
    GameTurnTaken,
    PlayerReady,
    SessionDeleted,
}
