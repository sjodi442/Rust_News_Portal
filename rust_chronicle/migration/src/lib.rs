#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;

mod m20220101_000001_users;
mod m20261013_024903_categories;
mod m20261013_030000_articles;
mod m20261013_030100_wire_dispatches;
mod m20261013_030200_market_ledgers;
mod m20261013_030300_reader_dispatches;
mod m20261013_030400_newsletter_subscribers;
mod m20261013_040000_optimize_indexes;
mod m20261014_050000_articles_fts;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20261013_024903_categories::Migration),
            Box::new(m20261013_030000_articles::Migration),
            Box::new(m20261013_030100_wire_dispatches::Migration),
            Box::new(m20261013_030200_market_ledgers::Migration),
            Box::new(m20261013_030300_reader_dispatches::Migration),
            Box::new(m20261013_030400_newsletter_subscribers::Migration),
            Box::new(m20261013_040000_optimize_indexes::Migration),
            Box::new(m20261014_050000_articles_fts::Migration),
        ]
    }
}
