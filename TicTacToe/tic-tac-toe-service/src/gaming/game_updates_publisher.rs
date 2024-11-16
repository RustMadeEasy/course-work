// Tic-Tac-Toe Service
//
// Provides 2-client Game-play of Tic-Tac-Toe.
//
// © 2024 Rust Made Easy. All rights reserved.
// @author JoelDavisEngineering@Gmail.com

use crate::gaming::game_trait::GameTrait;
use crate::gaming::gaming_session::GamingSession;
use crate::gaming::gaming_session_observer_trait::GamingSessionObserverTrait;
use crate::gaming::gaming_session_state_changes::GamingSessionStateChanges;
use crate::models::event_plane::EventPlaneTopicNames;
use crate::models::play_status::PlayStatus;
use async_trait::async_trait;
use function_name::named;
use log::debug;
use mqtt_publisher_lib::broker_info::{BrokerInfo, MqttProtocolVersion};
use mqtt_publisher_lib::publisher::Publisher;
use mqtt_publisher_lib::publisher_qos::PublisherQoS;
use std::time::Duration;
use uuid::Uuid;

/// Observes Game updates and publishes the via MQTT.
#[derive(Clone)]
pub(crate) struct GameUpdatesPublisher {
    //

    /// Provides MQTT message publishing functionality.
    event_publisher: Publisher,

    /// Unique ID of this Publisher instance.
    unique_id: String,
}

impl GameUpdatesPublisher {
    //

    /// Creates a new instance.
    #[named]
    pub(crate) fn new(broker_address: String, broker_port: u16) -> Self {
        debug!("{} called", function_name!());
        let config = BrokerInfo::new(broker_address,
                                     10,
                                     broker_port,
                                     Duration::from_secs(60),
                                     MqttProtocolVersion::V5);
        Self { event_publisher: Publisher::new(config), unique_id: Uuid::new_v4().to_string() }
    }
}

#[async_trait]
impl<T: GameTrait + Clone + Send + Sync + 'static> GamingSessionObserverTrait<T> for GameUpdatesPublisher {
    //

    #[named]
    async fn session_updated(&self, state_change: &GamingSessionStateChanges, session: &GamingSession<T>, game: Option<T>) {
        //

        debug!("{} called for session {}", function_name!(), session.session_id);

        let event_channel_id = session.session_id.as_str();

        let topic = match state_change {
            GamingSessionStateChanges::GameDeleted => {
                EventPlaneTopicNames::GameDeleted.build(event_channel_id)
            }
            GamingSessionStateChanges::GameTurnTaken => {
                if let Some(game) = game {
                    match game.get_current_game_state().play_status {
                        PlayStatus::EndedInStalemate => EventPlaneTopicNames::GameEndedInStalemate.build(event_channel_id),
                        PlayStatus::EndedInWin => EventPlaneTopicNames::GameEndedInWin.build(event_channel_id),
                        PlayStatus::InProgress => EventPlaneTopicNames::TurnTaken.build(event_channel_id),
                        PlayStatus::NotStarted => return, // Early return. Nothing to publish.
                    }
                } else {
                    return; // Early return. Nothing to publish.
                }
            }
            GamingSessionStateChanges::GameIsReady => {
                EventPlaneTopicNames::AllPlayersReady.build(event_channel_id)
            }
            GamingSessionStateChanges::GamingSessionDeleted => {
                EventPlaneTopicNames::SessionDeleted.build(event_channel_id)
            }
        };

        let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
    }

    fn unique_id(&self) -> String {
        self.unique_id.clone()
    }
}
