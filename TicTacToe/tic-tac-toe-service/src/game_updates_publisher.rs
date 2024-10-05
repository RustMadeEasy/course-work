use crate::game_observer_trait::{GameObserverTrait, GameStateChange};
use crate::game_trait::GameTrait;
use crate::models::event_plane::EventPlaneTopicNames;
use crate::play_status::PlayStatus;
use async_trait::async_trait;
use mqtt_publisher_lib::broker_info::{BrokerInfo, MqttProtocolVersion};
use mqtt_publisher_lib::publisher::Publisher;
use mqtt_publisher_lib::publisher_qos::PublisherQoS;
use std::time::Duration;

pub(crate) struct GameUpdatesPublisher {
    //

    /// Provides MQTT message publishing functionality.
    event_publisher: Publisher,
}

impl GameUpdatesPublisher {
    //

    pub(crate) fn new(broker_address: String, broker_port: u16) -> Self {
        let config = BrokerInfo::new(broker_address,
                                     10,
                                     broker_port,
                                     Duration::from_secs(60),
                                     MqttProtocolVersion::V5);
        Self { event_publisher: Publisher::new(config) }
    }
}

#[async_trait]
impl<T: GameTrait + Clone + Send + Sync> GameObserverTrait<T> for GameUpdatesPublisher {
    //

    async fn game_updated(&self, game_state_change: &GameStateChange, game: &mut T) {
        //

        let topic: String;
        let topic_prefix = game.get_event_plane_config().topic_prefix;
        let topic_prefix = topic_prefix.as_str();

        match game_state_change {
            GameStateChange::PlayerAdded => {
                topic = EventPlaneTopicNames::PlayerAdded.build(topic_prefix);
            }
            GameStateChange::TurnTaken => {
                match game.get_current_game_state().play_status {
                    PlayStatus::EndedInStalemate => topic = EventPlaneTopicNames::GameEndedInStalemate.build(topic_prefix),
                    PlayStatus::EndedInWin => topic = EventPlaneTopicNames::GameEndedInWin.build(topic_prefix),
                    PlayStatus::InProgress => topic = EventPlaneTopicNames::TurnTaken.build(topic_prefix),
                    PlayStatus::NotStarted => return, // Early return. Nothing to publish.
                }
            }
        }

        let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
    }
}
