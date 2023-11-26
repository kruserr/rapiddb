use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};
use tokio::sync::RwLock;

use rapiddb_web::api::helpers::with_db;
use rapiddb_web::rapiddb::traits::IAsyncDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/custom/:String/latest
pub fn get_latest_custom(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "custom" / String / "latest")
    .and(warp::get())
    .and(with_db(db))
    .and_then(_get_latest_custom)
}

/// GET /api/custom/:String/latest
pub async fn _get_latest_custom(
  id: String,
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
  let result = db.write().await.get_latest(&id).await;

  if !result.is_empty() {
    return Ok(
      warp::hyper::Response::builder()
        .status(warp::http::StatusCode::OK)
        .body(result),
    );
  }

  Ok(
    warp::hyper::Response::builder()
      .status(warp::http::StatusCode::NOT_FOUND)
      .body(Default::default()),
  )
}

#[tokio::main]
async fn main() {
  let mut aggregates_fn: HashMap<
    String,
    rapiddb_web::rapiddb::types::AggregateFn,
  > = Default::default();

  let test_fn = Arc::new(Mutex::new(
    |_: &str, value: &[u8], aggregate: &Arc<Mutex<Vec<u8>>>| {
      let obj =
        serde_json::from_slice::<serde_json::Value>(value).unwrap_or_default();

      if obj["temp"].is_null() {
        return;
      }

      aggregate
        .lock()
        .map(|mut x| {
          let mut aggregate_obj =
            serde_json::from_slice::<serde_json::Value>(&x).unwrap_or_default();

          let mut temp_sum =
            aggregate_obj["temp_sum"].as_f64().unwrap_or_default();
          let mut temp_sum_count =
            aggregate_obj["temp_sum_count"].as_f64().unwrap_or_default();

          temp_sum += obj["temp"].as_f64().unwrap_or_default();
          temp_sum_count += 1.;
          let temp_avg = temp_sum / temp_sum_count;

          aggregate_obj["temp_sum"] = serde_json::json!(temp_sum);
          aggregate_obj["temp_sum_count"] = serde_json::json!(temp_sum_count);
          aggregate_obj["temp_avg"] = serde_json::json!(temp_avg);

          *x = aggregate_obj.to_string().as_bytes().to_vec();
        })
        .err();
    },
  ));

  aggregates_fn.insert("test-0".to_string(), test_fn.clone());
  aggregates_fn.insert("test-1".to_string(), test_fn);

  let db = Arc::new(RwLock::new(
    rapiddb_web::rapiddb::db::MMAVAsyncDatabase::new_with_all(
      ".db",
      aggregates_fn,
    ),
  ));

  let value = b"{\"key\": \"value\"}";
  db.write().await.post("test-0", value).await;
  assert_eq!(db.write().await.get_latest("test-0").await, value);

  warp::serve(
    rapiddb_web::api::endpoints(db.clone()).or(get_latest_custom(db)),
  )
  .run(([0, 0, 0, 0], 3030))
  .await;
}
