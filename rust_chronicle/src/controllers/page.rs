use crate::models::{
    _entities::{articles, categories, market_ledgers, reader_dispatches, wire_dispatches},
    market_ledgers as market_ledgers_model, meteorology,
};
use axum::{
    http::{header, HeaderMap},
    response::IntoResponse,
};
use loco_rs::prelude::*;
use sea_orm::{
    ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchParams {
    pub q: Option<String>,
}

async fn get_folio_context(db: &DatabaseConnection) -> (String, String, String) {
    let current_date = chrono::Local::now().format("%A, %B %-d, %Y").to_string();
    let meteorology = meteorology::get_meteorology().await;
    let commodities_summary = market_ledgers_model::get_commodities_summary(db).await;
    (current_date, meteorology, commodities_summary)
}

/// Serves the modern monochrome broadsheet home page at `/`.
#[debug_handler]
async fn index(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let lead_story = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("lead"))
        .one(&ctx.db)
        .await?
        .or_else(|| None);

    let bulletins = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("bulletin"))
        .order_by_desc(articles::Column::Id)
        .limit(5)
        .all(&ctx.db)
        .await?;

    let oped_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("oped"))
        .order_by_desc(articles::Column::Id)
        .limit(3)
        .all(&ctx.db)
        .await?;

    let wires = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .order_by_desc(wire_dispatches::Column::Id)
        .limit(8)
        .all(&ctx.db)
        .await?;

    let markets = market_ledgers::Entity::find()
        .order_by_asc(market_ledgers::Column::Id)
        .all(&ctx.db)
        .await?;

    let dispatches = reader_dispatches::Entity::find()
        .filter(reader_dispatches::Column::IsEditorsPick.eq(true))
        .order_by_desc(reader_dispatches::Column::EndorsementsCount)
        .limit(4)
        .all(&ctx.db)
        .await?;

    let (current_date, meteorology, commodities_summary) = get_folio_context(&ctx.db).await;

    format::render().view(
        &v,
        "home/index.html",
        data!({
            "categories": cats,
            "lead_story": lead_story,
            "bulletins": bulletins,
            "oped_articles": oped_articles,
            "wire_dispatches": wires,
            "market_ledger": markets,
            "reader_dispatches": dispatches,
            "current_date": current_date,
            "meteorology": meteorology,
            "commodities_summary": commodities_summary,
        }),
    )
}

/// Serves the broadsheet editorial article reader page by dynamic slug at `/articles/:slug`.
#[debug_handler]
async fn article_by_slug(
    State(ctx): State<AppContext>,
    Path(slug): Path<String>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let article = articles::Entity::find()
        .filter(articles::Column::Slug.eq(&slug))
        .filter(articles::Column::Status.eq("published"))
        .one(&ctx.db)
        .await?;

    let article = match article {
        Some(a) => a,
        None => return not_found(),
    };

    let category = match article.category_id {
        Some(cid) => categories::Entity::find_by_id(cid).one(&ctx.db).await?,
        None => None,
    };

    let dispatches = reader_dispatches::Entity::find()
        .filter(reader_dispatches::Column::ArticleId.eq(article.id))
        .all(&ctx.db)
        .await?;

    let sidebar_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::Id.ne(article.id))
        .order_by_desc(articles::Column::Id)
        .limit(3)
        .all(&ctx.db)
        .await?;

    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let wires = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .order_by_desc(wire_dispatches::Column::Id)
        .limit(5)
        .all(&ctx.db)
        .await?;

    let (current_date, meteorology, commodities_summary) = get_folio_context(&ctx.db).await;

    format::render().view(
        &v,
        "articles/reader.html",
        data!({
            "article": article,
            "category": category,
            "dispatches": dispatches,
            "sidebar_articles": sidebar_articles,
            "categories": cats,
            "wire_dispatches": wires,
            "current_date": current_date,
            "meteorology": meteorology,
            "commodities_summary": commodities_summary,
        }),
    )
}

