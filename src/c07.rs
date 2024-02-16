use std::collections::HashMap;

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use axum_extra::{headers::Cookie, TypedHeader};
use base64::{
  engine::general_purpose::{self, STANDARD},
  Engine,
};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, from_value, json, Value};
use tracing::info;

// use TypedHeader to extract header content, e.g. the cookie header
// parse the Cookie header, decode the value in the recipe field, and return it.
// given something like: Cookie: recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==
pub async fn cookie_handler(
  TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<Value>, StatusCode> {
  let recipe = extract_cookie(cookie)?;
  info!("recipe json: {:?}", recipe);
  Ok(Json(recipe))
}

pub fn extract_cookie(cookie: Cookie) -> Result<Value, StatusCode> {
  let recipe = cookie.get("recipe").ok_or(StatusCode::BAD_REQUEST)?;
  // info!("recipe bytes: {:?}", recipe);
  let recipe = STANDARD.decode(recipe).map_err(|e| {
    eprintln!("ERR: error while decoding recipe from base64 {e}");
    StatusCode::BAD_REQUEST
  })?;
  // info!("recipe decoded: {:?}", recipe);
  let recipe: Value = serde_json::from_slice(&recipe).map_err(|e| {
    eprintln!("ERR: error while deserialize from json {e}");
    StatusCode::BAD_REQUEST
  })?;
  Ok(recipe)
}

pub type AnyRecipe = HashMap<String, usize>;
pub type AnyPantry = HashMap<String, usize>;
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecipePantry {
  recipe: AnyRecipe,
  pantry: AnyPantry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecipePantryResponse {
  cookies: usize,
  pantry:  AnyPantry,
}

pub async fn secret_cookie_handler(
  TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<RecipePantryResponse>, StatusCode> {
  let recipe_pantry: RecipePantry = serde_json::from_value(extract_cookie(cookie)?).unwrap();
  let RecipePantry { pantry, recipe } = recipe_pantry;
  info!("pantry initial: {pantry:?}, recipe: {recipe:?}");

  let cookies = how_many(&pantry, &recipe);
  let pantry = reduce_pantry(&pantry, &recipe, cookies);
  info!("pantry remaining: {pantry:?}, cookies baked: {cookies:?}");

  let result = RecipePantryResponse { cookies, pantry };
  Ok(Json(result))
}

fn how_many(pantry: &AnyPantry, recipe: &AnyRecipe) -> usize {
  pantry
    .iter()
    .filter(|(k, v)| {
      recipe.get(*k).map_or(false, |recipe_value| *v >= recipe_value && *recipe_value > 0)
    })
    .map(|(k, v)| {
      let recipe_value = recipe.get(k).unwrap();
      v.checked_div(*recipe_value).unwrap_or(*v)
    })
    .min()
    .unwrap_or(0)
}

fn reduce_pantry(pantry: &AnyPantry, recipe: &AnyRecipe, count: usize) -> AnyPantry {
  pantry
    .iter()
    .map(|(k, v)| {
      let recipe_value = recipe.get(k).unwrap_or(&0);
      let remaining = v - count * recipe_value;
      let out = (k.clone(), remaining);
      info!("{remaining}= {v} - {count} * {recipe_value}");
      out
    })
    .collect()
}

// these are not general as requested in task 3

async fn _secret_cookie_handler(
  TypedHeader(cookie): TypedHeader<Cookie>,
) -> Result<Json<_RecipePantryResponse>, StatusCode> {
  let mut recipe_pantry: _RecipePantry = serde_json::from_value(extract_cookie(cookie)?).unwrap();
  info!("recipe json: {:?}", recipe_pantry);

  let cookies = recipe_pantry.pantry.how_many(&recipe_pantry.recipe);
  recipe_pantry.pantry.reduce_by(&recipe_pantry.recipe, cookies);
  info!("pantry: {:?}", recipe_pantry.pantry);
  info!("cookies: {:?}", cookies);

  let result = _RecipePantryResponse { cookies, pantry: recipe_pantry.pantry };
  Ok(Json(result))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Recipe {
  flour:           usize,
  sugar:           usize,
  butter:          usize,
  #[serde(rename = "baking powder")]
  baking_powder:   usize,
  #[serde(rename = "chocolate chips")]
  chocolate_chips: usize,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Pantry {
  flour:           usize,
  sugar:           usize,
  butter:          usize,
  #[serde(rename = "baking powder")]
  baking_powder:   usize,
  #[serde(rename = "chocolate chips")]
  chocolate_chips: usize,
}

impl Pantry {
  // count how many iterations of Recipe we can make, given a stock of Pantry
  fn how_many(&self, recipe: &Recipe) -> usize {
    let mut count = usize::MAX;
    count = count.min(self.flour / recipe.flour);
    count = count.min(self.sugar / recipe.sugar);
    count = count.min(self.butter / recipe.butter);
    count = count.min(self.baking_powder / recipe.baking_powder);
    count = count.min(self.chocolate_chips / recipe.chocolate_chips);
    count
  }

  // reduce stock by the amount of Recipe needed times count
  fn reduce_by(&mut self, recipe: &Recipe, count: usize) {
    self.flour -= count * recipe.flour;
    self.sugar -= count * recipe.sugar;
    self.butter -= count * recipe.butter;
    self.baking_powder -= count * recipe.baking_powder;
    self.chocolate_chips -= count * recipe.chocolate_chips;
  }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct _RecipePantry {
  recipe: Recipe,
  pantry: Pantry,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct _RecipePantryResponse {
  cookies: usize,
  pantry:  Pantry,
}
