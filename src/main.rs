use actix_web::{App, HttpServer};
use simple_shop_backend::config::{log::init_logger, AppConfig, AppEnv};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Config init
    let config = AppConfig::new(AppEnv::Dev);

    // Setup log
    init_logger(config.server_config.log_level);

    let server = HttpServer::new(|| App::new())
        .bind((
            config.server_config.host.as_str(),
            config.server_config.port,
        ))?
        .run();

    log::info!(
        "Start server at http://{}:{}",
        config.server_config.host,
        config.server_config.port
    );

    server.await
}
