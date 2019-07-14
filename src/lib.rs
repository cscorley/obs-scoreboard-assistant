#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::env;
use diesel::{r2d2::ConnectionManager, r2d2::Pool, PgConnection};

pub fn establish_connection() -> models::Pool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
