//! don't need to win em all
use axum::{
  debug_handler,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tracing::info;

pub fn router() -> Router { Router::new().route("/nice", post(nice)).route("/game", post(game)) }
// pub fn router() -> Router { Router::new().route("/nice", post(nice)).route("/game", post(game)) }

// async fn game_test () -> impl IntoResponse {
//   (StatusCode::OK, Json(json!({ "result": "nice"})))
// }

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
impl Password {
  fn groups_add_to(&self, arg: i32) -> bool {
    let mut sum = 0;
    let mut group: String = "".to_string();
    for c in self.input.chars() {
      if c.is_ascii_digit() {
        group.push(c);
        // sum += s.to_digit(10).unwrap() as i32;
      } else {
        sum += group.parse::<i32>().unwrap_or(0);
        group = "".to_string();
      }
    }
    sum == arg
  }
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
//
// for the first rule broken (in listed order) return a status code:
// Rule broken	Status Code	Reason
// 1	400	8 chars
// 2	400	more types of chars
// 3	400	55555
// 4	400	math is hard
// 5	406	not joyful enough
// 6	451	illegal: no sandwich
// 7	416	outranged
// 8	426	😳
// 9	418	not a coffee brewer
// None	200	that's a nice password
#[rustfmt::skip]
async fn game(Json(password): Json<Password>) -> GameResponse {
  // let inp = password.input.to_lowercase();
  if password.input.len() < 8 {
    info!("Naughty: {}; 8 chars", password.input);
    respond(1)
  } else if !(password.input.chars().any(|c| c.is_uppercase())
              && password.input.chars().any(|c| c.is_lowercase())
              && password.input.chars().any(|c| c.is_ascii_digit())) {
    info!("Naughty: {}; more types of chars", password.input); 
    respond(2)
  } else if password.input.chars().filter(|c| c.is_numeric()).count() < 5 { 
    info!("Naughty: {}; 55555", password.input);
    respond(3)
  } else if !password.groups_add_to(2023) { 
    info!("Naughty: {}; math is hard", password.input); // bug
    respond(4)
  } else if !joy_regex(&password.input) { 
    info!("Naughty: {}; not joyful enough", password.input);
    respond(5)
  } else if !password.input.chars().zip(password.input.chars().skip(2))
              .filter(|(a,b)| a.is_alphabetic() && b.is_alphabetic()).any(|(a, b)| a == b){
    info!("Naughty: {}; illegal: no sandwich", password.input);
    respond(6)
  } else if contains_unicode_in_range(&password.input){
    info!("Naughty: {}; outranged", password.input);
    respond(7)
  // } else if !password.input.chars().any(|c|  c as u32 >= 0x1F600 && c as u32 <= 0x1F64F) {
  } else if contains_emoji(&password.input) {
    info!("Naughty: {}; 😳", password.input);
    respond(8)
  } else if format!("{:x}", sha2::Sha256::digest(password.input.as_bytes())).ends_with('a') {
    let s = format!("{:x}", sha2::Sha256::digest(password.input.as_bytes()));
    info!("Naughty: {}; not a coffee brewer: {s}", password.input);
    respond(9)
  } else {
    respond(0)
  }
}

fn joy_regex(input: &str) -> bool {
  let pattern = fancy_regex::Regex::new(r"^(?!.*o.*j)(?!.*y.*j)(?!.*y.*o).*j.*o.*y.*$").unwrap();
  pattern.is_match(input).unwrap()
}

fn contains_unicode_in_range(s: &str) -> bool {
  s.chars().any(|c| ('\u{2980}'..='\u{2BFF}').contains(&c))
}

fn contains_emoji(s: &str) -> bool {
  let re = Regex::new(r"\p{Extended_Pictographic}").unwrap();
  if let Ok(m) = re.is_match(s) {
    return m;
  }
  false
}

type GameResponse = (StatusCode, Json<Value>);
#[rustfmt::skip]
fn respond(idx: u8) -> GameResponse { 
  match idx {
    0 => (StatusCode::OK, Json(json!({ "result": "nice"}))),
    1 => (StatusCode::BAD_REQUEST, Json(json!({ "result": "naughty", "reason": "8 chars" }))),
    2 => (StatusCode::BAD_REQUEST, Json(json!({ "result": "naughty", "reason": "more types of chars" }))),
    3 => (StatusCode::BAD_REQUEST, Json(json!({ "result": "naughty", "reason": "55555" }))),
    4 => (StatusCode::BAD_REQUEST, Json(json!({ "result": "naughty", "reason": "math is hard" }))),
    5 => (StatusCode::NOT_ACCEPTABLE, Json(json!({ "result": "naughty", "reason": "not joyful enough" }))),
    6 => (StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS, Json(json!({ "result": "naughty", "reason": "illegal: no sandwich" }))),
    7 => (StatusCode::RANGE_NOT_SATISFIABLE, Json(json!({ "result": "naughty", "reason": "outranged" }))),
    8 => (StatusCode::UPGRADE_REQUIRED, Json(json!({ "result": "naughty", "reason": "😳" }))),
    9 => (StatusCode::IM_A_TEAPOT, Json(json!({ "result": "naughty", "reason": "not a coffee brewer" }))),
    _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "result": "naughty", "reason": "unknown" }))),
  }
}
