/*
 * Tic-Tac-Toe Service
 *
 * Tic-Tac-Toe Game Service
 *
 * The version of the OpenAPI document: 0.4.0
 * Contact: JoelDavisEngineering@Gmail.com
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

/// GamePiece : Models a Game Piece with which the Tic-Tac-Toe game is played.
/// Models a Game Piece with which the Tic-Tac-Toe game is played.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum GamePiece {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "X")]
    X,
    #[serde(rename = "O")]
    O,

}

impl std::fmt::Display for GamePiece {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "None"),
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}

impl Default for GamePiece {
    fn default() -> GamePiece {
        Self::None
    }
}

