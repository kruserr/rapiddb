<p align="center">
  <a href="https://github.com/kruserr/rapiddb" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/rapiddb/main/assets/logo/logo.svg">
  </a>
  <br/>
  <br/>
  <a href="https://github.com/kruserr/rapiddb/releases" target="_blank">
    <img src="https://img.shields.io/github/v/release/kruserr/rapiddb?sort=semver&logo=GitHub&logoColor=white">
  </a>
  <a href="https://crates.io/crates/rapiddb" target="_blank">
    <img src="https://img.shields.io/crates/v/rapiddb?logo=Rust&logoColor=white"/> 
  </a>
  <br/>
  <a href="https://hub.docker.com/r/kruserr/rapiddb" target="_blank">
    <img src="https://img.shields.io/docker/v/kruserr/rapiddb?sort=semver&logo=docker&logoColor=white"/> 
  </a>
  <a href="https://codecov.io/gh/kruserr/rapiddb" target="_blank"> 
    <img src="https://img.shields.io/codecov/c/gh/kruserr/rapiddb?logo=Codecov&logoColor=white"/> 
  </a>
</p>

# RapidDB
A reasonably fast configurable embedded key-value sensor database

## Features
- Simple and flexible optional embedded REST API
- Simple key-value database interface
- Lightweight embedded database
- Store sensor data inside a sensor database
- Memory first with on-disk persistence
- Memory Mapped Append-only Vector backing storage
- Bring your own database or API implementation

## Documentation
Visit the [Documentation](https://docs.rs/rapiddb).

## Optional REST API
Visit the [rapiddb-web crates.io page](https://crates.io/crates/rapiddb-web).

## Examples
Using the database directly
```rust
use rapiddb::traits::IDatabase;

let db = std::sync::Arc::new(
  std::sync::RwLock::new(
    rapiddb::db::MMAVDatabase::new()
  )
);

let value = b"{\"key\": \"value\"}";
db.write().unwrap().post("test-0", value);
assert_eq!(db.write().unwrap().get_latest("test-0"), value);
```
