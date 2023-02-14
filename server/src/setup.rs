use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgConnection;
use std::net::TcpListener;

pub fn run(listner: TcpListener, connection: PgConnection) -> Result<Server, std::io::Error> {
    // DBサーバに対しては各スレッドで同一の接続定義を保有する必要があるため、
    // connectionのポインタを取得して新しいスレッドに引き渡す
    let connection = web::Data::new(connection);

    // 新しいスレッドを起動する
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(connection.clone())
    })
    .listen(listner)?
    .run();
    Ok(server)
}
