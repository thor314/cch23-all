

use std::{net::SocketAddr, vec};

use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};
use serde::Deserialize;
use tracing::debug;
#[derive(Deserialize)]
pub struct PathParams {
  num1: i32,
  num2: i32,
}

/// An endpoint that takes 2 numbers and returns (n1 xor n2).pow(3)
// use the Axum Path extractor to get the path parameters
pub async fn xor_and_pow(Path(PathParams { num1, num2 }): Path<PathParams>) -> String {
  let xor_result = num1 ^ num2;
  let pow_result = xor_result.pow(3);
  format!("{}", pow_result)
}
// Adapt the handler to work with a variable number of integers
pub async fn calculate_sled_id(Path(path): Path<String>) -> String {
  debug!("numbers: {:?}", path);
  let numbers = path.split('/').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let xor_result = numbers.into_iter().fold(0, |acc, num| acc ^ num);
  let pow_result = xor_result.pow(3);
  format!("{}", pow_result)
}
