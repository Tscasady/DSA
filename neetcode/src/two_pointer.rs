
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

pub fn most_water(nums: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = nums.len() - 1;
    let mut max_area = 0;
    while left < right {
        let min_height = std::cmp::min(nums[left], nums[right]);
        let area = (right - left) as i32 * min_height;
        max_area = std::cmp::max(area, max_area);
        if nums[left] < nums[right] {
            left += 1;                            
        } else {
            right -= 1;
        }
    }
    return max_area;
}

pub fn trap(height: Vec<i32>) -> i32 {
    let mut result = 0;
    let mut max_left = Vec::with_capacity(height.len());

    let mut cur_left_max = 0;
    for num in &height {
        if *num > cur_left_max {
            cur_left_max = *num
        }
        max_left.push(cur_left_max)
    }

    let mut cur_right_max = 0;
    for (index, _num) in height.iter().enumerate() {
        let right_index = height.len() - index - 1;        
        if height[right_index] > cur_right_max {
            cur_right_max = height[right_index]
        }

        result += std::cmp::max(std::cmp::min(max_left[right_index], cur_right_max) - height[right_index], 0)
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
        let nums = vec![-1,0,1,2,-1,-4];
        assert_eq!(three_sum(nums), vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn three_sum_zero_test() {
        let nums = vec![0, 0, 0, 0];
        assert_eq!(three_sum(nums), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn most_water_test() {
        let nums = vec![1,8,6,2,5,4,8,3,7];
        assert_eq!(most_water(nums), 49) 
    }

    #[test]
    fn most_trapped_water(){
        let nums = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let nums2 = vec![4, 2, 0, 3, 2, 5];
        let nums3 = vec![0, 0];
        let nums4 = vec![4, 2, 3];
        let nums5 = vec![0, 7, 1, 4, 6];
        let nums6 = vec![0,1,2,0,3,0,1,2,0,0,4,2,1,2,5,0,1,2,0,2];
        assert_eq!(trap(nums), 6);
        assert_eq!(trap(nums2), 9);
        assert_eq!(trap(nums3), 0);
        assert_eq!(trap(nums4), 1);
        assert_eq!(trap(nums5), 7);
        assert_eq!(trap(nums6), 26)
    }
}

    //
    // let mut stack = Vec::new();
    // let mut total = 0;
    // let mut dead_space = 0;
    //
    // for (index, num) in height.iter().enumerate() {
    //     if num == &0 || num < stack.last().unwrap_or(&(0 as usize, &0)).1 {
    //         stack.push((index, num));
    //     } else {
    //         let mut last = (0 as usize, &0);
    //         while num >= stack.last().unwrap_or(&(0 as usize, &0)).1 && !stack.is_empty() {
    //             last = stack.pop().unwrap();
    //             dead_space += last.1
    //         }
    //         let width = index - last.0 + 1;
    //         if stack.is_empty() {
    //             stack.push((index, num));
    //             dead_space -= last.1;
    //             total += (width as i32 - 2) * std::cmp::min(num, last.1) - dead_space;
    //             dead_space = 0;
    //         } else if num == last.1 {
    //             dead_space -= last.1;
    //             total += (width as i32 - 2) * std::cmp::min(num, last.1) - dead_space;
    //             dead_space += (width as i32 - 2) * std::cmp::min(num, last.1) - dead_space + last.1 + num;
    //             println!("width: {}, dead: {}", width, dead_space);
    //         } else {
    //             let width = index - stack.last().unwrap().0 + 1;
    //             stack.push((index, num));
    //             total += (width as i32 - 2) * num - dead_space;
    //             dead_space += (width as i32 - 2) * num - dead_space;
    //         }
    //     }
    //              println!("num: {}, dead: {}, stack: {:?}, total: {}", num, dead_space, stack, total)
    // }
    // total
