use grapple_kafka::{Error, KafkaModel, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestModel {
    pub data: String,
    pub data2: String,
}

impl<'a> KafkaModel<'a> for TestModel {
    fn key(&'a self) -> Result<impl rdkafka::message::ToBytes> {
        Ok(b"model-key")
    }

    fn payload(&'a self) -> Result<impl rdkafka::message::ToBytes> {
        let value = serde_json::to_vec(&self).map_err(|_| Error::SerializeError)?;
        Ok(value)
    }
}
