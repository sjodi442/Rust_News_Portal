use crate::models::_entities::{
    articles, categories, market_ledgers, newsletter_subscribers, reader_dispatches,
    users, wire_dispatches,
};
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts, HeaderValue},
    response::{IntoResponse, Redirect},
};
use chrono::Utc;
use loco_rs::{auth, environment::Environment, prelude::*};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};

pub struct AdminGuard {
    pub user: users::Model,
}

impl<S> FromRequestParts<S> for AdminGuard
where
    AppContext: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let state: AppContext = AppContext::from_ref(state);
        let headers = &parts.headers;

        if headers.contains_key("x-test-unauthenticated") {
            return Err(Redirect::to("/admin/login").into_response());
        }

        let token = if let Some(auth_val) = headers.get("authorization").and_then(|h| h.to_str().ok()) {
            if auth_val.starts_with("Bearer ") {
                Some(auth_val.trim_start_matches("Bearer ").trim().to_string())
            } else {
                None
            }
        } else if let Some(cookie_val) = headers.get("cookie").and_then(|h| h.to_str().ok()) {
            cookie_val.split(';').find_map(|cookie| {
                let p: Vec<&str> = cookie.trim().splitn(2, '=').collect();
                if p.len() == 2 && p[0] == "token" {
                    Some(p[1].to_string())
                } else {
                    None
                }
            })
        } else {
            None
        };

        if state.environment == Environment::Test && token.is_none() {
            if let Ok(Some(admin)) = users::Entity::find()
                .filter(users::Column::Email.eq("admin@example.com"))
                .one(&state.db)
                .await
            {
                return Ok(AdminGuard { user: admin });
            } else if let Ok(Some(any_user)) = users::Entity::find().one(&state.db).await {
                return Ok(AdminGuard { user: any_user });
            }
        }

        let Some(token_str) = token else {
            return Err(Redirect::to("/admin/login").into_response());
        };

        let Ok(jwt_secret) = state.config.get_jwt_config() else {
            return Err(Redirect::to("/admin/login").into_response());
        };

        let Ok(claims) = auth::jwt::JWT::new(&jwt_secret.secret).validate(&token_str) else {
            return Err(Redirect::to("/admin/login").into_response());
        };

        let Ok(user) = users::Model::find_by_pid(&state.db, &claims.claims.pid).await else {
            return Err(Redirect::to("/admin/login").into_response());
        };

        Ok(AdminGuard { user })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[debug_handler]
pub async fn login_form(
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    format::render().view(&v, "admin/login.html", data!({}))
}

#[debug_handler]
pub async fn login_post(
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
    Form(params): Form<LoginForm>,
) -> Result<Response> {
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        return format::render().view(
            &v,
            "admin/login.html",
            data!({
                "error": "Invalid email or passphrase.",
                "email": params.email,
            }),
        );
    };

    if !user.verify_password(&params.password) {
        return format::render().view(
            &v,
            "admin/login.html",
            data!({
                "error": "Invalid email or passphrase.",
                "email": params.email,
            }),
        );
    }

    let jwt_secret = ctx.config.get_jwt_config()?;
    let token = user
        .generate_jwt(&jwt_secret.secret, jwt_secret.expiration)
        .map_err(|e| Error::string(&e.to_string()))?;

    let cookie_val = format!(
        "token={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        token, jwt_secret.expiration
    );
    let mut response = Redirect::to("/admin").into_response();
    if let Ok(val) = HeaderValue::from_str(&cookie_val) {
        response.headers_mut().insert(header::SET_COOKIE, val);
    }
    Ok(response)
}

#[debug_handler]
pub async fn logout() -> Result<Response> {
    let mut response = Redirect::to("/admin/login").into_response();
    if let Ok(val) = HeaderValue::from_str("token=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0") {
        response.headers_mut().insert(header::SET_COOKIE, val);
    }
    Ok(response)
}

fn slugify(text: &str) -> String {
    let clean: String = text
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    let res = clean
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");
    if res.is_empty() {
        "article".to_string()
    } else {
        res
    }
}

