use regex::Regex;
use std::io;

#[derive(Default)]
pub struct Input {
    pub data: Option<String>,
}

impl Input {
    pub fn get(&mut self) -> Result<Self, String> {
        if self.data.is_some() {
            return Err("struct data is not None".to_owned());
        }

        let mut data: String = String::new();
        println!("Enter a calculation:");
        io::stdin().read_line(&mut data).expect("Error");

        let data: Option<String> = Some(data);

        Ok(Input { data })
    }

    pub fn calc(&self) -> Result<i32, String> {
        let data: &String = self.data.as_ref().unwrap();
        let addition_regex: Regex = Regex::new(r"(.*) \+ (.*)").unwrap();
        let captures: regex::Captures<'_> = match addition_regex.captures(data) {
            Some(captures) => captures,
            None => return Err("invalid input".to_owned()),
        };

        let mut int_vers: Vec<i32> = vec![];

        for n in captures.iter() {
            if captures.iter().position(|v| v == n).unwrap() == 0 {
                continue;
            };

            let finalized: i32 = match n.unwrap().as_str().trim().parse::<i32>() {
                Ok(n) => n,
                Err(err) => return Err(format!("parse error: {}", err)),
            };

            int_vers.push(finalized);
        }

        let result: i32 = match int_vers[0].checked_add(int_vers[1]) {
            Some(res) => res,
            None => return Err("addition out of bounds; please satisfy 32-bit addition".to_owned()),
        };

        Ok(result)
    }
}
