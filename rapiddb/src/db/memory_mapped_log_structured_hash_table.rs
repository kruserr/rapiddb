use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use async_trait::async_trait;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::traits::{IDatabase, IAsyncDatabase};
use crate::db::memory_mapped_hash_map::MemoryMappedHashMap;
use std::io::{Read, Seek, Write}; // Import necessary traits
use tokio::io::AsyncSeekExt;

pub struct MemoryMappedLogStructuredHashTable {
    index: MemoryMappedHashMap,
    data_files: Vec<MemoryMappedHashMap>,
    current_file_id: usize,
}

impl MemoryMappedLogStructuredHashTable {
    pub fn new() -> Self {
        Self::new_with_all(".index.db", 1024)
    }

    pub fn new_with_all(index_file: &str, index_capacity: usize) -> Self {
        Self {
            index: MemoryMappedHashMap::new(index_file, index_capacity),
            data_files: vec![MemoryMappedHashMap::new("data_0.db", 1024)],
            current_file_id: 0,
        }
    }

    fn get_file_path(&self, file_id: usize) -> String {
        format!("data_{}.db", file_id)
    }

    fn compact(&mut self) {
        // Implement compaction logic here
    }

    fn switch_to_new_file(&mut self) {
        self.current_file_id += 1;
        let new_file_path = self.get_file_path(self.current_file_id);
        self.data_files.push(MemoryMappedHashMap::new(&new_file_path, 1024));
    }
}

impl IDatabase for MemoryMappedLogStructuredHashTable {
    fn contains(&self, id: &str) -> bool {
        self.index.get(self.index.hash(&id)).is_some()
    }

    fn get(&mut self, id: &str, _: usize) -> Vec<u8> {
        if let Some(value) = self.index.get(self.index.hash(&id)) {
            let (file_id, offset): (usize, usize) = match bincode::deserialize(&value) {
                Ok(v) => v,
                Err(_) => return vec![], // Handle deserialization error
            };
            let data_file = &self.data_files[file_id];
            data_file.get(offset).unwrap_or_else(Vec::new)
        } else {
            vec![]
        }
    }

    fn post(&mut self, id: &str, value: &[u8]) {
        let mut data_file = &mut self.data_files[self.current_file_id];

        if let Some(offset) = data_file.insert(id, value) {
            let index_value = bincode::serialize(&(self.current_file_id, offset)).unwrap();
            self.index.insert(&self.index.hash(&id).to_string(), &index_value);
        } else {
            self.switch_to_new_file();
            data_file = &mut self.data_files[self.current_file_id];
            let offset = data_file.insert(id, value).expect("Failed to insert into new data file");
            let index_value = bincode::serialize(&(self.current_file_id, offset)).unwrap();
            self.index.insert(&self.index.hash(&id).to_string(), &index_value);
        }
    }

    fn get_meta(&mut self, id: &str) -> Vec<u8> {
        // Implement metadata retrieval
        vec![]
    }

    fn post_meta(&mut self, id: &str, data: Vec<u8>) {
        // Implement metadata posting
    }

    fn get_aggregates(&self, id: &str) -> Vec<u8> {
        // Implement aggregates retrieval
        vec![]
    }

    fn get_latest(&mut self, id: &str) -> Vec<u8> {
        // Implement latest retrieval
        vec![]
    }

    fn get_latest_with_limit(&mut self, id: &str, limit: usize) -> Vec<Vec<u8>> {
        // Implement latest with limit retrieval
        vec![]
    }

    fn get_range(&mut self, id: &str, start: usize, end: usize) -> Vec<Vec<u8>> {
        // Implement range retrieval
        vec![]
    }

    fn get_all_meta(&mut self) -> HashMap<&str, Vec<u8>> {
        // Implement all meta retrieval
        HashMap::new()
    }

    fn get_all_aggregates(&self) -> HashMap<&str, Vec<u8>> {
        // Implement all aggregates retrieval
        HashMap::new()
    }

    fn get_all_latest(&mut self) -> HashMap<&str, Vec<u8>> {
        // Implement all latest retrieval
        HashMap::new()
    }

    fn get_all_latest_with_limit(&mut self, limit: usize) -> HashMap<&str, Vec<Vec<u8>>> {
        // Implement all latest with limit retrieval
        HashMap::new()
    }
}