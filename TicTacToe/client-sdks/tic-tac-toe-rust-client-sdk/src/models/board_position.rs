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

/// BoardPosition : Models a position on the Game board.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardPosition {
    /// The position's column
    #[serde(rename = "column")]
    pub column: i32,
    /// The position's row
    #[serde(rename = "row")]
    pub row: i32,
}

impl BoardPosition {
    /// Models a position on the Game board.
    pub fn new(column: i32, row: i32) -> BoardPosition {
        BoardPosition {
            column,
            row,
        }
    }
}

