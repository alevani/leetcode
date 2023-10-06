fn main() {
    println!("{:?}", Solution::integer_break(10));
}

struct Solution;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n <= 3 {
            return n - 1;
        }
        match n % 3 {
            2 => 2 * 3_i32.pow((n / 3) as u32),
            1 => 4 * 3_i32.pow(((n / 3) - 1) as u32),
            _ => 3_i32.pow((n / 3) as u32),
        }
    }
}