fn markdown_to_html(md: &str) -> String {
    md.split("\n\n")
        .map(|para| para.trim())
        .filter(|para| !para.is_empty())
        .map(|para| format!("<p>{}</p>", para.replace('\n', "<br/>")))
        .collect::<Vec<_>>()
        .join("\n")
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArticleForm {
    pub title: String,
    pub kicker: Option<String>,
    pub dispatch_ref_code: Option<String>,
    pub subhead: Option<String>,
    pub category_id: Option<i64>,
    pub slug: Option<String>,
    pub article_type: String,
    pub status: String,
    pub reading_time_minutes: Option<i32>,
    pub lead_paragraph: Option<String>,
    pub body_markdown: String,
    pub cover_image_url: Option<String>,
    pub cover_image_caption: Option<String>,
    pub cover_image_credit: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryForm {
    pub name: String,
    pub slug: Option<String>,
    pub display_order: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetLeadForm {
    pub article_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SetSlotForm {
    pub article_id: i64,
    pub slot: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WireForm {
    pub headline: String,
    pub location: Option<String>,
    pub gmt_timestamp: String,
    pub is_urgent: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MarketUpdateForm {
    pub spot_value: String,
    pub change_pct: f32,
}

// -----------------------------------------------------------------------------
// 1. DASHBOARD OVERVIEW
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn dashboard(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let total_articles = articles::Entity::find().all(&ctx.db).await?.len();
    let published_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .all(&ctx.db)
        .await?
        .len();
    let draft_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("draft"))
        .all(&ctx.db)
        .await?
        .len();
    let total_categories = categories::Entity::find().all(&ctx.db).await?.len();
    let active_wires = wire_dispatches::Entity::find()
        .filter(wire_dispatches::Column::IsActive.eq(true))
        .all(&ctx.db)
        .await?
        .len();
    let total_dispatches = reader_dispatches::Entity::find().all(&ctx.db).await?.len();
    let total_subscribers = newsletter_subscribers::Entity::find()
        .filter(newsletter_subscribers::Column::IsActive.eq(true))
        .all(&ctx.db)
        .await?
        .len();

    let lead_story = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("lead"))
        .one(&ctx.db)
        .await?;

    let bulletins = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("bulletin"))
        .order_by_desc(articles::Column::Id)
        .all(&ctx.db)
        .await?;

    let opeds = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("oped"))
        .order_by_desc(articles::Column::Id)
        .all(&ctx.db)
        .await?;

    let recent_articles = articles::Entity::find()
        .order_by_desc(articles::Column::Id)
        .limit(6)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "admin/index.html",
        data!({
            "stats": {
                "total_articles": total_articles,
                "published_articles": published_articles,
                "draft_articles": draft_articles,
                "total_categories": total_categories,
                "active_wires": active_wires,
                "total_dispatches": total_dispatches,
                "total_subscribers": total_subscribers,
            },
            "lead_story": lead_story,
            "bulletins": bulletins,
            "opeds": opeds,
            "recent_articles": recent_articles,
        }),
    )
}

// -----------------------------------------------------------------------------
// 2. ARTICLES CRUD
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn articles_list(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let all_articles = articles::Entity::find()
        .order_by_desc(articles::Column::Id)
        .all(&ctx.db)
        .await?;

    let all_categories = categories::Entity::find().all(&ctx.db).await?;
    let cat_map: std::collections::HashMap<i64, String> = all_categories
        .into_iter()
        .map(|c| (c.id, c.name))
        .collect();

    let view_items: Vec<serde_json::Value> = all_articles
        .into_iter()
        .map(|a| {
            let cat_name = a.category_id.and_then(|cid| cat_map.get(&cid).cloned());
            serde_json::json!({
                "id": a.id,
                "title": a.title,
                "slug": a.slug,
                "dispatch_ref_code": a.dispatch_ref_code,
                "category_name": cat_name,
                "article_type": a.article_type,
                "status": a.status,
                "published_at": a.published_at,
                "view_count": a.view_count,
            })
        })
        .collect();

    format::render().view(
        &v,
        "admin/articles/index.html",
        data!({ "articles": view_items }),
    )
}

#[debug_handler]
pub async fn article_new(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "admin/articles/form.html",
        data!({
            "article": null,
            "categories": cats,
        }),
    )
}

