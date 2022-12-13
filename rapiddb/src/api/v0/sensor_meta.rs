use crate::traits::IDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/:String/meta
pub fn get(
  db: std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / "meta").and(warp::get()).map(
    move |id: String| {
      let mut lock = db.write().unwrap();
      let result = lock.get_meta(&id);

      if !result.is_empty() {
        return warp::hyper::Response::builder()
          .status(warp::http::StatusCode::OK)
          .body(result);
      }

      warp::hyper::Response::builder()
        .status(warp::http::StatusCode::NOT_FOUND)
        .body(Default::default())
    },
  )
}

/// POST /api/v0/:String/meta
pub fn post(
  db: std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / "meta")
    .and(warp::post())
    .and(warp::body::content_length_limit(1024 * 16))
    .and(warp::body::json())
    .map(move |id: String, data: serde_json::Value| {
      db.write().unwrap().post_meta(&id, data.to_string().as_bytes().to_vec());

      warp::hyper::Response::builder()
        .status(warp::http::StatusCode::ACCEPTED)
        .body("")
    })
}

#[tokio::test]
async fn test_get() {
  let database_test_factory =
    crate::db::DatabaseTestFactory::new(".temp/test/sensor_meta/test_get");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/meta"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write().unwrap().post_meta(
      id,
      serde_json::json!({ "id": &id }).to_string().as_bytes().to_vec(),
    );

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/meta"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({ "id": &id })
    );

    db.write().unwrap().post_meta(
      id,
      serde_json::json!({ "id0": &id }).to_string().as_bytes().to_vec(),
    );

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/meta"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({ "id0": &id })
    );
  }
}

#[tokio::test]
async fn test_post() {
  let database_test_factory =
    crate::db::DatabaseTestFactory::new(".temp/test/sensor_meta/test_post");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";

    let resp = warp::test::request()
      .method("POST")
      .path(&format!("/api/v0/{id}/meta"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 411);

    let resp = warp::test::request()
      .method("POST")
      .body(id)
      .path(&format!("/api/v0/{id}/meta"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 400);

    let resp = warp::test::request()
      .method("POST")
      .json(&serde_json::json!({ "id": &id }))
      .path(&format!("/api/v0/{id}/meta"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 202);
    assert_eq!(resp.body().len(), 0);

    let mut lock = db.write().unwrap();
    let id_db = lock.get_meta(id);
    assert_eq!(
      id_db,
      serde_json::json!({ "id": &id }).to_string().as_bytes().to_vec()
    );
  }
}
