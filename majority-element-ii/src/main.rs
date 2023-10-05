use std::collections::HashMap;

fn main() {
    println!("{:?}", Solution::majority_element(vec![3,2,3]));
}

struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let mut counter: HashMap<i32, i32> = HashMap::new();
        let len = nums.len();
        let mut holder: Vec<i32> = Vec::new();

        nums.into_iter().for_each(|v| {
            counter.entry(v).and_modify(|x| *x += 1).or_insert(1);
            if !holder.contains(&v) && counter.get(&v).unwrap_or(&0) > &(len as i32 / 3) {
                holder.push(v);
            }
        });

        holder
    }
}
