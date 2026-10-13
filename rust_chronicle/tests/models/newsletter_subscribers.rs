use loco_rs::testing::prelude::*;
use rust_chronicle::{app::App, models::_entities::newsletter_subscribers};
use sea_orm::EntityTrait;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_query_newsletter_subscribers() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let items = newsletter_subscribers::Entity::find()
        .all(&boot.app_context.db)
        .await
        .expect("`newsletter_subscribers` should be queryable");

    assert_eq!(items.len(), 2, "expected 2 seeded newsletter subscribers");
}
