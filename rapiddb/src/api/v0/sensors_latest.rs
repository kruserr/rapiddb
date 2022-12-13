use crate::traits::IDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/sensors/latest
pub fn get(
  db: std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / "sensors" / "latest").and(warp::get()).map(
    move || {
      let mut lock = db.write().unwrap();
      let data = lock.get_all_latest();

      if !data.is_empty() {
        let mut result: String = Default::default();
        result += "{";
        for (key, value) in data {
          if value.is_empty() {
            continue;
          }

          result += &format!(
            "\"{key}\":{},",
            std::str::from_utf8(&value).unwrap_or_default()
          );
        }
        result.pop();
        result += "}";

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

#[tokio::test]
async fn test_get() {
  let database_test_factory =
    crate::db::DatabaseTestFactory::new(".temp/test/sensors_latest/test_get");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";
    let id0 = "test-1";
    let id1 = "test-2";
    let id2 = "test-3";

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/latest")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .unwrap()
      .post(id, serde_json::json!({ "id": &id }).to_string().as_bytes());

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/latest")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({id: { "id": &id }})
    );

    db.write()
      .unwrap()
      .post(id, serde_json::json!({ "id1": &id }).to_string().as_bytes());

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/latest")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({id: { "id1": &id }})
    );

    db.write()
      .unwrap()
      .post(id0, serde_json::json!({ "id2": &id2 }).to_string().as_bytes());

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/latest")
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

    db.write().unwrap().post_meta(
      id1,
      serde_json::json!({ "id1": &id1 }).to_string().as_bytes().to_vec(),
    );
    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/latest")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
  }
}
