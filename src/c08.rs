
use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use serde::Deserialize;
use serde_json::from_str;
use tracing::info;

#[derive(Deserialize, Debug, Clone)]
pub struct PokeWeight {
  weight: u32,
}

/// add a GET endpoint /8/weight/<pokedex_number> that, given a pokédex number, responds with the
/// corresponding Pokémon's weight in kilograms as a number in plain text.
///
/// example
/// ```sh
/// curl http://localhost:8000/8/weight/25
///
/// 6
pub async fn poke_weight(Path(pokedex): Path<u32>) -> String { get_weight(pokedex).await.to_string() }

pub async fn get_weight(pokedex: u32) -> f64 {
  let url = format!("https://pokeapi.co/api/v2/pokemon/{}", pokedex);
  let res =
    reqwest::get(url).await.expect("poke-api failed").text().await.expect("could not get text");
  let res: PokeWeight = from_str(&res).expect("could not deserialize");
  info!("number: {pokedex}, weight: {}", res.weight);
  res.weight as f64 / 10f64
}

/// Calculate the momentum it would have at the time of impact with the floor if dropped from a 10-meter high chimney.
/// The GET endpoint /8/drop/<pokedex_number> shall respond with a plain text floating point number.
///
/// Use gravitational acceleration g = 9.825 m/s². Ignore air resistance.
pub async fn poke_drop(Path(pokedex): Path<u32>) -> String {
  let weight = get_weight(pokedex).await;
  let g = 9.825f64;
  // (d=10) = (s=0) * t + 0.5 * g * t^2
  let time: f64 = (2.0 * 10.0 / g).sqrt();
  let speed = g * time;
  let momentum = speed * weight;
  momentum.to_string()
}
