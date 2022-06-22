use std::collections::VecDeque;

mod client;
mod eventloop;
mod notifier;
mod outgoing_buf;
mod state;
#[cfg(feature = "use-rustls")]
mod tls;

pub use client::{AsyncClient, Client, ClientError, Connection};
pub use eventloop::{ConnectionError, EventLoop};
pub use flume::{SendError, Sender, TrySendError};
pub use notifier::Notifier;
pub use state::{MqttState, StateError};
#[cfg(feature = "use-rustls")]
pub use tls::Error;
#[cfg(feature = "use-rustls")]
pub use tokio_rustls::rustls::ClientConfig;

pub use crate::mqttbytes::v5 as packet;
pub use packet::*;

pub type MqttOptions = crate::MqttOptions<LastWill>;
pub type Incoming = Packet;

/// Requests by the client to mqtt event loop. Request are
/// handled one by one.
#[derive(Clone, Debug, PartialEq)]
pub enum Request {
    Publish(Publish),
    PubAck(PubAck),
    PubRec(PubRec),
    PubComp(PubComp),
    PubRel(PubRel),
    PingReq,
    PingResp,
    Subscribe(Subscribe),
    SubAck(SubAck),
    Unsubscribe(Unsubscribe),
    UnsubAck(UnsubAck),
    Disconnect,
}

impl From<Publish> for Request {
    fn from(publish: Publish) -> Request {
        Request::Publish(publish)
    }
}

impl From<Subscribe> for Request {
    fn from(subscribe: Subscribe) -> Request {
        Request::Subscribe(subscribe)
    }
}

impl From<Unsubscribe> for Request {
    fn from(unsubscribe: Unsubscribe) -> Request {
        Request::Unsubscribe(unsubscribe)
    }
}

pub async fn connect(options: MqttOptions, cap: usize) -> Result<(AsyncClient, Notifier), ()> {
    let mut eventloop = EventLoop::new(options, cap);
    let outgoing_buf = eventloop.state.outgoing_buf.clone();
    let incoming_buf = eventloop.state.incoming_buf.clone();
    let incoming_buf_cache = VecDeque::with_capacity(cap);
    let request_tx = eventloop.handle();

    let client = AsyncClient {
        outgoing_buf,
        request_tx,
    };

    tokio::spawn(async move {
        loop {
            // TODO: maybe do something like retries for some specific errors? or maybe give user
            // options to configure these retries?
            eventloop.poll().await.unwrap();
        }
    });

    Ok((client, Notifier::new(incoming_buf, incoming_buf_cache)))
}
