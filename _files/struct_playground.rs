use regex::Regex;
use std::io;
use std::ops::Index;

struct InputValues {
    username: String,
    password: String,
}

struct Command {
    validator: Regex,
    cmd_input: String,
}

impl Index<&'_ str> for InputValues {
    type Output = String;
    fn index(&self, s: &str) -> &String {
        match s {
            "username\r" => &self.username,
            "password\r" => &self.password,
            _ => panic!("Invalid command."),
        }
    }
}

fn main() {
    let (input, command) = prompt();

    if !command.validator.is_match(&command.cmd_input) {
        println!("Invalid command.");
    }

    let captures = command.validator.captures(&command.cmd_input);

    match captures {
        Some(cptr) => {
            let raw_capture: &str = &cptr[1];
            println!("Callback: {}", input[raw_capture]);
        }
        None => {}
    }
}

fn prompt() -> (InputValues, Command) {
    let mut input: InputValues = InputValues {
        username: String::new(),
        password: String::new(),
    };

    println!("Username:");
    io::stdin().read_line(&mut input.username).expect("Error");

    println!("Password:");
    io::stdin().read_line(&mut input.password).expect("Error");

    let validator = Regex::new("^access @(.*)").unwrap();
    let mut cmd_input = String::new();
    
    println!("Command:");
    io::stdin().read_line(&mut cmd_input).expect("Error");

    return (
        input,
        Command {
            validator,
            cmd_input,
        },
    );
}
