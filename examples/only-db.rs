use rapiddb::traits::IDatabase;

pub fn main() {
  let db = std::sync::Arc::new(
    std::sync::RwLock::new(
      rapiddb::db::MMAVDatabase::new()
    )
  );

  let value = b"{\"key\": \"value\"}";
  db.write().unwrap().post("test-0", value);
  assert_eq!(db.write().unwrap().get_latest("test-0"), value);
}
