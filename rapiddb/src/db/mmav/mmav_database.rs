use std::collections::HashMap;
use std::io::{Read, Write};

use crate::db::mmav::mmav::MMAV;
use crate::traits::IDatabase;
use crate::types::AggregateFn;

/// Memory Mapped Append-only Vector Database
///
/// This is the database abstraction, it futures a further
/// abstraction on the MMAV which allows using multiple MMAVs.
/// Each sensor is stored in a MMAV.
/// And enables meta data and index data to be stored for each sensor.
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
pub struct MMAVDatabase {
  db_path: String,
  sensors: std::collections::HashMap<String, MMAV>,
  meta: std::collections::HashMap<String, Vec<u8>>,
  aggregates: std::collections::HashMap<
    String,
    std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
  >,
  aggregates_fn: HashMap<String, AggregateFn>,
}
impl MMAVDatabase {
  /// Memory Mapped Append-only Vector Database Constructor
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
  pub fn new() -> Self {
    Self::new_with_all(".db", Default::default())
  }

  /// Memory Mapped Append-only Vector Database Constructor with all
  ///
  /// ## Panics
  /// if invalid `db_path` is provided
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::MMAVDatabase::new_with_all(".temp/my_path/", Default::default())
  ///   )
  /// );
  ///
  /// let value = b"{\"key\": \"value\"}";
  /// db.write().unwrap().post("test-0", value);
  /// assert_eq!(db.write().unwrap().get_latest("test-0"), value);
  /// ```
  pub fn new_with_all(
    db_path: &str,
    aggregates_fn: HashMap<String, AggregateFn>,
  ) -> Self {
    let mut sensors: std::collections::HashMap<String, MMAV> =
      Default::default();
    let mut meta: std::collections::HashMap<String, Vec<u8>> =
      Default::default();

    let paths = std::fs::read_dir(db_path).unwrap_or_else(|_| {
      std::fs::create_dir_all(db_path).unwrap_or_default();
      std::fs::read_dir(db_path).unwrap()
    });

    for path in paths {
      path
        .unwrap()
        .file_name()
        .into_string()
        .unwrap_or_default()
        .parse::<String>()
        .map(|x| {
          sensors.insert(x.clone(), MMAV::new(&format!("{db_path}/{x}")));

          let mut data = vec![];
          let file_name = format!("{db_path}/{x}/meta");

          let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&file_name)
            .unwrap_or_else(|error| {
              if error.kind() == std::io::ErrorKind::NotFound {
                std::fs::create_dir_all(
                  file_name
                    .split('/')
                    .collect::<Vec<_>>()
                    .split_last()
                    .unwrap()
                    .1
                    .join("/"),
                )
                .unwrap_or_default();
              }

              std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(file_name)
                .unwrap()
            });

          file.read_to_end(&mut data).unwrap_or_default();

          meta.insert(x, data);
        })
        .unwrap_or_default();
    }

    Self {
      db_path: db_path.to_owned(),
      sensors,
      meta,
      aggregates: Default::default(),
      aggregates_fn,
    }
  }
}

impl Default for MMAVDatabase {
  fn default() -> Self {
    Self::new()
  }
}
impl IDatabase for MMAVDatabase {
  fn contains(&self, id: &str) -> bool {
    self.sensors.get(id).is_some()
  }

  fn get(&mut self, id: &str, rec_id: usize) -> Vec<u8> {
    if !self.contains(id) {
      return Default::default();
    }

    self.sensors.get_mut(id).unwrap().get(rec_id)
  }

  fn post(&mut self, id: &str, value: &[u8]) {
    if !self.contains(id) {
      self
        .sensors
        .insert(id.to_owned(), MMAV::new(&format!("{}/{id}", self.db_path)));
    }

    if !self.aggregates.contains_key(id) {
      self.aggregates.insert(
        id.to_string(),
        std::sync::Arc::new(std::sync::Mutex::new(
          serde_json::json!({}).to_string().as_bytes().to_owned(),
        )),
      );
    }

    if let Some(aggregate) = self.aggregates.get(id) {
      if let Some(x) = self.aggregates_fn.get(id) {
        x.lock().map(|f| f(id, value, aggregate)).err();
      }
    }

    self.sensors.get_mut(id).unwrap().push(value);
  }

  fn get_meta(&mut self, id: &str) -> Vec<u8> {
    if !self.contains(id) {
      return Default::default();
    }

    match self.meta.get(id) {
      Some(x) => x.clone(),
      None => Default::default(),
    }
  }

  fn post_meta(&mut self, id: &str, data: Vec<u8>) {
    if !self.contains(id) {
      self
        .sensors
        .insert(id.to_owned(), MMAV::new(&format!("{}/{id}", self.db_path)));
    }

    let file_name = format!("{}/{id}/meta", self.db_path);

    let mut file = std::fs::OpenOptions::new()
      .read(true)
      .write(true)
      .create(true)
      .open(&file_name)
      .unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
          std::fs::create_dir_all(
            file_name
              .split('/')
              .collect::<Vec<_>>()
              .split_last()
              .unwrap()
              .1
              .join("/"),
          )
          .unwrap_or_default();
        }

        std::fs::OpenOptions::new()
          .read(true)
          .write(true)
          .create(true)
          .open(file_name)
          .unwrap()
      });

    file.write_all(&data).unwrap_or_default();
    self.meta.insert(id.to_owned(), data);
  }

  fn get_aggregates(&self, id: &str) -> Vec<u8> {
    if self.aggregates.get(id).is_none() {
      return Default::default();
    }

    self.aggregates[id].lock().unwrap().clone()
  }

  fn get_latest(&mut self, id: &str) -> Vec<u8> {
    if !self.contains(id) {
      return Default::default();
    }

    self.sensors.get(id).unwrap().last()
  }

  fn get_latest_with_limit(&mut self, id: &str, limit: usize) -> Vec<Vec<u8>> {
    if !self.contains(id) {
      return Default::default();
    }

    self.sensors.get_mut(id).unwrap().last_limit(limit)
  }

  fn get_range(&mut self, id: &str, start: usize, end: usize) -> Vec<Vec<u8>> {
    if !self.contains(id) {
      return Default::default();
    }

    self.sensors.get_mut(id).unwrap().range(start, end)
  }

  fn get_all_meta(&mut self) -> std::collections::HashMap<&str, Vec<u8>> {
    let mut result: std::collections::HashMap<&str, Vec<u8>> =
      Default::default();

    for (id, value) in &self.meta {
      result.insert(id, value.clone());
    }

    result
  }

  fn get_all_aggregates(&self) -> std::collections::HashMap<&str, Vec<u8>> {
    let mut result: std::collections::HashMap<&str, Vec<u8>> =
      Default::default();

    for (key, value) in &self.aggregates {
      result.insert(key, value.lock().unwrap().clone());
    }

    result
  }

  fn get_all_latest(&mut self) -> std::collections::HashMap<&str, Vec<u8>> {
    let mut result: std::collections::HashMap<&str, Vec<u8>> =
      Default::default();

    for (id, sensor) in &mut self.sensors {
      result.insert(id, sensor.last());
    }

    result
  }

  fn get_all_latest_with_limit(
    &mut self,
    limit: usize,
  ) -> std::collections::HashMap<&str, Vec<Vec<u8>>> {
    let mut result: std::collections::HashMap<&str, Vec<Vec<u8>>> =
      Default::default();

    for (id, sensor) in &mut self.sensors {
      let item = sensor.last_limit(limit);
      if !item.is_empty() {
        result.insert(id, item);
      }
    }

    result
  }
}
