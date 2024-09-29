// MQTT Publisher
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use rumqttc::v5::mqttbytes::QoS as QoSV5;
use rumqttc::QoS as QoSV3;

/// Enumerates the Quality of Service options for message publishing. These options correspond
/// 1-to-1 with those of MQTT. IMPORTANT NOTE: This redundant enum exists so that users of this
/// library are not tightly-coupled to whichever MQTT implementation we utilize.
#[derive(Clone)]
pub enum PublisherQoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl From<PublisherQoS> for QoSV3 {
    fn from(value: PublisherQoS) -> Self {
        match value {
            PublisherQoS::AtMostOnce => QoSV3::AtLeastOnce,
            PublisherQoS::AtLeastOnce => QoSV3::AtLeastOnce,
            PublisherQoS::ExactlyOnce => QoSV3::ExactlyOnce,
        }
    }
}

impl From<PublisherQoS> for QoSV5 {
    fn from(value: PublisherQoS) -> Self {
        match value {
            PublisherQoS::AtMostOnce => QoSV5::AtLeastOnce,
            PublisherQoS::AtLeastOnce => QoSV5::AtLeastOnce,
            PublisherQoS::ExactlyOnce => QoSV5::ExactlyOnce,
        }
    }
}
