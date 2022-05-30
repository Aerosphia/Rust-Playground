use std::env;

use minigrep::Config;

mod utils;

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        utils::run_and_exit(|| eprintln!("Error parsing arguments: {err}"), 1)
    });

    if let Err(err) = minigrep::run(config) {
        utils::run_and_exit(|| eprintln!("Application error: {err}"), 1);
    }
}
