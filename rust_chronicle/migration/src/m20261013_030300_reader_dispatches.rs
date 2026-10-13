use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "reader_dispatches",
            &[
                ("id", ColType::PkAuto),
                ("article_id", ColType::BigIntegerNull),
                ("author_name", ColType::String),
                ("affiliation", ColType::StringNull),
                ("body", ColType::Text),
                ("is_editors_pick", ColType::BooleanWithDefault(false)),
                ("is_technical_note", ColType::BooleanWithDefault(false)),
                ("endorsements_count", ColType::IntegerWithDefault(0)),
            ],
            &[("articles?", "article_id")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "reader_dispatches").await
    }
}
