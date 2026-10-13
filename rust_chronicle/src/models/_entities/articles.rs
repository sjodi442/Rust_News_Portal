use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "articles")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub category_id: Option<i64>,
    #[sea_orm(unique)]
    pub slug: String,
    pub title: String,
    pub kicker: Option<String>,
    pub subhead: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub lead_paragraph: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub body_markdown: String,
    #[sea_orm(column_type = "Text")]
    pub body_html: String,
    pub article_type: String,
    pub status: String,
    pub dispatch_ref_code: Option<String>,
    pub cover_image_url: Option<String>,
    pub cover_image_caption: Option<String>,
    pub cover_image_credit: Option<String>,
    pub reading_time_minutes: i32,
    pub view_count: i32,
    pub published_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Category,
    #[sea_orm(has_many = "super::reader_dispatches::Entity")]
    ReaderDispatches,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::reader_dispatches::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ReaderDispatches.def()
    }
}
