use actix_web::dev::Server;
use std::net::TcpListener;

struct App {
    ip_addr: String,
    port: u16,
    server: Server,
}

impl App {
    fn new() -> App {
        let ip_addr = "127.0.0.1".to_string();
        let bind_addr = format!("{ip_addr}:0");
        let listner = TcpListener::bind(bind_addr).expect("Failed to bind app_address");
        let port = listner.local_addr().unwrap().port();
        let server = zero2prod::run(listner).expect("Failed to bind app_address");

        App {
            ip_addr,
            port,
            server,
        }
    }
}

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = App::new();
    let app_address = format!("http:{}:{}", app.ip_addr, app.port);
    let future = tokio::spawn(app.server);
    let client = reqwest::Client::new();

    println!("{}", app_address);

    // Act
    let response = client
        .get(&format!("{}/health_check", &app_address))
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    // Finishing
    drop(future);
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = App::new();
    let app_address = format!("http:{}:{}", app.ip_addr, app.port);
    let future = tokio::spawn(app.server);
    let client = reqwest::Client::new();

    println!("{}", app_address);

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());

    // Finishing
    drop(future);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = App::new();
    let app_address = format!("http:{}:{}", app.ip_addr, app.port);
    let future = tokio::spawn(app.server);
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing boh name and email"),
    ];

    println!("{}", app_address);

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }

    // Finishing
    drop(future);
}
