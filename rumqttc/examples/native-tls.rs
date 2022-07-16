#![feature(byte_slice_trim_ascii)]

use native_tls::Identity;
use rumqttc::{AsyncClient, MqttOptions, QoS, TlsConfiguration, Transport};
use std::time::Duration;
#[tokio::main]
async fn main() {
    let mut options = MqttOptions::new("device", "localhost", 1883);
    let idx_bytes = include_bytes!("../../identity.pfx").trim_ascii_end();
    let identity = Identity::from_pkcs12(idx_bytes, "12345")
        .expect("identity file should be manually generated");
    options.set_transport(Transport::Tls(TlsConfiguration::Native(identity)));

    let (client, mut eventloop) = AsyncClient::new(options, 10);

    client
        .publish("topic", QoS::AtLeastOnce, false, "content")
        .await
        .unwrap();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(2)).await;
        client.disconnect().await.unwrap();
    });

    loop {
        match eventloop.poll().await {
            Ok(v) => {
                println!("Event = {:?}", v);
            }
            Err(e) => {
                println!("Error = {:?}", e);
                break;
            }
        }
    }
}
