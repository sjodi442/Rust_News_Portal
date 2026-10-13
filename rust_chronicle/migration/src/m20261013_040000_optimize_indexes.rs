use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_articles_status_pub ON articles (status, published_at DESC);
             CREATE INDEX IF NOT EXISTS idx_articles_cat_pub ON articles (category_id, published_at DESC);
             CREATE INDEX IF NOT EXISTS idx_articles_type ON articles (article_type);
             CREATE INDEX IF NOT EXISTS idx_wires_active_urgent ON wire_dispatches (is_active, is_urgent);
             CREATE INDEX IF NOT EXISTS idx_dispatches_editors_pick ON reader_dispatches (is_editors_pick);
             CREATE INDEX IF NOT EXISTS idx_markets_category ON market_ledgers (category);"
        ).await?;
        Ok(())
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        let db = m.get_connection();
        db.execute_unprepared(
            "DROP INDEX IF EXISTS idx_articles_status_pub;
             DROP INDEX IF EXISTS idx_articles_cat_pub;
             DROP INDEX IF EXISTS idx_articles_type;
             DROP INDEX IF EXISTS idx_wires_active_urgent;
             DROP INDEX IF EXISTS idx_dispatches_editors_pick;
             DROP INDEX IF EXISTS idx_markets_category;"
        ).await?;
        Ok(())
    }
}
