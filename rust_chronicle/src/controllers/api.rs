use crate::models::_entities::{
    articles, market_ledgers, newsletter_subscribers, reader_dispatches, wire_dispatches,
};
use axum::extract::Form;
use loco_rs::prelude::*;
use sea_orm::{ActiveModelTrait, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDispatchParams {
    pub author_name: String,
    pub affiliation: Option<String>,
    pub body: String,
    pub article_id: Option<i64>,
    pub is_technical_note: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateDispatchForm {
    pub author_name: String,
    pub affiliation: Option<String>,
    pub body: String,
    pub article_id: Option<i64>,
    pub is_technical_note: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeForm {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchQuery {
    pub q: Option<String>,
}

/// GET /api/ticker/wires — List active telegraph wire dispatches (JSON).
#[debug_handler]
pub async fn list_wire_ticker(State(ctx): State<AppContext>) -> Result<Response> {
    let wires = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .order_by_desc(wire_dispatches::Column::Id)
        .all(&ctx.db)
        .await?;

    format::json(wires)
}

/// GET /api/ticker/markets — List current market spot quotations (JSON).
#[debug_handler]
pub async fn list_market_ticker(State(ctx): State<AppContext>) -> Result<Response> {
    let markets = market_ledgers::Entity::find()
        .order_by_asc(market_ledgers::Column::Id)
        .all(&ctx.db)
        .await?;

    format::json(markets)
}

/// POST /api/dispatches — Submit a reader dispatch / inquiry (JSON).
#[debug_handler]
pub async fn submit_dispatch(
    State(ctx): State<AppContext>,
    Json(params): Json<CreateDispatchParams>,
) -> Result<Response> {
    if params.author_name.trim().is_empty() || params.body.trim().is_empty() {
        return bad_request("Author name and body cannot be empty");
    }

    let item = reader_dispatches::ActiveModel {
        author_name: Set(params.author_name.trim().to_string()),
        affiliation: Set(params.affiliation.map(|a| a.trim().to_string())),
        body: Set(params.body.trim().to_string()),
        article_id: Set(params.article_id),
        is_editors_pick: Set(false),
        is_technical_note: Set(params.is_technical_note.unwrap_or(false)),
        endorsements_count: Set(0),
        ..Default::default()
    };

    let saved = item.insert(&ctx.db).await?;

    format::json(data!({
        "success": true,
        "dispatch_id": saved.id,
        "author": saved.author_name,
    }))
}

/// POST /api/subscribe — Subscribe to daily morning dispatch (JSON).
#[debug_handler]
pub async fn subscribe(
    State(ctx): State<AppContext>,
    Json(params): Json<SubscribeParams>,
) -> Result<Response> {
    let email = params.email.trim().to_lowercase();
    if !email.contains('@') || !email.contains('.') {
        return bad_request("Invalid email address provided");
    }

    let existing = newsletter_subscribers::Entity::find()
        .filter(newsletter_subscribers::Column::Email.eq(&email))
        .one(&ctx.db)
        .await?;

    if let Some(sub) = existing {
        let mut active: newsletter_subscribers::ActiveModel = sub.into();
        active.is_active = Set(true);
        active.update(&ctx.db).await?;
    } else {
        let new_sub = newsletter_subscribers::ActiveModel {
            email: Set(email.clone()),
            is_active: Set(true),
            ..Default::default()
        };
        new_sub.insert(&ctx.db).await?;
    }

    format::json(data!({
        "success": true,
        "email": email,
        "message": "Enrolled in daily morning dispatch",
    }))
}

/// GET /api/wire/latest — Live HTMX wire ticker polling fragment.
#[debug_handler]
pub async fn wire_ticker_fragment(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let wire_dispatches = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .order_by_desc(wire_dispatches::Column::Id)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "partials/_ticker_fragment.html",
        data!({ "wire_dispatches": wire_dispatches }),
    )
}

/// POST /api/dispatches/submit — Inline HTMX dispatch submission.
#[debug_handler]
pub async fn submit_dispatch_fragment(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Form(form): Form<CreateDispatchForm>,
) -> Result<Response> {
    if form.author_name.trim().is_empty() || form.body.trim().is_empty() {
        return bad_request("Author name and body cannot be empty");
    }

    let is_tech = form
        .is_technical_note
        .as_deref()
        .map(|v| v == "on" || v == "true" || v == "1")
        .unwrap_or(false);

    let item = reader_dispatches::ActiveModel {
        author_name: Set(form.author_name.trim().to_string()),
        affiliation: Set(form
            .affiliation
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().to_string())),
        body: Set(form.body.trim().to_string()),
        article_id: Set(form.article_id),
        is_editors_pick: Set(false),
        is_technical_note: Set(is_tech),
        endorsements_count: Set(0),
        ..Default::default()
    };

    let saved = item.insert(&ctx.db).await?;

    format::render().view(
        &v,
        "partials/_dispatch_card.html",
        data!({
            "author_name": saved.author_name,
            "affiliation": saved.affiliation,
            "body": saved.body,
            "is_technical_note": saved.is_technical_note,
            "endorsements_count": saved.endorsements_count,
        }),
    )
}

/// POST /api/subscribe/submit — Inline HTMX newsletter subscription.
#[debug_handler]
pub async fn subscribe_fragment(
    State(ctx): State<AppContext>,
    Form(form): Form<SubscribeForm>,
) -> Result<Response> {
    let email = form.email.trim().to_lowercase();
    if !email.contains('@') || !email.contains('.') {
        return format::html(
            r#"<div class="border border-error bg-surface-container-high text-error px-3 py-1.5 text-xs font-mono">⚠ Invalid email address provided.</div>"#,
        );
    }

    let existing = newsletter_subscribers::Entity::find()
        .filter(newsletter_subscribers::Column::Email.eq(&email))
        .one(&ctx.db)
        .await?;

    if let Some(sub) = existing {
        let mut active: newsletter_subscribers::ActiveModel = sub.into();
        active.is_active = Set(true);
        active.update(&ctx.db).await?;
    } else {
        let new_sub = newsletter_subscribers::ActiveModel {
            email: Set(email.clone()),
            is_active: Set(true),
            ..Default::default()
        };
        new_sub.insert(&ctx.db).await?;
    }

    format::html(
        r#"<div class="border border-primary bg-surface-container-high text-primary px-3 py-1.5 text-xs font-mono font-bold">✓ Enrolled successfully. Dispatch scheduled for 05:00 GMT.</div>"#,
    )
}

/// GET /api/search/fragment — Instant search results fragment.
#[debug_handler]
pub async fn search_fragment(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Query(params): Query<SearchQuery>,
) -> Result<Response> {
    let query_str = params.q.unwrap_or_default().trim().to_string();

    let articles = if query_str.is_empty() {
        articles::Entity::find()
            .filter(articles::Column::Status.eq("published"))
            .order_by_desc(articles::Column::PublishedAt)
            .all(&ctx.db)
            .await?
    } else {
        let cond = Condition::any()
            .add(articles::Column::Title.contains(&query_str))
            .add(articles::Column::Subhead.contains(&query_str))
            .add(articles::Column::LeadParagraph.contains(&query_str));

        articles::Entity::find()
            .filter(articles::Column::Status.eq("published"))
            .filter(cond)
            .order_by_desc(articles::Column::PublishedAt)
            .all(&ctx.db)
            .await?
    };

    format::render().view(
        &v,
        "partials/_search_results.html",
        data!({
            "articles": articles,
            "query": query_str,
        }),
    )
}

pub fn routes() -> Routes {
    Routes::new()
        .prefix("api")
        .add("/ticker/wires", get(list_wire_ticker))
        .add("/ticker/markets", get(list_market_ticker))
        .add("/dispatches", post(submit_dispatch))
        .add("/subscribe", post(subscribe))
        .add("/wire/latest", get(wire_ticker_fragment))
        .add("/dispatches/submit", post(submit_dispatch_fragment))
        .add("/subscribe/submit", post(subscribe_fragment))
        .add("/search/fragment", get(search_fragment))
}
