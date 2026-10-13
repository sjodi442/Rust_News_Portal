use sea_orm::entity::prelude::*;
use sea_orm::{IntoActiveModel, Set};
pub use super::_entities::market_ledgers::{self, ActiveModel, Entity, Model};

#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {}

pub struct CommodityTarget {
    pub symbol: &'static str,
    pub item_name: &'static str,
    pub category: &'static str,
    pub specification: &'static str,
    pub unit: &'static str,
}

pub const COMMODITY_TARGETS: &[CommodityTarget] = &[
    CommodityTarget {
        symbol: "GC=F",
        item_name: "Gold Bullion 99.5%",
        category: "commodities",
        specification: "COMEX Physical Delivery Spot",
        unit: "/ Troy Oz",
    },
    CommodityTarget {
        symbol: "SI=F",
        item_name: "Silver Ingot Standard",
        category: "commodities",
        specification: "COMEX 5,000oz Standard Bar",
        unit: "/ Troy Oz",
    },
    CommodityTarget {
        symbol: "CL=F",
        item_name: "Crude Petroleum WTI",
        category: "commodities",
        specification: "NYMEX Light Sweet Cushing",
        unit: "/ Barrel",
    },
    CommodityTarget {
        symbol: "HG=F",
        item_name: "Copper Grade A Cathode",
        category: "commodities",
        specification: "COMEX High Grade Benchmark",
        unit: "/ Lb",
    },
    CommodityTarget {
        symbol: "NG=F",
        item_name: "Henry Hub Natural Gas",
        category: "commodities",
        specification: "Louisiana Pipeline Terminal",
        unit: "/ MMBtu",
    },
];

fn format_price(val: f64) -> String {
    if val >= 1000.0 {
        let int_part = val.trunc() as i64;
        let frac_part = ((val.fract().abs()) * 100.0).round() as i64;
        let s = int_part.to_string();
        let mut formatted = String::new();
        let len = s.len();
        for (idx, ch) in s.chars().enumerate() {
            formatted.push(ch);
            if (len - 1 - idx) % 3 == 0 && idx != len - 1 {
                formatted.push(',');
            }
        }
        format!("{}.{:02}", formatted, frac_part)
    } else {
        format!("{:.2}", val)
    }
}

pub async fn fetch_yahoo_quote(client: &reqwest::Client, symbol: &str) -> Option<(f64, f32)> {
    let url = format!("https://query1.finance.yahoo.com/v8/finance/chart/{symbol}");
    let resp = client
        .get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
        .timeout(std::time::Duration::from_secs(4))
        .send()
        .await
        .ok()?;

    let json: serde_json::Value = resp.json().await.ok()?;
    let meta = json.get("chart")?.get("result")?.get(0)?.get("meta")?;
    let price = meta.get("regularMarketPrice")?.as_f64()?;
    let prev = meta
        .get("chartPreviousClose")
        .and_then(|v| v.as_f64())
        .unwrap_or(price);
    let pct = if prev > 0.0 {
        (((price - prev) / prev * 100.0 * 100.0).round() / 100.0) as f32
    } else {
        0.0
    };
    Some((price, pct))
}

pub async fn sync_live_commodities(db: &DatabaseConnection) -> std::result::Result<usize, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .map_err(|e| e.to_string())?;

    let mut updated_count = 0;

    for (idx, target) in COMMODITY_TARGETS.iter().enumerate() {
        if let Some((price, change_pct)) = fetch_yahoo_quote(&client, target.symbol).await {
            let formatted_spot = format_price(price);
            let target_id = (idx + 1) as i64;

            // Cari berdasarkan ID atau item_name
            let existing = Entity::find_by_id(target_id)
                .one(db)
                .await
                .map_err(|e| e.to_string())?;

            if let Some(model) = existing {
                let mut active = model.into_active_model();
                active.item_name = Set(target.item_name.to_string());
                active.category = Set(target.category.to_string());
                active.specification = Set(Some(target.specification.to_string()));
                active.spot_value = Set(formatted_spot);
                active.change_pct = Set(change_pct);
                active.unit = Set(Some(target.unit.to_string()));
                let _ = active.update(db).await.map_err(|e| e.to_string())?;
            } else {
                let new_item = ActiveModel {
                    id: Set(target_id),
                    item_name: Set(target.item_name.to_string()),
                    category: Set(target.category.to_string()),
                    specification: Set(Some(target.specification.to_string())),
                    spot_value: Set(formatted_spot),
                    change_pct: Set(change_pct),
                    unit: Set(Some(target.unit.to_string())),
                    ..Default::default()
                };
                let _ = new_item.insert(db).await.map_err(|e| e.to_string())?;
            }
            updated_count += 1;
        }
    }

    Ok(updated_count)
}
