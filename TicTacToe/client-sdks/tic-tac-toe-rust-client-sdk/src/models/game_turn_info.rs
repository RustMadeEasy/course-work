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

/// GameTurnInfo : Models info needed to perform a Game turn.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GameTurnInfo {
    #[serde(rename = "destination")]
    pub destination: models::BoardPosition,
    #[serde(rename = "player_id")]
    pub player_id: String,
}

impl GameTurnInfo {
    /// Models info needed to perform a Game turn.
    pub fn new(destination: models::BoardPosition, player_id: String) -> GameTurnInfo {
        GameTurnInfo {
            destination,
            player_id,
        }
    }
}

