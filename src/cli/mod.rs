use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use rpassword::read_password as get_password;
use std::io::{self, Write};

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

pub fn get_confirmation(text: &str) -> bool {
    loop {
        print!("{}", text);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter 'Y' for yes or 'N' for no."),
        }
    }
}

pub fn read_password(text: &str) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let password: String = get_password().unwrap();
    password
}

pub fn create_password(text: &str, text_repeat: &str) -> Result<String, String> {
    let pwd: String = read_password(text);
    let pwd2: String = read_password(text_repeat);

    if pwd == pwd2 {
        return Ok(pwd);
    }

    Err("Passwords don't match!".to_string())
}

pub fn warn(message: &str) {
    println!("{}{}{}", YELLOW, message, RESET);
}

pub fn error(message: &str) {
    println!("{}{}{}", RED, message, RESET);
}

pub fn print_entry(domain: &str, username: &str, password: &str) {
    let table = vec![vec![
        domain.cell(),
        username.cell(),
        password.cell().justify(Justify::Right),
    ]]
    .table()
    .title(vec![
        "Domain".cell(),
        "Username".cell(),
        "Password".cell().justify(Justify::Right),
    ])
    .bold(true);

    print_stdout(table).unwrap();
}
