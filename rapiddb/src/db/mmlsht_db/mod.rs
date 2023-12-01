use serde_derive::{Serialize, Deserialize};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::OpenOptions;
use std::io::Write;
use memmap2::MmapMut;
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub id: String,
    pub value: Vec<u8>,
}

pub struct Database {
    mmap: MmapMut,
    size: usize,
    record_size: usize,
}

impl Database {
    pub fn new(path: &str, size: usize, record_size: usize) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;

        file.set_len((size * record_size) as u64)?;

        let mmap = unsafe { MmapMut::map_mut(&file)? };
        Ok(Self { mmap, size, record_size })
    }

    fn get_index(&self, id: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        (hasher.finish() as usize) % self.size
    }

    pub fn get(&self, id: &str) -> Option<Record> {
        let index = self.get_index(id);
        let bytes = &self.mmap[index * self.record_size..(index + 1) * self.record_size];
        let bytes = bytes.iter().take_while(|&&x| x != 0).cloned().collect::<Vec<_>>();
        let s = String::from_utf8(bytes).ok()?;
        let record: Result<Record> = serde_json::from_str(&s);
        match record {
            Ok(r) => Some(r),
            Err(_) => None,
        }
    }

    pub fn put(&mut self, record: Record) {
        let index = self.get_index(&record.id);
        let s = serde_json::to_string(&record).unwrap();
        let mut bytes = s.into_bytes();
        bytes.resize(self.record_size, 0);
        let target = &mut self.mmap[index * self.record_size..(index + 1) * self.record_size];
        target.copy_from_slice(&bytes);
    }
}

fn main() {
    let mut db = Database::new("test.db", 100, 1024).unwrap();

    let key0 = "test";
    let value0 = vec![1, 2, 3];

    db.put(Record { id: key0.to_owned(), value: value0.clone() });

    match db.get(key0) {
        Some(record) => assert_eq!(record.value, value0),
        None => println!("Record not found"),
    }
}
