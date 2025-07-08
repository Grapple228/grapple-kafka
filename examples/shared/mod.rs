use bincode::{Decode, Encode};
use grapple_kafka::KafkaModel;

#[derive(Debug, Encode, Decode)]
pub struct TestModel {
    pub data: String,
    pub data2: String,
}

impl KafkaModel for TestModel {
    fn key(&self) -> impl Encode {
        "model-key"
    }
}
