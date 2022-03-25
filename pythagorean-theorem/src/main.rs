fn main() {
    // Generate prologue
    println!("=======================================");
  
    println!("Welcome to the Pythagorean Theorem checker!");
    println!("🎲 🎲\n");
  
    // Initialize and iterate
    let mut digits = [0, 0];
  
    for i in 0 .. 2 {
        let mut raw_input: String = String::new();
        let read_i: usize = i + 1;
      
        println!("Digit {}:", read_i);
  
        std::io::stdin()
            .read_line(&mut raw_input)
            .expect("Error: read_line operation failed");
  
        digits[i] = match raw_input.trim().parse::<usize>() {
            Ok(n) => n,
            Err(_) => {
                std::process::exit(0);
            },
        };
    }
  
    // Calculate results
    println!("🤔 🤔\n");
    let equation: usize = digits[0].pow(2) + digits[1].pow(2);
    let final_result: f32 = (equation as f32).sqrt();
  
    println!("c = {:.2}", final_result)
}

/*
fn is_numeric_string(string: &String) -> bool {
    for char in string.chars() {
        let ignore_chars: [char; 4] = [' ', '\n', '-', '.'];
        let includes: bool = ignore_chars.iter().any(|v| v == &char);
        if !char.is_numeric() && !includes {
            return false
        }
    }
    return true
}
*/