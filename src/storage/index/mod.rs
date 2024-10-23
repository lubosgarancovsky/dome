use std::fs::File;
use std::io::{Read, Write};

pub fn index_read() -> Vec<u8> {
    let path = super::index_file_path();
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn index_write(data: &[u8]) {
    let path = super::index_file_path();
    let mut file = File::create(path).unwrap();
    file.write_all(data).unwrap();
}
