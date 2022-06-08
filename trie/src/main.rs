use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    map: HashMap<char, Option<Trie>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            map: HashMap::new(),
        }
    }

    fn new_with(words: &[&str]) -> Self {
        let mut new = Trie::new();
        for word in words.iter() {
            new.insert(word);
        }
        new
    }

    fn search(&self, word: &str) -> bool {
        let mut current_node = &self.map;
        for ch in word.chars() {
            let value = current_node.get(&ch);
            let unwrapped = if let Some(val) = value {
                val
            } else {
                return false;
            };
            if let Some(unwrapped) = &unwrapped {
                current_node = &unwrapped.map;
            }
        }
        current_node.contains_key(&'\0')
    }

    fn insert(&mut self, word: &str) -> () {
        let mut current_node = &mut self.map;
        for ch in word.chars() {
            let entry = current_node.entry(ch).or_insert_with(|| {
                Some(Trie {
                    map: HashMap::new(),
                })
            });

            current_node = &mut entry.as_mut().unwrap().map;
        }
        current_node.insert('\0', None);
    }

    /*
    fn read<F>(&self, func: F) -> ()
    where
        F: FnOnce(String),
    {
        let mut return_str = String::new();
        for (k, mut v) in &self.map {
            while v.is_some() {
                return_str.push(*k);
                v = &v.as_mut().unwrap().map;
            }
        }
    }
    */

    fn correct<'a>(&self, word: &'a str) -> &'a str {
        if self.search(word) {
            return word;
        }

        let mut lcp = String::new();
        let mut current_node = &self.map;

        for ch in word.chars() {
            if let Some(value) = current_node.get(&ch) {
                current_node = &value.as_ref().unwrap().map;
                lcp.push(ch);
            }
        }

        println!("{lcp}");
        return "Hello";
    }
}

fn main() {
    let my_trie: Trie = Trie::new_with(&["cat", "catnap"]);
    println!("{}", my_trie.correct("catn"));
    //println!("{:?}", my_trie.map);
}
