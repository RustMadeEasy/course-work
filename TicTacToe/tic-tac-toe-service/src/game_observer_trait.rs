use crate::game_session::GamingSession;
use crate::game_trait::GameTrait;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait GameObserverTrait<T: GameTrait + Clone + Send + Sync + 'static> {
    async fn game_updated(&self, state_change: &StateChanges, session: &GamingSession<T>, game: &T);
    async fn session_updated(&self, state_change: &StateChanges, session: &GamingSession<T>);
    fn unique_id(&self) -> String;
}

#[derive(Debug)]
pub(crate) enum StateChanges {
    GameTurnTaken,
    PlayerAddedToGame,
    PlayerAddedToSession,
}
