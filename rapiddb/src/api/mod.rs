//! RapidDB REST API

mod api_endpoint;
mod v0;

use crate::traits::IDatabase;
use warp::Filter;

/// Sensor API Endpoints
pub fn endpoints(
    db: std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    api_endpoint::get().or(v0::endpoints(db))
}
