use actix_web::{App, HttpServer, middleware, web};

#[actix_web::main]
async fn main() {
    HttpServer::new(move || App::new().app_data(data))
}
