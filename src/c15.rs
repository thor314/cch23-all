use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::info;

pub fn router() -> Router { Router::new().route("/nice", post(nice)).route("/game", post(game)) }

// Create an endpoint at /15/nice that accepts POST requests with a JSON payload containing a
// password string to validate.
//
// The rules at this endpoint are:
//     Nice Strings: Must contain at least three vowels (aeiouy), at least one letter that appears
// twice in a row, and must not contain the substrings: ab, cd, pq, or xy.     Naughty Strings: Do
// not meet the criteria for nice strings.
async fn nice(Json(password): Json<Password>) -> impl IntoResponse {
  let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
  let inp = password.input.to_lowercase();
  let vowels_count = inp.chars().filter(|c| vowels.contains(c)).count();
  let contains_naughty_pair =
    inp.contains("ab") || inp.contains("cd") || inp.contains("pq") || inp.contains("xy");

  let contains_double = inp
    .chars()
    .zip(inp.chars().skip(1))
    .any(|(a, b)| a == b && a.is_alphabetic() && b.is_alphabetic());

  if vowels_count >= 3 && contains_double && !contains_naughty_pair {
    info!("Nice: {}", inp);
    (StatusCode::OK, Json(json!({ "result": "nice"})))
  } else {
    info!(
      "Naughty: {}; vowelcount: {vowels_count}, contains_double: {contains_double}, naughty_pair: \
       {contains_naughty_pair} ",
      inp
    );
    (StatusCode::BAD_REQUEST, Json(json!( {"result": "naughty"} )))
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Password {
  input: String,
}

// task 2
// Add a similar endpoint, POST /15/game, that has this set of rules:
//
// Nice Strings: Must adhere to all the rules:
// Rule 1: must be at least 8 characters long
// Rule 2: must contain uppercase letters, lowercase letters, and digits
// Rule 3: must contain at least 5 digits
// Rule 4: all integers (sequences of consecutive digits) in the string must add up to 2023
// Rule 5: must contain the letters j, o, and y in that order and in no other order
// Rule 6: must contain a letter that repeats with exactly one other letter between them (like xyx)
// Rule 7: must contain at least one unicode character in the range [U+2980, U+2BFF]
// Rule 8: must contain at least one emoji
// Rule 9: the hexadecimal representation of the sha256 hash of the string must end with an a
// Naughty Strings: Do not meet the criteria for nice strings.
async fn game(Json(password): Json<Password>) -> impl IntoResponse { todo!() }
