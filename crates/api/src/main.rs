use actix_web::{App, HttpServer, middleware, web};
use db::connection;
#[actix_web::main]
async fn main() {
    let pool = web::Data::new(connection());
    HttpServer::new(move || App::new().app_data(pool.clone()))
}
