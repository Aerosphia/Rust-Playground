use std::collections::HashMap;

mod english_words;

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

    fn insert(&mut self, word: &str) {
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

    fn correct(&self, word: &str) -> Vec<String> {
        if self.search(word) || word.is_empty() {
            return vec![word.to_owned()];
        }

        let mut progress = String::new();
        let mut current_node = &self.map;
        let mut to_return = vec![];

        for ch in word.chars() {
            if let Some(value) = current_node.get(&ch) {
                current_node = &value.as_ref().unwrap().map;
                progress.push(ch);
                continue;
            }

            for (k, _) in current_node.iter() {
                if *k != '\0' {
                    let to_push = format!("{progress}{}", *k);
                    to_return.push(to_push);
                }
            }
        }

        to_return
    }
}

fn main() {
    let my_trie: Trie = Trie::new_with(english_words::get());
    println!("{:?}", my_trie.correct("penax"));
}
