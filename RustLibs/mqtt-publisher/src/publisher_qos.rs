use rumqttc::QoS as QoSV3;
use rumqttc::v5::mqttbytes::QoS as QoSV5;

/// Enumerates the Quality of Service options for message publishing. These options correspond
/// 1-to-1 with those of MQTT. IMPORTANT NOTE: This redundant enum exists so that users of this
/// library are not tightly-coupled to whichever MQTT implementation we utilize.
#[derive(Clone)]
pub enum PublisherQoS {
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

impl Into<QoSV3> for PublisherQoS {
    fn into(self) -> QoSV3 {
        match self {
            PublisherQoS::AtMostOnce => QoSV3::AtLeastOnce,
            PublisherQoS::AtLeastOnce => QoSV3::AtLeastOnce,
            PublisherQoS::ExactlyOnce => QoSV3::ExactlyOnce,
        }
    }
}

impl Into<QoSV5> for PublisherQoS {
    fn into(self) -> QoSV5 {
        match self {
            PublisherQoS::AtMostOnce => QoSV5::AtLeastOnce,
            PublisherQoS::AtLeastOnce => QoSV5::AtLeastOnce,
            PublisherQoS::ExactlyOnce => QoSV5::ExactlyOnce,
        }
    }
}
