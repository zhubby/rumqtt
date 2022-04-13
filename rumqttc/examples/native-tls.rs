//! Example of how to configure rumqttd to connect to a server using TLS and authentication.
use std::error::Error;

#[cfg(feature = "use-native-tls")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use rumqttc::{self, AsyncClient, MqttOptions, TlsConfiguration, Transport};

    pretty_env_logger::init();
    color_backtrace::install();

    let mut mqtt_options = MqttOptions::new("test-1", "localhost", 8883);
    mqtt_options.set_keep_alive(std::time::Duration::from_secs(5));

    let transport = Transport::Tls(TlsConfiguration::Native);

    mqtt_options.set_transport(transport);

    let (_client, mut eventloop) = AsyncClient::new(mqtt_options, 10);

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

    Ok(())
}

#[cfg(not(feature = "use-native-tls"))]
fn main() -> Result<(), Box<dyn Error>> {
    panic!("Enable feature 'use-native-tls'");
}
