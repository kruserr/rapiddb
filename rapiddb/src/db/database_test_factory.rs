use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use crate::db::MMAVDatabase;
use crate::db::RedisMysqlDatabase;
use crate::traits::IDatabase;

static REDIS_COUNTER: std::sync::atomic::AtomicUsize =
  std::sync::atomic::AtomicUsize::new(1);

/// Database test factory, stores a hashmap with all databases for
/// testing
///
/// ## Examples
/// ```no_run
/// let database_test_factory =
/// rapiddb::db::DatabaseTestFactory::new(".temp/test/api_endpoint/test_get");
///
/// for db in database_test_factory.get_instance().values() {
///   let value = b"{\"key\": \"value\"}";
///   db.write().unwrap().post("test-0", value);
///   assert_eq!(db.write().unwrap().get_latest("test-0"), value);
/// }
/// ```
pub struct DatabaseTestFactory {
  db_path: String,
  databases: std::collections::HashMap<
    String,
    std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
  >,
}
impl Drop for DatabaseTestFactory {
  fn drop(&mut self) {
    for db_path in self.databases.keys() {
      std::fs::remove_dir_all(db_path).unwrap_or_default();
    }
    std::fs::remove_dir_all(&self.db_path).unwrap_or_default();
  }
}
impl DatabaseTestFactory {
  /// Database test factory Constructor
  ///
  /// ## Examples
  /// ```no_run
  /// let database_test_factory =
  /// rapiddb::db::DatabaseTestFactory::new(".temp/test/api_endpoint/test_get");
  ///
  /// for db in database_test_factory.get_instance().values() {
  ///   let value = b"{\"key\": \"value\"}";
  ///   db.write().unwrap().post("test-0", value);
  ///   assert_eq!(db.write().unwrap().get_latest("test-0"), value);
  /// }
  /// ```
  pub fn new(db_path: &str) -> Self {
    let mut databases: std::collections::HashMap<
      String,
      std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
    > = Default::default();

    let mut aggregates_fn: HashMap<
      String,
      Arc<Mutex<dyn Fn(&str, &[u8], &Arc<Mutex<Vec<u8>>>) + Send>>,
    > = Default::default();

    let test_fn = Arc::new(Mutex::new(
      |_: &str, value: &[u8], aggregate: &Arc<Mutex<Vec<u8>>>| {
        let obj = serde_json::from_slice::<serde_json::Value>(value)
          .unwrap_or_default();

        if obj["temp"].is_null() {
          return;
        }

        aggregate
          .lock()
          .map(|mut x| {
            let mut aggregate_obj =
              serde_json::from_slice::<serde_json::Value>(&x)
                .unwrap_or_default();

            let mut temp_sum =
              aggregate_obj["temp_sum"].as_f64().unwrap_or_default();
            let mut temp_sum_count = aggregate_obj["temp_sum_count"]
              .as_f64()
              .unwrap_or_default();

            temp_sum += obj["temp"].as_f64().unwrap_or_default();
            temp_sum_count += 1.;
            let temp_avg = temp_sum / temp_sum_count;

            aggregate_obj["temp_sum"] = serde_json::json!(temp_sum);
            aggregate_obj["temp_sum_count"] =
              serde_json::json!(temp_sum_count);
            aggregate_obj["temp_avg"] = serde_json::json!(temp_avg);

            *x = aggregate_obj.to_string().as_bytes().to_vec();
          })
          .err();
      },
    ));

    aggregates_fn.insert("test-0".to_string(), test_fn.clone());
    aggregates_fn.insert("test-1".to_string(), test_fn);

    let mmav_db_path = format!("{db_path}_mmav");
    databases.insert(
      mmav_db_path.clone(),
      std::sync::Arc::new(std::sync::RwLock::new(
        MMAVDatabase::new_with_all(
          &mmav_db_path,
          aggregates_fn.clone(),
        ),
      )),
    );

    std::env::var("TEST_RM")
      .map(|var| {
        if var != "true" {
          return;
        }

        let rm_db_path = format!("{db_path}_rm");
        databases.insert(
          rm_db_path,
          std::sync::Arc::new(std::sync::RwLock::new(
            RedisMysqlDatabase::new_with_all(
              &REDIS_COUNTER
                .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
                .to_string(),
              aggregates_fn,
            ),
          )),
        );
      })
      .err();

    Self { db_path: db_path.to_string(), databases }
  }

  /// Get all databases for testing
  pub fn get_instance(
    &self,
  ) -> &std::collections::HashMap<
    String,
    std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
  > {
    &self.databases
  }
}
