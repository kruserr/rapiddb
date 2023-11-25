use crate::traits::IDatabase;

use warp::{Filter, Rejection, Reply};

/// GET /api/v0/:String
pub fn get(
  db: std::sync::Arc<std::sync::RwLock<impl IDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String)
    .and(warp::get())
    .map(move |id: String| {
        if db.write().unwrap().contains(&id) {
          return warp::hyper::Response::builder()
            .status(warp::http::StatusCode::OK)
            .body(format!("{}", &serde_json::json!({
              "resources": [
                {"endpoint": format!("/api/v0/{id}/latest"), "description": format!("GET latest measurment from {id}")},
                {"endpoint": format!("/api/v0/{id}/latest/:count"), "description": format!("GET latest :count measurments from {id}")},
                {"endpoint": format!("/api/v0/{id}/:id"), "description": format!("GET measurment by id from {id}")},
                {"endpoint": format!("/api/v0/{id}/:start/:end"), "description": format!("GET measurment by id in range :start to :end from {id}")},
                {"endpoint": format!("/api/v0/{id}/meta"), "description": format!("GET metadata from {id}")},
                {"endpoint": format!("/api/v0/{id}/aggregates"), "description": format!("GET aggregates from {id}")},
                {"endpoint": format!("/api/v0/{id}"), "description": format!("POST data to {id}")},
                {"endpoint": format!("/api/v0/{id}/meta"), "description": format!("POST metadata to {id}")},
              ],
              "description": format!("Discover resources available under {id}"),
            })));
        }

        warp::hyper::Response::builder()
          .status(warp::http::StatusCode::NOT_FOUND)
          .body(Default::default())
    })
}

/// POST /api/v0/:String
pub fn post(
  db: std::sync::Arc<std::sync::RwLock<impl IDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api" / "v0" / String)
    .and(warp::post())
    .and(warp::body::content_length_limit(1024 * 16))
    .and(warp::body::bytes())
    .map(move |id: String, data: warp::hyper::body::Bytes| {
      db.write().unwrap().post(&id, &data);

      warp::hyper::Response::builder()
        .status(warp::http::StatusCode::ACCEPTED)
        .body("")
    })
}

#[tokio::test]
async fn test_get() {
  let database_test_factory =
    crate::db::DatabaseTestFactory::new(".temp/test/sensor/test_get");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 404);

    db.write()
      .unwrap()
      .post(id, serde_json::json!({ "id": &id }).to_string().as_bytes());

    let resp = warp::test::request()
      .method("GET")
      .path(&format!("/api/v0/{id}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
  }
}

#[tokio::test]
async fn test_post() {
  let database_test_factory =
    crate::db::DatabaseTestFactory::new(".temp/test/sensor/test_post");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let id = "test-0";

    let resp = warp::test::request()
      .method("POST")
      .path(&format!("/api/v0/{id}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 411);

    let resp = warp::test::request()
      .method("POST")
      .body(id)
      .path(&format!("/api/v0/{id}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 202);

    let resp = warp::test::request()
      .method("POST")
      .json(&serde_json::json!({ "id": &id }))
      .path(&format!("/api/v0/{id}"))
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 202);
    assert_eq!(resp.body().len(), 0);

    let id_db =
      db.write().map(|mut lock| lock.get_latest(id)).unwrap_or_default();
    assert_eq!(
      id_db,
      serde_json::json!({ "id": &id }).to_string().as_bytes().to_vec()
    );
  }
}
