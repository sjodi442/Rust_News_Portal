use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};
use std::time::Duration;
use crate::models::market_ledgers;

pub struct MarketSyncInitializer;

#[async_trait]
impl Initializer for MarketSyncInitializer {
    fn name(&self) -> String {
        "market-sync".to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> Result<()> {
        let db = ctx.db.clone();

        tokio::spawn(async move {
            tracing::info!("Starting realtime market background sync worker...");
            if let Err(e) = market_ledgers::sync_live_commodities(&db).await {
                tracing::warn!("Initial realtime market sync note: {}", e);
            }

            let mut interval = tokio::time::interval(Duration::from_secs(300));
            loop {
                interval.tick().await;
                if let Err(e) = market_ledgers::sync_live_commodities(&db).await {
                    tracing::warn!("Periodic realtime market sync note: {}", e);
                }
            }
        });

        Ok(())
    }
}
