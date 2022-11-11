use colored::*;
use std::{io, panic};

mod utils;

fn main() {
    panic::set_hook(Box::new(|err| {
        let parsed: String = match err.payload().downcast_ref::<&str>() {
            Some(info) => info.to_string(),
            None => match err.payload().downcast_ref::<String>() {
                Some(message) => message.clone(),
                None => "no_info".to_string(),
            },
        };
        eprintln!("There was a critical error: {}", parsed);
    }));

    let mut calculator_input: String = String::new();

    utils::flush();
    print!(
        "\nText Calculator v1.0.0\n\nFor help, run {}\n\n",
        "> help".red().italic()
    );
    utils::flush();

    loop {
        utils::flush();
        print!("{}", "> ".yellow().italic());
        utils::flush();

        io::stdin()
            .read_line(&mut calculator_input)
            .expect("fail read line");

        calculator_input = calculator_input.trim().to_string();
        let vector_input: Vec<&str> = calculator_input.split("").collect();
        println!("{:?}", vector_input);

        println!("{}", "Command not recognized\n".italic());
    }
}
