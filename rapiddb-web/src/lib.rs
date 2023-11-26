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
//! - Lightweight embedded database
//! - Simple key-value database interface
//! - Simple and flexible optional embedded REST API
//! - Memory first with on-disk persistence
//! - Memory Mapped Append-only Vector backing storage
//! - Bring your own database or API implementation
//! - Store sensor data inside a sensor database
//!
//! ## Getting Started
//! ### Docker
//! Run database with docker
//! ```bash
//! docker run -dit --rm -p 3030:3030 --name rapiddb kruserr/rapiddb:0.1
//! ```
//!
//! [Further install options](docs/install-README.md)
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
//! look at the examples.
//!
//! ## Documentation
//! Visit the [Documentation](https://docs.rs/rapiddb-web).
//!
//! ## Examples
//! Visit the [Examples](https://github.com/kruserr/rapiddb/tree/main/examples).

pub mod api;
pub use rapiddb;
