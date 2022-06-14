use entity::user;
use sea_orm::schema::Schema;
use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220120_000001_create_post_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db_backend = manager.get_database_backend();
        let schema = Schema::new(db_backend);
        manager
            .create_table(schema.create_table_from_entity(user::Entity))
            .await
    }

    // if you are against backward migrations, you do not have to impl this
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(sea_query::Table::drop().table(user::Entity).to_owned())
            .await
    }
}
