mod app;
pub mod db;
pub mod log;

use crate::utils::env::{read_int_env, read_str_env};
use derive_more::Display;

#[derive(Debug)]
pub struct AppConfig {
    pub server_config: app::ServerConfig,
    pub db_config: db::DBConfig,
}

impl AppConfig {
    pub fn new(env: &AppEnv) -> Self {
        load_env(env);
        AppConfig {
            server_config: app::ServerConfig {
                host: read_str_env("SERVER_HOST", "localhost".into()),
                port: read_int_env("SERVER_PORT", 8082) as u16,
                log_level: read_str_env("LOG_LEVEL", "info".into()),
            },
            db_config: db::DBConfig {
                db_url: read_str_env(
                    "DATABASE_URL",
                    "postgres://postgres:postgres@localhost:5432/shop-db".into(),
                ),
            },
        }
    }
}

fn load_env(env: &AppEnv) {
    match env {
        AppEnv::Dev => match dotenvy::dotenv() {
            Ok(_) => {}
            Err(e) => panic!("Could not load .env file: {e}"),
        },
        AppEnv::Prod => {}
    }
}

#[derive(Display)]
pub enum AppEnv {
    #[display("dev")]
    Dev,
    #[display("prod")]
    Prod,
}
