use crate::{api::helpers::with_db, traits::IAsyncDatabase};

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/:String/:usize/:usize
pub fn get(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String / usize / usize)
    .and(warp::get())
    .and(with_db(db))
    .and_then(_get)
}

pub async fn _get(
  id: String,
  start: usize,
  end: usize,
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> Result<impl warp::Reply, std::convert::Infallible> {
  let data = db.write().await.get_range(&id, start, end).await;

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
  let database_test_factory =
    crate::db::DatabaseTestFactory::new(".temp/test/sensor_range/test_get");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";
    let n = 10;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/0/{n}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .await
      .post(id, serde_json::json!({ "id": &id }).to_string().as_bytes())
      .await;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/0/0"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!([{ "id": &id }])
    );

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/0/1"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!([{ "id": &id }])
    );

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/0/{n}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!([{ "id": &id }])
    );

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/2/{n}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    for _ in 0..n - 1 {
      db.write()
        .await
        .post(id, serde_json::json!({ "id0": &id }).to_string().as_bytes())
        .await;
    }

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/0/{n}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_array()
        .unwrap()
        .len(),
      n
    );

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}/{n}/0"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);
  }
}
