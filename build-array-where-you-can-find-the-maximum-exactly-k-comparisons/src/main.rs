use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    println!("{:?}", Solution::num_of_arrays(2, 3, 1));
}

struct Solution;
impl Solution {
    pub fn search_function(a: &Vec<&i32>) -> i32 {
        let mut max_value = -1;
        let mut max_index = -1_i32;
        let mut search_cost = 0;
        a.iter().enumerate().for_each(|(i, b)| {
            if max_value < **b {
                max_value = **b;
                max_index = i as i32;
                search_cost += 1;
            }
        });

        search_cost
    }

    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let items = (1..=m).collect::<Vec<i32>>();

        std::iter::repeat(items.iter())
            .take(n as usize)
            .multi_cartesian_product()
            .filter(|a| self::Solution::search_function(a) <= k)
            .count() as i32
    }
}
