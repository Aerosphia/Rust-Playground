use std::time::Instant;

fn main() {
    let start = Instant::now();
    let number_forced = 197856349;
    match brute_force(number_forced) {
        Some(result) => println!("Cracked {:?} in {:?}.", result, start.elapsed()),
        None => println!("Could not crack code."),
    }
}

fn brute_force(n: i32) -> Option<String> {
    for i in 0..10000 {
        if n == i {
            return Some(format!("{:0>4}", i));
        }
    }
    None
}
