use std::collections::HashMap;
use std::io;

fn main() {
    let mut integers: Vec<i32> = prompt();
    // Sort the integers.
    integers.sort();

    // Get the median.
    let length: i32 = integers.len() as i32;
    let is_even: bool = length % 2 == 0;
    let median: i32;

    if is_even {
        median = length / 2;
    } else {
        median = (((length - 1) / 2) + ((length + 1) / 2)) / 2;
    }

    // Get the mode.
    let mut occurrences: HashMap<i32, u32> = HashMap::new();

    for n in integers {
        if occurrences.contains_key(&n) {
            *occurrences.get_mut(&n).unwrap() += 1;
            continue;
        }

        occurrences.insert(n, 1);
    }

    let occurence_values: Vec<u32> = occurrences.values().cloned().collect();
    let mode: i32 = *key_from(&occurrences, *occurence_values.iter().max().unwrap()).unwrap();

    // Output results.
    println!("Median: {}, Mode: {}", median, mode);
}

// Get a list of integers, insert them into a vector and parse them.
fn prompt() -> Vec<i32> {
    let mut input: String = String::new();

    println!("Input comma-separated list of numbers:");
    io::stdin().read_line(&mut input).expect("Error");

    let split: Vec<i32> = input
        .split(", ")
        .map(|slice| slice.trim().parse::<i32>().expect("Invalid Integer(s)"))
        .collect();

    split
}

// Get key from value (HashMap) - specifically for `occurences`
fn key_from<'a>(map: &'a HashMap<i32, u32>, value: u32) -> Option<&'a i32> {
    map.iter()
        .find_map(|(key, &val)| if val == value { Some(key) } else { None })
}
