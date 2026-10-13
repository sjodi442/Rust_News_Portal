use loco_rs::testing::prelude::*;
use rust_chronicle::{
    app::App,
    models::_entities::{articles, categories},
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_query_articles_and_relation() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let items = articles::Entity::find()
        .all(&boot.app_context.db)
        .await
        .expect("`articles` should be queryable");

    assert_eq!(items.len(), 5, "expected 5 seeded articles");

    // Verify lead story article exists and relates to category
    let lead = articles::Entity::find()
        .filter(articles::Column::ArticleType.eq("lead"))
        .one(&boot.app_context.db)
        .await
        .unwrap()
        .expect("lead article should exist");

    assert!(lead
        .title
        .contains("The Artery of Salt & Steel: Inside the 100-Year Overhaul"));

    // Verify relation to category
    let category = lead
        .find_related(categories::Entity)
        .one(&boot.app_context.db)
        .await
        .unwrap()
        .expect("related category should exist");

    assert_eq!(category.slug, "infrastructure");
}
