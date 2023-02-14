use crate::routes::{health_check, subscribe};
use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listner: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    // DBサーバに対しては各worker間で同一の接続定義を共有する必要があるため、
    // db_poolのポインタを取得して新しいworkerに引き渡す
    let db_pool = web::Data::new(db_pool);

    // 新しいworkerを起動する
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listner)?
    .run();
    Ok(server)
}
