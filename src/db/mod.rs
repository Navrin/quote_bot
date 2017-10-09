use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn create_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL isn't defined as an env var!");

    PgConnection::establish(&database_url)
        .expect(&format!("Could not connect to: {}", database_url))
}

pub mod models;
pub mod schema;
pub mod actions;
