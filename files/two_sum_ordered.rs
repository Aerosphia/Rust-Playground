use std::cmp::Ordering;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // This example is linear, O(N) time complexity.
        // It assumes that the array passed is ordered.
        
        let mut low: usize = 0;
        let mut hi: usize = nums.len() - 1;
        
        let result: Option<Vec<i32>> = loop {
            if low > hi {
                break None;
            }
            
            let lindex: i32 = nums[low];
            let hindex: i32 = nums[hi];
            
            match (lindex + hindex).cmp(&target) {
                Ordering::Equal => {
                    break Some(vec![low as i32, hi as i32]);
                },
                
                Ordering::Less => {
                    // Move the left pointer.
                    low += 1;
                },
                
                Ordering::Greater => {
                    // Move the right pointer.
                    hi -= 1;
                }
            }
        };
        
        match result {
            Some(vector) => vector,
            None => panic!("No result"),
        }
    }
}