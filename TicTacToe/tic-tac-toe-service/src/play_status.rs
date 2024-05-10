use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Lists valid game play statuses.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum PlayStatus {
    EndedInStalemate,
    EndedInWin,
    InProgress,
    #[default]
    NotStarted,
}
