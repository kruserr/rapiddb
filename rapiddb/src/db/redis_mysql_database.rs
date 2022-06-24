use std::{
  collections::HashMap,
  sync::{Arc, Mutex},
};

use crate::traits::IDatabase;

use mysql::prelude::Queryable;
use redis::Commands;

#[derive(Debug)]
pub struct Sensor {
  key: String,
  value: usize,
}

/// Redis Mysql Database
///
/// This is a database abstraction that allows interfacing with both
/// Redis for memory, and mysql for disk storage.
///
/// ## Examples
/// ```no_run
/// use crate::rapiddb::traits::IDatabase;
///
/// let db = std::sync::Arc::new(
///   std::sync::RwLock::new(
///     rapiddb::db::RedisMysqlDatabase::new()
///   )
/// );
///
/// let value = b"{\"key\": \"value\"}";
/// db.write().unwrap().post("test-0", value);
/// assert_eq!(db.write().unwrap().get_latest("test-0"), value);
/// ```
pub struct RedisMysqlDatabase {
  sensors: std::collections::HashMap<String, usize>,
  db_path: String,
  redis_con: redis::Connection,
  mysql_con: std::sync::Arc<std::sync::RwLock<mysql::PooledConn>>,
  aggregates: std::collections::HashMap<
    String,
    std::sync::Arc<std::sync::Mutex<Vec<u8>>>,
  >,
  aggregates_fn: HashMap<
    String,
    Arc<Mutex<dyn Fn(&str, &[u8], &Arc<Mutex<Vec<u8>>>) + Send>>,
  >,
}
impl Drop for RedisMysqlDatabase {
  fn drop(&mut self) {
    if self.db_path != "0" {
      let _: () =
        redis::cmd("FLUSHDB").query(&mut self.redis_con).unwrap();
      self
        .mysql_con
        .write()
        .unwrap()
        .query_drop(format!(r"DROP DATABASE rapiddb{}", self.db_path))
        .err();
    }
  }
}
impl RedisMysqlDatabase {
  /// Redis Mysql Database Constructor
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::RedisMysqlDatabase::new()
  ///   )
  /// );
  ///
  /// let value = b"{\"key\": \"value\"}";
  /// db.write().unwrap().post("test-0", value);
  /// assert_eq!(db.write().unwrap().get_latest("test-0"), value);
  /// ```
  pub fn new() -> Self {
    Self::new_with_all("0", Default::default())
  }

  /// Redis Mysql Database Constructor with `db_path`
  ///
  /// ## Examples
  /// ```no_run
  /// use crate::rapiddb::traits::IDatabase;
  ///
  /// let db = std::sync::Arc::new(
  ///   std::sync::RwLock::new(
  ///     rapiddb::db::RedisMysqlDatabase::new()
  ///   )
  /// );
  ///
  /// let value = b"{\"key\": \"value\"}";
  /// db.write().unwrap().post("test-0", value);
  /// assert_eq!(db.write().unwrap().get_latest("test-0"), value);
  /// ```
  pub fn new_with_all(
    db_path: &str,
    aggregates_fn: HashMap<
      String,
      Arc<Mutex<dyn Fn(&str, &[u8], &Arc<Mutex<Vec<u8>>>) + Send>>,
    >,
  ) -> Self {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut redis_con = client.get_connection().unwrap();

    let _: () = redis::cmd("CLIENT")
      .arg("SETNAME")
      .arg(db_path)
      .query(&mut redis_con)
      .unwrap();

    let _: () =
      redis::cmd("SELECT").arg(db_path).query(&mut redis_con).unwrap();

    let url = "mysql://root@localhost:3306";
    let pool =
      mysql::Pool::new(mysql::Opts::from_url(url).unwrap()).unwrap();
    let mut mysql_con = pool.get_conn().unwrap();

    mysql_con
      .query_drop(format!(
        r"CREATE DATABASE IF NOT EXISTS rapiddb{db_path}"
      ))
      .err();

    mysql_con
      .query_drop(format!(
        r"CREATE TABLE rapiddb{db_path}.sensors (
            id varchar(255) NOT NULL,
            rec_id int NOT NULL,
            value blob NOT NULL,
            PRIMARY KEY (id, rec_id)
        )"
      ))
      .err();

