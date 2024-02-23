<p align="center">
  <a href="https://github.com/kruserr/rapiddb" target="_blank">
    <img width="300" src="https://raw.githubusercontent.com/kruserr/rapiddb/main/assets/logo/logo.svg">
  </a>
  <br/>
  <br/>
  <a href="https://crates.io/crates/rapiddb" target="_blank">
    <img src="https://img.shields.io/crates/v/rapiddb?logo=Rust&logoColor=white"/> 
  </a>
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
- Lightweight embedded database
- Simple key-value database interface
- Simple and flexible optional embedded REST API
- Memory first with on-disk persistence
- Memory Mapped Append-only Vector backing storage
- Bring your own database or API implementation
- Store sensor data inside a sensor database

## Getting started
Cargo.toml
```toml
[dependencies]
rapiddb = "0.1"
```

src/main.rs
```rust
use rapiddb::traits::IDatabase;

pub fn main() {
  let mut db = rapiddb::db::MMAVDatabase::new();

  let value = b"{\"key\": \"value\"}";
  db.post("test-0", value);
  assert_eq!(db.get_latest("test-0"), value);
}
```

Run the database with cargo
```sh
cargo run --release
```

## Optional REST API
Visit [further install options](https://github.com/kruserr/rapiddb/blob/main/docs/install.md#add-to-your-cargo-project).

## Documentation
Visit the [Documentation](https://docs.rs/rapiddb).

## Examples
Visit the [Examples](https://github.com/kruserr/rapiddb/tree/main/examples).
