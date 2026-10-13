use crate::models::_entities::{
    articles, categories, market_ledgers, reader_dispatches, wire_dispatches,
};
use loco_rs::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchParams {
    pub q: Option<String>,
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

    format::render().view(
        &v,
        "categories/show.html",
        data!({
            "category": category,
            "current_category": slug,
            "articles": cat_articles,
            "categories": cats,
            "wire_dispatches": wires,
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

    format::render().view(
        &v,
        "articles/search.html",
        data!({
            "query": query_str,
            "articles": matched_articles,
            "results_count": matched_articles.len(),
            "categories": cats,
            "wire_dispatches": wires,
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

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(index))
        .add("/article", get(article_reader))
        .add("/articles/{slug}", get(article_by_slug))
        .add("/category/{slug}", get(category_by_slug))
        .add("/search", get(search))
        .add("/partials/market_ledger", get(partial_market_ledger))
}
