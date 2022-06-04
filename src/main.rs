mod config;
mod crud;
mod error;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use env_logger::Env;
use error::MyError;
use log::info;
use tokio_postgres::NoTls;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("migrations");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::new().default_filter_or("info"));

    let config = crate::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(None, NoTls).unwrap();

    run_migrations(&pool)
        .await
        .expect("can run DB migrations: {}");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api").service(crud::services()))
    })
    .bind(config.server_addr.clone())?
    .run();

    info!("Server running at http://{}/", config.server_addr);

    server.await
}

async fn run_migrations(db_pool: &deadpool_postgres::Pool) -> std::result::Result<(), Error> {
    info!("Running DB migrations...");
    let mut client = db_pool.get().await.map_err(MyError::PoolError)?;
    let migration_report = embedded::migrations::runner()
        .run_async(&mut **client)
        .await?;

    for migration in migration_report.applied_migrations() {
        info!(
            "Migration Applied -  Name: {}, Version: {}",
            migration.name(),
            migration.version()
        );
    }

    info!("DB migrations finished!");

    Ok(())
}
