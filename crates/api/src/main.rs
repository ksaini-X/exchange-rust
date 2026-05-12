use actix_web::{App, web};
pub mod redis;
pub mod routes;
pub mod types;

use crate::routes::order::config as order_config;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    actix_web::HttpServer::new(move || App::new().service(web::scope("/").configure(order_config)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
