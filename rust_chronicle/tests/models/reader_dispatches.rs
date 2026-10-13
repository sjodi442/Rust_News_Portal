use loco_rs::testing::prelude::*;
use rust_chronicle::{
    app::App,
    models::_entities::{articles, reader_dispatches},
};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_query_reader_dispatches() {
    let boot = boot_test::<App>().await.unwrap();
    seed::<App>(&boot.app_context).await.unwrap();

    let items = reader_dispatches::Entity::find()
        .all(&boot.app_context.db)
        .await
        .expect("`reader_dispatches` should be queryable");

    assert_eq!(items.len(), 2, "expected 2 seeded reader dispatches");

    let dispatch = reader_dispatches::Entity::find()
        .filter(reader_dispatches::Column::AuthorName.eq("Dr. Alistair MacIntyre"))
        .one(&boot.app_context.db)
        .await
        .unwrap()
        .expect("dispatch should exist");

    assert!(dispatch.is_editors_pick);
    assert!(dispatch.is_technical_note);

    let article = dispatch
        .find_related(articles::Entity)
        .one(&boot.app_context.db)
        .await
        .unwrap()
        .expect("related article should exist");

    assert_eq!(article.id, 1);
}
