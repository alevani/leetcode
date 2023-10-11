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

        fn binary_search(into: Vec<i32>, index: &i32) -> i32 {
            let mut len = into.len();
            let mut mid;

            let mut memory = into.clone();
            let mut index_memory = 0;

            println!("INDEX: ____{index}");
            while len > 1 {
                println!("len {len}");
                
                mid = len / 2 - 1;
                println!("mid {mid}"); 
                index_memory += mid;
                
                if &memory[mid] == index {
                    println!("yeee 1");
                    println!("{memory:?}");
                    println!("exiting index_memory: {index_memory:?}");
                    return index_memory as i32; // todo index_memory 
                } 
                else if &memory[mid] < index && &memory[mid + 1] > index {
                    println!("yeee 2");
                    println!("{memory:?}");
                    println!("exiting index_memory: {index_memory:?}");
                    return index_memory as i32 + 1; // todo index_memory 
                }

                else if &memory[mid] > index {
                    memory = memory[0..=mid].to_vec();
                    index_memory -= mid;
                } else {
                    memory = memory[(mid + 1)..].to_vec();
                    index_memory += 1;
                }
                println!("index_memory: {index_memory:?}");
                println!("{memory:?}");
                
                len = memory.len();
            }
            
            println!("yeee 3");
            println!("exiting index_memory: {index_memory:?}");
            index_memory as i32 
        }

        people
            .iter()
            .map(|t| binary_search(first.clone(), t) - binary_search(second.clone(), t))
            .collect()
    }
}
