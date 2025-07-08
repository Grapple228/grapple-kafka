mod shared;

use grapple_kafka::{producer::produce, Result};

use crate::shared::TestModel;

#[tokio::main]
async fn main() -> Result<()> {
    let uri = "localhost:9094";

    // -- Create producer
    let producer = grapple_kafka::producer::create(uri)?;

    let topic = "test-topic";

    // -- Produce message from tuple
    let key = "test-key".to_string();
    let payload = "test".to_string();
    produce(&producer, &topic, &(&key, &payload)).await?;

    // -- Produce message from struct
    let model = TestModel {
        data: "test".to_string(),
        data2: "test2".to_string(),
    };
    produce(&producer, &topic, &model).await?;

    Ok(())
}
