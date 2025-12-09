use sqlx::{postgres::PgPoolOptions, Pool};

use crate::config::db::DBConfig;
pub mod repository;

pub trait DB {}
impl DB for Postgres {}

pub struct Postgres {
    pub pool: Pool<sqlx::Postgres>,
}

impl Postgres {
    pub async fn new(config: DBConfig) -> Self {
        let pool = match PgPoolOptions::new().connect(&config.db_url).await {
            Ok(p) => p,
            Err(e) => {
                panic!("Cannot connect to postgres db: {}", e)
            }
        };

        log::info!("Connect to Postgres successful");

        Postgres { pool }
    }
}
