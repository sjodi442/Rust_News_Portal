use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "categories",
            &[
                ("id", ColType::PkAuto),
                ("name", ColType::String),
                ("slug", ColType::StringUniq),
                ("display_order", ColType::IntegerWithDefault(0)),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "categories").await
    }
}
