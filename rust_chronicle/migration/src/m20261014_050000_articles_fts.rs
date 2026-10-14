use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();

        // 1. Buat FTS5 virtual table
        db.execute_unprepared(
            "CREATE VIRTUAL TABLE IF NOT EXISTS articles_fts USING fts5(
                title,
                subhead,
                lead_paragraph,
                body_markdown,
                content='articles',
                content_rowid='id'
            );"
        ).await?;

        // 2. Populasi awal data yang sudah ada di tabel articles
        db.execute_unprepared(
            "INSERT INTO articles_fts(rowid, title, subhead, lead_paragraph, body_markdown)
             SELECT id, title, coalesce(subhead, ''), lead_paragraph, body_markdown FROM articles;"
        ).await?;

        // 3. Triggers sinkronisasi otomatis: INSERT, UPDATE, DELETE
        db.execute_unprepared(
            "CREATE TRIGGER IF NOT EXISTS articles_ai AFTER INSERT ON articles BEGIN
                INSERT INTO articles_fts(rowid, title, subhead, lead_paragraph, body_markdown)
                VALUES (new.id, new.title, coalesce(new.subhead, ''), new.lead_paragraph, new.body_markdown);
             END;
             
             CREATE TRIGGER IF NOT EXISTS articles_ad AFTER DELETE ON articles BEGIN
                INSERT INTO articles_fts(articles_fts, rowid, title, subhead, lead_paragraph, body_markdown)
                VALUES ('delete', old.id, old.title, coalesce(old.subhead, ''), old.lead_paragraph, old.body_markdown);
             END;

             CREATE TRIGGER IF NOT EXISTS articles_au AFTER UPDATE ON articles BEGIN
                INSERT INTO articles_fts(articles_fts, rowid, title, subhead, lead_paragraph, body_markdown)
                VALUES ('delete', old.id, old.title, coalesce(old.subhead, ''), old.lead_paragraph, old.body_markdown);
                INSERT INTO articles_fts(rowid, title, subhead, lead_paragraph, body_markdown)
                VALUES (new.id, new.title, coalesce(new.subhead, ''), new.lead_paragraph, new.body_markdown);
             END;"
        ).await?;

        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            "DROP TRIGGER IF EXISTS articles_ai;
             DROP TRIGGER IF EXISTS articles_ad;
             DROP TRIGGER IF EXISTS articles_au;
             DROP TABLE IF EXISTS articles_fts;"
        ).await?;
        Ok(())
    }
}
