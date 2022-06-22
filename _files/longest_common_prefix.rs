// WIP
use std::collections::HashMap;

#[derive(Debug)]
struct Trie {
    map: HashMap<char, Option<Trie>>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            map: HashMap::new(),
        }
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
        current_node.insert('*', None);
    }
    pub fn lcp(&self) -> String {
        let mut current_node = &self.map;
        let mut progress = String::new();
        while current_node.len() == 1 && current_node.get(&'\0').is_none() {
            if let Some(new_pair) = current_node.iter().next().as_ref() {
                let ch = *new_pair.0;
                progress.push(ch);
                if let Some(next) = &new_pair.1.as_ref() {
                    println!("{:?}", next);
                    current_node = &next.map;
                }
            }
        }

        progress
    }
}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut trie: Trie = Trie::new();

        for st in strs {
            trie.insert(st.as_str());
        }

        return trie.lcp();
    }
}
