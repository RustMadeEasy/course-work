/*
 * Tic-Tac-Toe Service
 *
 * Tic-Tac-Toe Game Service
 *
 * The version of the OpenAPI document: 0.2.0
 * Contact: Support@RustMadeEasy.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// PlayerInfo : * Models a Tic-Tac-Toe game Player.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Joel@RustMadeEasy.com Models a Tic-Tac-Toe game Player.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlayerInfo {
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "game_piece")]
    pub game_piece: models::GamePiece,
    #[serde(rename = "player_id")]
    pub player_id: String,
}

impl PlayerInfo {
    /// * Models a Tic-Tac-Toe game Player.  *  * © 2024 Rust Made Easy. All rights reserved.  * @author Joel@RustMadeEasy.com Models a Tic-Tac-Toe game Player.
    pub fn new(
        display_name: String,
        game_piece: models::GamePiece,
        player_id: String,
    ) -> PlayerInfo {
        PlayerInfo {
            display_name,
            game_piece,
            player_id,
        }
    }
}
