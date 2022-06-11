use colored::*;
use regex::Regex;
use std::io::{self, Write};
use std::process;

use trie::Trie;

fn main() {
    let mut trie: Trie = Trie::new();

    // The code is horribly messy. I know.
    loop {
        let command: String = get_command();

        let re_add = Regex::new(r"ADD (.*)").unwrap();
        let re_search = Regex::new(r"^SEARCH (.*)").unwrap();
        let re_insert = Regex::new(r"^INSERT (.*)").unwrap();
        let re_correct = Regex::new(r"^(AUTOCORRECT|CORRECT) (.*)").unwrap();
        let re_reset = Regex::new(r"^RESET").unwrap();
        let re_exit = Regex::new(r"^EXIT").unwrap();

        if re_add.is_match(&command) {
            if let Some(capture) = re_add.captures(&command) {
                let parsed = capture[1].trim();
                let split: Vec<&str> = parsed.split(',').collect();
                for word in split {
                    trie.insert(word);
                }
                continue;
            }
        } else if re_search.is_match(&command) {
            if let Some(capture) = re_search.captures(&command) {
                let parsed = capture[1].trim();
                let result = trie.search(parsed);
                println!("{result}");
                continue;
            }
        } else if re_insert.is_match(&command) {
            if let Some(capture) = re_insert.captures(&command) {
                let parsed = capture[1].trim();
                trie.insert(parsed);
                continue;
            }
        } else if re_correct.is_match(&command) {
            if let Some(capture) = re_correct.captures(&command) {
                let parsed = capture[2].trim();
                let results = trie.correct(parsed);
                if !results.is_empty() {
                    println!("Suggestion(s): {}", results.join(", "));
                }
                continue;
            }
        } else if re_reset.is_match(&command) {
            trie = Trie::new();
            continue;
        } else if re_exit.is_match(&command) {
            process::exit(0);
        } else {
            println!(
                "\n{}\n\n{}\n{}\n{}\n{}\n{}\n{}\n",
                "Invalid Command".red(),
                "ADD list".bold(),
                "SEARCH query".bold(),
                "INSERT query".bold(),
                "CORRECT query".bold(),
                "RESET".bold(),
                "EXIT".bold(),
            );
        }
    }
}

fn get_command() -> String {
    io::stdout().flush().unwrap();
    print!("{}", "Enter Command > ".italic());
    io::stdout().flush().unwrap();

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Error");
    input
}
