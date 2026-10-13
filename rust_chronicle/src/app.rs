use async_trait::async_trait;
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    bgworker::{BackgroundWorker, Queue},
    boot::{create_app, BootResult, StartMode},
    config::Config,
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    task::Tasks,
    Result,
};
use migration::Migrator;
use std::path::Path;

#[allow(unused_imports)]
use crate::{
    controllers, initializers,
    models::{
        articles, categories, market_ledgers, newsletter_subscribers, reader_dispatches, users,
        wire_dispatches,
    },
    tasks, workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(
        mode: StartMode,
        environment: &Environment,
        config: Config,
    ) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment, config).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![
            Box::new(initializers::view_engine::ViewEngineInitializer),
            Box::new(initializers::market_sync::MarketSyncInitializer),
        ])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::auth::routes())
            .add_route(controllers::page::routes())
            .add_route(controllers::api::routes())
            .add_route(controllers::admin::routes())
    }
    async fn connect_workers(ctx: &AppContext, queue: &Queue) -> Result<()> {
        queue.register(DownloadWorker::build(ctx)).await?;
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
        tasks.register(tasks::user_create::UserCreate);
        tasks.register(tasks::user_delete::UserDelete);
    }
    async fn truncate(ctx: &AppContext) -> Result<()> {
        truncate_table(&ctx.db, reader_dispatches::Entity).await?;
        truncate_table(&ctx.db, articles::Entity).await?;
        truncate_table(&ctx.db, categories::Entity).await?;
        truncate_table(&ctx.db, wire_dispatches::Entity).await?;
        truncate_table(&ctx.db, market_ledgers::Entity).await?;
        truncate_table(&ctx.db, newsletter_subscribers::Entity).await?;
        truncate_table(&ctx.db, users::Entity).await?;
        Ok(())
    }
    async fn seed(ctx: &AppContext, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(&ctx.db, &base.join("users.yaml").display().to_string())
            .await?;
        db::seed::<categories::ActiveModel>(
            &ctx.db,
            &base.join("categories.yaml").display().to_string(),
        )
        .await?;
        db::seed::<articles::ActiveModel>(
            &ctx.db,
            &base.join("articles.yaml").display().to_string(),
        )
        .await?;
        db::seed::<wire_dispatches::ActiveModel>(
            &ctx.db,
            &base.join("wire_dispatches.yaml").display().to_string(),
        )
        .await?;
        db::seed::<market_ledgers::ActiveModel>(
            &ctx.db,
            &base.join("market_ledgers.yaml").display().to_string(),
        )
        .await?;
        db::seed::<reader_dispatches::ActiveModel>(
            &ctx.db,
            &base.join("reader_dispatches.yaml").display().to_string(),
        )
        .await?;
        db::seed::<newsletter_subscribers::ActiveModel>(
            &ctx.db,
            &base.join("newsletter_subscribers.yaml").display().to_string(),
        )
        .await?;
        Ok(())
    }
}
