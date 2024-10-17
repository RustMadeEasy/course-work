use crate::game_observer_trait::{GameObserverTrait, StateChanges};
use crate::game_trait::GameTrait;
use crate::gaming_session::GamingSession;
use crate::models::event_plane::EventPlaneTopicNames;
use crate::play_status::PlayStatus;
use async_trait::async_trait;
use log::debug;
use mqtt_publisher_lib::broker_info::{BrokerInfo, MqttProtocolVersion};
use mqtt_publisher_lib::publisher::Publisher;
use mqtt_publisher_lib::publisher_qos::PublisherQoS;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub(crate) struct GameUpdatesPublisher {
    //

    /// Provides MQTT message publishing functionality.
    event_publisher: Publisher,

    unique_id: String,
}

impl GameUpdatesPublisher {
    //

    pub(crate) fn new(broker_address: String, broker_port: u16) -> Self {
        let config = BrokerInfo::new(broker_address,
                                     10,
                                     broker_port,
                                     Duration::from_secs(60),
                                     MqttProtocolVersion::V5);
        Self { event_publisher: Publisher::new(config), unique_id: Uuid::new_v4().to_string() }
    }
}

#[async_trait]
impl<T: GameTrait + Clone + Send + Sync + 'static> GameObserverTrait<T> for GameUpdatesPublisher {
    //

    async fn session_updated(&self, state_change: &StateChanges, session: &GamingSession<T>, game: Option<&T>) {
        //

        debug!("GameUpdatesPublisher: received session_updated() for session {}", session.session_id);

        let topic_prefix = session.get_event_plane_config().topic_prefix;
        let topic_prefix = topic_prefix.as_str();

        let topic = match state_change {
            StateChanges::GameDeleted => {
                EventPlaneTopicNames::GameDeleted.build(topic_prefix)
            }
            StateChanges::GameStarted => {
                EventPlaneTopicNames::GameStarted.build(topic_prefix)
            }
            StateChanges::GameTurnTaken => {
                if let Some(game) = game {
                    match game.get_current_game_state().play_status {
                        PlayStatus::EndedInStalemate => EventPlaneTopicNames::GameEndedInStalemate.build(topic_prefix),
                        PlayStatus::EndedInWin => EventPlaneTopicNames::GameEndedInWin.build(topic_prefix),
                        PlayStatus::InProgress => EventPlaneTopicNames::TurnTaken.build(topic_prefix),
                        PlayStatus::NotStarted => return, // Early return. Nothing to publish.
                    }
                } else {
                    return; // Early return. Nothing to publish.
                }
            }
            StateChanges::PlayerAddedToSession => {
                EventPlaneTopicNames::PlayerAddedToSession.build(topic_prefix)
            }
            StateChanges::PlayerReady => {
                EventPlaneTopicNames::PlayerReady.build(topic_prefix)
            }
            StateChanges::SessionDeleted => {
                EventPlaneTopicNames::SessionDeleted.build(topic_prefix)
            }
        };

        let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
    }

    fn unique_id(&self) -> String {
        self.unique_id.clone()
    }
}
