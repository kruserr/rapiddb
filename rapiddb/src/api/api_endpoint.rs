use warp::{Filter, Rejection, Reply};

/// GET /api
pub fn get(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  warp::path!("api").and(warp::get()).map(|| {
    warp::reply::json(&serde_json::json!({
      "resources": [
        {"endpoint": "/api/v0", "description": "Discover resources available under API v0"},
        {"endpoint": "/api/v1", "description": "Discover resources available under API v1"},
        {"endpoint": "/api/custom", "description": "Discover user defined resources"},
      ],
      "description": "Discover resources available",
    }))
  })
}

#[tokio::test]
async fn test_get() {
  let database_test_factory = crate::db::DatabaseTestFactory::new(
    ".temp/test/api_endpoint/test_get",
  );

  for db in database_test_factory.get_instance().values() {
    let api = super::endpoints((*db).clone());

    let resp = warp::test::request()
      .method("GET")
      .path("/api")
      .reply(&api)
      .await;
    assert_eq!(resp.status(), 200);
  }
}
