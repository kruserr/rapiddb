use bincode::{deserialize_from, serialize_into};
use memmap2::MmapMut;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::sync::Mutex;

use crate::traits::IDatabase;

#[derive(Serialize, Deserialize)]
struct SensorData {
  meta: Vec<u8>,
  records: Vec<Vec<u8>>,
}

pub struct MmapDatabase {
  data: Mutex<HashMap<String, SensorData>>,
  mmap: MmapMut,
}

impl MmapDatabase {
  pub fn new(path: &str) -> std::io::Result<Self> {
    let file =
      OpenOptions::new().read(true).write(true).create(true).open(path)?;
    file.set_len(4096)?; // Set initial size of the file
    let mmap = unsafe { MmapMut::map_mut(&file)? };

    let data: HashMap<String, SensorData> = match deserialize_from(&*mmap) {
      Ok(data) => data,
      Err(_) => HashMap::new(), // If deserialization fails, create a new HashMap
    };

    Ok(Self { data: Mutex::new(data), mmap })
  }

  fn save(&mut self) -> bincode::Result<()> {
    let data = self.data.lock().unwrap();
    serialize_into(&mut *self.mmap, &*data)
  }
}

impl IDatabase for MmapDatabase {
  fn contains(&self, id: &str) -> bool {
    let data = self.data.lock().unwrap();
    data.contains_key(id)
  }

  fn get(&mut self, id: &str, rec_id: usize) -> Vec<u8> {
    let data = self.data.lock().unwrap();
    data.get(id).unwrap().records[rec_id].clone()
  }

  fn post(&mut self, id: &str, value: &[u8]) {
    {
      let mut data = self.data.lock().unwrap();
      data
        .entry(id.to_string())
        .or_insert_with(|| SensorData { meta: vec![], records: vec![] })
        .records
        .push(value.to_vec());
    }
    self.save().unwrap();
  }

  fn get_meta(&mut self, id: &str) -> Vec<u8> {
    let data = self.data.lock().unwrap();
    data.get(id).unwrap().meta.clone()
  }

  fn post_meta(&mut self, id: &str, value: Vec<u8>) {
    {
      let mut data = self.data.lock().unwrap();
      data
        .entry(id.to_string())
        .or_insert_with(|| SensorData { meta: vec![], records: vec![] })
        .meta = value;
    }
    self.save().unwrap();
  }

  fn get_aggregates(&self, id: &str) -> Vec<u8> {
    // This method isn't typically associated with a HashMap.
    // You might need to store additional metadata in the SensorData struct.
    unimplemented!()
  }

  fn get_latest(&mut self, id: &str) -> Vec<u8> {
    let data = self.data.lock().unwrap();
    let sensor_data = data.get(id).unwrap();
    sensor_data.records.last().unwrap().clone()
  }

  fn get_latest_with_limit(&mut self, id: &str, limit: usize) -> Vec<Vec<u8>> {
    let data = self.data.lock().unwrap();
    let sensor_data = data.get(id).unwrap();
    sensor_data.records.iter().rev().take(limit).cloned().collect()
  }

  fn get_range(&mut self, id: &str, start: usize, end: usize) -> Vec<Vec<u8>> {
    let data = self.data.lock().unwrap();
    let sensor_data = data.get(id).unwrap();
    sensor_data.records[start..end].to_vec()
  }

  fn get_all_meta(&mut self) -> HashMap<&str, Vec<u8>> {
    unimplemented!()
    // let data = self.data.lock().unwrap();
    // data.iter().map(|(id, sensor_data)| (id.clone(), sensor_data.meta.clone())).collect()
  }

  fn get_all_aggregates(&self) -> std::collections::HashMap<&str, Vec<u8>> {
    // This method isn't typically associated with a HashMap.
    // You might need to store additional metadata in the SensorData struct.
    unimplemented!()
  }

  fn get_all_latest(&mut self) -> HashMap<&str, Vec<u8>> {
    unimplemented!()
    // let data = self.data.lock().unwrap();
    // data.iter().map(|(id, sensor_data)| (id.clone(), sensor_data.records.last().unwrap().clone())).collect()
  }

  fn get_all_latest_with_limit(
    &mut self,
    limit: usize,
  ) -> HashMap<&str, Vec<Vec<u8>>> {
    unimplemented!()
    // let data = self.data.lock().unwrap();
    // data.iter().map(|(id, sensor_data)| (id.clone(), sensor_data.records.iter().rev().take(limit).cloned().collect())).collect()
  }
}