/// Serves the default / legacy broadsheet article reader page at `/article`.
#[debug_handler]
async fn article_reader(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let lead_story = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("lead"))
        .one(&ctx.db)
        .await?;

    let article = match lead_story {
        Some(a) => a,
        None => {
            let first = articles::Entity::find()
                .filter(articles::Column::Status.eq("published"))
                .one(&ctx.db)
                .await?;
            match first {
                Some(a) => a,
                None => return not_found(),
            }
        }
    };

    let category = match article.category_id {
        Some(cid) => categories::Entity::find_by_id(cid).one(&ctx.db).await?,
        None => None,
    };

    let dispatches = reader_dispatches::Entity::find()
        .filter(reader_dispatches::Column::ArticleId.eq(article.id))
        .all(&ctx.db)
        .await?;

    let sidebar_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::Id.ne(article.id))
        .order_by_desc(articles::Column::Id)
        .limit(3)
        .all(&ctx.db)
        .await?;

    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let wires = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .order_by_desc(wire_dispatches::Column::Id)
        .limit(5)
        .all(&ctx.db)
        .await?;

    let (current_date, meteorology, commodities_summary) = get_folio_context(&ctx.db).await;

    format::render().view(
        &v,
        "articles/reader.html",
        data!({
            "article": article,
            "category": category,
            "dispatches": dispatches,
            "sidebar_articles": sidebar_articles,
            "categories": cats,
            "wire_dispatches": wires,
            "current_date": current_date,
            "meteorology": meteorology,
            "commodities_summary": commodities_summary,
        }),
    )
}

/// Serves category articles by category slug at `/category/:slug`.
#[debug_handler]
async fn category_by_slug(
    State(ctx): State<AppContext>,
    Path(slug): Path<String>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let cat = categories::Entity::find()
        .filter(categories::Column::Slug.eq(&slug))
        .one(&ctx.db)
        .await?;

    let category = match cat {
        Some(c) => c,
        None => return not_found(),
    };

    let cat_articles = articles::Entity::find()
        .filter(articles::Column::CategoryId.eq(category.id))
        .filter(articles::Column::Status.eq("published"))
        .order_by_desc(articles::Column::Id)
        .all(&ctx.db)
        .await?;

    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let wires = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .order_by_desc(wire_dispatches::Column::Id)
        .limit(5)
        .all(&ctx.db)
        .await?;

    let (current_date, meteorology, commodities_summary) = get_folio_context(&ctx.db).await;

    format::render().view(
        &v,
        "categories/show.html",
        data!({
            "category": category,
            "current_category": slug,
            "articles": cat_articles,
            "categories": cats,
            "wire_dispatches": wires,
            "current_date": current_date,
            "meteorology": meteorology,
            "commodities_summary": commodities_summary,
        }),
    )
}

/// Serves historical archive search query at `/search?q=...`.
#[debug_handler]
async fn search(
    State(ctx): State<AppContext>,
    Query(params): Query<SearchParams>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let query_str = params.q.unwrap_or_default().trim().to_string();

    let matched_articles = if query_str.is_empty() {
        Vec::new()
    } else {
        let cond = Condition::any()
            .add(articles::Column::Title.contains(&query_str))
            .add(articles::Column::Subhead.contains(&query_str))
            .add(articles::Column::LeadParagraph.contains(&query_str));

        articles::Entity::find()
            .filter(articles::Column::Status.eq("published"))
            .filter(cond)
            .order_by_desc(articles::Column::Id)
            .all(&ctx.db)
            .await?
    };

    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let wires = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .order_by_desc(wire_dispatches::Column::Id)
        .limit(5)
        .all(&ctx.db)
        .await?;

    let (current_date, meteorology, commodities_summary) = get_folio_context(&ctx.db).await;

    format::render().view(
        &v,
        "articles/search.html",
        data!({
            "query": query_str,
            "articles": matched_articles,
            "results_count": matched_articles.len(),
            "categories": cats,
            "wire_dispatches": wires,
            "current_date": current_date,
            "meteorology": meteorology,
            "commodities_summary": commodities_summary,
        }),
    )
}

#[debug_handler]
pub async fn partial_market_ledger(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let markets = market_ledgers::Entity::find()
        .order_by_asc(market_ledgers::Column::Id)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "partials/_market_ledger.html",
        data!({
            "market_ledger": markets,
        }),
    )
}

