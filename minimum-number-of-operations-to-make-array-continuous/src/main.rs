use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        Solution::min_operations(vec![41, 33, 29, 33, 35, 26, 47, 24, 18, 28])
    );
}

struct Solution;
impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut duplicate_memory = HashMap::<usize, i32>::new();

        nums.sort();
        println!("{nums:?}");
        let max = (nums.len() - 1) as i32 + nums[0];
        let operations = nums
            .clone()
            .into_iter()
            .filter(|x| x > &max)
            .collect::<Vec<_>>()
            .len();

        nums.iter()
            .for_each(|e| *duplicate_memory.entry(*e as usize).or_insert(0) += 1);

        println!("{duplicate_memory:?}");

        let m = duplicate_memory
            .into_iter()
            .filter(|&(key, value)| key <= max as usize && value > 1)
            .collect::<HashMap<_, _>>();
        
        let b = m.values();
        
        ((nums.len()/2) as i32).min(operations as i32 + b.clone().sum::<i32>() - b.collect::<Vec<&i32>>().len() as i32)
    }
}
