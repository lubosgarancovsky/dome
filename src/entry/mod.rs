pub mod index_entry;
pub mod vault_entry;

pub const DOMAIN_SIZE: usize = 32;

pub trait BinarySerialization {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self;
}

fn deserialize_string(start: usize, end: usize, data: &[u8]) -> String {
    let string_bytes = match data.get(start..=end) {
        Some(bytes) => bytes,
        None => panic!(
            "String bytes out of bounds. End index: {}, data length: {}",
            end,
            data.len()
        ),
    };

    let s = String::from_utf8_lossy(string_bytes);
    s.trim_end_matches('\0').to_string()
}
