// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use serde::{Deserialize, Serialize};
use strum::Display;
use utoipa::ToSchema;

/// Domain to use for the base of the topic prefix
const DOMAIN_NAME: &str = "RustMadeEasy.com";

/// Models the configuration required for clients to subscribe to real-time Game state updates
#[derive(Clone, Default, Deserialize, Serialize, ToSchema)]
pub struct EventPlaneConfig {
    //

    /// Address of the real-time messaging broker
    pub broker_address: String,

    /// Channel used to namespace the messaging
    pub channel_id: String,

    /// Broker port number of the real-time messaging broker
    pub broker_port: u16,

    /// The topic prefix that allows the clients to subscribe to real-time Game state updates
    pub topic_prefix: String,
}

impl EventPlaneConfig {
    /// Creates a new instance
    pub fn new(broker_address: String, broker_port: u16, channel_id: String) -> Self {
        Self {
            broker_address,
            channel_id: channel_id.clone(),
            broker_port,
            topic_prefix: EventPlaneTopicNames::build_topic_prefix(&channel_id),
        }
    }
}

/// Defines the names of the subscription topics used in the real-time messaging event plane.
///
/// A full topic takes the form:
///
/// `[topic_prefix]/[event topic name]`
///
/// NOTE: The topic_prefix can be obtained from the event_plane_config field of the
/// GameCreationResponse model that is returned when creating or joining a Game.
#[derive(Deserialize, Display, Serialize, ToSchema)]
pub enum EventPlaneTopicNames {
    //

    /// Published when all Players are ready to begin the Game
    AllPlayersReady,

    /// Published when the Game has been deleted from the platform
    GameDeleted,

    /// Published when the Game has ended in a stalemate
    GameEndedInStalemate,

    /// Published when the Game has ended in a win
    GameEndedInWin,

    /// Published when the Game has started
    GameStarted,

    /// Published when the Gaming Session has been deleted from the platform
    SessionDeleted,

    /// Published when a Player has taken a new turn
    TurnTaken,
}

impl EventPlaneTopicNames {
    //

    /// Constructs a topic specific to the Session ID
    pub fn build(&self, topic_prefix: &str) -> String {
        match self {
            EventPlaneTopicNames::GameDeleted => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameDeleted),
            EventPlaneTopicNames::GameEndedInStalemate => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameEndedInStalemate),
            EventPlaneTopicNames::GameEndedInWin => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameEndedInWin),
            EventPlaneTopicNames::GameStarted => format!("{topic_prefix}/{}", EventPlaneTopicNames::GameStarted),
            EventPlaneTopicNames::AllPlayersReady => format!("{topic_prefix}/{}", EventPlaneTopicNames::AllPlayersReady),
            EventPlaneTopicNames::TurnTaken => format!("{topic_prefix}/{}", EventPlaneTopicNames::TurnTaken),
            EventPlaneTopicNames::SessionDeleted => format!("{topic_prefix}/{}", EventPlaneTopicNames::SessionDeleted),
        }
    }

    /// Constructs a topic prefix specific to the Channel ID
    fn build_topic_prefix(event_channel_id: &str) -> String {
        format!("{DOMAIN_NAME}/Channels/{event_channel_id}")
    }
}