use serde::{Deserialize, Serialize};
// use utoipa_gen::derive_to_schema as ToSchema;
// use validator_derive::derive_validation as Validate;
use utoipa::ToSchema;
use validator::Validate;

/// Models a position on the Game board.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema, Validate)]
pub struct BoardPosition {
    #[validate(range(min = 0, max = 2))]
    pub(crate) row: usize,
    #[validate(range(min = 0, max = 2))]
    pub(crate) column: usize,
}

impl BoardPosition {
    /// Creates a new BoardPosition instance.
    pub(crate) fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}