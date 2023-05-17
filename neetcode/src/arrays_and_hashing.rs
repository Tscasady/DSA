use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut hash = HashMap::new();
    for num in nums {
        let entry = hash.entry(num).and_modify(|count| *count += 1).or_insert(1);
        if *entry > 1 {
            return true;
        }
    }
    return false;
}

//can be O(1) space complexity if you compare two sorted arrays, but slower
//this solution is O(n) time and space
pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    };

    let mut s_hash = HashMap::new();
    for char in s.chars() {
        s_hash
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut t_hash = HashMap::new();
    for char in t.chars() {
        t_hash
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    s_hash == t_hash
}

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<String, Vec<String>> = HashMap::new(); 
    for word in strs {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let anagram: String = chars.iter().collect();
        anagrams.entry(anagram).and_modify(|vec| vec.push(word.to_owned())).or_insert(vec![word]);
    }
    anagrams.values().cloned().collect()
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash: HashMap<i32, i32> = HashMap::new();

    for (index, &value) in nums.iter().enumerate() {
        hash.entry(value).or_insert(index as i32);
        let pair = hash.get(&(target - value));
        if let Some(value) = pair {
            if index as i32 != *value {
                return vec![index as i32, *value];
            }
        }
    }

    return vec![];
}

pub fn is_palindrome(s: String) -> bool {
    let sanitized_string: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|char| char.is_alphanumeric())
        .collect();
    for (index, &char) in sanitized_string
        .iter()
        .enumerate()
        .take(sanitized_string.len() / 2)
    {
        let reversed_index = sanitized_string.len() - index - 1;
        if char != sanitized_string[reversed_index] {
            return false;
        }
    }
    return true;
}
