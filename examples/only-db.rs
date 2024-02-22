use rapiddb::traits::IDatabase;

pub fn main() {
  let mut db = rapiddb::db::MMAVDatabase::new();

  let value = b"{\"key\": \"value\"}";
  db.post("test-0", value);
  assert_eq!(db.get_latest("test-0"), value);
}
