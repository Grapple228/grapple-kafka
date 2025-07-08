mod shared;

use crate::shared::TestModel;
use grapple_kafka::{
    async_trait::async_trait,
    consumer::{Consumer, ConsumerConfig, Receiver},
    Error, Result,
};
use rdkafka::consumer::CommitMode;

#[tokio::main]
async fn main() -> Result<()> {
    let config = ConsumerConfig {
        group_id: "test-group".to_string(),
        topics: vec!["test-topic".to_string()],
        uri: "localhost:9094".to_string(),
        offset_reset: "earliest".to_string(),
        commit_mode: CommitMode::Async,
    };

    let consumer = Consumer::new(&config)?;
    consumer.consume::<MyReceiver>().await?;

    Ok(())
}

enum MyReceiver {
    Model(TestModel),
    Test(String),
}

#[async_trait]
impl Receiver for MyReceiver {
    fn from(key: &[u8], payload: Option<&[u8]>) -> Result<Self> {
        let payload = payload.ok_or(Error::PayloadMissing)?;

        match key {
            b"model-key" => {
                let model = serde_json::from_slice::<TestModel>(payload)
                    .map_err(|_| Error::DeserializeError)?;
                Ok(Self::Model(model))
            }
            b"test-key" => {
                let payload = String::from_utf8_lossy(payload);
                Ok(Self::Test(payload.to_string()))
            }
            _ => Err(Error::KeyNotRegistered),
        }
    }

    async fn process(&self) -> Result<()> {
        match self {
            Self::Model(model) => {
                println!("Model: {:?}", model);
            }
            Self::Test(payload) => {
                println!("Test: {}", payload);
            }
        }

        Ok(())
    }
}
