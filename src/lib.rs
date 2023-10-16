mod async_std_compat;
pub mod client;
mod request;

pub use client::Client;
use hyper::{Body, Response};
pub use request::ResponseExt;

pub use hyper::StatusCode;

pub async fn get(uri: impl AsRef<str>) -> Result<Response<Body>, eyre::Error> {
    Client::new().get(uri).send().await
}
