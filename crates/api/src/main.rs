use actix_web::App;
pub mod redis;
pub mod types;
#[actix_web::main]
async fn main() {
    actix_web::HttpServer::new(move || {
        App::new().
    })
}
