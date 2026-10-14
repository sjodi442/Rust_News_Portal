use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Initializer},
    Result,
};
use std::time::Duration;
use crate::models::{market_ledgers, meteorology};

pub struct MarketSyncInitializer;

#[async_trait]
impl Initializer for MarketSyncInitializer {
    fn name(&self) -> String {
        "market-sync".to_string()
    }

    async fn before_run(&self, ctx: &AppContext) -> Result<()> {
        if ctx.environment == loco_rs::environment::Environment::Test {
            return Ok(());
        }

        let db = ctx.db.clone();

        tokio::spawn(async move {
            tracing::info!("Starting realtime market and meteorology background sync worker...");
            if let Err(e) = market_ledgers::sync_live_commodities(&db).await {
                tracing::warn!("Initial realtime market sync note: {}", e);
            }
            meteorology::sync_meteorology().await;

            let mut interval = tokio::time::interval(Duration::from_secs(300));
            let mut tick_count: u32 = 0;
            loop {
                interval.tick().await;
                tick_count += 1;
                if let Err(e) = market_ledgers::sync_live_commodities(&db).await {
                    tracing::warn!("Periodic realtime market sync note: {}", e);
                }
                if tick_count % 2 == 0 {
                    meteorology::sync_meteorology().await;
                }
            }
        });

        Ok(())
    }
}
