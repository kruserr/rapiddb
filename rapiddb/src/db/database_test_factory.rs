use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use crate::traits::IAsyncDatabase;
use crate::types::AggregateFn;

use super::MMAVAsyncDatabase;

/// Database test factory, stores a hashmap with all databases for
/// testing
///
/// ## Examples
/// ```no_run
/// # tokio_test::block_on(async {
///   let database_test_factory =
///   rapiddb::db::DatabaseTestFactory::new(".temp/test/api_endpoint/test_get");
///
///   for db in database_test_factory.get_instance().values() {
///     let value = b"{\"key\": \"value\"}";
///     db.write().await.post("test-0", value).await;
///     assert_eq!(db.write().await.get_latest("test-0").await, value);
///   }
/// # })
/// ```
pub struct DatabaseTestFactory {
  db_path: String,
  databases: std::collections::HashMap<
    String,
    std::sync::Arc<tokio::sync::RwLock<dyn IAsyncDatabase>>,
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
  /// # tokio_test::block_on(async {
  ///   let database_test_factory =
  ///   rapiddb::db::DatabaseTestFactory::new(".temp/test/api_endpoint/test_get");
  ///
  ///   for db in database_test_factory.get_instance().values() {
  ///     let value = b"{\"key\": \"value\"}";
  ///     db.write().await.post("test-0", value).await;
  ///     assert_eq!(db.write().await.get_latest("test-0").await, value);
  ///   }
  /// # })
  /// ```
  pub fn new(db_path: &str) -> Self {
    let mut databases: std::collections::HashMap<
      String,
      std::sync::Arc<tokio::sync::RwLock<dyn IAsyncDatabase>>,
    > = Default::default();

    // type AggregateFn = Arc<Mutex<dyn Fn(&str, &[u8], &Arc<Mutex<Vec<u8>>>) +
    // Send>>;

    let mut aggregates_fn: HashMap<String, AggregateFn> = Default::default();

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
            let mut temp_sum_count =
              aggregate_obj["temp_sum_count"].as_f64().unwrap_or_default();

            temp_sum += obj["temp"].as_f64().unwrap_or_default();
            temp_sum_count += 1.;
            let temp_avg = temp_sum / temp_sum_count;

            aggregate_obj["temp_sum"] = serde_json::json!(temp_sum);
            aggregate_obj["temp_sum_count"] = serde_json::json!(temp_sum_count);
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
      std::sync::Arc::new(tokio::sync::RwLock::new(
        MMAVAsyncDatabase::new_with_all(&mmav_db_path, aggregates_fn.clone()),
      )),
    );

    Self { db_path: db_path.to_string(), databases }
  }

  /// Get all databases for testing
  pub fn get_instance(
    &self,
  ) -> &std::collections::HashMap<
    String,
    std::sync::Arc<tokio::sync::RwLock<dyn IAsyncDatabase>>,
  > {
    &self.databases
  }
}
