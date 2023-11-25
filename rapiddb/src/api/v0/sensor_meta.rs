use crate::{api::helpers::with_db, traits::IAsyncDatabase};

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/:String/meta
pub fn get(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / "meta")
    .and(warp::get())
    .and(with_db(db))
    .and_then(_get)
}

pub async fn _get(
  id: String,
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
  let result = db.write().await.get_meta(&id).await;

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

/// POST /api/v0/:String/meta
pub fn post(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / "meta")
    .and(warp::post())
    .and(warp::body::content_length_limit(1024 * 16))
    .and(warp::body::json())
    .and(with_db(db))
    .and_then(_post)
}

pub async fn _post(
  id: String,
  data: serde_json::Value,
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
  db.write().await.post_meta(&id, data.to_string().as_bytes().to_vec()).await;

  Ok(
    warp::hyper::Response::builder()
      .status(warp::http::StatusCode::ACCEPTED)
      .body(""),
  )
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

    db.write()
      .await
      .post_meta(
        id,
        serde_json::json!({ "id": &id }).to_string().as_bytes().to_vec(),
      )
      .await;

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

    db.write()
      .await
      .post_meta(
        id,
        serde_json::json!({ "id0": &id }).to_string().as_bytes().to_vec(),
      )
      .await;

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

    let id_db = db.write().await.get_meta(id).await;

    assert_eq!(
      id_db,
      serde_json::json!({ "id": &id }).to_string().as_bytes().to_vec()
    );
  }
}