    let mut sensors: std::collections::HashMap<String, usize> =
      Default::default();

    // GET ALL LATEST
    let result_2 = mysql_con
      .query_map(
        format!(
          r"SELECT t0.id, t0.rec_id
                    FROM rapiddb{db_path}.sensors t0
                    INNER JOIN (
                        SELECT id, MAX(rec_id) rec_id
                        FROM rapiddb{db_path}.sensors
                        GROUP BY id
                    ) t1
                    ON t0.id = t1.id AND
                    t0.rec_id = t1.rec_id"
        ),
        |(id, rec_id)| Sensor { key: id, value: rec_id },
      )
      .unwrap_or_default();

    for item in result_2 {
      sensors.insert(item.key, item.value + 1);
    }

    Self {
      redis_con,
      mysql_con: std::sync::Arc::new(std::sync::RwLock::new(mysql_con)),
      db_path: db_path.to_string(),
      sensors,
      aggregates: Default::default(),
      aggregates_fn,
    }
  }
}

impl Default for RedisMysqlDatabase {
  fn default() -> Self {
    Self::new()
  }
}

impl IDatabase for RedisMysqlDatabase {
  fn contains(&self, id: &str) -> bool {
    self.sensors.get(id).is_some()
  }

  fn get(&mut self, id: &str, rec_id: usize) -> Vec<u8> {
    if !self.contains(id) {
      return Default::default();
    }

    if *self.sensors.get(id).unwrap() == 0
      || rec_id > *self.sensors.get(id).unwrap() - 1
    {
      return Default::default();
    }

    self.redis_con.get(format!("{id}:{}", rec_id)).unwrap()
  }

  fn post(&mut self, id: &str, value: &[u8]) {
    if !self.contains(id) {
      self.sensors.insert(id.to_string(), 0);
    }

    if !self.aggregates.contains_key(id) {
      self.aggregates.insert(
        id.to_string(),
        std::sync::Arc::new(std::sync::Mutex::new(
          serde_json::json!({}).to_string().as_bytes().to_owned(),
        )),
      );
    }

    self.aggregates.get(id).map(|aggregate| {
      self.aggregates_fn.get(id).map(|x| {
        x.lock().map(|f| f(id, value, aggregate)).err();
      });
    });

    let rec_id = self.sensors.get_mut(id).unwrap();

    let _: () = self
      .redis_con
      .set(format!("{id}:{}", *rec_id % 10000), value)
      .unwrap();

    let id_ref = std::sync::Arc::new(id.to_owned());
    let rec_id_ref = std::sync::Arc::new(*rec_id);
    let value_ref = std::sync::Arc::new(value.to_owned());
    let mysql_con_ref = self.mysql_con.clone();
    let db_path_ref = self.db_path.clone();

    tokio::task::spawn_blocking(move || {
      mysql_con_ref
        .write()
        .unwrap()
        .exec_drop(
          format!(
            r"INSERT INTO rapiddb{}.sensors (id, rec_id, value) VALUES (?, ?, ?)",
            db_path_ref
          ),
          (&*id_ref, *rec_id_ref, &*value_ref),
        )
        .unwrap();
    });

    *rec_id += 1;
  }

  fn get_meta(&mut self, id: &str) -> Vec<u8> {
    if !self.contains(id) {
      return Default::default();
    }

    self.redis_con.get(format!("{id}:meta")).unwrap()
  }

