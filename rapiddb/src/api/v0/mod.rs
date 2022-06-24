mod api;
mod sensor;
mod sensor_aggregates;
mod sensor_latest;
mod sensor_latest_limit;
mod sensor_meta;
mod sensor_range;
mod sensor_single;
mod sensors;
mod sensors_aggregates;
mod sensors_latest;
mod sensors_latest_limit;
mod sensors_meta;

use warp::{Filter, Rejection, Reply};

use crate::traits::IDatabase;

/// Sensor API Endpoints
pub fn endpoints(
  db: std::sync::Arc<std::sync::RwLock<dyn IDatabase>>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
  api::get()
    .or(sensors_latest_limit::get(db.clone()))
    .or(sensors_latest::get(db.clone()))
    .or(sensors_meta::get(db.clone()))
    .or(sensors_aggregates::get(db.clone()))
    .or(sensors::get())
    .or(sensor_range::get(db.clone()))
    .or(sensor_latest_limit::get(db.clone()))
    .or(sensor_single::get(db.clone()))
    .or(sensor_latest::get(db.clone()))
    .or(sensor_meta::post(db.clone()))
    .or(sensor_meta::get(db.clone()))
    .or(sensor_aggregates::get(db.clone()))
    .or(sensor::post(db.clone()))
    .or(sensor::get(db))
}
