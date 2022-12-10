//! RapidDB Types

use std::sync::Arc;
use std::sync::Mutex;

pub type AggregateFn =
  Arc<Mutex<dyn Fn(&str, &[u8], &Arc<Mutex<Vec<u8>>>) + Send>>;
