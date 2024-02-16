use axum::{
  extract::Query,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::{debug, info};

// #[axum::debug_handler]
pub async fn paginate_names(
  Query(params): Query<PaginationQ>,
  Json(names): Json<Vec<String>>,
) -> impl IntoResponse {
  let offset = params.offset.unwrap_or(0);
  // let split = params.split.unwrap_or(names.len());
  let limit = params.limit.unwrap_or(names.len());

  let end = std::cmp::min(offset + limit, names.len());
  debug!("names: {:?}", &names);
  debug!("no split");

  let names = names.into_iter().skip(offset).take(limit).collect::<Vec<_>>();
  match params.split {
    Some(split) => {
      let paginated_names = names.chunks(split).map(|c| c.to_vec()).collect::<Vec<Vec<String>>>();
      Json(json!(paginated_names))
    },
    None => Json(json!(names)),
  }
}

#[derive(Deserialize, Debug)]
pub struct PaginationQ {
  split:  Option<usize>,
  offset: Option<usize>,
  limit:  Option<usize>,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
  data: Vec<T>,
}
