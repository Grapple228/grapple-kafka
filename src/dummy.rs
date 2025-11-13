use crate::{consumer::StateReceiver, Result};
use async_trait::async_trait;

/// Dummy state для случаев когда state не нужен
#[derive(Debug, Clone)]
pub struct DummyState;

/// Dummy receiver который игнорирует все сообщения
pub struct DummyReceiver;

#[async_trait]
impl StateReceiver for DummyReceiver {
    type State = DummyState;

    async fn process(_key: &str, _payload: Option<&[u8]>, _state: &Self::State) -> Result<()> {
        // Игнорируем все сообщения - ничего не делаем
        Ok(())
    }
}

// region:    --- Logging

/// Dummy receiver который логирует сообщения (для дебага)
pub struct LoggingDummyReceiver;

#[async_trait]
impl StateReceiver for LoggingDummyReceiver {
    type State = DummyState;

    async fn process(key: &str, payload: Option<&[u8]>, _state: &Self::State) -> Result<()> {
        tracing::debug!(
            "Dummy receiver got message - key: {}, payload_len: {:?}",
            key,
            payload.map(|p| p.len())
        );
        Ok(())
    }
}

// endregion: --- Logging
