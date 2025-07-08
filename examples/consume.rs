mod shared;

use std::time::Duration;

use crate::shared::TestModel;
use grapple_kafka::{
    async_trait::async_trait,
    consumer::{Consumer, ConsumerConfig, Receiver},
    decode, Error, Result,
};
use rdkafka::consumer::CommitMode;
use tracing::warn;

#[tokio::main]
#[allow(unused)]
async fn main() -> Result<()> {
    // -- Create consumer
    let config = ConsumerConfig {
        group_id: "test-group".to_string(),
        topics: vec!["test-topic".to_string()],
        uri: "localhost:9094".to_string(),
        offset_reset: "earliest".to_string(),
        commit_mode: CommitMode::Async,
    };

    let consumer = Consumer::new(&config)?;

    // -- Start consuming as a separate task
    let consumer_task = tokio::spawn(async move {
        consumer.consume::<MyReceiver>().await.unwrap();
    });

    // do something else
    loop {
        tokio::time::sleep(Duration::from_secs(3)).await;
        println!("Doing something else...");
    }

    // -- Wait for consumer task to finish
    consumer_task.await.unwrap();

    Ok(())
}

// region:    --- MyReceiver

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
            _ => {
                warn!("Key not registered: {}", key);
                return Err(Error::KeyNotRegistered(key.to_string()));
            }
        }

        Ok(())
    }
}

// endregion: --- MyReceiver
