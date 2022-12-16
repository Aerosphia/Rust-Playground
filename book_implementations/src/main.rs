use std::ops::Drop;

struct Town {
    name: String,
    homes: Vec<String>,
}

impl Drop for Town {
    fn drop(&mut self) {
        print!("Destroying the town `{}`", self.name);
        if !self.homes.is_empty() {
            print!(" containing the household(s): {}", self.homes.join(", "));
        }
    }
}

fn main() {
    // Testing the Drop utility trait.
    {
        let _home_town: Town = Town {
            name: String::from("foo"),
            homes: vec![String::from("abc123"), String::from("def456")],
        };
    }
}