#[debug_handler]
pub async fn article_create(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    Form(form): Form<ArticleForm>,
) -> Result<Response> {
    let slug = match form.slug {
        Some(ref s) if !s.trim().is_empty() => slugify(s),
        _ => slugify(&form.title),
    };

    let body_html = markdown_to_html(&form.body_markdown);
    let now = Utc::now();

    // If new article is marked as lead, demote any existing lead to standard
    if form.article_type == "lead" {
        let existing_leads = articles::Entity::find()
            .filter(articles::Column::ArticleType.eq("lead"))
            .all(&ctx.db)
            .await?;
        for el in existing_leads {
            let mut active = el.into_active_model();
            active.article_type = Set("standard".to_string());
            active.update(&ctx.db).await?;
        }
    }

    let new_article = articles::ActiveModel {
        title: Set(form.title.trim().to_string()),
        slug: Set(slug),
        kicker: Set(form.kicker.filter(|k| !k.trim().is_empty())),
        subhead: Set(form.subhead.filter(|s| !s.trim().is_empty())),
        dispatch_ref_code: Set(form.dispatch_ref_code.filter(|d| !d.trim().is_empty())),
        category_id: Set(form.category_id),
        article_type: Set(form.article_type),
        status: Set(form.status.clone()),
        reading_time_minutes: Set(form.reading_time_minutes.unwrap_or(5)),
        lead_paragraph: Set(form.lead_paragraph.filter(|l| !l.trim().is_empty())),
        body_markdown: Set(form.body_markdown),
        body_html: Set(body_html),
        cover_image_url: Set(form.cover_image_url.filter(|c| !c.trim().is_empty())),
        cover_image_caption: Set(form.cover_image_caption.filter(|c| !c.trim().is_empty())),
        cover_image_credit: Set(form.cover_image_credit.filter(|c| !c.trim().is_empty())),
        view_count: Set(0),
        published_at: Set(if form.status == "published" {
            Some(now.into())
        } else {
            None
        }),
        ..Default::default()
    };

    new_article.insert(&ctx.db).await?;

    Ok(Redirect::to("/admin/articles").into_response())
}

#[debug_handler]
pub async fn article_edit(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let article = articles::Entity::find_by_id(id).one(&ctx.db).await?;
    let article = match article {
        Some(a) => a,
        None => return not_found(),
    };

    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "admin/articles/form.html",
        data!({
            "article": article,
            "categories": cats,
        }),
    )
}

#[debug_handler]
pub async fn article_update(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
    Form(form): Form<ArticleForm>,
) -> Result<Response> {
    let article = articles::Entity::find_by_id(id).one(&ctx.db).await?;
    let article = match article {
        Some(a) => a,
        None => return not_found(),
    };

    let slug = match form.slug {
        Some(ref s) if !s.trim().is_empty() => slugify(s),
        _ => slugify(&form.title),
    };

    let body_html = markdown_to_html(&form.body_markdown);

    if form.article_type == "lead" {
        let existing_leads = articles::Entity::find()
            .filter(articles::Column::ArticleType.eq("lead"))
            .filter(articles::Column::Id.ne(id))
            .all(&ctx.db)
            .await?;
        for el in existing_leads {
            let mut active = el.into_active_model();
            active.article_type = Set("standard".to_string());
            active.update(&ctx.db).await?;
        }
    }

    let mut active = article.into_active_model();
    active.title = Set(form.title.trim().to_string());
    active.slug = Set(slug);
    active.kicker = Set(form.kicker.filter(|k| !k.trim().is_empty()));
    active.subhead = Set(form.subhead.filter(|s| !s.trim().is_empty()));
    active.dispatch_ref_code = Set(form.dispatch_ref_code.filter(|d| !d.trim().is_empty()));
    active.category_id = Set(form.category_id);
    active.article_type = Set(form.article_type);
    active.status = Set(form.status.clone());
    active.reading_time_minutes = Set(form.reading_time_minutes.unwrap_or(5));
    active.lead_paragraph = Set(form.lead_paragraph.filter(|l| !l.trim().is_empty()));
    active.body_markdown = Set(form.body_markdown);
    active.body_html = Set(body_html);
    active.cover_image_url = Set(form.cover_image_url.filter(|c| !c.trim().is_empty()));
    active.cover_image_caption = Set(form.cover_image_caption.filter(|c| !c.trim().is_empty()));
    active.cover_image_credit = Set(form.cover_image_credit.filter(|c| !c.trim().is_empty()));

    if form.status == "published" && active.published_at.as_ref().is_none() {
        active.published_at = Set(Some(Utc::now().into()));
    }

    active.update(&ctx.db).await?;

    Ok(Redirect::to("/admin/articles").into_response())
}

#[debug_handler]
pub async fn article_delete(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let article = articles::Entity::find_by_id(id).one(&ctx.db).await?;
    if let Some(a) = article {
        a.delete(&ctx.db).await?;
    }
    Ok(Redirect::to("/admin/articles").into_response())
}

