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

/// GamingSessionCreationResult : Models the results of a call to the Create Gaming Session endpoint.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GamingSessionCreationResult {
    #[serde(rename = "event_plane_config")]
    pub event_plane_config: models::EventPlaneConfig,
    /// Unique Code that is used to invite others to the Gaming Session.
    #[serde(rename = "invitation_code")]
    pub invitation_code: String,
    /// Identifies the Gaming Session. This also serves as the communication channel for MQTT notifications.
    #[serde(rename = "session_id")]
    pub session_id: String,
}

impl GamingSessionCreationResult {
    /// Models the results of a call to the Create Gaming Session endpoint.
    pub fn new(event_plane_config: models::EventPlaneConfig, invitation_code: String, session_id: String) -> GamingSessionCreationResult {
        GamingSessionCreationResult {
            event_plane_config,
            invitation_code,
            session_id,
        }
    }
}

