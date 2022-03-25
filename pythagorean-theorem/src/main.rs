use std::{io, process};

fn main() {
    // Generate prologue
    println!("=======================================");
  
    println!("Welcome to the Pythagorean Theorem Calculator!");
    println!("🎲 🎲\n");
  
    // Initialize and iterate
    let mut digits = [0, 0];
  
    for i in 0 .. 2 {
        let mut raw_input: String = String::new();
        let read_i: usize = i + 1;
      
        println!("Digit {}:", read_i);
  
        io::stdin()
            .read_line(&mut raw_input)
            .expect("Error: read_line operation failed");
  
        digits[i] = match raw_input.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                process::exit(0);
            },
        };
    }
  
    // Calculate results
    println!("🤔 🤔\n");
    let equation: usize = digits[0].pow(2) + digits[1].pow(2);
    let final_result: f32 = (equation as f32).sqrt();
  
    println!("c = {:.2}", final_result)
}