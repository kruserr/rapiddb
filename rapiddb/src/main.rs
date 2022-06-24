#[tokio::main]
async fn main() {
  let db = std::sync::Arc::new(std::sync::RwLock::new(
    rapiddb::db::MMAVDatabase::new(),
  ));

  warp::serve(rapiddb::api::endpoints(db))
    .run(([0, 0, 0, 0], 3030))
    .await;
}
