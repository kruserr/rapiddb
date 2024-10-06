use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct MemoryMappedHashMap {
    pub mmap: MmapMut,
    capacity: usize,
    current_offset: usize,
}

impl MemoryMappedHashMap {
    pub fn new(file_path: &str, capacity: usize) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();
        file.set_len(capacity as u64).unwrap();
        let mmap = unsafe { MmapOptions::new().map_mut(&file).unwrap() };
        Self { mmap, capacity, current_offset: 0 }
    }

    fn hash<T: Hash>(&self, key: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % self.capacity
    }

    pub fn insert(&mut self, key: &str, value: &[u8]) -> Option<usize> {
        let key_bytes = key.as_bytes();
        let key_len = key_bytes.len();
        let value_len = value.len();
        let entry_size = key_len + value_len + 2 * std::mem::size_of::<usize>();

        if self.current_offset + entry_size > self.mmap.len() {
            return None; // Not enough space
        }

        let offset = self.current_offset;

        self.mmap[offset..offset + std::mem::size_of::<usize>()].copy_from_slice(&key_len.to_ne_bytes());
        let mut offset = offset + std::mem::size_of::<usize>();
        self.mmap[offset..offset + key_len].copy_from_slice(key_bytes);
        offset += key_len;
        self.mmap[offset..offset + std::mem::size_of::<usize>()].copy_from_slice(&value_len.to_ne_bytes());
        offset += std::mem::size_of::<usize>();
        self.mmap[offset..offset + value_len].copy_from_slice(value);

        self.current_offset += entry_size;
        Some(self.current_offset - entry_size)
    }

    pub fn get(&self, offset: usize) -> Option<Vec<u8>> {
        if offset + std::mem::size_of::<usize>() > self.mmap.len() {
            return None;
        }

        let key_len = usize::from_ne_bytes(self.mmap[offset..offset + std::mem::size_of::<usize>()].try_into().unwrap());
        let mut offset = offset + std::mem::size_of::<usize>();

        if offset + key_len + std::mem::size_of::<usize>() > self.mmap.len() {
            return None;
        }

        let key = &self.mmap[offset..offset + key_len];
        offset += key_len;

        let value_len = usize::from_ne_bytes(self.mmap[offset..offset + std::mem::size_of::<usize>()].try_into().unwrap());
        offset += std::mem::size_of::<usize>();

        if offset + value_len > self.mmap.len() {
            return None;
        }

        Some(self.mmap[offset..offset + value_len].to_vec())
    }
}