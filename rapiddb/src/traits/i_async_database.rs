/// IDatabase trait abstracts the underlying Database implementation
///
/// ## Examples
/// ```no_run
/// use crate::rapiddb::traits::IDatabase;
///
/// let db = std::sync::Arc::new(
///   std::sync::RwLock::new(
///     rapiddb::db::MMAVDatabase::new()
///   )
/// );
///
/// let value = b"{\"key\": \"value\"}";
/// db.write().unwrap().post("test-0", value);
/// assert_eq!(db.write().unwrap().get_latest("test-0"), value);
/// ```
#[async_trait::async_trait]
pub trait IAsyncDatabase: Send + Sync {
  /// Checks if the sensor with `id` exists in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// let id = "test-0";
  ///
  /// if db.read().unwrap().contains(id) {
  ///     println!("{id} exists in the Database");
  /// }
  /// ```
  async fn contains(&self, id: &str) -> bool;

  /// Get the record with `rec_id` from the sensor with `id` in the
  /// Database
  ///
  /// May load data from disk, if it is not in-memory,
  /// as such it is mutable, even though a get normaly
  /// is immutable.
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get("test-0", 0);
  /// ```
  async fn get(&mut self, id: &str, rec_id: usize) -> Vec<u8>;

  /// Post a record with `value` to the sensor with `id` in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().post(
  ///   "test-0",
  ///   b"{\"key\": \"value\"}"
  /// );
  /// ```
  async fn post(&mut self, id: &str, value: &[u8]);

  /// Get metadata from the sensor with `id` in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get_meta("test-0");
  /// ```
  async fn get_meta(&mut self, id: &str) -> Vec<u8>;

  /// Post metadata with `value` to the sensor with `id` in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().post_meta(
  ///   "test-0",
  ///   b"{\"key\": \"value\"}".to_vec()
  /// );
  /// ```
  async fn post_meta(&mut self, id: &str, value: Vec<u8>);

  /// Get aggregates from the sensor with `id` in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.read().unwrap().get_aggregates("test-0");
  /// ```
  async fn get_aggregates(&self, id: &str) -> Vec<u8>;

  /// Get the latest record from the sensor with `id` in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get_latest("test-0");
  /// ```
  // fn get_latest(&mut self, id: &str) -> Vec<u8>;
  async fn get_latest(&mut self, id: &str) -> Vec<u8>;

  /// Get the latest `limit` number of records from the sensor with `id`
  /// in the Database
  ///
  /// May load data from disk, if it is not in-memory,
  /// as such it is mutable, even though a get latest limit normaly is
  /// immutable.
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get_latest_with_limit("test-0", 10);
  /// ```
  async fn get_latest_with_limit(
    &mut self,
    id: &str,
    limit: usize,
  ) -> Vec<Vec<u8>>;

  /// Get a range from `start` to `end` of records from the sensor with
  /// `id` in the Database
  ///
  /// May load data from disk, if it is not in-memory,
  /// as such it is mutable, even though a range scan normaly is
  /// immutable.
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get_range("test-0", 0, 10);
  /// ```
  async fn get_range(
    &mut self,
    id: &str,
    start: usize,
    end: usize,
  ) -> Vec<Vec<u8>>;

  /// Get metadata from all sensors in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get_all_meta();
  /// ```
  async fn get_all_meta(&mut self) -> std::collections::HashMap<&str, Vec<u8>>;

  /// Get aggregates from all sensors in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.read().unwrap().get_all_aggregates();
  /// ```
  async fn get_all_aggregates(
    &self,
  ) -> std::collections::HashMap<&str, Vec<u8>>;

  /// Get the latest record from all sensors in the Database
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get_all_latest();
  /// ```
  async fn get_all_latest(
    &mut self,
  ) -> std::collections::HashMap<&str, Vec<u8>>;

  /// Get the latest `limit` number of records from all sensors in the
  /// Database
  ///
  /// May load data from disk, if it is not in-memory,
  /// as such it is mutable, even though a get latest limit normaly is
  /// immutable.
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new()
  ///   )
  /// );
  ///
  /// db.write().unwrap().get_all_latest_with_limit(10);
  /// ```
  async fn get_all_latest_with_limit(
    &mut self,
    limit: usize,
  ) -> std::collections::HashMap<&str, Vec<Vec<u8>>>;
}
