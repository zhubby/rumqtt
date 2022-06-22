//! This module offers a high level synchronous and asynchronous abstraction to
//! async eventloop.
use super::{AsyncClient, ClientError};
use crate::mqttbytes::{v4::*, QoS};
use crate::v4::{EventLoop, MqttOptions};
use crate::Connection;

use tokio::runtime;

/// `Client` to communicate with MQTT eventloop `Connection`.
///
/// Client is cloneable and can be used to synchronously Publish, Subscribe.
/// Asynchronous channel handle can also be extracted if necessary
#[derive(Clone)]
pub struct Client {
    client: AsyncClient,
}

impl Client {
    /// Create a new `Client`
    pub fn new(options: MqttOptions, cap: usize) -> (Client, Connection<EventLoop>) {
        let (client, eventloop) = AsyncClient::new(options, cap);
        let client = Client { client };
        let runtime = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let connection = Connection::new(eventloop, runtime);
        (client, connection)
    }

    /// Sends a MQTT Publish to the eventloop
    pub fn publish<S, V>(
        &mut self,
        topic: S,
        qos: QoS,
        retain: bool,
        payload: V,
    ) -> Result<(), ClientError>
    where
        S: Into<String>,
        V: Into<Vec<u8>>,
    {
        pollster::block_on(self.client.publish(topic, qos, retain, payload))?;
        Ok(())
    }

    pub fn try_publish<S, V>(
        &mut self,
        topic: S,
        qos: QoS,
        retain: bool,
        payload: V,
    ) -> Result<(), ClientError>
    where
        S: Into<String>,
        V: Into<Vec<u8>>,
    {
        self.client.try_publish(topic, qos, retain, payload)?;
        Ok(())
    }

    /// Sends a MQTT PubAck to the eventloop. Only needed in if `manual_acks` flag is set.
    pub fn ack(&self, publish: &Publish) -> Result<(), ClientError> {
        pollster::block_on(self.client.ack(publish))?;
        Ok(())
    }

    /// Sends a MQTT PubAck to the eventloop. Only needed in if `manual_acks` flag is set.
    pub fn try_ack(&self, publish: &Publish) -> Result<(), ClientError> {
        self.client.try_ack(publish)?;
        Ok(())
    }

    /// Sends a MQTT Subscribe to the eventloop
    pub fn subscribe<S: Into<String>>(&mut self, topic: S, qos: QoS) -> Result<(), ClientError> {
        pollster::block_on(self.client.subscribe(topic, qos))?;
        Ok(())
    }

    /// Sends a MQTT Subscribe to the eventloop
    pub fn try_subscribe<S: Into<String>>(
        &mut self,
        topic: S,
        qos: QoS,
    ) -> Result<(), ClientError> {
        self.client.try_subscribe(topic, qos)?;
        Ok(())
    }

    /// Sends a MQTT Subscribe for multiple topics to the eventloop
    pub fn subscribe_many<T>(&mut self, topics: T) -> Result<(), ClientError>
    where
        T: IntoIterator<Item = SubscribeFilter>,
    {
        pollster::block_on(self.client.subscribe_many(topics))
    }

    pub fn try_subscribe_many<T>(&mut self, topics: T) -> Result<(), ClientError>
    where
        T: IntoIterator<Item = SubscribeFilter>,
    {
        self.client.try_subscribe_many(topics)
    }

    /// Sends a MQTT Unsubscribe to the eventloop
    pub fn unsubscribe<S: Into<String>>(&mut self, topic: S) -> Result<(), ClientError> {
        pollster::block_on(self.client.unsubscribe(topic))?;
        Ok(())
    }

    /// Sends a MQTT Unsubscribe to the eventloop
    pub fn try_unsubscribe<S: Into<String>>(&mut self, topic: S) -> Result<(), ClientError> {
        self.client.try_unsubscribe(topic)?;
        Ok(())
    }

    /// Sends a MQTT disconnect to the eventloop
    pub fn disconnect(&mut self) -> Result<(), ClientError> {
        pollster::block_on(self.client.disconnect())?;
        Ok(())
    }

    /// Sends a MQTT disconnect to the eventloop
    pub fn try_disconnect(&mut self) -> Result<(), ClientError> {
        self.client.try_disconnect()?;
        Ok(())
    }

    /// Stops the eventloop right away
    pub fn cancel(&mut self) -> Result<(), ClientError> {
        pollster::block_on(self.client.cancel())?;
        Ok(())
    }
}
