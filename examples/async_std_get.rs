use fluvio_mini_http::ResponseExt;

#[async_std::main]
async fn main() {
    let r = fluvio_mini_http::get("https://www.rust-lang.org/learn")
        .await
        .unwrap();

    println!("{}", r.status());

    let with_auth = fluvio_mini_http::client::Client::new()
        .get("https://api.github.com/octocat")
        .header("authorization", "Bearer <token>")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .unwrap();

    println!("AUTH: {}", with_auth.status());

    let bytes = with_auth.bytes().await.unwrap();

    println!("{}", std::str::from_utf8(&bytes).unwrap());
}
