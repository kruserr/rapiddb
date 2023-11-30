use std::collections::HashMap;
use std::fs::OpenOptions;
use std::sync::Mutex;
use memmap2::MmapMut;
use serde_derive::{Serialize, Deserialize};
use bincode::{serialize_into, deserialize_from};

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
        let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;
        file.set_len(4096)?; // Set initial size of the file
        let mmap = unsafe { MmapMut::map_mut(&file)? };

        let data: HashMap<String, SensorData> = match deserialize_from(&*mmap) {
            Ok(data) => data,
            Err(_) => HashMap::new(), // If deserialization fails, create a new HashMap
        };

        Ok(Self {
            data: Mutex::new(data),
            mmap,
        })
    }

    fn save(&mut self) -> bincode::Result<()> {
        let data = self.data.lock().unwrap();
        serialize_into(&mut *self.mmap, &*data)
    }

    // ... implement other methods ...
}