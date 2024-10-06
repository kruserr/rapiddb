use rapiddb::traits::IDatabase;

#[tokio::main]
async fn main() {
  let mut db = rapiddb::db::memory_mapped_log_structured_hash_table::MemoryMappedLogStructuredHashTable::new();

  let key0 = "0";
  let key1 = "1";
  let key2 = "2";
  let value0 = b"3";
  let value1 = b"4";
  let value2 = b"5";

  db.post(key0, value0);
  db.post(key1, value1);
  db.post(key2, value2);

  let res0 = db.get(key0, 0);
  let res1 = db.get(key1, 0);
  let res2 = db.get(key2, 0);

  assert!(res0 == value0);
  assert!(res1 == value1);
  assert!(res2 == value2);

  // let db = rapiddb::db::MMAVAsyncDatabase::new();

  // warp::serve(rapiddb_web::api::endpoints(db)).run(([0, 0, 0, 0], 3030)).await;
}
