use serde_derive::{Serialize, Deserialize};

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::fs::OpenOptions;
use std::io::{Write, Read};
use memmap2::MmapMut;
use serde::{Serialize, Deserialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub id: String,
    pub value: Vec<u8>,
}

#[derive(Clone, Copy)]
pub struct Entry {
    pub id_hash: u64,
    pub value_offset: usize,
    pub value_len: usize,
}

pub struct Database {
    mmap: MmapMut,
    size: usize,
    data: Vec<u8>,
    path: String,
}

impl Database {
    pub fn new(path: &str, size: usize) -> std::io::Result<Self> {
        let file = OpenOptions::new().read(true).write(true).create(true).open(path)?;

        file.set_len((size * std::mem::size_of::<Entry>()) as u64)?;

        let mmap = unsafe { MmapMut::map_mut(&file)? };

        let data_path = format!("{}.data", path);
        let mut data_file = OpenOptions::new().read(true).write(true).create(true).open(&data_path)?;
        let mut data = Vec::new();
        data_file.read_to_end(&mut data)?;

        Ok(Self { mmap, size, data, path: path.to_owned() })
    }

    pub fn save(&self) -> std::io::Result<()> {
        let data_path = format!("{}.data", self.path);
        let mut data_file = OpenOptions::new().write(true).truncate(true).open(&data_path)?;
        data_file.write_all(&self.data)
    }

    fn get_index(&self, id: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        (hasher.finish() as usize) % self.size
    }

    pub fn get(&self, id: &str) -> Option<Record> {
        let index = self.get_index(id);
        let entry = unsafe { *(self.mmap.as_ptr().add(index * std::mem::size_of::<Entry>()) as *const Entry) };
        let mut hasher = DefaultHasher::new();
        id.hash(&mut hasher);
        if entry.id_hash == hasher.finish() {
            let bytes = &self.data[entry.value_offset..entry.value_offset + entry.value_len];
            let record: Result<Record> = serde_json::from_slice(bytes);
            match record {
                Ok(r) => Some(r),
                Err(_) => None,
            }
        } else {
            None
        }
    }

    pub fn put(&mut self, record: Record) {
        let index = self.get_index(&record.id);
        let bytes = serde_json::to_vec(&record).unwrap();
        let value_offset = self.data.len();
        let value_len = bytes.len();
        self.data.extend_from_slice(&bytes);
        let mut hasher = DefaultHasher::new();
        record.id.hash(&mut hasher);
        let entry = Entry { id_hash: hasher.finish(), value_offset, value_len };
        unsafe { *(self.mmap.as_mut_ptr().add(index * std::mem::size_of::<Entry>()) as *mut Entry) = entry; }
    }
}

fn main() {
    let mut db = Database::new("test.db", 100).unwrap();

    let key0 = "test";
    let value0 = vec![1, 2, 3];

    // db.put(Record { id: key0.to_owned(), value: value0.clone() });

    match db.get(key0) {
        Some(record) => assert_eq!(record.value, value0),
        None => println!("Record not found"),
    }
}
