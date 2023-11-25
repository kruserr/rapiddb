//! RapidDB REST API

mod api_endpoint;
mod v0;

use crate::traits::IDatabase;
use warp::{Filter, Rejection, Reply};

/// Sensor API Endpoints
pub fn endpoints(
  db: std::sync::Arc<std::sync::RwLock<impl IDatabase + ?Sized>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  api_endpoint::get().or(v0::endpoints(db))
}
