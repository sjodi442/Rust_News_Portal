use loco_rs::testing::prelude::*;
use rust_chronicle::app::App;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_wire_ticker() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/api/ticker/wires").await;

        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("EDINBURGH"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_market_ticker() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/api/ticker/markets").await;

        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("Bessemer Heavy 50"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_submit_reader_dispatch() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let payload = serde_json::json!({
            "author_name": "Prof. Alistair Sterling",
            "affiliation": "Institute of Marine Engineering",
            "body": "Regarding the tidal barrier maintenance frequency, our telemetry suggests bi-weekly inspections are warranted.",
            "article_id": 1,
            "is_technical_note": true
        });

        let res = request.post("/api/dispatches").json(&payload).await;

        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("\"success\":true"));
        assert!(res.text().contains("Prof. Alistair Sterling"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_submit_empty_dispatch() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let payload = serde_json::json!({
            "author_name": "",
            "body": ""
        });

        let res = request.post("/api/dispatches").json(&payload).await;

        assert_eq!(res.status_code(), 400);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_subscribe_newsletter() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let payload = serde_json::json!({
            "email": "reader.dispatch@industrial-gazette.org"
        });

        let res = request.post("/api/subscribe").json(&payload).await;

        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("\"success\":true"));
        assert!(res.text().contains("reader.dispatch@industrial-gazette.org"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_subscribe_invalid_email() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let payload = serde_json::json!({
            "email": "not-an-email"
        });

        let res = request.post("/api/subscribe").json(&payload).await;

        assert_eq!(res.status_code(), 400);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_wire_ticker_fragment() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/api/wire/latest").await;

        assert_eq!(res.status_code(), 200);
        let text = res.text();
        assert!(text.contains("TELEGRAPH WIRE:"));
        assert!(text.contains("EDINBURGH"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_search_fragment() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/api/search/fragment?q=Salt").await;

        assert_eq!(res.status_code(), 200);
        let text = res.text();
        assert!(text.contains("The Artery of Salt"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_submit_dispatch_fragment() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let payload = [
            ("author_name", "Capt. Horatio Nelson"),
            ("affiliation", "Admiralty"),
            ("body", "Navigation charts updated for the estuary."),
            ("is_technical_note", "true"),
        ];

        let res = request.post("/api/dispatches/submit").form(&payload).await;

        assert_eq!(res.status_code(), 200);
        let text = res.text();
        assert!(text.contains("Capt. Horatio Nelson"));
        assert!(text.contains("TECHNICAL MEMORANDUM"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_subscribe_fragment() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let payload = [("email", "officer@naval-yard.org")];

        let res = request.post("/api/subscribe/submit").form(&payload).await;

        assert_eq!(res.status_code(), 200);
        let text = res.text();
        assert!(text.contains("Enrolled successfully"));
    })
    .await;
}
