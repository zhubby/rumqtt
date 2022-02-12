use std::error::Error;

use rumqttc::{AsyncClient, Event, MqttOptions};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let options = MqttOptions::new("publisher", "example.broker", 1883);
    let (client, mut eventloop) = AsyncClient::new(options, 10);
    let topic = "hello/world";
    client.subscribe(topic, rumqttc::QoS::AtLeastOnce).await?;

    loop {
        if let Event::Incoming(rumqttc::Packet::Publish(publish)) = eventloop.poll().await? {
            // NOTE: This is an unnecessary check, but here becasue the eventloop can be subscribed to multiple topics
            if publish.topic == topic {
                println!("Payload: {}", String::from_utf8(publish.payload.to_vec())?);
            }
        }
    }
}
