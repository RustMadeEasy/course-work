/*
 * Tic-Tac-Toe Service
 *
 * Tic-Tac-Toe Game Service
 *
 * The version of the OpenAPI document: 0.4.0
 * Contact: Info@RustMadeEasy.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// BoardPosition : Models a position on the game board.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoardPosition {
    #[serde(rename = "column")]
    pub column: i32,
    #[serde(rename = "row")]
    pub row: i32,
}

impl BoardPosition {
    /// Models a position on the game board.
    pub fn new(column: i32, row: i32) -> BoardPosition {
        BoardPosition {
            column,
            row,
        }
    }
}

