use axum::{
  debug_handler,
  extract::State,
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
  Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{prelude::FromRow, PgPool, Pool, Postgres};
use tracing::info;

pub fn router(pool: Pool<Postgres>) -> Router {
  let state = SqlState { pool };
  Router::new()
    .route("/sql", get(shuttle_database_get))
    .route("/reset", post(reset))
    .route("/orders", post(orders))
    .route("/orders/total", get(orders_total))
    .route("/orders/popular", get(orders_popular))
    .with_state(state)
}

/// Add a Postgres database with the Shuttle Shared Database plugin, and add the pool to your
/// application state. Add a GET endpoint /13/sql that executes the SQL query SELECT 20231213 and
/// responds with the query result (an i32 turned into a string).
/// ```sh
/// curl http://localhost:8000/13/sql
///
/// 20231213
/// ```
// we'll need to store some state
#[derive(Clone)]
struct SqlState {
  pub pool: PgPool,
}

// examples:
// https://github.com/shuttle-hq/shuttle-examples/blob/main/axum/postgres/src/main.rs
//
// sqlx cli:
// https://lib.rs/crates/sqlx-cli
async fn shuttle_database_get(State(state): State<SqlState>) -> Result<String, StatusCode> {
  // query makes an SQL query
  // query_as is query w type-checking; makes a query that is mapped to a concrete type using
  // FromRow
  let santa: Santa = sqlx::query_as::<Postgres, Santa>("SELECT 20231213 id")
  // let santa: Santa = sqlx::query_as!(Santa, "SELECT 20231213 id") // no go idk
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
      tracing::error!("error while fetching from database {e}");
      StatusCode::INTERNAL_SERVER_ERROR
    })?;
  info!("row in sql {santa:?}");
  Ok(santa.id.to_string())
}

// see:
// https://github.com/shuttle-hq/shuttle-examples/blob/main/axum/postgres/src/main.rs
#[derive(Serialize, FromRow, Debug)]
struct Santa {
  pub id: i32,
}

// demo migration in function
async fn reset(State(state): State<SqlState>) -> Result<(), StatusCode> {
  // drop orders table if exists
  sqlx::query!("DROP TABLE IF EXISTS orders;").execute(&state.pool).await.map_err(|e| {
    tracing::error!("error droping orders table {e}");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  // create order table
  sqlx::query!(
    r#"
        CREATE TABLE orders (
            id INT PRIMARY KEY,
            region_id INT,
            gift_name VARCHAR(50),
            quantity INT
        );
    "#
  )
  .execute(&state.pool)
  .await
  .map_err(|e| {
    tracing::error!("error creating orders table {e}");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;
  Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::FromRow)]
struct Order {
  id:        i32,
  region_id: i32,
  gift_name: String,
  quantity:  i32,
}

// demo a post order that executes a database transaction
/// Takes a JSON array of order objects and inserts them into the table (see below). Return a plain
/// 200 OK.
async fn orders(
  State(state): State<SqlState>,
  Json(orders): Json<Vec<Order>>,
) -> Result<(), StatusCode> {
  // for order in orders {
  //   info!("create orders with order: {order:?}");

  // transaction: either all operations succeed or none do
  let transaction = state.pool.begin().await.map_err(|e| {
    tracing::error!("error starting transaction {e}");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  for order in orders {
    info!(
      "insert into orders(id, region_id, gift_name, quantity) values({}, {}, {}, {})",
      order.id, order.region_id, order.gift_name, order.quantity
    );
    sqlx::query!(
      "insert into orders(id, region_id, gift_name, quantity) values($1, $2, $3, $4)",
      order.id,
      order.region_id,
      order.gift_name,
      order.quantity,
    )
    .execute(&state.pool)
    .await
    .map_err(|e| {
      tracing::error!("error inserting to the database {e}");
      StatusCode::INTERNAL_SERVER_ERROR
    })?;
  }

  transaction.commit().await.map_err(|e| {
    tracing::error!("error commiting the transaction {e}");
    StatusCode::INTERNAL_SERVER_ERROR
  })?;

  Ok(())
}

// execute a query and do some sql nonsense
async fn orders_total(State(state): State<SqlState>) -> Result<Json<Value>, StatusCode> {
  let total = sqlx::query!("select sum(quantity) from orders")
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
      tracing::error!("error creating orders table {e}");
      StatusCode::INTERNAL_SERVER_ERROR
    })?
    .sum
    .unwrap_or_default();

  info!("total orders: {:?}", total);
  Ok(Json(json!({ "total": total})))
}

async fn orders_popular(State(state): State<SqlState>) -> Result<Json<Value>, StatusCode> {
  let popular =
    sqlx::query!("select sum(quantity) as quantity, gift_name from orders group by gift_name")
      .fetch_all(&state.pool)
      .await
      .map_err(|e| {
        tracing::error!("error creating orders table {e}");
        StatusCode::INTERNAL_SERVER_ERROR
      })?
      .into_iter()
      .max_by_key(|p| p.quantity);
  if let Some(p) = popular {
    info!("popular gift: {:?} has count {:?}", p.gift_name, p.quantity);
    Ok(Json(json!({ "popular": p.gift_name})))
  } else {
    Ok(Json(json!({ "popular": Value::Null})))
  }
}
