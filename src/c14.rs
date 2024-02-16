use askama::Template;
use axum::{
  debug_handler,
  extract::State,
  http::StatusCode,
  response::{Html, IntoResponse},
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, PgPool, Pool, Postgres};
use tracing::info;

pub fn router() -> Router {
  Router::new().route("/14/unsafe", post(unsafe_santa))
  // .route("/14/safe", post(safe_santa))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Template)]
// https://github.com/djc/askama
// https://docs.rs/askama/latest/askama/
// load the html template and override template extension
#[template(path = "day14.html", escape = "none")]
struct UnsafeHtmlContent {
  pub content: String,
}

async fn unsafe_santa(
  Json(content): Json<UnsafeHtmlContent>,
) -> Result<(StatusCode, Html<String>), StatusCode> {
  // println!("{content:?}");
  info!("content {content:?}");
  let reply_html = UnsafeHtmlContent { content: content.content }.render().map_err(|e| {
    tracing::error!("error while rendering html {e}");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;
  Ok((StatusCode::OK, Html(reply_html)))
}
