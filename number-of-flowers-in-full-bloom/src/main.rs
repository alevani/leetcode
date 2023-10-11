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
        let mut answer = HashMap::<i32, i32>::new();

        flowers.iter().for_each(|range| {
            (range[0]..range[1] + 1).for_each(|x| *answer.entry(x).or_insert(0) += 1)
        });
        println!("{answer:?}");
        people
            .iter()
            .map(|p| answer.get(p).unwrap_or(&0))
            .copied()
            .collect()
    }
}
