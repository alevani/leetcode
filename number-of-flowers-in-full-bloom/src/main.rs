use std::collections::HashMap;

fn main() {
    println!(
        "{:?}",
        Solution::full_bloom_flowers(vec![vec![19, 37], vec![19, 38], vec![19,35]], vec![6,7,21,1,13,37,5,37,46,43])
    );
}

struct Solution;
impl Solution {
    pub fn full_bloom_flowers(flowers: Vec<Vec<i32>>, people: Vec<i32>) -> Vec<i32> {
        let mut first = Vec::new();
        let mut second = Vec::new();

        for inner in flowers {
            if let [a, b] = inner.as_slice() {
                first.push(*a);
                second.push(*b);
            }
        }

        first.sort();
        second.sort();

        fn binary_search(into: Vec<i32>, index: &i32, r_done: bool) -> i32 {
            let mut len = into.len();
            let mut mid;

            let mut memory = into.clone();
            let mut index_memory = len / 2 - 1;
            let mut start = 0;

            println!("index____{index}");
            while len > 1 {
                mid = if len % 2 == 0 { len / 2 - 1} else { len / 2 };
                // mid = len / 2 - 1;

                if &memory[mid] == index {
                    println!("Equal");
                    memory = if r_done && mid != 0{
                        vec![memory[mid - 1]]
                    } else {
                        vec![memory[mid]]
                    };
                    println!("89 {index_memory:?}");
                    index_memory -= if r_done && mid != 0 { 1 } else { 0 };
                    println!("-- {index_memory:?}");
                } else if &memory[mid] < index && &memory[mid + 1] > index {
                    println!("In between");
                    memory = vec![memory[mid]];
                    // index_memory += mid;
                } else if &memory[mid] >= index {
                    println!("Lower");
                    memory = memory[0..=mid].to_vec();
                    index_memory = start;
                } else {
                    println!("Higher");
                    memory = memory[(mid + 1)..].to_vec();
                    index_memory += 1;
                    start = index_memory;
                }

                len = memory.len();
            }

            println!("memory: {memory:?}");

          
            
            if r_done && &memory[0] >= index {
                println!("index_memory: {:?}", index_memory as i32 - 1);
                index_memory as i32 - 1
            } else {
                println!("index_memory: {index_memory:?}");
                index_memory as i32 
            }
        }

        people
            .iter()
            .map(|t| {
                binary_search(first.clone(), t, false) - binary_search(second.clone(), t, true)
            })
            .collect()
    }
}
