use loco_rs::testing::prelude::*;
use rust_chronicle::{app::App, models::_entities::categories};
use sea_orm::EntityTrait;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_query_categories() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let items = categories::Entity::find()
        .all(&boot.app_context.db)
        .await
        .expect("`categories` should be queryable — entity and migration must agree");

    assert_eq!(items.len(), 5, "expected 5 seeded categories");
}
