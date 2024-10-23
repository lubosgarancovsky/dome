use std::mem;

#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub key: String,
    pub value: u64,
}

impl IndexEntry {
    pub fn new(key: &str, value: u64) -> IndexEntry {
        IndexEntry {
            key: String::from(key),
            value,
        }
    }
}

impl PartialEq for IndexEntry {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key && self.value == other.value
    }
}

impl super::BinarySerialization for IndexEntry {
    fn serialize(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        let mut buffer = [0u8; super::DOMAIN_SIZE];
        let bytes = self.key.as_bytes();

        let len = self.key.len().min(super::DOMAIN_SIZE);
        buffer[..len].copy_from_slice(&bytes[..len]);

        binary_data.extend(buffer);
        binary_data.extend(self.value.to_ne_bytes());

        binary_data
    }

    fn deserialize(data: &[u8]) -> IndexEntry {
        let (k_start, k_end): (usize, usize) = (0, 31);

        let key = super::deserialize_string(k_start, k_end, data);
        let bytes = data.get((k_end + 1)..).unwrap();

        if bytes.len() == mem::size_of::<usize>() {
            let value = u64::from_ne_bytes(bytes.try_into().unwrap());
            return IndexEntry::new(&key, value);
        }

        panic!("Deserialization error.");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entry::BinarySerialization;

    #[test]
    fn test_serialization() {
        let entry: IndexEntry = IndexEntry::new("gmail", 789);
        let binary_data = entry.serialize();
        let new_entry = IndexEntry::deserialize(&binary_data);

        assert_eq!(entry.key, new_entry.key);
        assert_eq!(entry.value, new_entry.value);
        assert_eq!(new_entry.key, "gmail");
        assert_eq!(new_entry.value, 789);
    }

    #[test]
    fn test_equals() {
        let entry: IndexEntry = IndexEntry::new("gmail", 789);
        let entry_2: IndexEntry = IndexEntry::new("gmail", 789);

        assert_eq!(entry, entry_2);
    }

    #[test]
    fn test_not_equals() {
        let entry: IndexEntry = IndexEntry::new("gmail", 789);
        let entry_2: IndexEntry = IndexEntry::new("yahoo", 125);

        assert_ne!(entry, entry_2);
    }
}
