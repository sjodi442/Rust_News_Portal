use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(
            m,
            "articles",
            &[
                ("id", ColType::PkAuto),
                ("category_id", ColType::BigIntegerNull),
                ("slug", ColType::StringUniq),
                ("title", ColType::String),
                ("kicker", ColType::StringNull),
                ("subhead", ColType::StringNull),
                ("lead_paragraph", ColType::TextNull),
                ("body_markdown", ColType::Text),
                ("body_html", ColType::Text),
                (
                    "article_type",
                    ColType::StringWithDefault("standard".to_string()),
                ),
                (
                    "status",
                    ColType::StringWithDefault("published".to_string()),
                ),
                ("dispatch_ref_code", ColType::StringNull),
                ("cover_image_url", ColType::StringNull),
                ("cover_image_caption", ColType::StringNull),
                ("cover_image_credit", ColType::StringNull),
                ("reading_time_minutes", ColType::IntegerWithDefault(5)),
                ("view_count", ColType::IntegerWithDefault(0)),
                ("published_at", ColType::TimestampWithTimeZoneNull),
            ],
            &[("categories?", "category_id")],
        )
        .await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "articles").await
    }
}
