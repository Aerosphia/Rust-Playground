use error::Error;
use io::ErrorKind;
use std::{env, error, fs, io, thread, time};

mod utils;

pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("lack of required arguments");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();
        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(
        "Searching \"{}\" in file: `{}`..",
        config.query, config.file_name
    );

    let file_result: Result<String, io::Error> = fs::read_to_string(config.file_name);
    let file_contents: &String = match file_result {
        Ok(ref s) => s,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                utils::run_and_exit(|| eprintln!("File operation error: file not found"), 1)
            }
            other => utils::run_and_exit(|| eprintln!("File operation error: {}", other), 1),
        },
    };

    println!(
        "File contents:\n-------------------------\n{file_contents}\n-------------------------",
    );

    thread::sleep(time::Duration::from_secs(2));

    let mut matches: Vec<&str> = vec![];

    for line in search(&config.query, &file_contents, config.case_sensitive) {
        matches.push(line);
    }

    if matches.len() > 0 {
        println!("Found some matches!\n\n{}", matches.join("\n"));
    } else {
        println!("I couldn't find anything!");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    if case_sensitive {
        let mut return_values: Vec<&str> = vec![];

        for line in contents.lines() {
            if line.contains(query) {
                return_values.push(line.trim());
            }
        }
        return return_values;
    }

    let mut return_values: Vec<&str> = vec![];
    let query: String = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            return_values.push(line.trim());
        }
    }

    return_values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case() {
        let query: &'static str = "lorem";
        let contents: &'static str = "\n
        Lorem ipsum dolor\n
        sit amet Lorem,\n
        consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";

        assert_eq!(
            vec!["Lorem ipsum dolor", "sit amet Lorem,"],
            search(query, contents, false)
        );
    }
}
