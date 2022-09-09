use colored::*;
use crossterm::{Result, TerminalCursor};
use std::{io, panic};

mod utils;

fn main() -> Result<()> {
    let cursor = TerminalCursor::new();

    let mut calculator_input: String = String::new();

    utils::flush();
    print!(
        "\nText Calculator v1.0.0\n\nFor help, run {}\n\n{}",
        "> help".red().italic(),
        "> ".yellow().italic()
    );
    utils::flush();

    io::stdin().read_line(&mut calculator_input)?;
    cursor.blink(true)?;

    panic::set_hook(Box::new(|err| {
        let parsed: &str;

        match err.payload().downcast_ref::<&str>() {
            Some(info) => parsed = info,
            None => parsed = "<no_info>",
        }
        eprintln!("There was a critical error: {}", parsed);
    }));

    Ok(())
}
