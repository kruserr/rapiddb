//! RapidDB REST API

mod api_endpoint;
pub mod helpers;
mod v0;

use rapiddb::traits::IAsyncDatabase;
use warp::{Filter, Rejection, Reply};

/// Sensor API Endpoints
pub fn endpoints(
  db: std::sync::Arc<tokio::sync::RwLock<impl IAsyncDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  api_endpoint::get().or(v0::endpoints(db))
}
