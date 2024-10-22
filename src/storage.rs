use std::fs::{self, File, OpenOptions};
use std::io::{self, Write, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use crate::errs;

const INDEX_FILE: &str = "index.bin";
const VAULT_FILE: &str = "vault.bin";

pub fn initialize_files() {
    let dome_dir = dome_dir_path();

    fs::create_dir_all(&dome_dir).unwrap();
    
    let index_path = index_file_path();
    if !index_path.exists() {
        File::create(&index_path).unwrap();
    }

    let vault_file = vault_file_path();
    if !vault_file.exists() {
        File::create(&vault_file).unwrap();
    }
}

pub fn dome_dir_path() -> PathBuf {
    let destination: PathBuf = appdata_dir_path().unwrap();
    destination.join("dome")
}

pub fn index_file_path() -> PathBuf {
    let dome_dir = dome_dir_path();
    Path::new(&dome_dir).join(INDEX_FILE)
}

pub fn vault_file_path() -> PathBuf {
    let dome_dir = dome_dir_path();
    Path::new(&dome_dir).join(VAULT_FILE)
}

pub fn read_index() -> io::Result<Vec<u8>> {
    let path = index_file_path();
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn save_index(data: &Vec<u8>) -> io::Result<()> {
    let path = index_file_path();
    let mut file = File::create(path)?;
    file.write_all(data);
    Ok(())
}

pub fn vault_size() -> io::Result<usize> {
    let path = vault_file_path();
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len().try_into().unwrap())
}

pub fn read_entry(start_byte: usize) -> io::Result<Vec<u8>> {
    let path = vault_file_path();

    let mut file = File::open(path)?;
    file.seek(SeekFrom::Start(start_byte.try_into().unwrap()))?;

    let mut salt = [0u8; 16];
    file.read_exact(&mut salt);

    let mut nonce = [0u8; 12];
    file.read_exact(&mut nonce);

    let mut size = [0u8; 1];

    // Domain size
    file.read_exact(&mut size);
    let domain_len = size[0] as usize;

    let mut domain = vec![0u8; domain_len];
    file.read_exact(&mut domain);

    // Username size
    file.read_exact(&mut size);
    let username_len = size[0] as usize;

    let mut username = vec![0u8; username_len];
    file.read_exact(&mut username);

    // Password size
    file.read_exact(&mut size);
    let password_len = size[0] as usize;

    let mut password = vec![0u8; password_len];
    file.read_exact(&mut password);

    // debug_entry_binary(&salt, &nonce, &domain, &username, &password);

    let mut buffer: Vec<u8> = Vec::new();
    buffer.extend(salt);
    buffer.extend(nonce);
    buffer.push(domain.len().try_into().unwrap());
    buffer.extend(domain);
    buffer.push(username.len().try_into().unwrap());
    buffer.extend(username);
    buffer.push(password.len().try_into().unwrap());
    buffer.extend(password);

    Ok(buffer)
}

pub fn add_entry(data: &Vec<u8>) -> io::Result<()> {
    let path = vault_file_path();
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)?;

    file.write_all(data)?;
    Ok(())
}

fn appdata_dir_path() -> Result<PathBuf, &'static str> {
    let os = get_os();
    
    match os {
        Ok("windows") => {
            let home_dir = std::env::var("USERPROFILE").unwrap_or_else(|_| String::from("c:\\"));
            Ok(Path::new(&home_dir).join("AppData\\Local"))
        },
        Ok("linux") => {
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| String::from("/"));
            Ok(Path::new(&home_dir).join(".local/share"))
        },
        _ => {
            Err(errs::UNSUPPORTED_OS)
        }
    }
}

fn get_os() -> Result<&'static str, &'static str> {
    if cfg!(target_os = "windows") {
        Ok("windows")
    } else if cfg!(target_os = "linux") {
        Ok("linux")
    } else {
        Err(errs::UNSUPPORTED_OS)
    }
}

fn debug_entry_binary(salt: &[u8], nonce: &[u8], domain: &[u8], username: &[u8], password: &[u8]) {
    println!("--------------------------------------------------");
    println!("{:?}", &salt);
    println!("{:?}", &nonce);
    println!("{:?}", String::from_utf8_lossy(&domain));
    println!("{:?}", String::from_utf8_lossy(&username));
    println!("{:?}", String::from_utf8_lossy(&password));
    println!("--------------------------------------------------");
    println!("{:?}", &salt);
    println!("{:?}", &nonce);
    println!("[{}] - {:?}", &domain.len(), &domain);
    println!("[{}] - {:?}", &username.len(), &username);
    println!("[{}] - {:?}", &password.len(), &password);
    println!("--------------------------------------------------");
}
