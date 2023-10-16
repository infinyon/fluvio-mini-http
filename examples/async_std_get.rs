use fluvio_mini_http::ResponseExt;

#[async_std::main]
async fn main() {
    let bearer_token = std::env::var("GITHUB_TOKEN").expect("env var `GITHUB_TOKEN` not set");

    let with_auth = fluvio_mini_http::client::Client::new()
        .get("https://api.github.com/octocat")
        .header("Authorization", format!("Bearer {bearer_token}"))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await
        .unwrap();

    println!("Status: {}", with_auth.status());

    let bytes = with_auth.bytes().await.unwrap();
    let text = std::str::from_utf8(&bytes).unwrap();

    println!("{text}");
}
