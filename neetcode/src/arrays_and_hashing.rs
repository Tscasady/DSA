use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn contains_duplicate<T: Eq + Hash>(nums: &Vec<T>) -> bool {
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

#[allow(dead_code)]
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut anagrams: HashMap<String, Vec<String>> = HashMap::new();
    for word in strs {
        let mut chars: Vec<char> = word.chars().collect();
        chars.sort();
        let anagram: String = chars.iter().collect();
        anagrams
            .entry(anagram)
            .and_modify(|vec| vec.push(word.to_owned()))
            .or_insert(vec![word]);
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

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut hash = HashMap::new();
    let mut solution = Vec::new();
    for num in nums {
        hash.entry(num).and_modify(|count| *count += 1).or_insert(1);
    }

    while (solution.len() as i32) < k {
        let max_entry = hash
            .iter()
            .max_by_key(|&(_, value)| value)
            .map(|(&key, &value)| (key, value));
        if let Some((max_key, _)) = max_entry {
            solution.push(max_key);
            hash.remove(&max_key);
        }
    }

    return solution;
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let last_index = nums.len() - 1;
    let mut result: Vec<i32> = Vec::with_capacity(nums.len());

    for (index, _) in nums.iter().enumerate() {
        let prefix_product = match index {
            0 => Some(&1),
            _ => result.get(index - 1),
        };

        let prev_num = if index == 0 {1} else {nums[index - 1]};

        result.push(prev_num * prefix_product.unwrap())
    }

    let mut postfix_product = 1;
    for (index, _) in nums.iter().enumerate() {
        let post_index = nums.len() - 1 - index;

        if post_index != last_index {
            postfix_product *= nums[post_index + 1]
        };

        result[post_index] = result[post_index] * postfix_product
    }
    return result;
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    board.iter().enumerate().all(|(index, row)| !dup_check(row) && check_column(&board, index) && check_box(&board, index))
}

fn check_column(board: &Vec<Vec<char>>, index: usize) -> bool {
    let column: Vec<char> = board.iter().map(|row| row[index]).collect();
    !dup_check(&column)
}

fn check_box(board: &Vec<Vec<char>>, row_index: usize) -> bool {
    let box_index = (row_index / 3) * 3;
    let rows = board[box_index..box_index+3].to_vec(); 
    let column_index = (row_index % 3) * 3;
    let sudoku_box: Vec<Vec<char>> = rows.iter().map(|row| row[column_index..column_index+3].to_vec()).collect();
    !dup_check(&sudoku_box.concat())
}

fn dup_check(collection: &Vec<char>) -> bool {
    let mut hash = HashMap::new();
    for char in collection {
        if *char == '.' { continue };
        let entry = hash.entry(char).and_modify(|count| *count += 1).or_insert(1);
        if *entry > 1 {
            return true;
        }
    }
    return false;
}

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut hash = HashSet::new();
    for num in &nums {
        hash.insert(num);
    }

    for num in &nums {
        let mut count = 1;
        if !hash.contains(&(num - 1)) { continue };
        count = recursive_sequencer(&num, count, &hash);
        if count > max {
            max = count
        }
    }
    max
}

fn recursive_sequencer(num: &i32, mut count: i32, hash: &HashSet<&i32>) -> i32 {
    if hash.contains(num) {
        count += 1;
        return recursive_sequencer(&(num + 1), count, hash)
    } 
    count
}
