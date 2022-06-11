use std::collections::HashMap;

#[derive(Debug)]
pub struct Trie {
    map: HashMap<char, Option<Trie>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            map: HashMap::new(),
        }
    }

    pub fn new_with(words: &[&str]) -> Self {
        let mut new = Trie::new();
        for word in words.iter() {
            new.insert(word);
        }
        new
    }

    pub fn search(&self, word: &str) -> bool {
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

    pub fn insert(&mut self, word: &str) {
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


    }
    */

    pub fn correct(&self, word: &str) -> Vec<String> {
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
