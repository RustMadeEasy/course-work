//  Tic-Tac-Toe Bevy Client App
//
//  © 2024 Rust Made Easy. All rights reserved.
//  @author JoelDavisEngineering@Gmail.com

use rumqttc::MqttOptions;
use std::time::Duration;
use tic_tac_toe_rust_client_sdk::models::{EventPlaneConfig, EventPlaneTopicNames};

/// Defines handler for game info events.
pub(crate) trait GameInfoReceiverDelegate {
    fn on_game_ended_in_stalemate();
    fn on_game_ended_in_win();
    fn on_player_added();
    fn on_turn_taken();
}

/// Listens for game state changes published by our Tic Tac Toe service.
pub(crate) struct GameInfoReceiver<T: GameInfoReceiverDelegate> {
    //

    client: rumqttc::Client,
    connection: rumqttc::Connection,
    delegate: T,

    topic_prefix: String,

    topic_game_ended_in_stalemate: String,
    topic_game_ended_in_win: String,
    topic_player_added: String,
    topic_turn_taken: String,
}

impl<T: GameInfoReceiverDelegate> GameInfoReceiver<T> {
    //

    pub(crate) fn new(event_plane_config: EventPlaneConfig, delegate: T) -> Self {
        //

        let (client, connection) = Self::setup_mqtt_client(&event_plane_config);

        let mut instance = Self {
            client,
            connection,
            delegate,
            topic_prefix: event_plane_config.topic_prefix,
            topic_game_ended_in_stalemate: "".to_string(),
            topic_game_ended_in_win: "".to_string(),
            topic_player_added: "".to_string(),
            topic_turn_taken: "".to_string(),
        };

        instance.prebuild_topics();

        instance
    }

    /// Builds a full topic string based on the specified topic name.
    fn build_topic(topic_prefix: &str, topic: EventPlaneTopicNames) -> String {
        format!("{}{}", topic_prefix, topic)
    }

    /// Pre-builds the topics so that we are not parsing each time a message is received.
    fn prebuild_topics(&mut self) {
        self.topic_game_ended_in_stalemate = Self::build_topic(&self.topic_prefix, EventPlaneTopicNames::GameEndedInStalemate);
        self.topic_game_ended_in_win = Self::build_topic(&self.topic_prefix, EventPlaneTopicNames::GameEndedInWin);
        self.topic_player_added = Self::build_topic(&self.topic_prefix, EventPlaneTopicNames::PlayerAdded);
        self.topic_turn_taken = Self::build_topic(&self.topic_prefix, EventPlaneTopicNames::TurnTaken);
    }

    fn setup_mqtt_client(event_plane_config: &EventPlaneConfig) -> (rumqttc::Client, rumqttc::Connection) {
        let mut mqttoptions = MqttOptions::new("", event_plane_config.broker_address.clone(), event_plane_config.broker_port as u16);
        mqttoptions.set_keep_alive(Duration::from_secs(5));
        rumqttc::Client::new(mqttoptions, 10)
    }
}
