use actix_web::{App, HttpServer};
use simple_shop_backend::{
    config::{log::init_logger, AppConfig, AppEnv},
    db::{self, repository::UserRepo},
    models::user::Role,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Config init
    let env = AppEnv::Dev;
    let config = AppConfig::new(&env);

    // Setup log
    init_logger(config.server_config.log_level);

    let pdb = db::Postgres::new(config.db_config).await;

    // match sqlx::query(r#"SELECT * from "users""#)
    //     .fetch_all(&pdb.pool)
    //     .await
    // {
    //     Ok(data) => {
    //         log::info!("{:?}", data);
    //     }
    //     Err(e) => {
    //         log::error!("{}", e);
    //     }
    // };

    let user_repo = db::repository::users::UserRepository::new(pdb.pool);
    let id = user_repo
        .create_user("Johndoedoe", "123", Role::Admin)
        .await
        .unwrap();
    let user = user_repo.get_user(id).await.unwrap();
    log::info!("{:?}", user);

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
