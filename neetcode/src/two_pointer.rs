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

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left_index = 0;
    let mut right_index = numbers.len() - 1;

    loop {
        let left_num = numbers[left_index];
        let right_num = numbers[right_index];
        if left_num + right_num == target {
            return vec![left_index as i32 + 1, right_index as i32 + 1]
        } else if left_num + right_num > target {
            right_index -= 1
        } else {
            left_index += 1
        }
    } 
}
