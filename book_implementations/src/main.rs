use std::{fs::File, io::Write, ops::Drop};

struct DropTest {
    name: String,
    homes: Vec<String>,
}

// Find out why there's a dead code warning here but not in DropTest struct.
#[allow(dead_code)]
struct WriteTest<T: ?Sized> {
    foo: Box<T>,
}

impl Drop for DropTest {
    fn drop(&mut self) {
        print!("Destroying the town `{}`", self.name);
        if !self.homes.is_empty() {
            print!(" containing the household(s): {}", self.homes.join(", "));
        }
    }
}

fn main() -> std::io::Result<()> {
    // Testing the Drop utility trait.
    {
        let _home_town: DropTest = DropTest {
            name: String::from("foo"),
            homes: vec![String::from("abc123"), String::from("def456")],
        };
    }

    // Testing ?Sized with Write.
    // Using fs::File here as it implements the Write trait.
    let file = File::create("foo.txt")?;
    {
        // Doesn't have size known at compile time, therefore we must tell
        // Rust that T in WriteTest does not necessarily have to be sized,
        // via the ?Sized trait.
        let _sized_test: WriteTest<dyn Write> = WriteTest {
            foo: Box::new(file),
        };
    }

    Ok(())
}
