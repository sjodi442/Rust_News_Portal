use sea_orm::{entity::prelude::*, FromQueryResult, QueryOrder, QuerySelect};
pub use super::_entities::articles::{self, ActiveModel, Entity, Model};

#[derive(Debug, FromQueryResult)]
struct ArticleIdOnly {
    id: i64,
}

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

impl Model {
    /// Mengembalikan tanggal publikasi artikel dalam format broadsheet resmi (e.g. "Friday, October 24, 2025")
    pub fn formatted_published_date(&self) -> String {
        self.published_at
            .as_ref()
            .unwrap_or(&self.created_at)
            .format("%A, %B %-d, %Y")
            .to_string()
    }

    /// Melakukan pencarian artikel menggunakan SQLite FTS5 dengan fallback ke LIKE jika FTS query error.
    pub async fn search_fts(db: &DatabaseConnection, query_str: &str) -> Result<Vec<Model>, DbErr> {
        let trimmed = query_str.trim();
        if trimmed.is_empty() {
            return Ok(Vec::new());
        }

        // Membersihkan karakter berbahaya untuk sintaks query FTS5
        let cleaned: String = trimmed
            .chars()
            .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
            .collect();
        let words: Vec<&str> = cleaned.split_whitespace().collect();

        if words.is_empty() {
            return Ok(Vec::new());
        }

        // Format FTS5 query dengan wildcard prefix match pada kata terakhir
        let fts_query = if words.len() == 1 {
            format!("{}*", words[0])
        } else {
            let last_idx = words.len() - 1;
            words
                .iter()
                .enumerate()
                .map(|(idx, w)| {
                    if idx == last_idx {
                        format!("{}*", w)
                    } else {
                        (*w).to_owned()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ")
        };

        // Query FTS5 dengan ranking BM25 bawaan SQLite
        let raw_sql = format!(
            "SELECT a.id FROM articles a \
             JOIN articles_fts f ON a.id = f.rowid \
             WHERE articles_fts MATCH ? AND a.status = 'published' \
             ORDER BY bm25(articles_fts) ASC, a.id DESC \
             LIMIT 50"
        );

        let search_res = ArticleIdOnly::find_by_statement(
            sea_orm::Statement::from_sql_and_values(
                sea_orm::DatabaseBackend::Sqlite,
                &raw_sql,
                vec![fts_query.into()],
            )
        )
        .all(db)
        .await;

        match search_res {
            Ok(ids) if !ids.is_empty() => {
                let id_list: Vec<i64> = ids.into_iter().map(|item| item.id).collect();
                // Query model artikel terurut sesuai urutan ranking FTS5
                let mut articles = Entity::find()
                    .filter(articles::Column::Id.is_in(id_list.clone()))
                    .all(db)
                    .await?;

                // Pertahankan urutan ranking FTS5
                articles.sort_by_key(|a| {
                    id_list.iter().position(|&id| id == a.id).unwrap_or(usize::MAX)
                });

                Ok(articles)
            }
            Ok(_) => Ok(Vec::new()),
            Err(e) => {
                tracing::warn!("FTS5 query failed, falling back to LIKE: {}", e);
                // Fallback ke LIKE query tradisional jika FTS5 tidak tersedia / sintaks invalid
                let cond = sea_orm::Condition::any()
                    .add(articles::Column::Title.contains(trimmed))
                    .add(articles::Column::Subhead.contains(trimmed))
                    .add(articles::Column::LeadParagraph.contains(trimmed))
                    .add(articles::Column::BodyMarkdown.contains(trimmed));

                Entity::find()
                    .filter(articles::Column::Status.eq("published"))
                    .filter(cond)
                    .order_by_desc(articles::Column::Id)
                    .limit(50)
                    .all(db)
                    .await
            }
        }
    }
}
