use crate::game_trait::GameTrait;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait GameObserverTrait<T: GameTrait + Clone + Send + Sync + 'static> {
    async fn game_updated(&self, game_state_change: &GameStateChange, game: &T);
    fn unique_id(&self) -> String;
}

pub(crate) enum GameStateChange {
    PlayerAdded,
    TurnTaken,
}
