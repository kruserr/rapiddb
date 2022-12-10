use warp::{Filter, Rejection, Reply};

/// GET /api/v0/sensors
pub fn get() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path("api")
    .and(warp::path("v0"))
    .and(warp::path("sensors"))
    .and(warp::get())
    .map(|| {
      warp::reply::json(&serde_json::json!({
        "resources": [
          {"endpoint": "/api/v0/sensors/latest", "description": "GET latest measurment from every sensor"},
          {"endpoint": "/api/v0/sensors/latest/:count", "description": "GET latest :count measurments from every sensor"},
          {"endpoint": "/api/v0/sensors/meta", "description": "GET metadata from every sensor"},
          {"endpoint": "/api/v0/sensors/aggregates", "description": "GET aggregates from every sensor"},
        ],
        "description": "Discover resources available for all sensors",
      }))
    })
}

#[tokio::test]
async fn test_get() {
  let database_test_factory =
    crate::db::DatabaseTestFactory::new(".temp/test/sensors/test_get");

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let resp = warp::test::request()
      .method("GET")
      .path("/api/v0/sensors")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
  }
}
