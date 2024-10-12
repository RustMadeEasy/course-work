// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::game_trait::GameTrait;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::PlayerInfo;
use log::debug;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use verification_code_gen::verification_code_generator::VerificationCodeGenerator;

/// A GamingSession is the context under which Players communicate and play one or more Games.
#[derive(Clone, Default, Deserialize, Serialize, ToSchema, Validate)]
pub(crate) struct GamingSession<T: GameTrait + Clone + Send + Sync + 'static> {
    pub(crate) current_game: Option<T>,
    // pub(crate) current_game_id: Option<String>,
    pub(crate) event_plane_config: EventPlaneConfig,
    /// Identifies the Gaming Session. This also serves as the communication channel for MQTT notifications.
    pub(crate) session_id: String,
    /// Unique Code that is used to invite others to the Gaming Session.
    pub(crate) invitation_code: String,
    pub(crate) participants: Vec<PlayerInfo>,
    pub(crate) session_owner: PlayerInfo,
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSession<T> {
    //

    /// Adds a new participant.
    pub(crate) fn add_participant(&mut self, player_info: &PlayerInfo) {
        self.participants.push(player_info.clone());
    }

    /// Clears all Game-related fields.
    pub(crate) fn clear_game(&mut self) {
        self.current_game = None;
    }

    /// Creates a unique, 6-digit code for use as an Invitation.
    ///
    /// NOTE: We use a 6-digit Invitation Code instead of performing the handshaking
    /// with the Gaming Session ID for two reasons:
    ///     1) We don't want to expose the Session ID to clients.
    ///     2) A 6-digit code is practical for end-users to utilize.
    fn generate_invitation_code() -> String {
        debug!("GamingSession: generate_invitation_code() called.");
        VerificationCodeGenerator::generate()
    }

    /// Returns the configuration clients can use to subscribe to game change events.
    pub(crate) fn get_event_plane_config(&self) -> EventPlaneConfig {
        self.event_plane_config.clone()
    }

    /// Creates a new GamingSession instance.
    pub(crate) fn new(session_owner: PlayerInfo, broker_address: String, broker_port: u16) -> Self {
        Self {
            current_game: None,
            event_plane_config: EventPlaneConfig::new(broker_address, broker_port, Uuid::new_v4().to_string()),
            session_id: Uuid::new_v4().to_string(),
            invitation_code: Self::generate_invitation_code(),
            participants: vec![session_owner.clone()],
            session_owner,
        }
    }

    /// Sets the current Game.
    pub(crate) fn set_game(&mut self, game: &T) {
        self.current_game = Some(game.clone());
    }
}
