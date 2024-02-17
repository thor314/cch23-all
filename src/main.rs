#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod c01;
mod c04;
mod c05;
mod c06;
mod c07;
mod c08;
mod c11;
mod c12;
mod c13;
mod c14;
mod c15;
mod error;
#[cfg(test)] mod tests;
mod utils;

use std::{
  collections::HashMap,
  sync::{Arc, RwLock},
};

use axum::{
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Router,
};
use error::MyError;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{
  c01::calculate_sled_id,
  c04::{calculate_total_strength, contest_summary},
  c05::paginate_names,
  c06::elf_regex,
  c07::{cookie_handler, secret_cookie_handler},
  c08::{poke_drop, poke_weight},
  c11::red_pixels,
  c12::{elapsed_time, store_string, ulids_to_uuids, ulids_weekday},
};

async fn hello_world() -> &'static str { "Hello, world!" }

async fn error_handler() -> impl IntoResponse {
  (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
}

#[shuttle_runtime::main]
async fn main(
  #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
  #[shuttle_secrets::Secrets] secret_store: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
  utils::setup(&secret_store).unwrap();
  sqlx::migrate!().run(&pool).await.unwrap();

  info!("hello thor");

  let router = Router::new()
    .route("/", get(hello_world))
    .route("/-1/error", get(error_handler))
    .route("/-1/health", get(|| async { StatusCode::OK }))
    .route("/1/*path", get(calculate_sled_id))
    .route("/4/strength", post(calculate_total_strength))
    .route("/4/contest", post(contest_summary))
    .route("/5", post(paginate_names))
    .route("/6", post(elf_regex))
    .route("/7/decode", get(cookie_handler))
    .route("/7/bake", get(secret_cookie_handler))
    .route("/8/weight/:pokedex", get(poke_weight))
    .route("/8/drop/:pokedex", get(poke_drop))
    .nest_service("/11/assets", ServeDir::new("assets"))
    .route("/11/red_pixels", post(red_pixels))
    .nest("/12", c12::router())
    .nest("/13", c13::router(pool))
    .nest("/14", c14::router())
    .nest("/15", c15::router());

  Ok(router.into())
}
