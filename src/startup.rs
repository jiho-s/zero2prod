use std::io::Error;
use std::net::TcpListener;

use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use crate::email_client::EmailClient;

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener, connection_pool: PgPool, email_client: EmailClient) -> Result<Server, Error> {
    let db_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
        .listen(listener)?
        .run();

    Ok(server)
}
