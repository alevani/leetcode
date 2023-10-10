use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::min_operations(vec![1,10,100,1000]));
}

struct Solution;
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let len = nums.len();
        nums.sort();
    
        
        //[1,10,100,1000]
        let mut duplicate = false;
        for i in 1..len {
            // Duplicate
            if nums[i - 1] == nums[i] {
                duplicate = true;
            }
        }

        if nums.iter().max().unwrap() - nums.iter().min().unwrap() == len as i32 && !duplicate {
            0
        } else {
            2
        }
    }
}