use std::time::Duration;

use crate::{encode, KafkaModel, Result};
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
    ClientConfig,
};
use tracing::debug;

pub fn create(uri: &str) -> Result<FutureProducer> {
    let producer: FutureProducer = ClientConfig::new().set("bootstrap.servers", uri).create()?;

    Ok(producer)
}

pub async fn produce(
    future_producer: &FutureProducer,
    topic: &str,
    model: &impl KafkaModel,
) -> Result<()> {
    let key = encode(&model.key())?;
    let payload = encode(&model.payload()?)?;

    let record = FutureRecord::to(topic).key(&key).payload(&payload);

    let status_delivery = future_producer
        .send(record, Timeout::After(Duration::from_secs(2)))
        .await;

    match status_delivery {
        Ok(report) => debug!("Kafka send: {:?}", report),
        Err(e) => tracing::error!("Kafka send failed: {:?}", e),
    }

    Ok(())
}
