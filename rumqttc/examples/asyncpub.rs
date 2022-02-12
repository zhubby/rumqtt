use std::error::Error;

use rumqttc::{AsyncClient, MqttOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let options = MqttOptions::new("publisher", "example.broker", 1883);
    let (client, _) = AsyncClient::new(options, 10);
    client
        .publish(
            "hello/world",
            rumqttc::QoS::AtLeastOnce,
            false,
            "Hello, World!",
        )
        .await?;

    Ok(())
}
