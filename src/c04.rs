use axum::{
  extract::Json,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use serde::Deserialize;
use serde_json::json;
use tracing::{debug, info};

#[derive(serde::Deserialize)]
pub struct Reindeer {
  name:     String,
  strength: i32,
}

pub async fn calculate_total_strength(Json(reindeers): Json<Vec<Reindeer>>) -> impl IntoResponse {
  let total_strength: i32 = reindeers.into_iter().map(|reindeer| reindeer.strength).sum();
  debug!("Total strength: {}", total_strength);
  total_strength.to_string()
}

#[derive(serde::Deserialize)]
pub struct Reindeer2 {
  name:             String,
  strength:         i32,
  speed:            f32,
  height:           i32,
  antler_width:     i32,
  snow_magic_power: i32,
  favorite_food:    String,
  #[serde(alias = "cAnD13s_3ATeN-yesT3rdAy")]
  candies:          u32,
}

pub async fn contest_summary(Json(reindeers): Json<Vec<Reindeer2>>) -> impl IntoResponse {
  let fastest = reindeers.iter().max_by(|a, b| a.speed.total_cmp(&b.speed));
  let tallest = reindeers.iter().max_by_key(|r| r.height);
  let magician = reindeers.iter().max_by_key(|r| r.snow_magic_power);
  let candiest = reindeers.iter().max_by_key(|r| r.candies);
  let response = match (fastest, tallest, magician, candiest) {
    (Some(f), Some(t), Some(m), Some(c)) => Json(json!({
          "fastest": format!(
              "Speeding past the finish line with a strength of {} is {}",
              f.strength, f.name
          ),
          "tallest": format!(
              "{} is standing tall with his {} cm wide antlers",
              t.name, t.antler_width
          ),
          "magician": format!(
              "{} could blast you away with a snow magic power of {}",
              m.name, m.snow_magic_power
          ),
          "consumer": format!(
              "{} ate lots of candies, but also some {}",
              c.name, c.favorite_food
          ),
    }))
    .into_response(),
    _ => (StatusCode::BAD_REQUEST, "Invalid contest").into_response(),
  };

  debug!("{:?}", &response);

  response
}
