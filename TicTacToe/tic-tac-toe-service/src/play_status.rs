// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Lists valid Game play statuses.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize, ToSchema)]
pub(crate) enum PlayStatus {
    EndedInStalemate,
    EndedInWin,
    InProgress,
    #[default]
    NotStarted,
}
