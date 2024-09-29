// MQTT Publisher
//
// © 2024 Rust Made Easy. All rights reserved.
//
// @author JoelDavisEngineering@Gmail.com

use derive_more::{Display, Error};

/// Defines the errors used throughout the library.
#[derive(Clone, Debug, Display, Error, PartialEq)]
pub enum PublisherError {
    ClientNotConfigured,
    FailedToMessage,
}
