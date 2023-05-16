use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
   let mut hash = HashMap::new();
   for num in nums {
        let entry = hash.entry(num).and_modify(|count| *count += 1).or_insert(1);
        if *entry > 1 { return true }
   };
   return false
}

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

