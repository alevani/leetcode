fn main() {
    println!("{:?}", Solution::num_identical_pairs(vec![1,2,3,1,1,3]));
    
}
struct  Solution;
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut counter = 0;
        let len = nums.len();
        for i in 0..len {
            for j in (i + 1)..len {
                if nums[i] == nums[j] {
                    counter += 1;
                }
            }
        }

        counter
    }
}