  fn post_meta(&mut self, id: &str, value: Vec<u8>) {
    if !self.contains(id) {
      self.sensors.insert(id.to_string(), 0);
    }

    let _: () =
      self.redis_con.set(format!("{id}:meta"), value).unwrap();
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

    if *self.sensors.get(id).unwrap() == 0 {
      return Default::default();
    }

    self
      .redis_con
      .get(format!(
        "{id}:{}",
        (*self.sensors.get(id).unwrap() - 1) % 10000
      ))
      .unwrap()
  }

  fn get_latest_with_limit(
    &mut self,
    id: &str,
    limit: usize,
  ) -> Vec<Vec<u8>> {
    if !self.contains(id) {
      return Default::default();
    }

    if *self.sensors.get(id).unwrap() == 0 || limit == 0 {
      return Default::default();
    }

    let start = (|| {
      if limit < *self.sensors.get(id).unwrap() {
        return *self.sensors.get(id).unwrap() - limit;
      }
      0
    })();

    let end = *self.sensors.get(id).unwrap();

    let mut query_vec = vec![];
    for i in start..end {
      query_vec.push(format!("{id}:{i}"));
    }

    redis::cmd("MGET")
      .arg(query_vec)
      .query(&mut self.redis_con)
      .unwrap()
  }

  fn get_range(
    &mut self,
    id: &str,
    start: usize,
    end: usize,
  ) -> Vec<Vec<u8>> {
    if !self.contains(id) {
      return Default::default();
    }

    let mut mut_end = end;

    if end > *self.sensors.get(id).unwrap() {
      mut_end = *self.sensors.get(id).unwrap();
    }

    let mut query_vec = vec![];

    if start == mut_end {
      query_vec.push(format!("{id}:{start}"));
    }

    for i in start..mut_end {
      query_vec.push(format!("{id}:{i}"));
    }

    let temp = redis::cmd("MGET")
      .arg(query_vec)
      .query(&mut self.redis_con)
      .unwrap_or_default();

    temp
  }

  fn get_all_meta(
    &mut self,
  ) -> std::collections::HashMap<&str, Vec<u8>> {
    let mut result: std::collections::HashMap<&str, Vec<u8>> =
      Default::default();

    for id in self.sensors.keys() {
      result
        .insert(id, self.redis_con.get(format!("{id}:meta")).unwrap());
    }

    result
  }

  fn get_all_aggregates(
    &self,
  ) -> std::collections::HashMap<&str, Vec<u8>> {
    let mut result: std::collections::HashMap<&str, Vec<u8>> =
      Default::default();

    for (key, value) in self.aggregates.iter() {
      result.insert(key, value.lock().unwrap().clone());
    }

    result
  }

  fn get_all_latest(
    &mut self,
  ) -> std::collections::HashMap<&str, Vec<u8>> {
    let mut result: std::collections::HashMap<&str, Vec<u8>> =
      Default::default();

    for (id, value) in self.sensors.iter() {
      let mut temp_value = *value;
      if temp_value > 0 {
        temp_value -= 1;
      }
      result.insert(
        id,
        self.redis_con.get(format!("{id}:{temp_value}")).unwrap(),
      );
    }

    result
  }

  fn get_all_latest_with_limit(
    &mut self,
    limit: usize,
  ) -> std::collections::HashMap<&str, Vec<Vec<u8>>> {
    if limit == 0 {
      return Default::default();
    }

    let mut result: std::collections::HashMap<&str, Vec<Vec<u8>>> =
      Default::default();

    for (id, value) in self.sensors.iter() {
      if *value == 0 {
        continue;
      }

      let start = (|| {
        if limit < *value {
          return *value - limit;
        }
        0
      })();

      let end = *value;

      let mut query_vec = vec![];
      for i in start..end {
        query_vec.push(format!("{id}:{i}"));
      }

      let query_result: Vec<Vec<u8>> = redis::cmd("MGET")
        .arg(query_vec)
        .query(&mut self.redis_con)
        .unwrap();

      result.insert(id, query_result);
    }

    result
  }
}
