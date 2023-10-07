use itertools::Itertools;
use std::cmp::Ordering;

fn main() {
    println!("{:?}", Solution::num_of_arrays(2, 3, 1));
}

struct Solution;
impl Solution {
    pub fn num_of_arrays(n: i32, m: i32, k: i32) -> i32 {
        let items = (1..=m).collect::<Vec<i32>>();

      
        for a in std::iter::repeat(items.iter())
        .take(n as usize)
        .multi_cartesian_product()
        .filter(|a| {
            a.iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            // .map(|b| {println!("{b:?}"); b})
            .map(|(index, _)| {println!("{index:?}"); index}).expect("Partial compare failed") < k as usize
        })
         {
            println!("{a:?}");
        }

        std::iter::repeat(items.iter())
            .take(n as usize)
            .multi_cartesian_product()
            .count() as i32
    }
}
