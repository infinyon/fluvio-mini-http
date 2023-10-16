mod async_std_compat;
pub mod client;
mod request;

pub use client::Client;
use hyper::{Body, Response};
use request::RequestError;
pub use request::ResponseExt;

pub use hyper::StatusCode;

pub async fn get(uri: impl AsRef<str>) -> Result<Response<Body>, RequestError> {
    Client::new().get(uri).send().await
}
