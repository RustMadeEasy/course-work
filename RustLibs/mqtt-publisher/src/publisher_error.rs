use derive_more::{Display, Error};

/// Defines the errors used throughout the library.
#[derive(Clone, Debug, Display, Error, PartialEq)]
pub enum PublisherError {
    ClientNotConfigured,
    DuplicateMessage,
    FailedToMessage,
}
