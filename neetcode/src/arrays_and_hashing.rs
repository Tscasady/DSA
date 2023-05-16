use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
   let mut hash = HashMap::new();
   for num in nums {
        let entry = hash.entry(num).and_modify(|count| *count += 1).or_insert(1);
        if *entry > 1 { return true }
   };
   return false
}

//can be O(1) space complexity if you compare two sorted arrays, but slower
//this solution is O(n) time and space
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() { return false};

    let mut s_hash = HashMap::new();
    for char in s.chars() {
        s_hash.entry(char).and_modify(|count| *count += 1).or_insert(1);
    };

    let mut t_hash = HashMap::new();
    for char in t.chars() {
        t_hash.entry(char).and_modify(|count| *count += 1).or_insert(1);
    };
    s_hash == t_hash
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashMap<i32, i32> = HashMap::new();
    
    for (index, &value) in nums.iter().enumerate() {
        hash.entry(value).or_insert(index as i32);
        let pair = hash.get(&(target - value));
        if let Some(value) = pair {
            if index as i32 != *value {
                return vec![index as i32, *value]
            } 
        }
    };

    return vec![]
}
