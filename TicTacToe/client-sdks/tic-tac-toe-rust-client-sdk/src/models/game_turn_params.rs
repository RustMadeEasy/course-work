/*
 * Tic-Tac-Toe Service
 *
 * Tic-Tac-Toe Game Service
 *
 * The version of the OpenAPI document: 0.4.0
 * Contact: JoelDavisEngineering@Gmail.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// GameTurnParams : Models info needed to perform a Game turn
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameTurnParams {
    /// The location to which the Player's Game Piece is to be placed
    #[serde(rename = "destination")]
    pub destination: models::BoardPosition,
    /// ID of the Player whose turn is being taken
    #[serde(rename = "player_id")]
    pub player_id: String,
    /// ID of the Gaming Session
    #[serde(rename = "session_id")]
    pub session_id: String,
}

impl GameTurnParams {
    /// Models info needed to perform a Game turn
    pub fn new(destination: models::BoardPosition, player_id: String, session_id: String) -> GameTurnParams {
        GameTurnParams {
            destination,
            player_id,
            session_id,
        }
    }
}
