fn main() {
    let mut unreversed: Vec<&'static str> = vec!["Hello", "World"];
    let reversed: Vec<&'static str> = reverse_vector(&mut unreversed);
    println!("{:?}", reversed);
}

// Reverses a vector with O(1) extra space complexity.
fn reverse_vector<T>(vector: &mut Vec<T>) -> Vec<T>
where
    T: Copy,
{
    for v in 1..vector.len() {
        vector.insert(0, vector[v]);
        vector.remove(v + 1);
    }

    vector.to_vec()
}
