fn main() {
    println!("Hello, world!");

    let mut obj = MyHashMap::new();
    obj.put(1, 2);
    obj.put(2, 2);
    obj.put(1, 4);
    let ret_2: i32 = obj.get(1);
    println!("{ret_2:?}");
    obj.remove(1);
}

struct MyHashMap {
    pub holder: Vec<Vec<i32>>
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    
    fn new() -> Self {
        Self { holder: Vec::new() }
    }

    // I looked at the solution after submission
    // and obviously there also was the long way around
    // of properly making a hashmap, with a bucket and 
    // hash functions...
    
    // A better version would use binary search to directly
    // put the element in the right position
    // making lookup time 1 and put logN

    // This version is put O(N)
    // and get N
    fn put(&mut self, key: i32, value: i32) {
        for (k, v) in self.holder.iter().enumerate() {
            if v[0] == key {
                self.holder[k] = vec![key, value];
                return;
            }
        }

        self.holder.push(vec![key, value]);
    }
    
    fn get(&self, key: i32) -> i32 {
        for v in self.holder.iter() {
            if v[0] == key {
                return v[1];
            }
        }

        -1
    }
    
    fn remove(&mut self, key: i32) {
        for (k, v) in self.holder.iter().enumerate() {
            if v[0] == key {
                self.holder.remove(k);
                return;
            }
        }
    }
}