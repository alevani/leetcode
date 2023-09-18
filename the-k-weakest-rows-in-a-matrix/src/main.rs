fn main() {
    println!("Hello, world!");
    let solution = Solution::k_weakest_rows(
        vec![vec![1, 0, 0, 1], vec![1, 1, 1, 1], vec![0, 0, 0, 1]],
        2,
    );
    // 0: 2, 1: 4, 2: 1/_/

    println!("{solution:?}");
}
struct Solution;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    n_soldier: i32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if other.n_soldier == self.n_soldier {
            other.position.cmp(&self.position)
        } else {
            self.n_soldier.cmp(&other.n_soldier)
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

use std::collections::BinaryHeap;

impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        mat.iter().enumerate().for_each(|(i, row)| {
            heap.push(State {
                n_soldier: -row.iter().sum::<i32>(),
                position: i,
            })
        });

        (0..k)
            .map(|_| heap.pop().unwrap().position as i32)
            .collect::<Vec<i32>>()
    }
}
