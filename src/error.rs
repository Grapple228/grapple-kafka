//! Main Crate Error

use derive_more::derive::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    ConfigAlreadyInitialized,

    KeyMissing,
    PayloadMissing,
    KeyNotRegistered,

    SerializeError,
    DeserializeError,

    // -- Externals
    #[from]
    Envs(grapple_utils::envs::Error),

    #[from]
    Rdkafka(rdkafka::error::KafkaError),
}

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate
