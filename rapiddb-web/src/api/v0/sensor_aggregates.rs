use crate::api::helpers::with_db;
use rapiddb::traits::IAsyncDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/:String/aggregates
pub fn get(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / "aggregates")
    .and(warp::get())
    .and(with_db(db))
    .and_then(_get)
}

pub async fn _get(
  id: String,
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
  let lock = db.read().await;
  let result = lock.get_aggregates(&id).await;

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

#[tokio::test]
async fn test_get() {
  let database_test_factory = rapiddb::db::DatabaseTestFactory::new(
    ".temp/test/sensor_aggregates/test_get",
  );

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/aggregates"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .await
      .post(id, serde_json::json!({"temp": 8.00}).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/aggregates"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({"temp_avg": 8.0, "temp_sum": 8.0, "temp_sum_count": 1.0})
    );

    db.write()
      .await
      .post(id, serde_json::json!({"temp": 4.00}).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/aggregates"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({"temp_avg": 6.0, "temp_sum": 12.0, "temp_sum_count": 2.0})
    );
  }
}
