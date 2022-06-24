use std::{
  collections::HashMap,
  sync::{Arc, Mutex, RwLock},
};

use rapiddb::traits::IDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/custom/:String/latest
pub fn get_latest_custom(
  db: std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "custom" / String / "latest")
    .and(warp::get())
    .map(move |id: String| {
      let mut lock = db.write().unwrap();
      let result = lock.get_latest(&id);

      if !result.is_empty() {
        return warp::hyper::Response::builder()
          .status(warp::http::StatusCode::OK)
          .body(result);
      }

      warp::hyper::Response::builder()
        .status(warp::http::StatusCode::NOT_FOUND)
        .body(Default::default())
    })
}

#[tokio::main]
async fn main() {
  let mut aggregates_fn: HashMap<
    String,
    Arc<Mutex<dyn Fn(&str, &[u8], &Arc<Mutex<Vec<u8>>>) + Send>>,
  > = Default::default();

  let test_fn = Arc::new(Mutex::new(
    |_: &str, value: &[u8], aggregate: &Arc<Mutex<Vec<u8>>>| {
      let obj = serde_json::from_slice::<serde_json::Value>(value)
        .unwrap_or_default();

      if obj["temp"].is_null() {
        return;
      }

      aggregate
        .lock()
        .map(|mut x| {
          let mut aggregate_obj =
            serde_json::from_slice::<serde_json::Value>(&x)
              .unwrap_or_default();

          let mut temp_sum =
            aggregate_obj["temp_sum"].as_f64().unwrap_or_default();
          let mut temp_sum_count = aggregate_obj["temp_sum_count"]
            .as_f64()
            .unwrap_or_default();

          temp_sum += obj["temp"].as_f64().unwrap_or_default();
          temp_sum_count += 1.;
          let temp_avg = temp_sum / temp_sum_count;

          aggregate_obj["temp_sum"] = serde_json::json!(temp_sum);
          aggregate_obj["temp_sum_count"] =
            serde_json::json!(temp_sum_count);
          aggregate_obj["temp_avg"] = serde_json::json!(temp_avg);

          *x = aggregate_obj.to_string().as_bytes().to_vec();
        })
        .err();
    },
  ));

  aggregates_fn.insert("test-0".to_string(), test_fn.clone());
  aggregates_fn.insert("test-1".to_string(), test_fn);

  let db = Arc::new(RwLock::new(
    rapiddb::db::MMAVDatabase::new_with_all(".db", aggregates_fn),
  ));

  let value = b"{\"key\": \"value\"}";
  db.write().unwrap().post("test-0", value);
  assert_eq!(db.write().unwrap().get_latest("test-0"), value);

  warp::serve(
    rapiddb::api::endpoints(db.clone()).or(get_latest_custom(db)),
  )
  .run(([0, 0, 0, 0], 3030))
  .await;
}
