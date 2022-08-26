mod client;
mod eventloop;
mod framed;
mod notifier;
mod outgoing_buf;
#[allow(clippy::all)]
mod packet;
mod state;
#[cfg(feature = "use-rustls")]
mod tls;

pub use client::{AsyncClient, Client, ClientError, Connection, Iter};
pub use eventloop::{ConnectionError, EventLoop};
pub use notifier::Notifier;
pub use packet::*;
pub use state::{MqttState, StateError};
#[cfg(feature = "use-rustls")]
pub use tls::Error;

pub type Incoming = Packet;
pub type MqttOptions = crate::options::MqttOptions<LastWill>;

/// Requests by the client to mqtt event loop. Request are
/// handled one by one.
#[derive(Clone, Debug, PartialEq, Eq)]
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
