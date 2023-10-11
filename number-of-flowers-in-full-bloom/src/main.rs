use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        Solution::full_bloom_flowers(
            vec![vec![1, 6], vec![3, 7], vec![9, 12], vec![4, 13]],
            vec![2, 3, 7, 11]
        )
    );
}

struct Solution;
impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        people
            .iter()
            .map(|t| {
                flowers
                    .iter()
                    .map(|f| if &f[0] <= t { 1 } else { 0 } - if &f[1] < t { 1 } else { 0 })
                    .sum::<i32>()
            })
            .collect()
    }
}
