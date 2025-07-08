// region:    --- Modules

// -- Modules
pub mod consumer;
pub mod producer;

mod codec;
mod config;
mod error;

// -- Flatten
#[doc(hidden)]
pub use async_trait;
use bincode::Encode;
pub use error::{Error, Result};
#[doc(hidden)]
pub use rdkafka;
#[doc(hidden)]
pub use rdkafka::message::{FromBytes, ToBytes};

pub use codec::{decode, encode};

// endregion: --- Modules

pub trait KafkaModel: Encode {
    fn key(&self) -> impl Encode;
    fn payload(&self) -> Result<impl Encode> {
        Ok(self)
    }
}

impl<K, V> KafkaModel for (K, V)
where
    K: Encode,
    V: Encode,
{
    fn key(&self) -> impl Encode {
        &self.0
    }

    fn payload(&self) -> Result<impl Encode> {
        Ok(&self.1)
    }
}
