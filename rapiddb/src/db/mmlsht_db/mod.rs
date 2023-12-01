use std::fs::OpenOptions;
use std::io::Write;
use memmap2::MmapMut;
use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};
use serde_json::Result;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub id: String,
    pub value: Vec<u8>,
}

pub struct Database {
    mmap: MmapMut,
}

impl Database {
    pub fn new(path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;

        file.set_len(1_000_000)?;

        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self { mmap })
    }

    fn get_index(&self, id: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        (hasher.finish() as usize) % self.mmap.len()
    }

    pub fn get(&self, id: &str) -> Option<Record> {
        let index = self.get_index(id);
        // Deserialize the record from the memory-mapped file
        let record: Result<Record> = serde_json::from_slice(&self.mmap[index..]);
        match record {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    pub fn put(&mut self, record: Record) {
      let index = self.get_index(&record.id);
      // Serialize the record into the memory-mapped file
      let bytes = serde_json::to_vec(&record).unwrap();
      if bytes.len() <= self.mmap.len() - index {
          self.mmap[index..index+bytes.len()].copy_from_slice(&bytes);
      } else {
          // Find a new location in the memory-mapped file where there's enough space
          let mut new_index = index;
          while bytes.len() > self.mmap.len() - new_index {
              new_index += 1;
          }
          self.mmap[new_index..new_index+bytes.len()].copy_from_slice(&bytes);
      }
  }


    pub fn remove(&mut self, id: &str) {
        let index = self.get_index(id);
        // Overwrite the record in the memory-mapped file with zeros
        let record = self.get(id);
        match record {
            Some(r) => {
                let bytes = serde_json::to_vec(&r).unwrap();
                for i in 0..bytes.len() {
                    self.mmap[index + i] = 0;
                }
            },
            None => println!("Record not found"),
        }
    }
}

fn main() {
  let mut db = Database::new("test-0.db").unwrap();

  let key0 = "test";
  let value0 = vec![1, 2, 3];

  db.put(Record { id: key0.to_owned(), value: value0.clone()});

  match db.get(key0) {
    Some(record) => assert_eq!(record.value, value0),
    None => println!("Record not found"),
  }
}