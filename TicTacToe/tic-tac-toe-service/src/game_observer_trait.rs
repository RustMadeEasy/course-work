use crate::game_trait::GameTrait;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait GameObserverTrait<T: GameTrait + Clone + Send + Sync> {
    async fn game_updated(&self, game_state_change: &GameStateChange, game: &T);
}

pub(crate) enum GameStateChange {
    PlayerAdded,
    TurnTaken,
}
