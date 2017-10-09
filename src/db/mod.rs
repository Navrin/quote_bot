use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2;
use r2d2_diesel::{ConnectionManager};
use std::env;

use typemap::Key;

pub struct Connector(r2d2::Pool<ConnectionManager<PgConnection>>);

impl Connector {
    pub fn new() -> Connector {
        let config = r2d2::Config::builder()
            .pool_size(15)
            .build();
        
        Connector(r2d2::Pool::new(config, create_connection_manager()).unwrap())
    }

    pub fn get_conn(&self) -> Result<r2d2::PooledConnection<ConnectionManager<PgConnection>>, r2d2::GetTimeout> {
        self.0.clone().get() 
    }
}

impl Key for Connector {
    type Value = Connector;
}

fn create_connection_manager() -> ConnectionManager<PgConnection> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Database URL isn't defined as an env var!");

    ConnectionManager::new(database_url)    // .expect(&format!("Could not connect to: {}", database_url))
}

pub mod models;
pub mod schema;
pub mod actions;
