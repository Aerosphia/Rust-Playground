use std::{io, process};

const PROLOGUE: &str = "
    =======================================
    Welcome to the Pythagorean Theorem Calculator!
    It takes two integers, and calculates the hypotenuse.

    🎲 🎲
    ";

fn main() {
    // Generate prologue
    println!("{}", PROLOGUE);

    // Initialize and iterate
    let mut digits = [0, 0];

    for i in 0..2 {
        let mut raw_input: String = String::new();
        let read_i: usize = i + 1;

        println!("Digit {}:", read_i);

        io::stdin().read_line(&mut raw_input).expect("Error");

        digits[i] = raw_input
            .trim()
            .parse::<usize>()
            .unwrap_or_else(|_| process::exit(0));
    }

    // Calculate results
    let equation: usize = digits[0].pow(2) + digits[1].pow(2);
    let final_result: f32 = (equation as f32).sqrt();

    println!("🤔 🤔\n\nHypotenuse = {:.2}", final_result)
}
