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

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut result: Vec<Vec<i32>> = Vec::new();
    nums.sort_unstable();
    for index in 0..nums.len() - 2 {
        if index != 0 && nums[index] == nums[index - 1] { continue;}
        let mut left = index + 1;
        let mut right = nums.len() - 1;
        while left < right {
            if nums[left] + nums[right] + nums[index] == 0 {
                result.push(vec![nums[index], nums[left], nums[right]]);
                left += 1;
                right = nums.len() - 1;
                while nums[left] == nums[left - 1] && left < right {
                    left += 1;
                } 
            } else if nums[left] + nums[right] + nums[index] > 0 {
                right -= 1;
            } else {
                left += 1;
            } 
        }
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum_test() {
        let nums = vec![2,3,4];
        assert_eq!(two_sum(nums, 6), vec![1, 3]);
    }

    #[test]
    fn three_sum_test() {
        let mut nums = vec![-1,0,1,2,-1,-4];
        assert_eq!(three_sum(nums), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn three_sum_zero_test() {
        let mut nums = vec![0, 0, 0, 0];
        assert_eq!(three_sum(nums), vec![vec![0, 0, 0]]);
    }
}
