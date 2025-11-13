mod shared;

use crate::shared::TestModel;
use grapple_kafka::Result;

use grapple_kafka::dummy::DummyReceiver;
use grapple_kafka::service::KafkaService;

#[tokio::main]
async fn main() -> Result<()> {
    // Producer only
    println!("Producer only");

    // -- Create KafkaService (только producer)
    let kafka_service = KafkaService::<DummyReceiver>::with_dummy_state("localhost:9094")?;

    let topic = "test-topic";

    // -- Produce message from tuple
    let key = "test-key".to_string();
    let payload = "test".to_string();

    match kafka_service.produce(&topic, &(&key, &payload)).await {
        Ok(()) => println!("✅ Successfully produced tuple message"),
        Err(e) => eprintln!("❌ Failed to produce tuple: {}", e),
    }

    // -- Produce message from struct
    let model = TestModel {
        data: "test".to_string(),
        data2: "test2".to_string(),
    };

    match kafka_service.produce(&topic, &model).await {
        Ok(()) => println!("✅ Successfully produced model message"),
        Err(e) => eprintln!("❌ Failed to produce model: {}", e),
    }

    // -- Produce multiple messages
    let messages = vec![
        TestModel {
            data: "msg1".to_string(),
            data2: "data1".to_string(),
        },
        TestModel {
            data: "msg2".to_string(),
            data2: "data2".to_string(),
        },
    ];

    for msg in messages {
        kafka_service.produce(&topic, &msg).await?;
        println!("✅ Produced: {:?}", msg);
    }

    Ok(())
}
