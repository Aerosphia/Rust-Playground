fn main() {}

struct Solution {}

//

struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: vec![] }
    }
}

impl Stack<char> {
    fn read(&self) -> char {
        match self.stack.last() {
            Some(ch) => *ch,
            None => '\0',
        }
    }

    fn push(&mut self, item: char) {
        self.stack.push(item);
    }

    fn pop(&mut self) {
        self.stack.pop();
    }

    fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }
}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Stack<char> = Stack::new();

        for ch in s.chars() {
            if is_paired(stack.read(), ch) {
                stack.pop();
                continue;
            }

            if is_opening(ch) {
                stack.push(ch);
                continue;
            }

            return false;
        }

        if !stack.is_empty() {
            return false;
        }

        true
    }
}

fn is_paired(brace1: char, brace2: char) -> bool {
    matches!((brace1, brace2), ('(', ')') | ('[', ']') | ('{', '}'))
}

fn is_opening(brace: char) -> bool {
    "([{".contains(brace)
}
