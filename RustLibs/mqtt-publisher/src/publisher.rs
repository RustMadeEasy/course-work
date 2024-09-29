// MQTT Publisher
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use std::collections::HashSet;

use crate::async_mqtt_client::AsyncMqttClient;
use crate::broker_info::BrokerInfo;
use crate::publisher_error::PublisherError;
use crate::publisher_qos::PublisherQoS;
use log::error;

/// Provides MQTT message publishing functionality, including simulcast to disparate brokers and
/// different versions of the MQTT protocol.
#[derive(Clone)]
pub struct Publisher {
    clients: Vec<AsyncMqttClient>,
}

// Public contract.
impl Publisher {
    //

    /// Constructs a new Publisher instance for broadcasting via a single broker.
    pub fn new(broker: BrokerInfo) -> Self {
        Self { clients: vec!(AsyncMqttClient::new(broker)) }
    }

    /// Constructs a new Publisher instance for simulcasting via multiple brokers.
    pub fn new_for_simulcast(brokers: HashSet<BrokerInfo>) -> Self {
        let mut clients: Vec<AsyncMqttClient> = vec!();
        for config in brokers {
            clients.push(AsyncMqttClient::new(config))
        }
        Self {
            clients,
        }
    }

    /// Publishes an empty message to the specified topic.
    pub async fn publish(&self, topic: &str, qos: PublisherQoS) -> Result<(), Vec<PublisherError>> {
        self.publish_with_payload("", topic, qos).await
    }

    /// Publishes a message with the specified payload to the specified topic.
    pub async fn publish_with_payload(&self, payload: &str, topic: &str, qos: PublisherQoS) -> Result<(), Vec<PublisherError>> {
        //

        let mut failed_results: Vec<PublisherError> = vec!();

        for client in self.clients.clone() {
            match client.publish_with_payload(payload.to_string(), topic.to_string(), qos.clone()).await {
                Ok(_) => {}
                Err(error) => {
                    error!("Failed to publish message: {}", error.to_string());
                    failed_results.push(error)
                }
            }
        }

        match failed_results.is_empty() {
            true => Ok(()),
            false => Err(failed_results)
        }
    }
}
