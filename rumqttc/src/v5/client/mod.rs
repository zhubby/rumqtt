//! This module offers a high level synchronous and asynchronous abstraction to
//! async eventloop.
use crate::v5::{packet::*, Request};
use crate::QoS;

use flume::SendError;

mod asyncclient;
pub use asyncclient::AsyncClient;
mod syncclient;
pub use syncclient::Client;

/// Client Error
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Failed to send cancel request to eventloop")]
    Cancel(SendError<()>),
    #[error("Failed to send mqtt request to eventloop, the evenloop has been closed")]
    EventloopClosed,
    #[error("Failed to send mqtt request to evenloop, to requests buffer is full right now")]
    RequestsFull,
    #[error("Serialization error")]
    MqttBytes(#[from] crate::mqttbytes::Error),
}

fn get_ack_req(qos: QoS, pkid: u16) -> Option<Request> {
    let ack = match qos {
        QoS::AtMostOnce => return None,
        QoS::AtLeastOnce => Request::PubAck(PubAck::new(pkid)),
        QoS::ExactlyOnce => Request::PubRec(PubRec::new(pkid)),
    };
    Some(ack)
}
