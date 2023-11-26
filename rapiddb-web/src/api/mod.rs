//! RapidDB REST API

mod api_endpoint;
pub mod helpers;
mod v0;

use rapiddb::traits::IAsyncDatabase;
use warp::{Filter, Rejection, Reply};

/// Sensor API Endpoints
pub fn endpoints(
  db: impl IAsyncDatabase,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  let _db = std::sync::Arc::new(tokio::sync::RwLock::new(db));

  api_endpoint::get().or(v0::endpoints(_db))
}

/// Sensor API Endpoints
pub fn endpoints_with_arc_rwlock(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  api_endpoint::get().or(v0::endpoints(db))
}
