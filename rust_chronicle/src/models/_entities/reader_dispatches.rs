use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "reader_dispatches")]
pub struct Model {
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
    #[sea_orm(primary_key)]
    pub id: i64,
    pub article_id: Option<i64>,
    pub author_name: String,
    pub affiliation: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub body: String,
    pub is_editors_pick: bool,
    pub is_technical_note: bool,
    pub endorsements_count: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::articles::Entity",
        from = "Column::ArticleId",
        to = "super::articles::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Article,
}

impl Related<super::articles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Article.def()
    }
}
