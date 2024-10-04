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

/// AddPlayerParams : Models info needed to add a player to a game.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddPlayerParams {
    #[serde(rename = "game_invitation_code")]
    pub game_invitation_code: String,
    #[serde(rename = "player_display_name")]
    pub player_display_name: String,
}

impl AddPlayerParams {
    /// Models info needed to add a player to a game.
    pub fn new(game_invitation_code: String, player_display_name: String) -> AddPlayerParams {
        AddPlayerParams {
            game_invitation_code,
            player_display_name,
        }
    }
}

