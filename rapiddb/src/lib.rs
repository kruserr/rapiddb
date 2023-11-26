#![doc(
  html_logo_url = "https://raw.githubusercontent.com/kruserr/rapiddb/main/assets/logo/logo.svg",
  html_favicon_url = "https://raw.githubusercontent.com/kruserr/rapiddb/main/assets/logo/favicon.ico"
)]
#![allow(clippy::needless_doctest_main)]

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
//! - Simple key-value database interface
//! - Simple and flexible optional embedded REST API
//! - Memory first with on-disk persistence
//! - Lightweight embedded database
//! - Memory Mapped Append-only Vector backing storage
//! - Bring your own database or API implementation
//! - Store sensor data inside a sensor database
//!
//! ## Documentation
//! Visit the [Documentation](https://docs.rs/rapiddb).
//!
//! ## Optional REST API
//! Visit the [rapiddb-web crates.io page](https://crates.io/crates/rapiddb-web).
//!
//! ## Getting started
//! Cargo.toml
//! ```toml
//! [dependencies]
//! rapiddb = "0.1"
//! ```
//!
//! src/main.rs
//! ```rust
//! use rapiddb::traits::IDatabase;
//!
//! pub fn main() {
//!   let mut db = rapiddb::db::MMAVDatabase::new();
//!
//!   let value = b"{\"key\": \"value\"}";
//!   db.post("test-0", value);
//!   assert_eq!(db.get_latest("test-0"), value);
//! }
//! ```
//!
//! Run the database with cargo
//! ```sh
//! cargo run --release
//! ```
//!
//! ## Documentation
//! Visit the [Documentation](https://docs.rs/rapiddb).
//!
//! ## Examples
//! Visit the [Examples](https://github.com/kruserr/rapiddb/tree/main/examples).

pub mod db;
pub mod errors;
pub mod traits;
pub mod types;
