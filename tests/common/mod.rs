use migration::{Migrator, MigratorTrait};
use rust_crud_rest_api::config;
use sea_orm::DatabaseConnection;

pub async fn init_app_state() -> DatabaseConnection {
    let config = config::Config::from_file_name("tests/config").unwrap();
    let database_url = config.database_url();
    let db = sea_orm::Database::connect(database_url).await.unwrap();
    Migrator::reset(&db).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    db
}
