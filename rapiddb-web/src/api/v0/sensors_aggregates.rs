use rapiddb::{api::helpers::with_db, traits::IAsyncDatabase};

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/sensors/aggregates
pub fn get(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / "sensors" / "aggregates")
    .and(warp::get())
    .and(with_db(db))
    .and_then(_get)
}

pub async fn _get(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
  let lock = db.read().await;
  let data = lock.get_all_aggregates().await;

  if !data.is_empty() {
    let mut result: String = Default::default();
    result += "{";
    for (key, value) in data {
      result += &format!(
        "\"{key}\":{},",
        std::str::from_utf8(&value).unwrap_or_default()
      );
    }
    result.pop();
    result += "}";

    return Ok(
      warp::hyper::Response::builder()
        .status(warp::http::StatusCode::OK)
        .body(result),
    );
  }

  Ok(
    warp::hyper::Response::builder()
      .status(warp::http::StatusCode::NOT_FOUND)
      .body(String::new()),
  )
}

#[tokio::test]
async fn test_get() {
  let database_test_factory = rapiddb::db::DatabaseTestFactory::new(
    ".temp/test/sensors_aggregates/test_get",
  );

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";
    let id0 = "test-1";

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/aggregates")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .await
      .post(id, serde_json::json!({"temp": 8.00}).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/aggregates")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);

    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({id: {"temp_avg": 8.0, "temp_sum": 8.0, "temp_sum_count": 1.0}})
    );

    db.write()
      .await
      .post(id0, serde_json::json!({"temp": 4.00}).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/aggregates")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_object()
        .unwrap()
        .len(),
      2
    );
  }
}
