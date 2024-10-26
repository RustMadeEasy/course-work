// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use serde::Deserialize;
use utoipa::ToSchema;

/// The skill level at which the Automatic Player is to play.
#[derive(Clone, Debug, Default, Deserialize, ToSchema)]
pub enum AutomaticPlayerSkillLevel {
    /// Performs random moves.
    #[default]
    Beginner,
    /// Takes best tactical moves.
    Intermediate,
    /// Takes the best strategic moves, looking several moves into the future.
    Expert,
    /// Expands on the expert level by also making moves that can trick the other player into making
    /// wrong moves.
    Master,
}
