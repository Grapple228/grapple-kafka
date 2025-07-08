mod shared;

use crate::shared::TestModel;
use grapple_kafka::{
    async_trait::async_trait,
    consumer::{Consumer, ConsumerConfig, Receiver},
    decode, Error, Result,
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

struct MyReceiver;

#[async_trait]
impl Receiver for MyReceiver {
    async fn process(key: &str, payload: Option<&[u8]>) -> Result<()> {
        let payload = payload.ok_or(Error::PayloadMissing)?;

        match key {
            "model-key" => {
                let model = decode::<TestModel>(payload)?;
                println!("Model: {:?}", model);
            }
            "test-key" => {
                let payload = String::from_utf8_lossy(payload);
                println!("Test: {}", payload);
            }
            _ => return Err(Error::KeyNotRegistered),
        }

        Ok(())
    }
}
