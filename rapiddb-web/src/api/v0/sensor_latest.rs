use rapiddb::{api::helpers::with_db, traits::IAsyncDatabase};

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/:String/latest
pub fn get(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / "latest")
    .and(warp::get())
    .and(with_db(db))
    .and_then(_get)
}

pub async fn _get(
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

#[tokio::test]
async fn test_get() {
  let database_test_factory =
    rapiddb::db::DatabaseTestFactory::new(".temp/test/sensor_latest/test_get");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";
    let id1 = "test-1";

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .await
      .post(id, serde_json::json!({ "id": &id }).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({ "id": &id })
    );

    db.write()
      .await
      .post(id, serde_json::json!({ "id1": &id }).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/latest"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({ "id1": &id })
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
      .path(&format!("/api/v0/{id1}/latest"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);
  }
}
