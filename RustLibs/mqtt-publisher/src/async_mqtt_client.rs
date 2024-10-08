// MQTT Publisher
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use log::{debug, error, trace};
use rumqttc::v5::{AsyncClient as AsyncClientV5, EventLoop as EventLoopV5, MqttOptions as MqttOptionsV5};
use rumqttc::{AsyncClient as AsyncClientV3, EventLoop as EventLoopV3, MqttOptions as MqttOptionsV3};
use uuid::Uuid;

use crate::broker_info::{BrokerInfo, MqttProtocolVersion};
use crate::publisher_error::PublisherError;
use crate::publisher_qos::PublisherQoS;

/// Provides unified access to multiple client versions of the MQTT protocol.
#[derive(Clone)]
pub(crate) struct AsyncMqttClient {
    client_v3: Option<AsyncClientV3>,
    client_v5: Option<AsyncClientV5>,
}

impl AsyncMqttClient {
    //

    /// Constructs a new AsyncMqttClient instance.
    pub fn new(config: BrokerInfo) -> Self {
        //

        // Which version of the client is being requested?
        match config.protocol_version {
            //

            // Setup a V3 client
            MqttProtocolVersion::V3 => {
                let mut client_options = MqttOptionsV3::new(Uuid::new_v4().to_string(),
                                                            config.broker_address.clone(),
                                                            config.broker_port);
                client_options.set_keep_alive(config.keep_alive);

                let (client, event_loop) = AsyncClientV3::new(client_options, config.capacity);

                tokio::spawn(async move {
                    AsyncMqttClient::enter_event_loop_v3(event_loop).await;
                });

                Self {
                    client_v3: Some(client),
                    client_v5: None,
                }
            }

            // Setup a V5 client
            MqttProtocolVersion::V5 => {
                let mut client_options = MqttOptionsV5::new(Uuid::new_v4().to_string(),
                                                            config.broker_address.clone(),
                                                            config.broker_port);
                client_options.set_keep_alive(config.keep_alive);

                let (client, event_loop) = AsyncClientV5::new(client_options, config.capacity);

                tokio::spawn(async move {
                    AsyncMqttClient::enter_event_loop_v5(event_loop).await;
                });

                Self {
                    client_v3: None,
                    client_v5: Some(client),
                }
            }
        }
    }

    /// Begins the background event loop required for the rumqttc V3 Client to publish MQTT messages.
    async fn enter_event_loop_v3(mut event_loop: EventLoopV3) {
        trace!("Beginning event v3 loop");
        loop {
            let event = event_loop.poll().await;
            match &event {
                Ok(v) => {
                    debug!("Event = {v:?}");
                }
                Err(e) => {
                    debug!("Error = {e:?}");
                }
            }
        }
    }

    /// Begins the background event loop required for the rumqttc V5 Client to publish MQTT messages.
    async fn enter_event_loop_v5(mut event_loop: EventLoopV5) {
        trace!("Beginning event v5 loop");
        loop {
            let event = event_loop.poll().await;
            match &event {
                Ok(v) => {
                    debug!("Event = {v:?}");
                }
                Err(e) => {
                    debug!("Error = {e:?}");
                }
            }
        }
    }

    /// Publishes a message with the specified payload to the specified topic.
    pub(crate) async fn publish_with_payload(&self, payload: String, topic: String, qos: PublisherQoS) -> Result<(), PublisherError> {
        if let Some(client_v3) = self.client_v3.clone() {
            match client_v3.publish(topic, qos.clone().into(), false, payload.clone()).await {
                Ok(_) => {
                    trace!("Message published over v3 protocol");
                    Ok(())
                }
                Err(error) => {
                    error!("{}" ,error.to_string());
                    // TODO: JD: return PublisherError::DuplicateMessage if this is a dupe
                    Err(PublisherError::FailedToMessage)
                }
            }
        } else if let Some(client_v5) = self.client_v5.clone() {
            match client_v5.publish(topic, qos.clone().into(), false, payload.clone()).await {
                Ok(_) => {
                    trace!("Message published over v5 protocol");
                    Ok(())
                }
                Err(error) => {
                    error!("{}" ,error.to_string());
                    // TODO: JD: return PublisherError::DuplicateMessage if this is a dupe
                    Err(PublisherError::FailedToMessage)
                }
            }
        } else {
            Err(PublisherError::ClientNotConfigured)
        }
    }
}