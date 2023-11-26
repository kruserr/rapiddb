### [<-](../README.md)

## Install
### Git and cargo
Clone the repo and build the database from source
```sh
git clone https://github.com/kruserr/rapiddb.git
cd rapiddb
cargo run --release
```

### Add to your cargo project
Cargo.toml
```toml
tokio = { version = "1", features = ["full"] }
warp = "0.3"
rapiddb-web = "0.1"
```

src/main.rs
```rust
#[tokio::main]
async fn main() {
  let db = rapiddb_web::rapiddb::db::MMAVAsyncDatabase::new();

  warp::serve(rapiddb_web::api::endpoints(db)).run(([0, 0, 0, 0], 3030)).await;
}
```

Run the database with cargo
```sh
cargo run --release
```

### Using the database directly without the REST API
The database can be used by itself without building the REST API, by using the [rapiddb](https://crates.io/crates/rapiddb) crate instead of the [rapiddb-web](https://crates.io/crates/rapiddb-web) crate.

Cargo.toml
```toml
rapiddb = "0.1"
```

src/main.rs
```rust
use rapiddb::traits::IDatabase;

pub fn main() {
  let db = rapiddb::db::MMAVDatabase::new();

  let value = b"{\"key\": \"value\"}";
  db.post("test-0", value);
  assert_eq!(db.get_latest("test-0"), value);
}
```

Run the database with cargo
```sh
cargo run --release
```
