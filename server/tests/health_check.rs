use actix_web::dev::Server;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = App::new();
    let address = format!("http:{}:{}", app.ip_addr, app.port);
    let future = tokio::spawn(app.server);
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
    drop(future);
}

struct App {
    ip_addr: String,
    port: u16,
    server: Server,
}

impl App {
    fn new() -> App {
        let ip_addr = "127.0.0.1".to_string();
        let bind_addr = format!("{ip_addr}:0");
        let listner = TcpListener::bind(bind_addr).expect("Failed to bind address");
        let port = listner.local_addr().unwrap().port();
        let server = zero2prod::run(listner).expect("Failed to bind address");

        App {
            ip_addr,
            port,
            server,
        }
    }
}
