use std::{
    future::Future,
    net::ToSocketAddrs,
    pin::Pin,
    task::{Context, Poll},
};

use async_std::io::{Read, Write};
use fluvio_future::{
    net::TcpStream,
    rust_tls::{DefaultClientTlsStream, TlsConnector},
};
use hyper::{
    client::connect::{Connected, Connection},
    rt,
    service::Service,
    Uri,
};
use rustls::ClientConfig;

use std::sync::Arc;
#[derive(Clone)]
pub struct CompatConnector(Arc<TlsConnector>);

impl CompatConnector {
    pub fn new(tls_config: ClientConfig) -> Self {
        Self(Arc::new(TlsConnector::from(std::sync::Arc::new(
            tls_config,
        ))))
    }
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

impl Service<Uri> for CompatConnector {
    type Response = TlsStream;
    type Error = eyre::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, uri: Uri) -> Self::Future {
        let connector = self.0.clone();

        // TODO: move this to its own Future
        Box::pin(async move {
            let host = match uri.host() {
                Some(h) => h,
                None => return Err(eyre::eyre!("no host")),
            };

            match uri.scheme_str() {
                Some("http") => Err(eyre::eyre!("http not supported")),
                Some("https") => {
                    let socket_addr = {
                        let host = host.to_string();
                        let port = uri.port_u16().unwrap_or(443);
                        match (host.as_str(), port).to_socket_addrs()?.next() {
                            Some(addr) => addr,
                            None => return Err(eyre::eyre!("host resolution: {} failed", host)),
                        }
                    };
                    let tcp_stream = TcpStream::connect(&socket_addr).await?;

                    let stream = connector
                        .connect(host.try_into()?, tcp_stream)
                        .await
                        .map_err(|err| {
                            std::io::Error::new(
                                std::io::ErrorKind::Other,
                                format!("tls handshake: {}", err),
                            )
                        })?;
                    Ok(TlsStream(stream))
                }
                scheme => Err(eyre::eyre!("{:?}", scheme)),
            }
        })
    }
}

pub struct TlsStream(DefaultClientTlsStream);

impl Connection for TlsStream {
    fn connected(&self) -> Connected {
        Connected::new()
    }
}

impl tokio::io::AsyncRead for TlsStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match Pin::new(&mut self.0).poll_read(cx, buf.initialize_unfilled())? {
            Poll::Ready(bytes_read) => {
                buf.advance(bytes_read);
                Poll::Ready(Ok(()))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

impl tokio::io::AsyncWrite for TlsStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, std::io::Error>> {
        Pin::new(&mut self.0).poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.0).poll_flush(cx)
    }

    fn poll_shutdown(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), std::io::Error>> {
        Pin::new(&mut self.0).poll_close(cx)
    }
}

pub struct CompatExecutor;

impl<F> rt::Executor<F> for CompatExecutor
where
    F: Future + Send + 'static,
    F::Output: Send,
{
    fn execute(&self, fut: F) {
        async_std::task::spawn(fut);
    }
}
