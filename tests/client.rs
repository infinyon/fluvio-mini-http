use http::StatusCode;

#[async_std::test]
async fn simple_test() {
    let res = fluvio_mini_http::get("https://infinyon.com").await;

    assert!(res.is_ok());
    let status = res.unwrap().status();
    assert_eq!(status, StatusCode::OK);
}

#[async_std::test]
async fn get_json() {
    use fluvio_mini_http::ResponseExt;
    use serde::Deserialize;

    #[allow(dead_code)]
    #[derive(Deserialize)]
    struct Ip {
        origin: String,
    }

    let json = fluvio_mini_http::get("https://httpbin.org/ip")
        .await
        .unwrap()
        .json::<Ip>()
        .await;

    assert!(json.is_ok());
}

static FOO: &str = "https://www.mockbin.com/bin/1700ca28-0817-4998-a8af-e3b90e2aacc6";

#[async_std::test]
async fn json() {
    use fluvio_mini_http::ResponseExt;
    use std::collections::HashMap;

    let json = fluvio_mini_http::get(&FOO)
        .await
        .unwrap()
        .json::<HashMap<String, bool>>()
        .await;

    assert!(json.is_ok());
    let body = json.unwrap();

    assert_eq!(body["success"], true);
}

#[async_std::test]
async fn http_not_supported() {
    let res = fluvio_mini_http::get("http://infinyon.com").await;

    assert!(res.is_err());
}
