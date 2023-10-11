mod async_std_compat;
pub mod client;
mod request;

pub use client::Client;
pub use request::ResponseExt;

pub use hyper::StatusCode;

pub async fn get(uri: impl AsRef<str>) -> Result<hyper::Response<hyper::Body>, eyre::Error> {
    Client::new().get(uri).send().await
}
