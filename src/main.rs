use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use log::info;
use migration::{Migrator, MigratorTrait};
use rust_crud_rest_api::{config, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // load .env file
    dotenv().ok();

    // init config from environment
    let config = config::Config::from_env().unwrap();

    // init logger
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // init database connection
    let db = sea_orm::Database::connect(config.database_url())
        .await
        .unwrap();

    // migration database
    Migrator::up(&db, None).await.unwrap();

    // init state
    let state = web::Data::new(db);

    // inint server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(state.clone())
            .service(web::scope("/api").configure(users::config))
    })
    .bind(config.server_addr().clone())?;

    // run server
    info!("Starting server at http://{}/", config.server_addr());
    server.run().await
}
