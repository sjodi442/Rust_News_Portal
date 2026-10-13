use loco_rs::testing::prelude::*;
use rust_chronicle::{app::App, models::_entities::wire_dispatches};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_query_wire_dispatches() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let items = wire_dispatches::Entity::find()
        .all(&boot.app_context.db)
        .await
        .expect("`wire_dispatches` should be queryable");

    assert_eq!(items.len(), 5, "expected 5 seeded wire dispatches");

    let urgent = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsUrgent.eq(true))
        .all(&boot.app_context.db)
        .await
        .unwrap();

    assert_eq!(urgent.len(), 1);
    assert!(urgent[0].headline.contains("GALE WARNING"));
}
