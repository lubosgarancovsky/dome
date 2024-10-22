use std::mem;

pub const DOMAIN_SIZE: usize = 32;

#[derive(Debug)]
pub struct Entry {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
}

pub trait BinarySerialization {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &Vec<u8>) -> Self;
}

impl Entry {
    pub fn new(domain: &str, username: &str, password: &str, nonce: &Vec<u8>, salt: &Vec<u8>) -> Entry {
        Entry { 
            domain: String::from(domain), 
            username: String::from(username), 
            password: String::from(password), 
            nonce: nonce.clone(),
            salt: salt.clone()
        }
    } 
}

impl BinarySerialization for Entry {
    fn serialize(&self) -> Vec<u8> {
        let mut binary_data = Vec::new();

        for byte in &self.salt {
            binary_data.push(byte.clone());
        }
   
        for byte in &self.nonce {
            binary_data.push(byte.clone());
        }

        binary_data.push(self.domain.len() as u8);

        for byte in self.domain.as_bytes() {
            binary_data.push(byte.clone());
        }

        binary_data.push(self.username.len() as u8);

        for byte in self.username.as_bytes() {
            binary_data.push(byte.clone());
        }

        binary_data.push(self.password.len() as u8);

        for byte in self.password.as_bytes() {
            binary_data.push(byte.clone());
        }

        binary_data
    }

    fn deserialize(data: &Vec<u8>) -> Entry {
        let salt = Vec::from(data.get(0..=15).expect("Deserialization error"));
        let nonce = Vec::from(data.get(16..=27).expect("Deserialization error"));

        let domain_size: usize = data[28] as usize;
        let (d_start, d_end): (usize, usize) = (29, 29 + domain_size - 1);
        let domain = deserialize_string(d_start, d_end, data);

        let username_size: usize = data[d_end + 1] as usize;
        let (u_start, u_end): (usize, usize)= (d_end + 2, d_end + username_size + 1);
        let username = deserialize_string(u_start, u_end, data);

        let password_size: usize = data[u_end + 1] as usize;
        let (p_start, p_end): (usize, usize) = (u_end + 2, u_end + password_size + 1);
        let password = deserialize_string(p_start, p_end, data);

        Entry::new(&domain, &username, &password, &nonce, &salt)
    }
}

#[derive(Debug, Clone)]
pub struct IndexEntry {
    pub key: String,
    pub value: usize,
}

impl IndexEntry {
    pub fn new(key: &str, value: usize) -> IndexEntry {
        IndexEntry {
            key: String::from(key),
            value
        }
    }
}

impl PartialEq for IndexEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.key == other.key && self.value == other.value {
            true
        } else {
            false
        }
    }
}

impl BinarySerialization for IndexEntry {

    fn serialize(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        let mut buffer = [0u8; DOMAIN_SIZE];
        let bytes = self.key.as_bytes();
        
        let len = self.key.len().min(DOMAIN_SIZE);
        buffer[..len].copy_from_slice(&bytes[..len]);

        binary_data.extend(buffer);
        binary_data.extend(self.value.to_ne_bytes());

        binary_data
    }

    fn deserialize(data: &Vec<u8>) -> IndexEntry {
        let (k_start, k_end): (usize, usize) = (0, 31);
        
        let key = deserialize_string(k_start, k_end, data);
        let bytes = data.get((k_end + 1)..).expect("Deserialization error.");
        
        if bytes.len() == mem::size_of::<usize>() {
            let value: usize = usize::from_ne_bytes(bytes.try_into().expect("Deserialization error."));
            return IndexEntry::new(&key, value);
        }

        panic!("Deserialization error.");
    }
}


fn deserialize_string(start: usize, end: usize, data: &Vec<u8>) -> String {
    let string_bytes = match data.get(start..=end) {
        Some(bytes) => bytes,
        None => panic!("String bytes out of bounds. End index: {}, data length: {}", end, data.len()),
    };

    let s = String::from_utf8_lossy(string_bytes);
    s.trim_end_matches('\0').to_string()
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::entry::BinarySerialization;

    #[test]
    fn test_serialize_deserialize_entry() {
        let nonce: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ,11]);
        let salt: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ,11, 12, 13, 14, 15]);
        let entry: Entry = Entry::new("gmail", "john.doe@gmail.com", "123", &nonce, &salt);

        let binary_data = entry.serialize();
        let new_entry = Entry::deserialize(&binary_data);

        assert_eq!(entry.domain, new_entry.domain);
        assert_eq!(entry.username, new_entry.username);
        assert_eq!(entry.password, new_entry.password);
        assert_eq!(entry.nonce, new_entry.nonce);
        assert_eq!(entry.salt, new_entry.salt);
    }

    #[test]
    fn test_serialization_size() {
        let domain: &str = "gmail";
        let username: &str = "john.doe@gmail.com";
        let password: &str = "Pass123";
        let nonce: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ,11]);
        let salt: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 ,11, 12, 13, 14, 15]);

        let entry: Entry = Entry::new(domain, username, password, &nonce, &salt);

        let binary_data = entry.serialize();

        let should_be_of_size = domain.len() + username.len() + password.len() + nonce.len() + salt.len() + 3;

        assert_eq!(binary_data.len(), should_be_of_size);
    }

    #[test]
    fn test_serialize_deserialize_indexentry() {
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
