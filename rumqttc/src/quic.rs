use std::{io::ErrorKind, net::SocketAddr};

use futures_util::FutureExt;
use quinn::{ClientConfig, Endpoint, RecvStream, SendStream};
use tokio::io::{AsyncRead, AsyncWrite, AsyncWriteExt};

use crate::{MqttOptions, TlsConfiguration};

#[derive(Debug, thiserror::Error)]
pub enum QuicError {
    #[error("Io Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Tls Error: {0}")]
    Tls(#[from] crate::tls::Error),
    #[error("Connect Error: {0}")]
    Connect(#[from] quinn::ConnectError),
    #[error("Connectio Error: {0}")]
    Connection(#[from] quinn::ConnectionError),
    #[error("Parse Error: {0}")]
    Parse(#[from] std::net::AddrParseError),
}

pub struct Quic {
    tx: SendStream,
    rx: RecvStream,
}

impl Quic {
    pub async fn new(
        options: &MqttOptions,
        tls_config: &TlsConfiguration,
    ) -> Result<Self, QuicError> {
        let client_config = ClientConfig::new(tls_config.client_config()?);
        let mut addr: SocketAddr = options.broker_addr.parse()?;
        addr.set_port(options.port);
        let endpoint =
            Endpoint::client(options.quic_sock_addr.expect("Missing quic socket address"))?;

        let connection = endpoint
            .connect_with(client_config, addr, &options.broker_addr)?
            .await?;
        let (tx, rx) = connection.connection.open_bi().await?;

        Ok(Self { tx, rx })
    }
}

impl AsyncRead for Quic {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> std::task::Poll<std::io::Result<()>> {
        self.get_mut()
            .rx
            .read(buf.filled_mut())
            .poll_unpin(cx)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
            .map(|_| Ok(()))
    }
}

impl AsyncWrite for Quic {
    fn poll_write(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        self.get_mut()
            .tx
            .write(buf)
            .poll_unpin(cx)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        Box::pin(self.get_mut().tx.flush()).poll_unpin(cx)
    }

    fn poll_shutdown(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), std::io::Error>> {
        self.get_mut()
            .tx
            .finish()
            .poll_unpin(cx)
            .map_err(|e| std::io::Error::new(ErrorKind::Other, e))
    }

    fn poll_write_vectored(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        bufs: &[std::io::IoSlice<'_>],
    ) -> std::task::Poll<Result<usize, std::io::Error>> {
        let buf = bufs
            .iter()
            .find(|b| !b.is_empty())
            .map_or(&[][..], |b| &**b);
        self.poll_write(cx, buf)
    }

    fn is_write_vectored(&self) -> bool {
        false
    }
}
