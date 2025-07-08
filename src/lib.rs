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
pub use rdkafka;
#[doc(hidden)]
pub use rdkafka::message::{FromBytes, ToBytes};

// endregion: --- Modules

pub trait KafkaModel<'a> {
    fn key(&'a self) -> Result<impl ToBytes>;
    fn payload(&'a self) -> Result<impl ToBytes>;
}

impl<'a, K, V> KafkaModel<'a> for (K, V)
where
    K: ToBytes + 'a,
    V: ToBytes + 'a,
{
    fn key(&self) -> Result<impl ToBytes> {
        Ok(&self.0)
    }

    fn payload(&self) -> Result<impl ToBytes> {
        Ok(&self.1)
    }
}
