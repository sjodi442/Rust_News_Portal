use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "market_ledgers",
            &[
                ("id", ColType::PkAuto),
                ("item_name", ColType::String),
                (
                    "category",
                    ColType::StringWithDefault("commodities".to_string()),
                ),
                ("specification", ColType::StringNull),
                ("spot_value", ColType::String),
                ("change_pct", ColType::FloatWithDefault(0.0)),
                ("unit", ColType::StringNull),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "market_ledgers").await
    }
}
