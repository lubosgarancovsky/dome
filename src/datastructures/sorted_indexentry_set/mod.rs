use crate::entry::{index_entry::IndexEntry, BinarySerialization, DOMAIN_SIZE};
use cli_table::{print_stdout, Cell};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct IndexSet {
    pub size: usize,
    pub data: Vec<IndexEntry>,
}

impl IndexSet {
    pub fn new() -> IndexSet {
        IndexSet {
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn from_binary(data: &[u8]) -> IndexSet {
        let length = data.len();
        let step = DOMAIN_SIZE + std::mem::size_of::<usize>();

        let mut set = IndexSet::new();

        for i in (0..length).step_by(step) {
            let slice = Vec::from(data.get(i..i + step).expect("Deserialization error."));
            let entry: IndexEntry = IndexEntry::deserialize(&slice);
            set.add(&entry);
        }

        set
    }

    pub fn add(&mut self, item: &IndexEntry) -> bool {
        if self.size == 0 {
            self.data.push(item.clone());
            self.size += 1;
            return true;
        }

        if self.has(&item.key) {
            return false;
        }

        let last_index = self.data.len() - 1;
        for (index, data_item) in self.data.iter_mut().enumerate() {
            let cmp = item.key.cmp(&data_item.key);

            if cmp == Ordering::Less {
                self.data.insert(index, item.clone());
                self.size += 1;
                return true;
            } else if cmp == Ordering::Greater && index == last_index {
                self.data.push(item.clone());
                self.size += 1;
                return true;
            }
        }

        false
    }

    pub fn has(&self, key: &str) -> bool {
        for data_item in &self.data {
            if data_item.key.cmp(&key.to_string()) == Ordering::Equal {
                return true;
            }
        }

        false
    }

    pub fn find(&self, key: &str) -> Option<(usize, &IndexEntry)> {
        for (index, item) in self.data.iter().enumerate() {
            if item.key == key {
                return Some((index, item));
            }
        }

        None
    }

    pub fn remove(&mut self, key: &str, size: u64) -> Option<IndexEntry> {
        let (index, entry) = match self.find(key) {
            None => return None,
            Some((index, entry)) => (index, entry.clone()),
        };

        // Subtract the size of deleted entry from every entry further down in vault file
        for current in &mut self.data {
            if current.value > entry.value {
                println!("SUBTRACTING {} by {}", current.value, size);
                current.value -= size;
            }
        }

        self.data.remove(index);
        Some(entry)
    }

    pub fn is_empty(&self) -> bool {
        self.data.len() == 0
    }

    pub fn print(&self) {
        let mut result = Vec::new();

        for (index, item) in self.data.iter().enumerate() {
            let vec = vec![index.cell(), item.key.clone().cell()];
            result.push(vec);
        }

        let _ = print_stdout(result);
    }
}

impl PartialEq for IndexSet {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        }

        if self.data != other.data {
            return false;
        }

        true
    }
}

impl BinarySerialization for IndexSet {
    fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();

        for item in &self.data {
            data.extend(item.serialize());
        }

        data
    }

    fn deserialize(data: &[u8]) -> Self {
        IndexSet::from_binary(data)
    }
}

#[cfg(test)]
mod test;
