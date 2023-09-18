use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let solution = Solution::k_weakest_rows(
        vec![vec![1, 0, 0, 1], vec![1, 1, 1, 1], vec![0, 0, 0, 1]],
        2,
    );

    println!("{solution:?}");
}

struct Solution;
impl Solution {
    pub fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        // The index is the solider count, the value is the index in the mat vec
        let mut storage: HashMap<i32, i32> = HashMap::new();

        mat.iter().enumerate().for_each(|(i, row)| {
            if storage.get(&(i as i32)).is_none() {
                storage.insert(row.iter().sum(), i as i32);
            }
        });

        let mut solution: Vec<i32> = Vec::new();
        let mut counter = 0;
        while solution.len() != k as usize {
            if let Some(value) = storage.get(&counter) {
                solution.push(value.to_owned());
            }
            counter += 1;
        }

        solution
    }
}
