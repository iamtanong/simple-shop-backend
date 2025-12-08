mod app;
mod db;
pub mod log;

use crate::utils;
use derive_more::Display;

#[derive(Debug)]
pub struct AppConfig {
    pub server_config: app::ServerConfig,
    pub db_config: db::DBConfig,
}

impl AppConfig {
    pub fn new(env: AppEnv) -> Self {
        load_env(env);
        AppConfig {
            server_config: app::ServerConfig {
                host: utils::env::read_str_env("SERVER_HOST", "localhost".into()),
                port: utils::env::read_int_env("SERVER_PORT", 8082) as u16,
                log_level: utils::env::read_str_env("LOG_LEVEL", "info".into()),
            },
            db_config: db::DBConfig {
                host: utils::env::read_str_env("POSTGRES_HOST", "localhost".into()),
                port: utils::env::read_int_env("POSTGRES_PORT", 5432) as u16,
                username: utils::env::read_str_env("POSTGRES_USERNAME", "admin".into()),
                password: utils::env::read_str_env("POSTGRES_PASSWORD", "admin123".into()),
                db_name: utils::env::read_str_env("POSTGRES_DBNAME", "shopdb".into()),
            },
        }
    }
}

fn load_env(env: AppEnv) {
    match env {
        AppEnv::Dev => match dotenvy::dotenv() {
            Ok(_) => println!("Load env success"),
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
