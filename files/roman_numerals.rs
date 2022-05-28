struct Solution {}

fn roman_equiv(roman: char) -> Option<i32> {
    Some(match roman {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
        _ => return None,
    })
}

impl Solution {
    #[allow(dead_code)]
    pub fn roman_to_int(s: String) -> i32 {
        // Take the input string and insert all numbers into
        // a vector so they can be added to produce a result.
        let mut number_vector: Vec<i32> = vec![];
        let vector_sum: i32;
        // For each character in the provided string, input
        // the corresponding number value for the roman numeral
        // inside the vector.
        for c in s.chars() {
            // Guaranteed user validation.
            match roman_equiv(c) {
                Some(v) => number_vector.push(v),
                None => panic!("No correspondent or out of range [1, 3999]"),
            }
        }

        // If a roman numeral is less than the one before it, we want to
        // perform subtraction. This allows for "IV" to equal 4 instead of 6.
        for n in 0..number_vector.len() {
            let curr: i32 = number_vector[n];
            // Using .get() to get an Option.
            let next: i32 = match number_vector.get(n + 1) {
                Some(v) => *v,
                None => continue,
            };
            if curr > 0 && curr < next {
                number_vector[n] = next - curr;
                number_vector[n + 1] = 0;
            }
        }
        // Assign the sum of the vector.
        vector_sum = number_vector.iter().sum();
        // Return the result.
        vector_sum
    }
}

fn main() {}
