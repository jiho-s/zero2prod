use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");

    let socket_address = listener.local_addr().expect("Failed to get socket address");

    let port = socket_address.port();

    let server = zero2prod::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_test() {
    // given
    let address = spawn_app();

    let client = reqwest::Client::new();

    // when
    let response = client
        .get(&format!("{}/health_check", address))
        .send()
        .await
        .expect("Failed to execute request.");

    // then
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length())
}
