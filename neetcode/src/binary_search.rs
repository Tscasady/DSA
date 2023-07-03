pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;  
    let mut high = nums.len() as i32 - 1;

    while low <= high {
        let mid = (high - low) / 2 + low;

        if target == nums[mid as usize] {
            return mid
        } else if target < nums[mid as usize] {
            high = mid - 1
        } else {
            low = mid + 1
        }
    }    
    -1
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let nums = vec![-1,0,3,5,9,12];
        assert_eq!(binary_search(nums, 2), -1);
    }

    #[test]
    fn binary_search_one_element() {
        let nums = vec![5];
        assert_eq!(binary_search(nums, -5), -1);
    }
}
