mod routes;
mod models;
mod services;
mod utils;
mod mongo_connection;

use actix_web::{web, middleware::Logger};
use routes::{login, index, login_form};
use actix_web::{App, HttpServer};
use tera::{Tera, Context};
use crate::routes::{handle_client_registration, page_client_registration};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let data = 30;
    let my_data = web::Data::new(data);
    let tera = web::Data::new(Tera::new("src/templates/**/*").expect("Error parsing templates directory"));

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(my_data.clone())
            .app_data(tera.clone())
            .wrap(logger)
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(index)
            .service(login)
            .service(login_form)
            .service(page_client_registration)
            .service(handle_client_registration)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}