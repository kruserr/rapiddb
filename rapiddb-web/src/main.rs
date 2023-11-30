// #[tokio::main]
// async fn main() {
//   let db = rapiddb::db::MMAVAsyncDatabase::new();

//   warp::serve(rapiddb_web::api::endpoints(db)).run(([0, 0, 0, 0], 3030)).await;
// }

use rapiddb::traits::IDatabase;
fn main() {
  let mut db = rapiddb::db::MmapDatabase::new("test.db").unwrap();

  let value = b"{\"key-2\": \"value-2\"}";
  // db.post("test-1", value);
  println!("{:?}", String::from_utf8(db.get_latest("test-0")));
  println!("{:?}", String::from_utf8(db.get_latest("test-1")));

  println!("{:?}", String::from_utf8(db.get("test-1", 0)));
  println!("{:?}", String::from_utf8(db.get("test-1", 1)));
  println!("{:?}", String::from_utf8(db.get("test-0", 0)));
}
