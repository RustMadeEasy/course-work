// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Models a position on the Game board
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema, Validate)]
pub(crate) struct BoardPosition {
    //
    
    /// The position's row
    #[validate(range(min = 0, max = 2))]
    pub(crate) row: usize,

    /// The position's column
    #[validate(range(min = 0, max = 2))]
    pub(crate) column: usize,
}

impl BoardPosition {
    /// Creates a new instance
    pub(crate) fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}