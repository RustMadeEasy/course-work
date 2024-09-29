// MQTT Publisher
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use std::time::Duration;

/// Enumerates the MQTT protocols supported by this library.
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum MqttProtocolVersion { V3, V5 }

/// Defines the parameters required to configure a Publisher.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BrokerInfo {
    pub(crate) capacity: usize,
    pub(crate) broker_address: String,
    pub(crate) broker_port: u16,
    pub(crate) keep_alive: Duration,
    pub(crate) protocol_version: MqttProtocolVersion,
}

impl BrokerInfo {
    /// Creates a new BrokerInfo instance.
    pub fn new(broker_address: String,
               capacity: usize,
               broker_port: u16,
               keep_alive: Duration,
               protocol_version: MqttProtocolVersion) -> Self {
        Self {
            capacity,
            broker_address,
            broker_port,
            keep_alive,
            protocol_version,
        }
    }
}
