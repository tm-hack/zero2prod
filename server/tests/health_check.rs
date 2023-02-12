use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = App::new();
    let address = format!("{}:{}", app.address, app.port);
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // Finishing
    drop(app);
}

struct App {
    address: String,
    port: u16,
}

impl App {
    fn new() -> App {
        let address = ("127.0.0.1:0").to_string();
        let listner = TcpListener::bind(&address).expect("Failed to bind address");
        let port = listner.local_addr().unwrap().port();
        let server = zero2prod::run(listner).expect("Failed to bind address");
        let _ = tokio::spawn(server);

        App { address, port }
    }
}
