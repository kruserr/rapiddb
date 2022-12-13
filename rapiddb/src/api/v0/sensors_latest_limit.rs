use crate::traits::IDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/sensors/latest/:usize
pub fn get(
  db: std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / "sensors" / "latest" / usize).and(warp::get()).map(
    move |limit| {
      let mut lock = db.write().unwrap();
      let data = lock.get_all_latest_with_limit(limit);

      if !data.is_empty() {
        let mut result: String = Default::default();
        result += "{";
        for (key, value_arr) in data {
          result += &format!("\"{key}\":[");
          for value in value_arr {
            result +=
              &format!("{},", std::str::from_utf8(&value).unwrap_or_default());
          }
          result.pop();
          result += "],";
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
  let database_test_factory = crate::db::DatabaseTestFactory::new(
    ".temp/test/sensors_latest_limit/test_get",
  );

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";
    let id0 = "test-1";
    let id1 = "test-2";
    let limit = 10;

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .unwrap()
      .post(id, serde_json::json!({ "id": &id }).to_string().as_bytes());

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body()).unwrap(),
      serde_json::json!({id: [{"id": &id}]})
    );

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors/latest/0")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .unwrap()
      .post(id, serde_json::json!({ "id1": &id }).to_string().as_bytes());

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_object()
        .unwrap()
        .len(),
      1
    );
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_object()
        .unwrap()
        .get(id)
        .unwrap()
        .as_array()
        .unwrap()
        .len(),
      2
    );

    for _ in 0..8 {
      db.write()
        .unwrap()
        .post(id0, serde_json::json!({ "id2": &id }).to_string().as_bytes());
    }

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
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
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_object()
        .unwrap()
        .get(id)
        .unwrap()
        .as_array()
        .unwrap()
        .len()
        + serde_json::from_slice::<serde_json::Value>(resp.body())
          .unwrap()
          .as_object()
          .unwrap()
          .get(id0)
          .unwrap()
          .as_array()
          .unwrap()
          .len(),
      10
    );

    for _ in 0..8 {
      db.write()
        .unwrap()
        .post(id0, serde_json::json!({ "id2": &id }).to_string().as_bytes());
    }

    for _ in 0..7 {
      db.write()
        .unwrap()
        .post(id, serde_json::json!({ "id2": &id }).to_string().as_bytes());
    }

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
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
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_object()
        .unwrap()
        .get(id)
        .unwrap()
        .as_array()
        .unwrap()
        .len()
        + serde_json::from_slice::<serde_json::Value>(resp.body())
          .unwrap()
          .as_object()
          .unwrap()
          .get(id0)
          .unwrap()
          .as_array()
          .unwrap()
          .len(),
      19
    );

    db.write()
      .unwrap()
      .post(id, serde_json::json!({ "id2": &id }).to_string().as_bytes());

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
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
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_object()
        .unwrap()
        .get(id)
        .unwrap()
        .as_array()
        .unwrap()
        .len()
        + serde_json::from_slice::<serde_json::Value>(resp.body())
          .unwrap()
          .as_object()
          .unwrap()
          .get(id0)
          .unwrap()
          .as_array()
          .unwrap()
          .len(),
      20
    );

    for _ in 0..8 {
      db.write()
        .unwrap()
        .post(id, serde_json::json!({ "id2": &id }).to_string().as_bytes());
    }

    for _ in 0..8 {
      db.write()
        .unwrap()
        .post(id0, serde_json::json!({ "id2": &id }).to_string().as_bytes());
    }

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
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
    assert_eq!(
      serde_json::from_slice::<serde_json::Value>(resp.body())
        .unwrap()
        .as_object()
        .unwrap()
        .get(id)
        .unwrap()
        .as_array()
        .unwrap()
        .len()
        + serde_json::from_slice::<serde_json::Value>(resp.body())
          .unwrap()
          .as_object()
          .unwrap()
          .get(id0)
          .unwrap()
          .as_array()
          .unwrap()
          .len(),
      20
    );

    db.write().unwrap().post_meta(
      id1,
      serde_json::json!({ "id1": &id1 }).to_string().as_bytes().to_vec(),
    );
    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/sensors/latest/{limit}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
  }
}
