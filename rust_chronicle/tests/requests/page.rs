use loco_rs::testing::prelude::*;
use rust_chronicle::app::App;
use serial_test::serial;

/// The server-side rendered broadsheet homepage.
#[tokio::test]
#[serial]
async fn can_get_home_page() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("The Rust Chronicle"),
            "expected the rendered broadsheet view, got: {}",
            res.text()
        );
    })
    .await;
}

/// The server-side rendered editorial article reader.
#[tokio::test]
#[serial]
async fn can_get_article_reader_page() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/article").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("The Artery of Salt"),
            "expected the article headline, got: {}",
            res.text()
        );
    })
    .await;
}

/// The server-side rendered editorial article reader by slug.
#[tokio::test]
#[serial]
async fn can_get_article_reader_by_slug() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/articles/artery-of-salt-and-steel-estuary-span").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("The Artery of Salt"),
            "expected the article headline, got: {}",
            res.text()
        );

        // 404 for missing article
        let res_404 = request.get("/articles/non-existent-article-slug").await;
        assert_eq!(res_404.status_code(), 404);
    })
    .await;
}

/// The category archive view by category slug.
#[tokio::test]
#[serial]
async fn can_get_category_archive_page() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/category/infrastructure").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("DEPARTMENTAL ARCHIVE"),
            "expected department archive header, got: {}",
            res.text()
        );

        // 404 for missing category
        let res_404 = request.get("/category/non-existent-dept").await;
        assert_eq!(res_404.status_code(), 404);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_market_ledger_partial() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/partials/market_ledger").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("THE SPOT COMMODITY LEDGER"),
            "expected partial market ledger, got: {}",
            res.text()
        );
    })
    .await;
}

/// Historical archive search.
#[tokio::test]
#[serial]
async fn can_search_articles() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/search?q=Steel").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Search Archive"),
            "expected search results header, got: {}",
            res.text()
        );
        assert!(
            res.text().contains("The Artery of Salt"),
            "expected matching article, got: {}",
            res.text()
        );
    })
    .await;
}
