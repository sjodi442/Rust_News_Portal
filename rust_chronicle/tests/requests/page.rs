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

/// Syndicated RSS 2.0 Feed at /rss.xml and /feed.xml.
#[tokio::test]
#[serial]
async fn can_get_rss_feed() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/rss.xml").await;

        assert_eq!(res.status_code(), 200);
        let text = res.text();
        assert!(text.contains("<?xml"), "expected xml header");
        assert!(text.contains("<rss version=\"2.0\""), "expected rss 2.0 tag");
        assert!(text.contains("<channel>"), "expected channel tag");
        assert!(
            text.contains("<title>The Rust Chronicle"),
            "expected chronicle channel title"
        );
        assert!(text.contains("<item>"), "expected syndicated item");

        // Alternate route alias /feed.xml
        let res_alias = request.get("/feed.xml").await;
        assert_eq!(res_alias.status_code(), 200);
        assert!(res_alias.text().contains("<rss version=\"2.0\""));
    })
    .await;
}

/// Dynamic XML Sitemap at /sitemap.xml.
#[tokio::test]
#[serial]
async fn can_get_sitemap() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/sitemap.xml").await;

        assert_eq!(res.status_code(), 200);
        let text = res.text();
        assert!(text.contains("<?xml"), "expected xml header");
        assert!(
            text.contains("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">"),
            "expected sitemap urlset"
        );
        assert!(text.contains("<loc>"), "expected location tags");
        assert!(
            text.contains("/articles/artery-of-salt-and-steel-estuary-span"),
            "expected article URL in sitemap"
        );
    })
    .await;
}

/// Validates OpenGraph, Twitter card, and feed discovery meta tags.
#[tokio::test]
#[serial]
async fn can_render_opengraph_and_discovery_tags() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/articles/artery-of-salt-and-steel-estuary-span").await;

        assert_eq!(res.status_code(), 200);
        let text = res.text();
        assert!(
            text.contains("property=\"og:site_name\" content=\"The Rust Chronicle\""),
            "expected og:site_name"
        );
        assert!(
            text.contains("property=\"og:type\" content=\"article\""),
            "expected og:type=article for reader page"
        );
        assert!(
            text.contains("name=\"twitter:card\" content=\"summary_large_image\""),
            "expected twitter:card"
        );
        assert!(
            text.contains("rel=\"alternate\" type=\"application/rss+xml\""),
            "expected RSS feed auto-discovery link"
        );
        assert!(
            text.contains("rel=\"sitemap\" type=\"application/xml\""),
            "expected Sitemap discovery link"
        );
    })
    .await;
}
