use crate::error::{Error, Result};
use grapple_utils::envs::{get, get_parse};
use std::sync::OnceLock;

static INSTANCE: OnceLock<KafkaConfig> = OnceLock::new();

pub fn kafka_config() -> &'static KafkaConfig {
    INSTANCE.get_or_init(|| {
        KafkaConfig::load_from_env()
            .unwrap_or_else(|ex| panic!("FATAL - WHOLE LOADING CONF - Cause: {ex:?}"))
    })
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct KafkaConfig {
    pub KAFKA_URI: String,
    pub KAFKA_GROUP_ID: String,
    pub KAFKA_PRODUCE_TIMEOUT_MS: u64,
    pub KAFKA_PRODUCE_RETRIES_COUNT: u64,
}

#[allow(unused)]
impl KafkaConfig {
    fn load_from_env() -> Result<KafkaConfig> {
        Ok(KafkaConfig {
            KAFKA_URI: get("KAFKA_URI")?,
            KAFKA_GROUP_ID: get("KAFKA_GROUP_ID")?,
            KAFKA_PRODUCE_TIMEOUT_MS: get_parse("KAFKA_PRODUCE_TIMEOUT_MS").unwrap_or(2000),
            KAFKA_PRODUCE_RETRIES_COUNT: get_parse("KAFKA_PRODUCE_RETRIES_COUNT").unwrap_or(1),
        })
    }

    pub fn init_from(cfg: Self) -> Result<()> {
        INSTANCE
            .set(cfg)
            .map_err(|_| Error::ConfigAlreadyInitialized)
    }
}
