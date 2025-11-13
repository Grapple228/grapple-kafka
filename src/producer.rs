use crate::{encode, kafka_config, KafkaModel, Result};
use async_trait::async_trait;
use rdkafka::{producer::FutureProducer, util::Timeout};
use std::time::Duration;

#[async_trait]
pub trait ProducerLike: Send + Sync {
    async fn produce(&self, topic: &str, model: &impl KafkaModel) -> Result<()>;
    async fn produce_with_retries(
        &self,
        topic: &str,
        model: &impl KafkaModel,
        max_retries: u64,
    ) -> Result<()>;
}

pub struct KafkaProducer {
    inner: FutureProducer,
}

impl KafkaProducer {
    pub fn new(producer: FutureProducer) -> Self {
        Self { inner: producer }
    }

    pub fn create(uri: &str) -> Result<Self> {
        let producer: FutureProducer = rdkafka::ClientConfig::new()
            .set("bootstrap.servers", uri)
            .create()?;
        Ok(Self::new(producer))
    }

    pub fn inner(&self) -> &FutureProducer {
        &self.inner
    }
}

#[async_trait]
impl ProducerLike for KafkaProducer {
    async fn produce(&self, topic: &str, model: &impl KafkaModel) -> Result<()> {
        let start = std::time::Instant::now();

        let key = encode(&model.key())?;
        let payload = encode(&model.payload()?)?;

        let record = rdkafka::producer::FutureRecord::to(topic)
            .key(&key)
            .payload(&payload);

        let status_delivery = self
            .inner
            .send(
                record,
                Timeout::After(Duration::from_millis(
                    kafka_config().KAFKA_PRODUCE_TIMEOUT_MS,
                )),
            )
            .await;

        match status_delivery {
            Ok(report) => {
                let duration = start.elapsed();
                tracing::debug!("Kafka send successful: {:?}, took {:?}", report, duration);
                Ok(())
            }
            Err((e, _)) => {
                tracing::error!("Kafka send failed: {:?}", e);
                Err(crate::Error::Rdkafka(e))
            }
        }
    }

    async fn produce_with_retries(
        &self,
        topic: &str,
        model: &impl KafkaModel,
        max_retries: u64,
    ) -> Result<()> {
        for attempt in 0..=max_retries {
            match self.produce(topic, model).await {
                Ok(()) => return Ok(()),
                Err(e) if attempt < max_retries => {
                    tracing::warn!("Produce attempt {} failed: {}, retrying...", attempt + 1, e);
                    tokio::time::sleep(Duration::from_millis(100 * (attempt as u64 + 1))).await;
                }
                Err(e) => return Err(e),
            }
        }
        unreachable!()
    }
}
