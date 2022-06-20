use warp::Filter;

/// GET /api/v0
pub fn get() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "v0")
        .and(warp::get())
        .map(|| {
            warp::reply::json(&serde_json::json!({
                "resources": [
                    {"endpoint": "/api/v0/sensors", "description": "Discover resources available for all sensors"},
                    {"endpoint": "/api/v0/:id", "description": "Discover resources available for sensor with :id"},
                ],
                "description": "Discover resources available under API v0",
            }))
        })
}

#[tokio::test]
async fn test_get() {
    let database_test_factory = crate::db::DatabaseTestFactory::new(".temp/test/api_v0/test_get");

    for db in database_test_factory.get_instance().values() {
        let api = super::endpoints((*db).clone());

        let resp = warp::test::request()
            .method("GET")
            .path("/api/v0")
            .reply(&api)
            .await;
        assert_eq!(resp.status(), 200);
    }
}
