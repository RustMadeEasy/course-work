// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::game_trait::GameTrait;
use crate::models::event_plane::EventPlaneConfig;
use crate::models::player_info::PlayerInfo;
use function_name::named;
use log::debug;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;
use verification_code_gen::verification_code_generator::VerificationCodeGenerator;

/// A GamingSession is the context under which Players communicate and play one or more Games.
#[derive(Clone, Default, Deserialize, Serialize, ToSchema, Validate)]
pub(crate) struct GamingSession<T: GameTrait + Clone + Send + Sync + 'static> {
    /// The Game currently being played.
    pub(crate) current_game: Option<T>,
    /// MQTT configuration info
    pub(crate) event_plane_config: EventPlaneConfig,
    /// Uniquely identifies the Gaming Session. This also serves as the communication channel for MQTT notifications.
    pub(crate) session_id: String,
    /// Unique Code that is used to invite others to the Gaming Session.
    pub(crate) invitation_code: String,
    /// List of Players in the Gaming Session.
    pub(crate) participants: Vec<PlayerInfo>,
    /// The Player who created the Gaming Session.
    pub(crate) session_owner: PlayerInfo,
}

impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSession<T> {
    //

    /// Adds a new participant (Player).
    #[named]
    pub(crate) fn add_participant(&mut self, player_info: &PlayerInfo) {
        debug!("{} called", function_name!());
        self.participants.push(player_info.clone());
    }

    /// Creates a unique, 6-digit code for use as an Invitation.
    #[named]
    fn generate_invitation_code() -> String {
        debug!("{} called", function_name!());
        VerificationCodeGenerator::generate()
    }

    /// Creates a new instance.
    #[named]
    pub(crate) fn new(session_owner: PlayerInfo, broker_address: String, broker_port: u16) -> Self {
        debug!("{} called", function_name!());
        Self {
            current_game: None,
            event_plane_config: EventPlaneConfig::new(broker_address, broker_port),
            session_id: Uuid::new_v4().to_string(),
            invitation_code: Self::generate_invitation_code(),
            participants: vec![session_owner.clone()],
            session_owner,
        }
    }
}
