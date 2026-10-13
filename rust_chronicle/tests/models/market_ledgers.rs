use loco_rs::testing::prelude::*;
use rust_chronicle::{app::App, models::_entities::market_ledgers};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_query_market_ledgers() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let items = market_ledgers::Entity::find()
        .all(&boot.app_context.db)
        .await
        .expect("`market_ledgers` should be queryable");

    assert_eq!(items.len(), 5, "expected 5 seeded market ledgers");

    let bessemer = market_ledgers::Entity::find()
        .filter(market_ledgers::Column::ItemName.eq("Bessemer Heavy 50"))
        .one(&boot.app_context.db)
        .await
        .unwrap()
        .expect("Bessemer item should exist");

    assert_eq!(bessemer.spot_value, "4,128.90");
}
