fn roman_equiv(roman: char) -> Option<i32> {
    return match roman {
        'I' => Some(1),
        'V' => Some(5),
        'X' => Some(10),
        'L' => Some(50),
        'C' => Some(100),
        'D' => Some(500),
        'M' => Some(1000),
        _ => None,
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        // Take the input string and insert all numbers into
        // a vector so they can be added to produce a result.
        let mut number_vector: Vec<i32> = vec![];
        let mut vector_sum: i32;
        
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
            let mut curr: i32 = number_vector[n];
            // Using .get() to get an Option.
            let mut pnext: Option<&i32> = number_vector.get(n + 1);
            let mut next: i32;
            
            next = match pnext {
                Some(v) => *v,
                None => -1,
            };
            
            if (curr > 0 && next != -1) && curr < next {
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
