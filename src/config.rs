use crate::error::{Error, Result};
use grapple_utils::envs::get;
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
}

#[allow(unused)]
impl KafkaConfig {
    fn load_from_env() -> Result<KafkaConfig> {
        Ok(KafkaConfig {
            KAFKA_URI: get("KAFKA_URI")?,
            KAFKA_GROUP_ID: get("KAFKA_GROUP_ID")?,
        })
    }

    pub fn init_from(cfg: Self) -> Result<()> {
        INSTANCE
            .set(cfg)
            .map_err(|_| Error::ConfigAlreadyInitialized)
    }
}
