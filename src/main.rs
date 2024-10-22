use std::env;
mod command;
mod crypto;
mod display;
mod entry;
mod errs;
mod index_set;
mod storage;

fn main() {
    let args: Vec<String> = env::args().collect();

    // storage::initialize_files();

    match command::parse_args(&args) {
        Err(err) => {
            println!("{}", err);
        }
        Ok(c) => {
            if c.command == "--version" {
                command::command_version()
            }

            if c.command == "help" {
                command::command_help()
            }

            if c.command == "list" {
                command::command_list()
            }

            if c.command == "add" {
                match c.args.first() {
                    Some(domain) => {
                        let username = match c.flags.get("-u") {
                            Some(val) => val,
                            None => "",
                        };

                        command::command_add(domain, username)
                    }
                    None => println!("Domain not specified."),
                }
            }

            if c.command == "get" {
                match c.args.first() {
                    Some(domain) => command::command_get(domain),
                    None => println!("Domain not specified."),
                }
            }

            if c.command == "gen" {
                let len = match c.args.first() {
                    Some(value) => match value.parse::<u8>() {
                        Ok(num) => num,
                        Err(e) => panic!("Length must be a number between 0 and 255"),
                    },
                    None => 8,
                };

                command::command_generate(len);
            }
        }
    }
}