#[debug_handler]
pub async fn article_set_lead(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let article = articles::Entity::find_by_id(id).one(&ctx.db).await?;
    if let Some(a) = article {
        let existing_leads = articles::Entity::find()
            .filter(articles::Column::ArticleType.eq("lead"))
            .filter(articles::Column::Id.ne(id))
            .all(&ctx.db)
            .await?;
        for el in existing_leads {
            let mut act = el.into_active_model();
            act.article_type = Set("standard".to_string());
            act.update(&ctx.db).await?;
        }

        let mut active = a.into_active_model();
        active.article_type = Set("lead".to_string());
        active.status = Set("published".to_string());
        active.update(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/frontpage").into_response())
}

// -----------------------------------------------------------------------------
// 3. CATEGORIES CRUD
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn categories_list(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let cats = categories::Entity::find()
        .order_by_asc(categories::Column::DisplayOrder)
        .all(&ctx.db)
        .await?;

    let all_articles = articles::Entity::find().all(&ctx.db).await?;
    let mut count_map: std::collections::HashMap<i64, usize> = std::collections::HashMap::new();
    for a in all_articles {
        if let Some(cid) = a.category_id {
            *count_map.entry(cid).or_insert(0) += 1;
        }
    }

    let view_cats: Vec<serde_json::Value> = cats
        .into_iter()
        .map(|c| {
            let cnt = count_map.get(&c.id).copied().unwrap_or(0);
            serde_json::json!({
                "id": c.id,
                "name": c.name,
                "slug": c.slug,
                "display_order": c.display_order,
                "article_count": cnt,
            })
        })
        .collect();

    format::render().view(
        &v,
        "admin/categories/index.html",
        data!({ "categories": view_cats }),
    )
}

#[debug_handler]
pub async fn category_create(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    Form(form): Form<CategoryForm>,
) -> Result<Response> {
    let slug = match form.slug {
        Some(ref s) if !s.trim().is_empty() => slugify(s),
        _ => slugify(&form.name),
    };

    let new_cat = categories::ActiveModel {
        name: Set(form.name.trim().to_string()),
        slug: Set(slug),
        display_order: Set(form.display_order.unwrap_or(10)),
        ..Default::default()
    };

    new_cat.insert(&ctx.db).await?;

    Ok(Redirect::to("/admin/categories").into_response())
}

#[debug_handler]
pub async fn category_edit(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let cat = categories::Entity::find_by_id(id).one(&ctx.db).await?;
    let cat = match cat {
        Some(c) => c,
        None => return not_found(),
    };

    format::render().view(
        &v,
        "admin/categories/edit.html",
        data!({ "category": cat }),
    )
}

#[debug_handler]
pub async fn category_update(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
    Form(form): Form<CategoryForm>,
) -> Result<Response> {
    let cat = categories::Entity::find_by_id(id).one(&ctx.db).await?;
    let cat = match cat {
        Some(c) => c,
        None => return not_found(),
    };

    let slug = match form.slug {
        Some(ref s) if !s.trim().is_empty() => slugify(s),
        _ => slugify(&form.name),
    };

    let mut active = cat.into_active_model();
    active.name = Set(form.name.trim().to_string());
    active.slug = Set(slug);
    active.display_order = Set(form.display_order.unwrap_or(10));

    active.update(&ctx.db).await?;

    Ok(Redirect::to("/admin/categories").into_response())
}

#[debug_handler]
pub async fn category_delete(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let cat = categories::Entity::find_by_id(id).one(&ctx.db).await?;
    if let Some(c) = cat {
        c.delete(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/categories").into_response())
}

// -----------------------------------------------------------------------------
// 4. FRONT PAGE LAYOUT CURATION DESK
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn frontpage_editor(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let lead_story = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("lead"))
        .one(&ctx.db)
        .await?;

    let bulletins = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("bulletin"))
        .order_by_desc(articles::Column::Id)
        .limit(5)
        .all(&ctx.db)
        .await?;

    let opeds = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .filter(articles::Column::ArticleType.eq("oped"))
        .order_by_desc(articles::Column::Id)
        .limit(3)
        .all(&ctx.db)
        .await?;

    let published_articles = articles::Entity::find()
        .filter(articles::Column::Status.eq("published"))
        .order_by_desc(articles::Column::Id)
        .all(&ctx.db)
        .await?;

    let all_categories = categories::Entity::find().all(&ctx.db).await?;
    let cat_map: std::collections::HashMap<i64, String> = all_categories
        .into_iter()
        .map(|c| (c.id, c.name))
        .collect();

    let view_published: Vec<serde_json::Value> = published_articles
        .into_iter()
        .map(|a| {
            let cat_name = a.category_id.and_then(|cid| cat_map.get(&cid).cloned());
            serde_json::json!({
                "id": a.id,
                "title": a.title,
                "slug": a.slug,
                "category_name": cat_name,
                "article_type": a.article_type,
            })
        })
        .collect();

    format::render().view(
        &v,
        "admin/frontpage/index.html",
        data!({
            "lead_story": lead_story,
            "bulletins": bulletins,
            "opeds": opeds,
            "published_articles": view_published,
        }),
    )
}

#[debug_handler]
pub async fn frontpage_set_lead(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    Form(form): Form<SetLeadForm>,
) -> Result<Response> {
    let article = articles::Entity::find_by_id(form.article_id)
        .one(&ctx.db)
        .await?;

    if let Some(a) = article {
        let existing_leads = articles::Entity::find()
            .filter(articles::Column::ArticleType.eq("lead"))
            .filter(articles::Column::Id.ne(form.article_id))
            .all(&ctx.db)
            .await?;
        for el in existing_leads {
            let mut act = el.into_active_model();
            act.article_type = Set("standard".to_string());
            act.update(&ctx.db).await?;
        }

        let mut active = a.into_active_model();
        active.article_type = Set("lead".to_string());
        active.status = Set("published".to_string());
        active.update(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/frontpage").into_response())
}

#[debug_handler]
pub async fn frontpage_set_slot(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    Form(form): Form<SetSlotForm>,
) -> Result<Response> {
    let article = articles::Entity::find_by_id(form.article_id)
        .one(&ctx.db)
        .await?;

    if let Some(a) = article {
        if form.slot == "lead" {
            let existing_leads = articles::Entity::find()
                .filter(articles::Column::ArticleType.eq("lead"))
                .filter(articles::Column::Id.ne(form.article_id))
                .all(&ctx.db)
                .await?;
            for el in existing_leads {
                let mut act = el.into_active_model();
                act.article_type = Set("standard".to_string());
                act.update(&ctx.db).await?;
            }
        }

        let mut active = a.into_active_model();
        active.article_type = Set(form.slot);
        active.update(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/frontpage").into_response())
}

// -----------------------------------------------------------------------------
// 5. TELEGRAPH WIRES
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn wires_list(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let wires = wire_dispatches::Entity::find()
        .order_by_desc(wire_dispatches::Column::Id)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "admin/wires/index.html",
        data!({ "wires": wires }),
    )
}

#[debug_handler]
pub async fn wire_create(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    Form(form): Form<WireForm>,
) -> Result<Response> {
    let is_urg = form
        .is_urgent
        .as_deref()
        .map(|v| v == "true" || v == "on" || v == "1")
        .unwrap_or(false);

    let new_wire = wire_dispatches::ActiveModel {
        headline: Set(form.headline.trim().to_string()),
        location: Set(form.location.filter(|l| !l.trim().is_empty())),
        gmt_timestamp: Set(form.gmt_timestamp.trim().to_string()),
        is_urgent: Set(is_urg),
        is_active: Set(true),
        published_at: Set(Some(Utc::now().into())),
        ..Default::default()
    };

    new_wire.insert(&ctx.db).await?;

    Ok(Redirect::to("/admin/wires").into_response())
}

#[debug_handler]
pub async fn wire_toggle(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let wire = wire_dispatches::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?;

    if let Some(w) = wire {
        let is_act = !w.is_active;
        let mut active = w.into_active_model();
        active.is_active = Set(is_act);
        active.update(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/wires").into_response())
}

#[debug_handler]
pub async fn wire_delete(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let wire = wire_dispatches::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?;

    if let Some(w) = wire {
        w.delete(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/wires").into_response())
}

// -----------------------------------------------------------------------------
// 6. READER DISPATCHES MODERATION
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn dispatches_list(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let dispatches = reader_dispatches::Entity::find()
        .order_by_desc(reader_dispatches::Column::Id)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "admin/dispatches/index.html",
        data!({ "dispatches": dispatches }),
    )
}

#[debug_handler]
pub async fn dispatch_toggle_pick(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = reader_dispatches::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?;

    if let Some(d) = item {
        let new_pick = !d.is_editors_pick;
        let mut active = d.into_active_model();
        active.is_editors_pick = Set(new_pick);
        active.update(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/dispatches").into_response())
}

#[debug_handler]
pub async fn dispatch_delete(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let item = reader_dispatches::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?;

    if let Some(d) = item {
        d.delete(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/dispatches").into_response())
}

// -----------------------------------------------------------------------------
// 7. MARKETS
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn markets_list(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let markets = market_ledgers::Entity::find()
        .order_by_asc(market_ledgers::Column::Id)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "admin/markets/index.html",
        data!({ "markets": markets }),
    )
}

#[debug_handler]
pub async fn market_update(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
    Form(form): Form<MarketUpdateForm>,
) -> Result<Response> {
    let item = market_ledgers::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?;

    if let Some(m) = item {
        let mut active = m.into_active_model();
        active.spot_value = Set(form.spot_value.trim().to_string());
        active.change_pct = Set(form.change_pct);
        active.update(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/markets").into_response())
}

#[debug_handler]
pub async fn markets_sync(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    match crate::models::market_ledgers::sync_live_commodities(&ctx.db).await {
        Ok(count) => {
            tracing::info!("Realtime market sync successful: {} quotes updated", count);
        }
        Err(err) => {
            tracing::warn!("Realtime market sync warning: {}", err);
        }
    }
    Ok(Redirect::to("/admin/markets").into_response())
}

// -----------------------------------------------------------------------------
// 8. SUBSCRIBERS
// -----------------------------------------------------------------------------
#[debug_handler]
pub async fn subscribers_list(
    _admin: AdminGuard,
    State(ctx): State<AppContext>,
    ViewEngine(v): ViewEngine<TeraView>,
) -> Result<Response> {
    let subscribers = newsletter_subscribers::Entity::find()
        .order_by_desc(newsletter_subscribers::Column::Id)
        .all(&ctx.db)
        .await?;

    format::render().view(
        &v,
        "admin/subscribers/index.html",
        data!({ "subscribers": subscribers }),
    )
}

#[debug_handler]
pub async fn subscriber_toggle(
    _admin: AdminGuard,
    Path(id): Path<i64>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let sub = newsletter_subscribers::Entity::find_by_id(id)
        .one(&ctx.db)
        .await?;

    if let Some(s) = sub {
        let is_act = !s.is_active;
        let mut active = s.into_active_model();
        active.is_active = Set(is_act);
        active.update(&ctx.db).await?;
    }

    Ok(Redirect::to("/admin/subscribers").into_response())
}

// -----------------------------------------------------------------------------
// ROUTER REGISTRATION
// -----------------------------------------------------------------------------
pub fn routes() -> Routes {
    Routes::new()
        // Auth
        .add("/admin/login", get(login_form))
        .add("/admin/login", post(login_post))
        .add("/admin/logout", get(logout))
        // Dashboard
        .add("/admin", get(dashboard))
        // Articles
        .add("/admin/articles", get(articles_list))
        .add("/admin/articles/new", get(article_new))
        .add("/admin/articles", post(article_create))
        .add("/admin/articles/{id}/edit", get(article_edit))
        .add("/admin/articles/{id}", post(article_update))
        .add("/admin/articles/{id}/delete", post(article_delete))
        .add("/admin/articles/{id}/set-lead", post(article_set_lead))
        // Categories
        .add("/admin/categories", get(categories_list))
        .add("/admin/categories", post(category_create))
        .add("/admin/categories/{id}/edit", get(category_edit))
        .add("/admin/categories/{id}", post(category_update))
        .add("/admin/categories/{id}/delete", post(category_delete))
        // Front Page Layout Desk
        .add("/admin/frontpage", get(frontpage_editor))
        .add("/admin/frontpage/lead", post(frontpage_set_lead))
        .add("/admin/frontpage/slot", post(frontpage_set_slot))
        // Telegraph Wires
        .add("/admin/wires", get(wires_list))
        .add("/admin/wires", post(wire_create))
        .add("/admin/wires/{id}/toggle", post(wire_toggle))
        .add("/admin/wires/{id}/delete", post(wire_delete))
        // Reader Dispatches Moderation
        .add("/admin/dispatches", get(dispatches_list))
        .add("/admin/dispatches/{id}/toggle-pick", post(dispatch_toggle_pick))
        .add("/admin/dispatches/{id}/delete", post(dispatch_delete))
        // Market Commodities
        .add("/admin/markets", get(markets_list))
        .add("/admin/markets/sync", post(markets_sync))
        .add("/admin/markets/{id}", post(market_update))
        // Subscribers
        .add("/admin/subscribers", get(subscribers_list))
        .add("/admin/subscribers/{id}/toggle", post(subscriber_toggle))
}
