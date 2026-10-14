use std::sync::RwLock;
use std::time::{Duration, Instant};

static WEATHER_CACHE: RwLock<Option<(Instant, String)>> = RwLock::new(None);

pub fn wmo_code_to_condition(code: i64) -> &'static str {
    match code {
        0 => "Clear",
        1 => "Mainly Clear",
        2 => "Partly Cloudy",
        3 => "Overcast",
        45 | 48 => "Fog",
        51..=55 => "Drizzle",
        61..=65 => "Rain",
        71..=77 => "Snow",
        80..=82 => "Showers",
        85 | 86 => "Snow Showers",
        95..=99 => "Thunderstorm",
        _ => "Fair",
    }
}

async fn fetch_city_weather(client: &reqwest::Client, name: &str, lat: f64, lon: f64) -> Option<String> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,weather_code",
        lat, lon
    );

    let resp = client
        .get(&url)
        .header("User-Agent", "rust_chronicle/1.0")
        .timeout(Duration::from_secs(3))
        .send()
        .await
        .ok()?;

    let json: serde_json::Value = resp.json().await.ok()?;
    let current = json.get("current")?;
    let temp = current.get("temperature_2m")?.as_f64()?;
    let code = current.get("weather_code")?.as_i64().unwrap_or(0);
    let condition = wmo_code_to_condition(code);

    Some(format!("{} {:.0}°C {}", name, temp.round(), condition))
}

pub async fn sync_meteorology() -> String {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
    {
        Ok(c) => c,
        Err(_) => return get_cached_or_default(),
    };

    let london = fetch_city_weather(&client, "London", 51.5074, -0.1278).await;
    let ny = fetch_city_weather(&client, "New York", 40.7128, -74.0060).await;
    let tokyo = fetch_city_weather(&client, "Tokyo", 35.6762, 139.6503).await;

    let mut parts = Vec::new();
    if let Some(w) = london {
        parts.push(w);
    } else {
        parts.push("London 13°C Overcast".to_string());
    }

    if let Some(w) = ny {
        parts.push(w);
    } else {
        parts.push("New York 20°C Clear".to_string());
    }

    if let Some(w) = tokyo {
        parts.push(w);
    } else {
        parts.push("Tokyo 22°C Clear".to_string());
    }

    let summary = parts.join(" • ");

    if let Ok(mut cache) = WEATHER_CACHE.write() {
        *cache = Some((Instant::now(), summary.clone()));
    }

    summary
}

fn get_cached_or_default() -> String {
    if let Ok(cache) = WEATHER_CACHE.read() {
        if let Some((_, ref val)) = *cache {
            return val.clone();
        }
    }
    "London 13°C Overcast • New York 20°C Clear • Tokyo 22°C Clear".to_string()
}

pub async fn get_meteorology() -> String {
    if let Ok(cache) = WEATHER_CACHE.read() {
        if let Some((last, ref val)) = *cache {
            if last.elapsed() < Duration::from_secs(900) {
                return val.clone();
            }
        }
    }

    sync_meteorology().await
}
