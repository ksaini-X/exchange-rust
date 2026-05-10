use std::env;

pub mod models;
pub mod schema;

use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
};
use dotenvy::dotenv;

pub fn connection() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let db = env::var("DATABASE_URL").expect("DB should exist");
    let manager = ConnectionManager::<PgConnection>::new(db);
    r2d2::Pool::builder().build(manager).expect("Fail")
}
