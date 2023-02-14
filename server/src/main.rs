use sqlx::{Connection, PgConnection};
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::setup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // configuration.yamlから設定情報を取得する
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    // IPアドレスをバインドし、run()を呼び出す
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address)?;
    run(listner, connection)?.await
}
