use crate::entry::{IndexEntry, DOMAIN_SIZE, BinarySerialization};
use std::cmp::Ordering;

#[derive(Debug)]
pub struct IndexSet {
    pub size: usize,
    pub data: Vec<IndexEntry>
}

impl IndexSet {
    pub fn new() -> IndexSet {
        IndexSet { size: 0, data: Vec::new() }
    }

    pub fn from_binary(data: &Vec<u8>) -> IndexSet {
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

        if self.has_key(&item.key) {
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

    pub fn has_key(&self, key: &str) -> bool {
        for data_item in &self.data {
            if data_item.key.cmp(&key.to_string()) == Ordering::Equal {
                return true
            }
        }

        false
    }

    pub fn has(&self, item: &IndexEntry) -> bool {
        for data_item in &self.data {
            if data_item.key.cmp(&item.key) == Ordering::Equal {
                return true
            }
        }

        false
    }

    pub fn find_by_key(&self, key: &str) -> Option<&IndexEntry> {
        for (index, item) in self.data.iter().enumerate() {
            if item.key == key {
                return Some(item);
            }
        }

        None
    }

    pub fn find_key(&self, key: &str) -> Option<usize> {
        for (index, item) in self.data.iter().enumerate() {
            if item.key == key {
                return Some(index);
            }
        }

        None
    }

    pub fn remove_key(&mut self, key: &str) -> Option<usize> {
        let index: Option<usize> = self.find_key(key);
        match index {
            Some(val) => {
                self.data.remove(val);
                return Some(val);
            },
            None => return None,
        }
    }
}

impl PartialEq for IndexSet {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false
        }

        if self.data != other.data {
            return false
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

    fn deserialize(data: &Vec<u8>) -> Self {
        IndexSet::from_binary(data)
    }
}


#[cfg(test)]
mod test {
    use super::*;

    fn prepare_set() -> IndexSet {
        let mut set: IndexSet = IndexSet::new();

        set.add(&IndexEntry::new("gmail", 0));
        set.add(&IndexEntry::new("yahoo", 110));
        set.add(&IndexEntry::new("facebook", 95));
        set.add(&IndexEntry::new("adobe", 30));
        set.add(&IndexEntry::new("google", 1489));

        set
    }

    #[test]
    fn test_initialization() {
        let set = prepare_set();
        let binary_data = set.serialize();
        let new_set = IndexSet::deserialize(&binary_data);

        assert_eq!(set, new_set);
    }

    #[test]
    fn test_insert() {
        let set = prepare_set();

        println!("{:?}", set.data);

        assert_eq!(set.data[0].key, "adobe");
        assert_eq!(set.data[1].key, "facebook");
        assert_eq!(set.data[2].key, "gmail");
        assert_eq!(set.data[3].key, "google");
        assert_eq!(set.data[4].key, "yahoo");
    }

    #[test]
    fn test_find_key() {
        let set = prepare_set();

        assert_eq!(set.find_key("adobe"), Some(0));
        assert_eq!(set.find_key("google"), Some(3));
        assert_eq!(set.find_key("not_found"), None);
    }

    #[test]
    fn test_remove_key() {
        let mut set = prepare_set();

        set.remove_key("adobe");
        set.remove_key("yahoo");
        set.remove_key("gmail");

        assert_eq!(set.find_key("adobe"), None);
        assert_eq!(set.find_key("facebook"), Some(0));
        assert_eq!(set.find_key("google"), Some(1));
    }
}
