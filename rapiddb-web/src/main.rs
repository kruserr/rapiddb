// #[tokio::main]
// async fn main() {
//   let db = rapiddb::db::MMAVAsyncDatabase::new();

//   warp::serve(rapiddb_web::api::endpoints(db)).run(([0, 0, 0, 0], 3030)).await;
// }

use std::vec;

use rapiddb::traits::IDatabase;
use rapiddb::db::Database;
use rapiddb::db::Record;
fn main() {
  // let mut db = rapiddb::db::MmapDatabase::new("test.db").unwrap();

  // let value = b"{\"key-2\": \"value-2\"}";
  // // db.post("test-1", value);
  // println!("{:?}", String::from_utf8(db.get_latest("test-0")));
  // println!("{:?}", String::from_utf8(db.get_latest("test-1")));

  // println!("{:?}", String::from_utf8(db.get("test-1", 0)));
  // println!("{:?}", String::from_utf8(db.get("test-1", 1)));
  // println!("{:?}", String::from_utf8(db.get("test-0", 0)));

  // let mut db = Database::new("test-0.db").unwrap();

  // let key0 = "test";
  // let value0 = vec![1, 2, 3];

  // db.put(Record { id: key0.to_owned(), value: value0.clone()});

  // assert_eq!(db.get(key0).unwrap().value, value0);

  let mut db = Database::new("test-0.db").unwrap();

  let key0 = "test";
  let value0 = vec![1, 2, 3];

  db.put(Record { id: key0.to_owned(), value: value0.clone()});

  match db.get(key0) {
    Some(record) => assert_eq!(record.value, value0),
    None => println!("Record not found"),
  }
}
