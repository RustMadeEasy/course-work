// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Specifies the type of Game - single player or two players.
#[derive(Debug, Deserialize, PartialEq, Serialize, ToSchema, Clone)]
pub enum GameMode {
    SinglePlayer,
    TwoPlayers,
}