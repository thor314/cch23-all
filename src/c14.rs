//! this puzzle demonstrates avoiding cross-site scripting with askama templating. Pretty short and
//! sweet, especially after the slog of c13's sqlx.
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
  Router::new().route("/unsafe", post(unsafe_santa)).route("/safe", post(safe_santa))
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Template)]
// https://github.com/djc/askama
// https://docs.rs/askama/latest/askama/
// load the html template and override template extension to allow proper rendering, and also xss
// #[template(path = "day14.html")]
// before:
//     &lt;h1&gt;Welcome to the North Pole!&lt;/h1&gt;
#[template(path = "day14.html", escape = "none")]
// after:
//    <h1>Welcome to the North Pole!</h1>
struct UnsafeHtmlContent {
  pub content: String,
}

async fn unsafe_santa(Json(content): Json<UnsafeHtmlContent>) -> Result<Html<String>, StatusCode> {
  info!("content {content:?}");
  let reply_html = UnsafeHtmlContent { content: content.content }.render().map_err(|e| {
    tracing::error!("error while rendering html {e}");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;
  Ok(Html(reply_html))
}

// just remove the escape macro tag
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, Template)]
#[template(path = "day14.html")]
struct SafeHtmlContent {
  pub content: String,
}

async fn safe_santa(Json(content): Json<UnsafeHtmlContent>) -> Result<Html<String>, StatusCode> {
  println!("{content:?}");
  let reply_html = SafeHtmlContent { content: content.content }.render().map_err(|e| {
    tracing::error!("error while rendering html {e}");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;
  Ok(Html(reply_html))
}
