use std::collections::HashSet;
use std::time::Duration;

use crate::async_mqtt_client::AsyncMqttClient;
use crate::broker_config::PublisherConfig;
use crate::publisher_error::PublisherError;
use crate::publisher_qos::PublisherQoS;
use log::trace;

/// Provides MQTT message publishing functionality, including simulcast to disparate brokers and
/// different version of the MQTT protocol.
#[derive(Clone)]
pub struct Publisher {
    clients: Vec<AsyncMqttClient>,
}

// Public contract.
impl Publisher {
    //

    /// Constructs a new Publisher instance.
    pub fn new(configurations: HashSet<PublisherConfig>, keep_alive: Duration, capacity: usize) -> Self {
        //

        trace!("Constructing new Publisher instance");

        let mut clients: Vec<AsyncMqttClient> = vec!();

        for config in configurations {
            clients.push(AsyncMqttClient::new(config, keep_alive, capacity))
        }

        Self {
            clients,
        }
    }

    /// Publishes an empty message to the specified topic.
    pub async fn publish(&self, topic: &str, qos: PublisherQoS) -> Result<(), PublisherError> {
        self.publish_with_payload("", topic, qos).await
    }

    /// Publishes a message with the specified payload to the specified topic.
    pub async fn publish_with_payload(&self, payload: &str, topic: &str, qos: PublisherQoS) -> Result<(), PublisherError> {
        //

        let mut failed_results: Vec<PublisherError> = vec!();

        for client in self.clients.clone() {
            match client.publish_with_payload(payload.to_string(), topic.to_string(), qos.clone()).await {
                Ok(_) => {}
                Err(error) => {
                    failed_results.push(error)
                }
            }
        }

        return match failed_results.is_empty() {
            true => Ok(()),
            false => Err((*failed_results.first().unwrap()).clone())
        };
    }
}
