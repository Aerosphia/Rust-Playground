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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query: String = match args.next() {
            Some(arg) => arg,
            None => return Err("lack of required argument: query str"),
        };

        let file_name: String = match args.next() {
            Some(arg) => arg,
            None => return Err("lack of required argument: lookup file"),
        };

        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            file_name,
            case_sensitive,
        })
    }
}

/// Runs default application logic with a configuration structure.
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

    if !matches.is_empty() {
        println!("Found some matches!\n\n{}", matches.join("\n"));
    } else {
        println!("I couldn't find anything!");
    }

    Ok(())
}

/// Searches each line in a string slice for a certain subquery.
///
/// # Examples
///
/// ```
/// let query: &'static str = "lorem";
/// let contents: &'static str = "\n
/// Lorem ipsum dolor\n
/// sit amet Lorem,\n
/// consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.";
///
/// assert_eq!(
///     vec!["Lorem ipsum dolor", "sit amet Lorem,"],
///     search(query, contents, false)
/// );
/// ```
pub fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    if case_sensitive {
        return contents
            .lines()
            .filter(|line| line.contains(query))
            .collect();
    }

    let query: String = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.contains(&query))
        .collect()
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
