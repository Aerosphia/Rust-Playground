use std::{
    fs::File,
    io,
    io::Write,
    ops::{Deref, DerefMut, Drop},
};

struct DropTest {
    name: String,
    homes: Vec<String>,
}

// Find out why there's a dead code warning here but not in DropTest struct.
#[allow(dead_code)]
struct WriteTest<T: ?Sized> {
    foo: Box<T>,
}

struct DerefTest<T> {
    vec: Vec<T>,
    indexer: usize,
}

impl Drop for DropTest {
    fn drop(&mut self) {
        print!("Destroying the town `{}`", self.name);
        if !self.homes.is_empty() {
            print!(" containing the household(s): {}", self.homes.join(", "));
        }
    }
}

impl<T> Deref for DerefTest<T> {
    // Might need further clarification on the following line.
    type Target = T;
    fn deref(&self) -> &T {
        &self.vec[self.indexer]
    }
}

impl<T> DerefMut for DerefTest<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.vec[self.indexer]
    }
}

fn main() -> io::Result<()> {
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

    // Testing Deref & DerefMut with custom implementations.
    {
        let deref_testing: DerefTest<char> = DerefTest {
            vec: vec!['a', 'b', 'c'],
            indexer: 2,
        };

        assert_eq!(*deref_testing, 'c')
    }

    Ok(())
}
