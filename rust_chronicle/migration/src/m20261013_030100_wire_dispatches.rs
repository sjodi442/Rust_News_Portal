use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "wire_dispatches",
            &[
                ("id", ColType::PkAuto),
                ("headline", ColType::String),
                ("location", ColType::StringNull),
                ("gmt_timestamp", ColType::String),
                ("is_urgent", ColType::BooleanWithDefault(false)),
                ("is_active", ColType::BooleanWithDefault(true)),
                ("published_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "wire_dispatches").await
    }
}
