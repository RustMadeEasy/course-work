/// Defines the parameters required to configure a Publisher.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PublisherConfig {
    pub(crate) broker_address: String,
    pub(crate) broker_port: u16,
    pub(crate) protocol_version: MqttProtocolVersion,
}

impl PublisherConfig {
    /// Creates a new PublisherConfig instance.
    pub fn new(broker_address: String, broker_port: u16, protocol_version: MqttProtocolVersion) -> Self {
        Self {
            broker_address,
            broker_port,
            protocol_version,
        }
    }
}

/// Enumerates the MQTT protocols supported by this library.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MqttProtocolVersion { V3, V5 }