#[debug_handler]
pub async fn partial_folio_strip(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let (current_date, meteorology, commodities_summary) = get_folio_context(&ctx.db).await;

    format::render().view(
        &v,
        "partials/_folio_strip.html",
        data!({
            "current_date": current_date,
            "meteorology": meteorology,
            "commodities_summary": commodities_summary,
        }),
    )
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[debug_handler]
pub async fn rss_feed(
    headers: HeaderMap,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:5150");
    let scheme = if headers
        .get("x-forwarded-proto")
        .and_then(|p| p.to_str().ok())
        == Some("https")
    {
        "https"
    } else {
        "http"
    };
    let base_url = format!("{}://{}", scheme, host);

    let published_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .order_by_desc(articles::Column::PublishedAt)
        .order_by_desc(articles::Column::Id)
        .limit(30)
        .all(&ctx.db)
        .await?;

    let now_rfc2822 = chrono::Utc::now().to_rfc2822();

    let mut xml = String::with_capacity(8192);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<rss version=\"2.0\" xmlns:atom=\"http://www.w3.org/2005/Atom\">\n");
    xml.push_str("  <channel>\n");
    xml.push_str("    <title>The Rust Chronicle — Journal of Record &amp; Modern Inquiry</title>\n");
    xml.push_str(&format!("    <link>{}/</link>\n", base_url));
    xml.push_str("    <description>Authentic broadsheet newsroom, in-depth investigatory journalism, telegraph wires, and live market commodity ledger.</description>\n");
    xml.push_str("    <language>en-us</language>\n");
    xml.push_str(&format!("    <lastBuildDate>{}</lastBuildDate>\n", now_rfc2822));
    xml.push_str(&format!(
        "    <atom:link href=\"{}/rss.xml\" rel=\"self\" type=\"application/rss+xml\" />\n",
        base_url
    ));

    for article in published_articles {
        let article_url = format!("{}/articles/{}", base_url, article.slug);
        let pub_date = article
            .published_at
            .map(|dt| dt.to_rfc2822())
            .unwrap_or_else(|| article.created_at.to_rfc2822());
        let desc = article
            .lead_paragraph
            .as_deref()
            .or(article.subhead.as_deref())
            .unwrap_or("");

        xml.push_str("    <item>\n");
        xml.push_str(&format!("      <title><![CDATA[{}]]></title>\n", article.title));
        xml.push_str(&format!("      <link>{}</link>\n", article_url));
        xml.push_str(&format!(
            "      <guid isPermaLink=\"true\">{}</guid>\n",
            article_url
        ));
        xml.push_str(&format!("      <pubDate>{}</pubDate>\n", pub_date));
        xml.push_str(&format!(
            "      <description><![CDATA[{}]]></description>\n",
            desc
        ));
        xml.push_str("    </item>\n");
    }

    xml.push_str("  </channel>\n");
    xml.push_str("</rss>\n");

    Ok((
        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
        xml,
    )
        .into_response())
}

#[debug_handler]
pub async fn sitemap(
    headers: HeaderMap,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let host = headers
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("localhost:5150");
    let scheme = if headers
        .get("x-forwarded-proto")
        .and_then(|p| p.to_str().ok())
        == Some("https")
    {
        "https"
    } else {
        "http"
    };
    let base_url = format!("{}://{}", scheme, host);

    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let published_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .order_by_desc(articles::Column::Id)
        .all(&ctx.db)
        .await?;

    let mut xml = String::with_capacity(8192);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");

    // Static primary routes
    xml.push_str(&format!(
        "  <url>\n    <loc>{}/</loc>\n    <changefreq>hourly</changefreq>\n    <priority>1.0</priority>\n  </url>\n",
        base_url
    ));
    xml.push_str(&format!(
        "  <url>\n    <loc>{}/search</loc>\n    <changefreq>daily</changefreq>\n    <priority>0.7</priority>\n  </url>\n",
        base_url
    ));

    // Category routes
    for cat in cats {
        xml.push_str(&format!(
            "  <url>\n    <loc>{}/category/{}</loc>\n    <changefreq>daily</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            base_url,
            xml_escape(&cat.slug)
        ));
    }

    // Article routes
    for article in published_articles {
        let lastmod = article
            .published_at
            .as_ref()
            .unwrap_or(&article.updated_at)
            .format("%Y-%m-%d")
            .to_string();

        xml.push_str(&format!(
            "  <url>\n    <loc>{}/articles/{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            base_url,
            xml_escape(&article.slug),
            lastmod
        ));
    }

    xml.push_str("</urlset>\n");

    Ok((
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
        .into_response())
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(index))
        .add("/article", get(article_reader))
        .add("/articles/{slug}", get(article_by_slug))
        .add("/category/{slug}", get(category_by_slug))
        .add("/search", get(search))
        .add("/partials/market_ledger", get(partial_market_ledger))
        .add("/partials/folio", get(partial_folio_strip))
        .add("/rss.xml", get(rss_feed))
        .add("/feed.xml", get(rss_feed))
        .add("/sitemap.xml", get(sitemap))
}
