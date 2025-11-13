use crate::{
    consumer::{KafkaConsumer, StateReceiver},
    dummy::{DummyReceiver, DummyState},
    kafka_config,
    producer::{KafkaProducer, ProducerLike},
    KafkaModel, Result,
};
use std::sync::Arc;

pub struct KafkaService<R>
where
    R: StateReceiver + Send + Sync,
{
    consumer: Option<KafkaConsumer>,
    producer: Arc<KafkaProducer>,
    receiver: std::marker::PhantomData<R>,
    state: Arc<R::State>,
}

// Constructors

impl KafkaService<DummyReceiver> {
    pub fn with_dummy_state(kafka_uri: &str) -> Result<Arc<Self>> {
        let state = Arc::new(DummyState);
        let producer = KafkaProducer::create(kafka_uri)?;

        Ok(Arc::new(Self {
            consumer: None,
            producer: Arc::new(producer),
            receiver: std::marker::PhantomData,
            state,
        }))
    }

    pub fn producer_only_with_dummy(kafka_uri: &str) -> Result<Arc<Self>> {
        let state = Arc::new(DummyState);
        Self::producer_only(kafka_uri, state)
    }
}

impl<R> KafkaService<R>
where
    R: StateReceiver + Send + Sync + 'static,
    R::State: Send + Sync,
{
    pub fn new(
        consumer: Option<KafkaConsumer>,
        producer: KafkaProducer,
        state: Arc<R::State>,
    ) -> Self {
        Self {
            consumer,
            producer: Arc::new(producer),
            receiver: std::marker::PhantomData,
            state,
        }
    }

    pub fn producer_only(kafka_uri: &str, state: Arc<R::State>) -> Result<Arc<Self>> {
        let producer = KafkaProducer::create(kafka_uri)?;

        Ok(Arc::new(Self {
            consumer: None,
            producer: Arc::new(producer),
            receiver: std::marker::PhantomData,
            state,
        }))
    }

    pub fn from_config(
        consumer_config: &crate::consumer::ConsumerConfig,
        state: Arc<R::State>,
    ) -> Result<Self> {
        let consumer = KafkaConsumer::new(consumer_config)?;
        let producer = KafkaProducer::create(&consumer_config.uri)?;

        let service = Self {
            consumer: Some(consumer),
            producer: Arc::new(producer),
            receiver: std::marker::PhantomData,
            state,
        };

        Ok(service)
    }
}

impl<R> KafkaService<R>
where
    R: StateReceiver + Send + Sync + 'static,
    R::State: Send + Sync,
{
    pub async fn start_consumer(mut self) -> Result<Arc<Self>> {
        if let Some(consumer) = self.consumer.take() {
            let state = self.state.clone();

            tokio::spawn(async move {
                tracing::info!("🚀 Starting Kafka consumer...");
                if let Err(e) = consumer.consume_with_state::<R>(state).await {
                    tracing::error!("Kafka consumer error: {}", e);
                }
                tracing::info!("🛑 Kafka consumer stopped");
            });
            tracing::info!("✅ Kafka consumer started successfully");
        } else {
            tracing::info!("ℹ️  No consumer to start (producer-only mode)");
        }

        Ok(Arc::new(self))
    }

    pub async fn produce<M: KafkaModel>(&self, topic: &str, model: &M) -> Result<()> {
        self.producer.produce(topic, model).await
    }

    pub async fn produce_with_retry<M: KafkaModel>(&self, topic: &str, model: &M) -> Result<()> {
        self.producer
            .produce_with_retries(topic, model, kafka_config().KAFKA_PRODUCE_RETRIES_COUNT)
            .await
    }

    pub fn producer(&self) -> &Arc<KafkaProducer> {
        &self.producer
    }

    pub fn state(&self) -> &Arc<R::State> {
        &self.state
    }

    pub fn state_cloned(&self) -> Arc<R::State> {
        self.state.clone()
    }
}
