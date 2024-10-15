use crate::game_trait::GameTrait;
use crate::gaming_session::GamingSession;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait GameObserverTrait<T: GameTrait + Clone + Send + Sync + 'static> {
    async fn session_updated(&self, state_change: &StateChanges, session: &GamingSession<T>, game: Option<&T>);
    fn unique_id(&self) -> String;
}

#[derive(Debug)]
pub(crate) enum StateChanges {
    GameDeleted,
    GameStarted,
    GameTurnTaken,
    PlayerReady,
    PlayerAddedToSession,
    SessionDeleted,
}
