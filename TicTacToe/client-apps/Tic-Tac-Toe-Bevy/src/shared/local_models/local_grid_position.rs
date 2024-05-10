/// Specifies a location on the Tic-Tac-Toe 3x3 grid.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LocalGridPosition {
    pub(crate) column: usize,
    pub(crate) row: usize,
}

impl LocalGridPosition {
    pub(crate) fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}
