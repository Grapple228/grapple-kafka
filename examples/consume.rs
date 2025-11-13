mod shared;

use std::sync::Arc;

use crate::shared::TestModel;
use grapple_kafka::{
    async_trait::async_trait,
    consumer::{ConsumerConfig, StateReceiver},
    decode,
    service::KafkaService,
    Error, Result,
};
use rdkafka::consumer::CommitMode;
use tracing::warn;

#[tokio::main]
#[allow(unused)]
async fn main() -> Result<()> {
    println!("Consumer only");

    // -- Create consumer
    let consumer_config = ConsumerConfig {
        group_id: "test-group".to_string(),
        topics: vec!["test-topic".to_string()],
        uri: "localhost:9094".to_string(),
        offset_reset: "earliest".to_string(),
        commit_mode: CommitMode::Async,
    };

    let state = Arc::new(MyState);

    let kafka_service = KafkaService::<MyReceiver>::from_config(&consumer_config, state)?;
    let kafka_service = kafka_service.start_consumer().await?;

    tokio::signal::ctrl_c().await.unwrap();
    tracing::info!("Received Ctrl+C, shutting down...");

    Ok(())
}

// region:    --- MyReceiver

struct MyState;

struct MyReceiver;

#[async_trait]

impl StateReceiver for MyReceiver {
    type State = MyState;

    async fn process(key: &str, payload: Option<&[u8]>, _state: &Self::State) -> Result<()> {
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
