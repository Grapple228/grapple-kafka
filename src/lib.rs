// region:    --- Modules

// -- Modules
pub mod consumer;
pub mod producer;

mod config;
mod error;

// -- Flatten
#[doc(hidden)]
pub use async_trait;
pub use error::{Error, Result};
#[doc(hidden)]
pub use rdkafka::message::{FromBytes, ToBytes};

// endregion: --- Modules

pub trait KafkaModel<'a> {
    fn key(&'a self) -> impl ToBytes;
    fn payload(&'a self) -> impl ToBytes;
}

impl<'a, K, V> KafkaModel<'a> for (K, V)
where
    K: ToBytes + 'a,
    V: ToBytes + 'a,
{
    fn key(&self) -> impl ToBytes {
        &self.0
    }

    fn payload(&self) -> impl ToBytes {
        &self.1
    }
}
