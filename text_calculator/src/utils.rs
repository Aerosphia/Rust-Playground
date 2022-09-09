use std::io::{self, Write};

pub fn flush() {
    io::stdout().flush().unwrap();
}
