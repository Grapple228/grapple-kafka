use grapple_kafka::KafkaModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestModel {
    pub data: String,
    pub data2: String,
}

impl<'a> KafkaModel<'a> for TestModel {
    fn key(&'a self) -> impl rdkafka::message::ToBytes {
        b"model-key"
    }

    fn payload(&'a self) -> impl rdkafka::message::ToBytes {
        let value = serde_json::to_vec(&self).unwrap();
        value
    }
}
