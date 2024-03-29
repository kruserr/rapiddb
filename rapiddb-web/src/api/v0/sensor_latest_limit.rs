use crate::api::helpers::with_db;
use rapiddb::traits::IAsyncDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/:String/latest/:usize
pub fn get(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / "latest" / usize)
    .and(warp::get())
    .and(with_db(db))
    .and_then(_get)
}

pub async fn _get(
  id: String,
  limit: usize,
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
  let data = db.write().await.get_latest_with_limit(&id, limit).await;

  if !data.is_empty() {
    let mut result: String = Default::default();
    result += "[";
    for item in data {
      result += &format!("{},", std::str::from_utf8(&item).unwrap_or_default());
    }
    result.pop();
    result += "]";

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
    ".temp/test/sensor_latest_limit/test_get",
  );

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";
    let id1 = "test-1";
    let limit = 10;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .await
      .post(id, serde_json::json!({ "id": &id }).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest/0"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!([{ "id": &id }])
    );

    for _ in 0..limit - 2 {
      db.write()
        .await
        .post(id, serde_json::json!({ "id2": &id }).to_string().as_bytes())
        .await;
    }

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_array()
        .unwrap()
        .len(),
      limit - 1
    );

    db.write()
      .await
      .post(id, serde_json::json!({ "id3": &id }).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_array()
        .unwrap()
        .len(),
      limit
    );

    for _ in 0..8 {
      db.write()
        .await
        .post(id, serde_json::json!({ "id4": &id }).to_string().as_bytes())
        .await;
    }

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_array()
        .unwrap()
        .len(),
      limit
    );

    db.write()
      .await
      .post_meta(
        id1,
        serde_json::json!({ "id1": &id1 }).to_string().as_bytes().to_vec(),
      )
      .await;
    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id1}/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);
  }
}
