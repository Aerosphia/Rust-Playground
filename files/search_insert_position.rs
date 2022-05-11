// Unfinished

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // If target is found, return that index.
        if nums.contains(&target) {
            return (nums.iter().position(|&v| v == target).unwrap()) as i32;
        }
        
        // Use binary search to find the index the number would be at.
        let mut new_vector: Vec<i32> = nums;
        let mut result: i32;
        
        result = loop {
            let middle: i32 = new_vector.get(new_vector.len() / 2).unwrap();

            
        }
        
    }
}
