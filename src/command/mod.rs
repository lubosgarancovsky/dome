use crate::cli;
use crate::datastructures::sorted_indexentry_set::IndexSet;
use crate::entry::{index_entry::IndexEntry, vault_entry::Entry, BinarySerialization};
use crate::storage;
use crate::{crypto, errs};

use rand::Rng;
use std::collections::HashMap;

const COMMANDS: [&str; 7] = ["--version", "help", "add", "get", "list", "gen", "remove"];
const MASTER_PASSWORD_TEXT: &str = "Enter master password: ";
const PASSWORD_TEXT: &str = "Password: ";
const REPEAT_PASSWORD_TEXT: &str = "Repeat password: ";

// dome help
pub fn command_help() {
    let help = vec![
        vec!["--version", "Displays current version of Dome."],
        vec!["list", "Displays a list of all domains savedin the vault."],
        vec![
            "add <domain> -u <username>",
            "Adds new entry into the vault. Username is optional.",
        ],
        vec!["get <domain>", "Shows the password for given domain."],
        vec!["remove <domain>", "Removes entry from the vault."],
        vec![
            "gen <length>",
            "Generates random password of the given length.",
        ],
    ];

    println!("List of all availible commands:\n");

    for item in help {
        let (c, e) = (item.first().unwrap(), item.get(1).unwrap());
        println!("{:<32} {}", c, e);
    }
}

// dome list
pub fn command_list() {
    let index_binaries: Vec<u8> = storage::index::index_read();
    let index_set = IndexSet::from_binary(&index_binaries);

    if index_set.is_empty() {
        println!("Your wault is empty. Add new entry using 'add <domain> -u <username>' command.")
    } else {
        index_set.print();
    }
}

// dome gen <length>
pub fn command_generate(len: u8) {
    let mut password = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..len {
        let dec: u8 = rng.gen_range(33..=122);
        password.push(dec as char);
    }

    let result = String::from_iter(password);
    println!("{}", result)
}

// dome add <domain> -u <username>
pub fn command_add(domain: &str, username: &str) {
    let mut index_binaries: Vec<u8> = storage::index::index_read();
    let mut index_set = IndexSet::from_binary(&index_binaries);

    if index_set.has(domain) {
        return cli::error(format!("[ERROR] Domain {} already exist in a vault.", domain).as_str());
    }

    cli::warn("[ADD] You are adding new entry to the vault.\n----------------------------------------------------------------------");
    let master_password = match cli::create_password(MASTER_PASSWORD_TEXT, REPEAT_PASSWORD_TEXT) {
        Ok(value) => value,
        Err(err) => panic!("{}", err),
    };

    println!("\nCreate password for {}.", domain);

    match cli::create_password(PASSWORD_TEXT, REPEAT_PASSWORD_TEXT) {
        Ok(password) => {
            // Encrypt password using secret key generated from master password
            let salt = crypto::generate_salt();
            let key = crypto::derive_key(&master_password, &salt);
            let (cipher, nonce) = crypto::encrypt(&key, &password);

            // Create entry for vailt and entry for index file
            let vault_last_byte = storage::vault::vault_len();
            let entity = Entry::new(domain, username, &cipher, &nonce, &salt);
            let index_entity = IndexEntry::new(domain, vault_last_byte);
            let _ = index_set.add(&index_entity);

            // Serialize entries and save into binary files
            index_binaries = index_set.serialize();
            storage::index::index_write(&index_binaries);

            let entity_binaries = entity.serialize();
            storage::vault::vault_add(&entity_binaries);

            println!("\nNew entry was added to the vault.");
            cli::print_entry(domain, username, "");
        }
        Err(err) => println!("{}", err),
    }
}

// dome get <domain>
pub fn command_get(domain: &str) {
    let index_binaries: Vec<u8> = storage::index::index_read();
    let index_set = IndexSet::from_binary(&index_binaries);

    match index_set.find(domain) {
        Some((_, entry)) => {
            let binary_data = storage::vault::vault_read(entry.value);
            let entry = Entry::deserialize(&binary_data);

            let master_password = cli::read_password(MASTER_PASSWORD_TEXT);
            let key = crypto::derive_key(&master_password, &entry.salt);
            let text_password = crypto::decrypt(&key, &entry.nonce, &entry.password);

            cli::print_entry(&entry.domain, &entry.username, &text_password);
        }
        None => println!("Password for domain {} was not found in a vault.", domain),
    }
}

pub fn command_remove(domain: &str) {
    let mut set = get_index_set();
    match set.find(domain) {
        None => {
            return cli::warn(
                format!("[NOT FOUND] Domain {} is not in the vault.", domain).as_str(),
            );
        }
        Some((_, entry)) => {
            if !cli::get_confirmation(
                format!(
                    "Are you sure you want to delete {} from the vault? [y/n]: ",
                    domain
                )
                .as_str(),
            ) {
                println!("Action was aborted.");
            }

            let size = storage::vault::vault_remove(entry.value);
            set.remove(domain, size);
            let b_set = set.serialize();
            storage::index::index_write(&b_set);
            println!("{} was deleted from the vault.", domain);
        }
    }
}

// dome --version
pub fn command_version() {
    let version = env!("CARGO_PKG_VERSION");
    println!("Dome - {}", version);
}

fn get_index_set() -> IndexSet {
    let index_binaries: Vec<u8> = storage::index::index_read();
    IndexSet::from_binary(&index_binaries)
}

pub struct Command {
    pub command: String,
    pub args: Vec<String>,
    pub flags: HashMap<String, String>,
}

impl Command {
    pub fn new(command: &str) -> Command {
        Command {
            command: String::from(command),
            args: Vec::new(),
            flags: HashMap::new(),
        }
    }
}

pub fn parse_args(args: &[String]) -> Result<Command, String> {
    if args.len() < 2 {
        return Err(errs::INVALID_COMMAND.to_string());
    }

    let command = &args[1];
    if !COMMANDS.contains(&command.as_str()) {
        return Err(errs::INVALID_COMMAND.to_string());
    }

    let mut comm = Command::new(command);
    if args.len() == 2 {
        return Ok(comm);
    }

    let command_args = &args[2..];
    let len = command_args.len();
    let mut index = 0;

    while index < len {
        let current_arg = &command_args[index];
        if current_arg.starts_with("-") {
            let next_arg = if len > index + 1 {
                command_args[index + 1].clone()
            } else {
                "".to_string()
            };
            comm.flags.insert(current_arg.clone(), next_arg);
            index += 2;
        } else {
            comm.args.push(current_arg.clone());
            index += 1;
        }
    }

    Ok(comm)
}

#[cfg(test)]
mod test;
