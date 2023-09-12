/*
My first working solution is

*/
fn main() {
    let solution = Solution1::convert("AB".to_string(), 1);
    println!("The solution is: {solution}");
}

/*
My first naive attempt is to create a list per row and fill the list
from top to bottom, then bottom to top, then top to bottom... and so on.
*/
struct Solution1;
impl Solution1 {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut result_list = vec!["".to_string(); num_rows as usize];
        
        let mut counter: i32 = 0;
        let mut sign: i32 = 1;
        for letter in s.chars() {
            result_list[counter as usize].push(letter);
            
            counter += sign;
            if counter == num_rows {
                sign = -sign;
                counter -= 2;
            }

            if counter <= 0 {
                sign = -sign;
                counter = 0;
            }
        }
        
        result_list.join("")
    }
}