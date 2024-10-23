use crate::errs;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub mod index;
pub mod vault;

const INDEX_FILE: &str = "index.bin";
const VAULT_FILE: &str = "vault.bin";
const DOME_FOLDER: &str = "dome";

pub fn dome_dir_path() -> PathBuf {
    let destination: PathBuf = appdata_dir_path().unwrap();
    destination.join(DOME_FOLDER)
}

pub fn index_file_path() -> PathBuf {
    let dome_dir = dome_dir_path();
    Path::new(&dome_dir).join(INDEX_FILE)
}

pub fn vault_file_path() -> PathBuf {
    let dome_dir = dome_dir_path();
    Path::new(&dome_dir).join(VAULT_FILE)
}

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

fn appdata_dir_path() -> Result<PathBuf, &'static str> {
    let os = get_os();

    match os {
        Ok("windows") => {
            let home_dir = std::env::var("USERPROFILE").unwrap_or_else(|_| String::from("c:\\"));
            Ok(Path::new(&home_dir).join("AppData\\Local"))
        }
        Ok("linux") => {
            let home_dir = std::env::var("HOME").unwrap_or_else(|_| String::from("/"));
            Ok(Path::new(&home_dir).join(".local/share"))
        }
        _ => Err(errs::UNSUPPORTED_OS),
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
