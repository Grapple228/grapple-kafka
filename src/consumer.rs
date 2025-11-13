use std::sync::Arc;

use async_trait::async_trait;
use rdkafka::{
    consumer::{CommitMode, StreamConsumer},
    ClientConfig, Message,
};

use crate::{config::kafka_config, decode, Error, Result};

#[async_trait]
pub trait Receiver: Sized + Send + Sync {
    async fn process(key: &str, payload: Option<&[u8]>) -> Result<()>;
}

#[async_trait]
pub trait StateReceiver: Sized + Send + Sync {
    type State;

    async fn process(key: &str, payload: Option<&[u8]>, state: &Self::State) -> Result<()>;
}

pub struct ConsumerConfig {
    pub uri: String,
    pub group_id: String,
    pub topics: Vec<String>,
    pub offset_reset: String,
    pub commit_mode: CommitMode,
}

impl ConsumerConfig {
    pub fn default() -> Self {
        Self {
            uri: kafka_config().KAFKA_URI.clone(),
            group_id: kafka_config().KAFKA_GROUP_ID.clone(),
            topics: Vec::new(),
            offset_reset: "earliest".to_string(),
            commit_mode: CommitMode::Async,
        }
    }
}

#[async_trait]
pub trait ConsumerLike: Send + Sync {
    async fn consume_with_state<R>(self, state: Arc<R::State>) -> Result<()>
    where
        R: StateReceiver + Send + Sync,
        R::State: Send + Sync;
}

// Реализация для FutureProducer

#[async_trait]
impl ConsumerLike for crate::consumer::KafkaConsumer {
    async fn consume_with_state<R>(self, state: Arc<R::State>) -> Result<()>
    where
        R: StateReceiver + Send + Sync,
        R::State: Send + Sync,
    {
        // Просто делегируем к существующему методу
        crate::consumer::KafkaConsumer::consume_with_state::<R>(self, state).await
    }
}

pub struct KafkaConsumer {
    consumer: StreamConsumer,
    commit_mode: CommitMode,
}

impl KafkaConsumer {
    pub fn default() -> Result<Self> {
        let config = ConsumerConfig::default();

        Self::new(&config)
    }

    pub fn new(config: &ConsumerConfig) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &config.uri)
            .set("group.id", &config.group_id)
            .set("auto.offset.reset", &config.offset_reset)
            .create()?;

        let mut kafka_consumer = Self {
            consumer,
            commit_mode: config.commit_mode,
        };

        kafka_consumer.subscribe(&config.topics)?;

        Ok(kafka_consumer)
    }

    fn is_fatal_error(error: &rdkafka::error::KafkaError) -> bool {
        match error {
            rdkafka::error::KafkaError::ClientConfig(_, _, _, _) => true,
            rdkafka::error::KafkaError::ClientCreation(_) => true,
            rdkafka::error::KafkaError::Subscription(_) => true,
            _ => false,
        }
    }

    pub async fn consume_with_state<T: StateReceiver>(self, state: Arc<T::State>) -> Result<()> {
        use rdkafka::consumer::Consumer;

        loop {
            match self.consumer.recv().await {
                Err(e) => {
                    tracing::error!("Kafka error: {}", e);
                    if Self::is_fatal_error(&e) {
                        break Err(Error::Rdkafka(e));
                    }
                }
                Ok(message) => {
                    let key = message.key().ok_or(Error::KeyMissing)?;
                    let key = decode::<String>(key)?;

                    match T::process(&key, message.payload(), &state).await {
                        Ok(_) => {
                            if let Err(e) = self.consumer.commit_message(&message, self.commit_mode)
                            {
                                tracing::error!("Commit error: {}", e);
                            }
                        }
                        Err(e) => tracing::error!("Error processing message: {}", e),
                    };
                }
            }
        }
    }

    // Аналогично для consume без state
    pub async fn consume<T: Receiver>(self) -> Result<()> {
        use rdkafka::consumer::Consumer;

        loop {
            match self.consumer.recv().await {
                Err(e) => {
                    tracing::error!("Kafka error: {}", e);
                    if Self::is_fatal_error(&e) {
                        break Err(Error::Rdkafka(e));
                    }
                }
                Ok(message) => {
                    let key = message.key().ok_or(Error::KeyMissing)?;
                    let key = decode::<String>(key)?;

                    match T::process(&key, message.payload()).await {
                        Ok(_) => {
                            if let Err(e) = self.consumer.commit_message(&message, self.commit_mode)
                            {
                                tracing::error!("Commit error: {}", e);
                            }
                        }
                        Err(e) => tracing::error!("Error processing message: {}", e),
                    };
                }
            }
        }
    }

    pub fn subscribe(&mut self, topics: &[impl AsRef<str>]) -> Result<()> {
        use rdkafka::consumer::Consumer;

        let topics = topics.iter().map(|t| t.as_ref()).collect::<Vec<&str>>();
        self.consumer.subscribe(&topics)?;

        Ok(())
    }

    pub fn commit_mode(&mut self, commit_mode: CommitMode) -> Result<()> {
        self.commit_mode = commit_mode;

        Ok(())
    }
}
