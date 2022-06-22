//! This module offers a high level synchronous and asynchronous abstraction to
//! async eventloop.
mod asyncclient;
mod syncclient;

pub use asyncclient::AsyncClient;
pub use syncclient::Client;

use flume::{SendError, TrySendError};

use super::Request;
use crate::mqttbytes;

/// Client Error
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Failed to send cancel request to eventloop")]
    Cancel(#[from] SendError<()>),
    #[error("Failed to send mqtt requests to eventloop")]
    Request(#[from] SendError<Request>),
    #[error("Failed to send mqtt requests to eventloop")]
    TryRequest(#[from] TrySendError<Request>),
    #[error("Serialization error: {0}")]
    Mqtt4(#[from] mqttbytes::Error),
}
