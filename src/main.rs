// Non-functional program

use std::ops::AddAssign<&&i32>;

fn add_numbers(integers: &[&i32]) -> i32 {
    let mut net: i32 = 0;
    for i in integers {
        net += i;
    }
    return net;
}

fn main() {
    println!("{}", add_numbers(5, 10));
}