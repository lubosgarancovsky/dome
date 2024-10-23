#[derive(Debug)]
pub struct Entry {
    pub domain: String,
    pub username: String,
    pub password: String,
    pub nonce: Vec<u8>,
    pub salt: Vec<u8>,
}

impl Entry {
    pub fn new(domain: &str, username: &str, password: &str, nonce: &[u8], salt: &[u8]) -> Entry {
        Entry {
            domain: String::from(domain),
            username: String::from(username),
            password: String::from(password),
            nonce: Vec::from(nonce),
            salt: Vec::from(salt),
        }
    }
}

impl super::BinarySerialization for Entry {
    fn serialize(&self) -> Vec<u8> {
        let mut binary_data: Vec<u8> = Vec::new();

        for byte in &self.salt {
            binary_data.push(*byte);
        }

        for byte in &self.nonce {
            binary_data.push(*byte);
        }

        binary_data.push(self.domain.len() as u8);

        for byte in self.domain.as_bytes() {
            binary_data.push(*byte);
        }

        binary_data.push(self.username.len() as u8);

        for byte in self.username.as_bytes() {
            binary_data.push(*byte);
        }

        binary_data.push(self.password.len() as u8);

        for byte in self.password.as_bytes() {
            binary_data.push(*byte);
        }

        binary_data
    }

    fn deserialize(data: &[u8]) -> Entry {
        let salt = Vec::from(data.get(0..=15).expect("Deserialization error"));
        let nonce = Vec::from(data.get(16..=27).expect("Deserialization error"));

        let domain_size: usize = data[28] as usize;
        let (d_start, d_end): (usize, usize) = (29, 29 + domain_size - 1);
        let domain = super::deserialize_string(d_start, d_end, data);

        let username_size: usize = data[d_end + 1] as usize;
        let (u_start, u_end): (usize, usize) = (d_end + 2, d_end + username_size + 1);
        let username = super::deserialize_string(u_start, u_end, data);

        let password_size: usize = data[u_end + 1] as usize;
        let (p_start, p_end): (usize, usize) = (u_end + 2, u_end + password_size + 1);
        let password = super::deserialize_string(p_start, p_end, data);

        Entry::new(&domain, &username, &password, &nonce, &salt)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::entry::BinarySerialization;

    #[test]
    fn test_serialize_deserialize_entry() {
        let nonce: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let salt: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
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
        let nonce: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        let salt: Vec<u8> = Vec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);

        let entry: Entry = Entry::new(domain, username, password, &nonce, &salt);

        let binary_data = entry.serialize();

        let should_be_of_size =
            domain.len() + username.len() + password.len() + nonce.len() + salt.len() + 3;

        assert_eq!(binary_data.len(), should_be_of_size);
    }
}
