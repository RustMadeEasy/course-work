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

/// NewGameParams : Models info needed to start a new Game.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NewGameParams {
    #[serde(rename = "player_one_display_name")]
    pub player_one_display_name: String,
}

impl NewGameParams {
    /// Models info needed to start a new Game.
    pub fn new(player_one_display_name: String) -> NewGameParams {
        NewGameParams {
            player_one_display_name,
        }
    }
}

