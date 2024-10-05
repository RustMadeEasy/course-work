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

/// PlayStatus : Lists valid Game play statuses.
/// Lists valid Game play statuses.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlayStatus {
    #[serde(rename = "EndedInStalemate")]
    EndedInStalemate,
    #[serde(rename = "EndedInWin")]
    EndedInWin,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "NotStarted")]
    NotStarted,

}

impl std::fmt::Display for PlayStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::EndedInStalemate => write!(f, "EndedInStalemate"),
            Self::EndedInWin => write!(f, "EndedInWin"),
            Self::InProgress => write!(f, "InProgress"),
            Self::NotStarted => write!(f, "NotStarted"),
        }
    }
}

impl Default for PlayStatus {
    fn default() -> PlayStatus {
        Self::EndedInStalemate
    }
}

