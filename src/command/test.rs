use super::*;

#[test]
fn test_parse_command() {
    let args = vec![String::from("dome"), String::from("--version")];
    let command = parse_args(&args).unwrap();

    assert_eq!(command.command, "--version");
}

#[test]
fn test_parse_flags() {
    let args = vec![
        String::from("dome"),
        String::from("add"),
        String::from("domain"),
        String::from("-u"),
        String::from("username"),
    ];
    let command = parse_args(&args).unwrap();

    assert_eq!(command.flags.get("-u").unwrap(), "username");
}

#[test]
fn test_parse_args() {
    let args = vec![
        String::from("dome"),
        String::from("add"),
        String::from("domain"),
        String::from("-u"),
        String::from("username"),
    ];
    let command = parse_args(&args).unwrap();

    assert_eq!(command.args.first().unwrap(), "domain");
}
