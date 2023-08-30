mod routes;
mod models;
use actix_web::{middleware::Logger};
use routes::{login, index};
use actix_web::{App, HttpServer};



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(login)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}