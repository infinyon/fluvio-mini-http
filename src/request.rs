use std::{future::Future, pin::Pin};

use http::{request::Builder, HeaderName, HeaderValue};
use hyper::{body::Bytes, Body, Response};

use crate::client::Client;

pub struct RequestBuilder {
    client: Client,
    req_builder: Builder,
}

impl RequestBuilder {
    pub fn new(client: Client, req_builder: Builder) -> Self {
        Self {
            client,
            req_builder,
        }
    }

    pub fn header<K, V>(mut self, key: K, value: V) -> RequestBuilder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        self.req_builder = self.req_builder.header(key, value);
        self
    }

    pub async fn send(self) -> Result<Response<Body>, eyre::Error> {
        let req = self
            .req_builder
            .header(http::header::USER_AGENT, "fluvio-mini-http/0.1")
            .body(hyper::Body::empty())
            .unwrap();
        Ok(self
            .client
            .hyper
            .request(req)
            .await
            .map_err(|_err| eyre::eyre!("idk"))?)
    }
}

pub trait ResponseExt {
    // TODO: fix this once `async fn in traits` is stable
    // see: https://github.com/rust-lang/rust/pull/115822
    fn bytes(self) -> Pin<Box<dyn Future<Output = Result<Bytes, eyre::Error>> + Send + 'static>>;

    #[cfg(feature = "json")]
    fn json<T: serde::de::DeserializeOwned>(
        self,
    ) -> Pin<Box<dyn Future<Output = Result<T, eyre::Error>> + Send + 'static>>
    where
        Self: Sized + Send + 'static,
    {
        let fut = async move {
            let bytes = self.bytes().await?;
            serde_json::from_slice(&bytes).map_err(|e| eyre::eyre!("serde erro: {e}"))
        };

        Box::pin(fut)
    }
}

impl<T> ResponseExt for Response<T>
where
    T: hyper::body::HttpBody + Send + 'static,
    T::Data: Send,
{
    fn bytes(self) -> Pin<Box<dyn Future<Output = Result<Bytes, eyre::Error>> + Send + 'static>> {
        let fut = async move {
            hyper::body::to_bytes(self.into_body())
                .await
                .map_err(|_| eyre::eyre!("todo"))
        };

        Box::pin(fut)
    }
}
