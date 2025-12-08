use actix_web::{App, HttpServer};
use simple_shop_backend::{
    config::{log::init_logger, AppConfig, AppEnv},
    db,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Config init
    let env = AppEnv::Dev;
    let config = AppConfig::new(&env);

    // Setup log
    init_logger(config.server_config.log_level);

    let pdb = db::Postgres::new(config.db_config).await;

    match sqlx::query(r#"SELECT * from "users""#)
        .fetch_all(&pdb.pool)
        .await
    {
        Ok(data) => {
            log::info!("{:?}", data);
        }
        Err(e) => {
            log::error!("{}", e);
        }
    };

    let server = HttpServer::new(|| App::new())
        .bind((
            config.server_config.host.as_str(),
            config.server_config.port,
        ))?
        .run();

    log::info!(
        "Start server at http://{}:{} with {} env",
        config.server_config.host,
        config.server_config.port,
        env
    );

    server.await
}
