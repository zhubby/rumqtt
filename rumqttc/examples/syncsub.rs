use std::error::Error;

use rumqttc::{Client, Event, MqttOptions};

fn main() -> Result<(), Box<dyn Error>> {
    pretty_env_logger::init();

    let options = MqttOptions::new("publisher", "example.broker", 1883);
    let (client, mut connection) = Client::new(options, 10);
    let topic = "hello/world";
    client.subscribe(topic, rumqttc::QoS::AtLeastOnce)?;

    for event in connection.iter() {
        let event = event?;
        if let Event::Incoming(rumqttc::Packet::Publish(publish)) = event {
            // NOTE: This is an unnecessary check, but here becasue the connection can be subscribed to multiple topics
            if publish.topic == topic {
                println!("Payload: {}", String::from_utf8(publish.payload.to_vec())?);
            }
        }
    }

    Ok(())
}
