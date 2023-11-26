#![doc(
  html_logo_url = "https://raw.githubusercontent.com/kruserr/rapiddb/main/assets/logo/logo.svg",
  html_favicon_url = "https://raw.githubusercontent.com/kruserr/rapiddb/main/assets/logo/favicon.ico"
)]

//! <p align="center">
//!   <a href="https://github.com/kruserr/rapiddb" target="_blank">
//!     <img width="300" src="https://raw.githubusercontent.com/kruserr/rapiddb/main/assets/logo/logo.svg">
//!   </a>
//!   <br/>
//!   <br/>
//!   <a href="https://github.com/kruserr/rapiddb/releases" target="_blank">
//!     <img src="https://img.shields.io/github/v/release/kruserr/rapiddb?sort=semver&logo=GitHub&logoColor=white">
//!   </a>
//!   <a href="https://crates.io/crates/rapiddb" target="_blank">
//!     <img src="https://img.shields.io/crates/v/rapiddb?logo=Rust&logoColor=white"/>
//!   </a>
//!   <br/>
//!   <a href="https://hub.docker.com/r/kruserr/rapiddb" target="_blank">
//!     <img src="https://img.shields.io/docker/v/kruserr/rapiddb?sort=semver&logo=docker&logoColor=white"/>
//!   </a>
//!   <a href="https://codecov.io/gh/kruserr/rapiddb" target="_blank">
//!     <img src="https://img.shields.io/codecov/c/gh/kruserr/rapiddb?logo=Codecov&logoColor=white"/>
//!   </a>
//! </p>
//!
//! # RapidDB
//! A reasonably fast configurable embedded key-value sensor database
//!
//! ## Features
//! - Simple and flexible optional embedded REST API
//! - Simple key-value database interface
//! - Lightweight embedded database
//! - Store sensor data inside a sensor database
//! - Memory first with on-disk persistence
//! - Memory Mapped Append-only Vector backing storage
//! - Bring your own database or API implementation
//!
//! ## Documentation
//! Visit the [Documentation](https://docs.rs/rapiddb).
//!
//! ## Getting Started
//! ### Docker
//! Run database with docker
//! ```bash
//! docker run -dit --rm -p 3030:3030 --name rapiddb kruserr/rapiddb:0.1
//! ```
//!
//! ### Git and cargo
//! Clone the repo and build the database from source
//! ```bash
//! git clone https://github.com/kruserr/rapiddb.git
//! cd rapiddb
//! cargo run --release
//! ```
//!
//! ### Add to your cargo project
//! Add the following to your dependencies in Cargo.toml
//! ```toml
//! tokio = { version = "1", features = ["full"] }
//! warp = "0.3"
//! rapiddb = "0.1"
//! ```
//!
//! Paste the following to your main.rs
//! ```ignore
//! #[tokio::main]
//! async fn main() {
//!   let db = std::sync::Arc::new(tokio::sync::RwLock::new(
//!     rapiddb::db::MMAVAsyncDatabase::new(),
//!   ));
//!
//!   warp::serve(rapiddb::api::endpoints(db)).run(([0, 0, 0, 0], 3030)).await;
//! }
//! ```
//!
//! Run the database with cargo
//! ```bash
//! cargo run --release
//! ```
//!
//! ### Interact with the database using curl
//! Write to database with curl
//! ```bash
//! curl -X POST localhost:3030/api/v0/test-0 -H 'Content-Type: application/json' -d '{"temp":4.00}'
//! ```
//!
//! Read from database with curl
//! ```bash
//! curl localhost:3030/api/v0/test-0/latest
//! ```
//!
//! Explore the API with curl
//! ```bash
//! curl localhost:3030/api/v0
//! curl localhost:3030/api/v0/sensors
//! curl localhost:3030/api/v0/test-0
//! ```
//!
//! ### Explore and customize the database
//! The database is highly customizable, if you use the database inside
//! your cargo project. You can interact with the `db` object, and
//! explore the `IDatabase` interface. You can also use `warp::Filter`
//! to extend the API. You can also implement the `IDatabase` interface
//! yourself, for your own database. Explore the docs to learn more, or
//! look at the examples below, or inside the repo.
//!
//! ## Examples
//! Using the database directly
//! ```ignore
//! use rapiddb::traits::IDatabase;
//!
//! let db = std::sync::Arc::new(
//!   std::sync::RwLock::new(
//!     rapiddb::db::MMAVDatabase::new()
//!   )
//! );
//!
//! let value = b"{\"key\": \"value\"}";
//! db.write().unwrap().post("test-0", value);
//! assert_eq!(db.write().unwrap().get_latest("test-0"), value);
//! ```
//!
//! Extending the functionality of the REST API with custom endpoints
//! using warp Filters and custom aggregates
//! ```ignore
//! use std::{
//!   collections::HashMap,
//!   sync::{Arc, Mutex},
//! };
//! use tokio::sync::RwLock;
//!
//! #[tokio::main]
//! async fn main() {
//!   let mut aggregates_fn: HashMap<String, rapiddb::types::AggregateFn> =
//!     Default::default();
//!
//!   let test_fn = Arc::new(Mutex::new(
//!     |_: &str, value: &[u8], aggregate: &Arc<Mutex<Vec<u8>>>| {
//!       let obj =
//!         serde_json::from_slice::<serde_json::Value>(value).unwrap_or_default();
//!
//!       if obj["temp"].is_null() {
//!         return;
//!       }
//!
//!       aggregate
//!         .lock()
//!         .map(|mut x| {
//!           let mut aggregate_obj =
//!             serde_json::from_slice::<serde_json::Value>(&x).unwrap_or_default();
//!
//!           let mut temp_sum =
//!             aggregate_obj["temp_sum"].as_f64().unwrap_or_default();
//!           let mut temp_sum_count =
//!             aggregate_obj["temp_sum_count"].as_f64().unwrap_or_default();
//!
//!           temp_sum += obj["temp"].as_f64().unwrap_or_default();
//!           temp_sum_count += 1.;
//!           let temp_avg = temp_sum / temp_sum_count;
//!
//!           aggregate_obj["temp_sum"] = serde_json::json!(temp_sum);
//!           aggregate_obj["temp_sum_count"] = serde_json::json!(temp_sum_count);
//!           aggregate_obj["temp_avg"] = serde_json::json!(temp_avg);
//!
//!           *x = aggregate_obj.to_string().as_bytes().to_vec();
//!         })
//!         .err();
//!     },
//!   ));
//!
//!   aggregates_fn.insert("test-0".to_string(), test_fn.clone());
//!   aggregates_fn.insert("test-1".to_string(), test_fn);
//!
//!   let db = Arc::new(RwLock::new(rapiddb::db::MMAVAsyncDatabase::new_with_all(
//!     ".db",
//!     aggregates_fn,
//!   )));
//!
//!   warp::serve(rapiddb::api::endpoints(db)).run(([0, 0, 0, 0], 3030)).await;
//! }
//! ```

pub mod db;
pub mod errors;
pub mod traits;
pub mod types;
