use crate::game_state::GameState;
use async_trait::async_trait;

#[async_trait]
pub(crate) trait GameObserverTrait {
    async fn game_updated(&self, game_state_change: &GameStateChange, new_game_state: &GameState);
}

pub(crate) enum GameStateChange {
    GameEndedInStalemate,
    GameEndedInWin,
    PlayerAdded,
    TurnTaken,
}
