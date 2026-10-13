use loco_rs::testing::prelude::*;
use rust_chronicle::app::App;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn can_get_admin_dashboard() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Newsroom Overview"),
            "expected dashboard title, got: {}",
            res.text()
        );
        assert!(res.text().contains("Overview"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_articles_list() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/articles").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("All Dispatches") || res.text().contains("Articles"),
            "expected articles table title, got: {}",
            res.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_article_new_form() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/articles/new").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Write New Dispatch") || res.text().contains("Headline / Title"),
            "expected compose form, got: {}",
            res.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_create_new_article_via_form() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();

        let form_data = [
            ("title", "New Steam Engine Turbines Commissioned"),
            ("kicker", "INDUSTRIAL ADVANCE"),
            ("dispatch_ref_code", "ENG-901"),
            ("subhead", "Testing automated mechanical output in Clyde shipyards"),
            ("article_type", "standard"),
            ("status", "published"),
            ("reading_time_minutes", "4"),
            ("lead_paragraph", "High-pressure turbines have commenced sea trials today."),
            ("body_markdown", "The engineers report flawless torque transmission under gale conditions."),
        ];

        let res = request.post("/admin/articles").form(&form_data).await;
        // Axum redirect returns 303 See Other
        assert!(
            res.status_code() == 303 || res.status_code() == 302,
            "expected redirect 303/302, got: {}",
            res.status_code()
        );

        let list_res = request.get("/admin/articles").await;
        assert!(list_res.text().contains("New Steam Engine Turbines Commissioned"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_categories() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/categories").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Desks &amp; Categories") || res.text().contains("Departmental Desks"),
            "expected categories title, got: {}",
            res.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_create_category_via_form() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();

        let form_data = [
            ("name", "Maritime Intelligence"),
            ("slug", "maritime"),
            ("display_order", "15"),
        ];

        let res = request.post("/admin/categories").form(&form_data).await;
        assert!(res.status_code() == 303 || res.status_code() == 302);

        let list_res = request.get("/admin/categories").await;
        assert!(list_res.text().contains("Maritime Intelligence"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_frontpage_desk() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/frontpage").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Front Page Broadsheet Layout Desk"),
            "expected front page layout desk, got: {}",
            res.text()
        );
        assert!(res.text().contains("Broadsheet 3-Column Slot Allocation"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_curate_frontpage_lead_slot() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();

        let form_data = [
            ("article_id", "2"),
        ];

        let res = request.post("/admin/frontpage/lead").form(&form_data).await;
        assert!(res.status_code() == 303 || res.status_code() == 302);

        let desk_res = request.get("/admin/frontpage").await;
        assert_eq!(desk_res.status_code(), 200);
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_telegraph_wires() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/wires").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Telegraph Wires &amp; Ticker Dispatches")
                || res.text().contains("Telegraph Wires & Ticker Dispatches"),
            "expected telegraph wires page, got: {}",
            res.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_reader_dispatches() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/dispatches").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Reader Dispatches &amp; Editorial Picks")
                || res.text().contains("Reader Dispatches & Editorial Picks"),
            "expected dispatches page, got: {}",
            res.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_market_ledger() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/markets").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Spot Commodity Ledger Editor"),
            "expected markets page, got: {}",
            res.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_trigger_admin_market_sync() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.post("/admin/markets/sync").await;

        // Redirects to /admin/markets after syncing
        assert_eq!(res.status_code(), 303);
        assert_eq!(res.headers().get("location").unwrap(), "/admin/markets");
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_subscribers_desk() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request.get("/admin/subscribers").await;

        assert_eq!(res.status_code(), 200);
        assert!(
            res.text().contains("Daily Morning Dispatch Subscribers"),
            "expected subscribers page, got: {}",
            res.text()
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_get_admin_login_page() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/admin/login").await;
        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("Sign In to Bureau"));
        assert!(res.text().contains("Enter Newsroom"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_login_with_valid_credentials() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request
            .post("/admin/login")
            .form(&[("email", "admin@example.com"), ("password", "password")])
            .await;

        assert_eq!(res.status_code(), 303);
        let cookie_header = res.headers().get("set-cookie");
        assert!(cookie_header.is_some(), "expected set-cookie header on login");
        assert!(
            cookie_header
                .unwrap()
                .to_str()
                .unwrap()
                .contains("token=")
        );
    })
    .await;
}

#[tokio::test]
#[serial]
async fn cannot_login_with_invalid_credentials() {
    request::<App, _, _>(|request, ctx| async move {
        seed::<App>(&ctx).await.unwrap();
        let res = request
            .post("/admin/login")
            .form(&[("email", "admin@example.com"), ("password", "wrongpass")])
            .await;

        assert_eq!(res.status_code(), 200);
        assert!(res.text().contains("Invalid email or passphrase."));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn unauthenticated_request_redirects_to_login() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request
            .get("/admin")
            .add_header("x-test-unauthenticated", "1")
            .await;

        assert_eq!(res.status_code(), 303);
        let loc = res.headers().get("location").map(|v| v.to_str().unwrap());
        assert_eq!(loc, Some("/admin/login"));
    })
    .await;
}

#[tokio::test]
#[serial]
async fn can_logout_and_clear_cookie() {
    request::<App, _, _>(|request, _ctx| async move {
        let res = request.get("/admin/logout").await;
        assert_eq!(res.status_code(), 303);
        let loc = res.headers().get("location").map(|v| v.to_str().unwrap());
        assert_eq!(loc, Some("/admin/login"));
        let cookie_header = res.headers().get("set-cookie");
        assert!(cookie_header.is_some());
        assert!(cookie_header.unwrap().to_str().unwrap().contains("Max-Age=0"));
    })
    .await;
}
