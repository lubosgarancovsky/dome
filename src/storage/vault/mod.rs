use std::fs::{File, Metadata, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};

pub fn vault_len() -> u64 {
    let metadata = vault_metadata();
    metadata.len()
}

pub fn vault_read(start_byte: u64) -> Vec<u8> {
    let path = super::vault_file_path();

    let mut file = File::open(path).unwrap();
    file.seek(SeekFrom::Start(start_byte)).unwrap();

    let mut salt = [0u8; 16];
    file.read_exact(&mut salt).unwrap();

    let mut nonce = [0u8; 12];
    file.read_exact(&mut nonce).unwrap();

    let mut size = [0u8; 1];

    // Domain size
    file.read_exact(&mut size).unwrap();
    let domain_len = size[0] as usize;

    let mut domain = vec![0u8; domain_len];
    file.read_exact(&mut domain).unwrap();

    // Username size
    file.read_exact(&mut size).unwrap();
    let username_len = size[0] as usize;

    let mut username = vec![0u8; username_len];
    file.read_exact(&mut username).unwrap();

    // Password size
    file.read_exact(&mut size).unwrap();
    let password_len = size[0] as usize;

    let mut password = vec![0u8; password_len];
    file.read_exact(&mut password).unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(salt);
    buffer.extend(nonce);
    buffer.push(domain.len().try_into().unwrap());
    buffer.extend(domain);
    buffer.push(username.len().try_into().unwrap());
    buffer.extend(username);
    buffer.push(password.len().try_into().unwrap());
    buffer.extend(password);

    buffer
}

pub fn vault_add(data: &[u8]) {
    let path = super::vault_file_path();
    let mut file = OpenOptions::new().append(true).open(path).unwrap();
    file.write_all(data).unwrap();
}

pub fn vault_remove(start_byte: u64) -> u64 {
    let b_entry = vault_read(start_byte);
    let size = b_entry.len() as u64;

    let mut file = OpenOptions::new()
        .write(true)
        .open(super::vault_file_path())
        .unwrap();

    let file_len = file.metadata().unwrap().len();
    file.seek(SeekFrom::Start(start_byte + size)).unwrap();

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    file.seek(SeekFrom::Start(start_byte)).unwrap();
    file.write_all(&buffer).unwrap();
    file.set_len(size + file_len).unwrap();

    size
}

fn vault_metadata() -> Metadata {
    let path = super::vault_file_path();
    std::fs::metadata(path).unwrap()
}
