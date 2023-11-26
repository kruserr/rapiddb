#[tokio::main]
async fn main() {
  let db = std::sync::Arc::new(tokio::sync::RwLock::new(
    rapiddb::db::MMAVAsyncDatabase::new(),
  ));

  warp::serve(rapiddb::api::endpoints(db)).run(([0, 0, 0, 0], 3030)).await;
}
