pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
    let mut low = 0;  
    let mut high = nums.len() - 1;

    loop {
        let mid = (high - low) / 2 + low;

        if target == nums[mid] {
            return mid as i32
        } else if target < nums[mid] {
            high = mid - 1
        } else {
            low = mid
        }
    }    
}
