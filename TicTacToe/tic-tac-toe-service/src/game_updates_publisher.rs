use crate::game_observer_trait::{GameObserverTrait, GameStateChange};
use crate::game_state::GameState;
use crate::models::event_plane::EventPlaneTopicNames;
use async_trait::async_trait;
use mqtt_publisher_lib::broker_info::{BrokerInfo, MqttProtocolVersion};
use mqtt_publisher_lib::publisher::Publisher;
use mqtt_publisher_lib::publisher_qos::PublisherQoS;
use std::time::Duration;

pub(crate) struct GameUpdatesPublisher {
    //

    event_channel_id: String,

    /// Provides MQTT message publishing functionality.
    event_publisher: Publisher,
}

impl GameUpdatesPublisher {
    //

    pub(crate) fn new(event_channel_id: String,
                      mqtt_broker_address: impl Into<String>,
                      mqtt_port: u16,
    ) -> Self {
        let config = BrokerInfo::new(mqtt_broker_address.into(),
                                     10,
                                     mqtt_port,
                                     Duration::from_secs(60),
                                     MqttProtocolVersion::V5);
        Self {
            event_channel_id,
            event_publisher: Publisher::new(config),
        }
    }
}

#[async_trait]
impl GameObserverTrait for GameUpdatesPublisher {
    //

    async fn game_updated(&self, game_state_change: &GameStateChange, _new_game_state: &GameState) {
        match game_state_change {
            GameStateChange::GameEndedInStalemate => {
                let topic = EventPlaneTopicNames::GameEndedInStalemate.build(self.event_channel_id.as_str());
                let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
            }
            GameStateChange::GameEndedInWin => {
                let topic = EventPlaneTopicNames::GameEndedInWin.build(self.event_channel_id.as_str());
                let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
            }
            GameStateChange::PlayerAdded => {
                // Inform the listening clients that a Player has been added.
                let topic = EventPlaneTopicNames::PlayerAdded.build(self.event_channel_id.as_str());
                let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
            }
            GameStateChange::TurnTaken => {
                // Inform the listening clients that a Player has taken a new turn.
                let topic = EventPlaneTopicNames::TurnTaken.build(self.event_channel_id.as_str());
                let _ = self.event_publisher.publish(topic.as_str(), PublisherQoS::AtLeastOnce).await;
            }
        }
    }
}