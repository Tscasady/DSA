pub fn binary_search(nums: &Vec<i32>, target: i32) -> i32 {
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

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let mut low = 0;
    let mut high = matrix.len() as i32 - 1;

    while low <= high {
        let mid = (high - low) / 2 + low;
        let end = matrix[mid as usize].len() - 1;

        if target < matrix[mid as usize][0] { 
            high = mid - 1;
        } else if target > matrix[mid as usize][end] {
            low = mid + 1;
        } else {
            match binary_search(&matrix[mid as usize], target) {
                -1 => return false,
                _ => return true
            };           
        }
    };
    false
}
//
//     pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
//         let row = match matrix.binary_search_by(|vec| vec[0].cmp(&target)) {
//             Err(i) if i == 0 => return false,
//             Ok(i)            => return true,
//             Err(i)           => &matrix[i - 1],
//         };
//
//         match row.binary_search(&target) {
//             Ok(_)  => true,
//             Err(_) => false,
//         }
//     }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_search_test() {
        let nums = vec![-1,0,3,5,9,12];
        assert_eq!(binary_search(&nums, 2), -1);
    }

    #[test]
    fn binary_search_one_element() {
        let nums = vec![5];
        assert_eq!(binary_search(&nums, -5), -1);
    }

    #[test]
    fn search_matrix_test() {
        let nums = vec![vec![1,3,5,7],vec![10,11,16,20],vec![23,30,34,60]];
        assert_eq!(search_matrix(nums, 3), true)
    }
    
    #[test]
    fn search_matrix_test_one_true() {
        let nums = vec![vec![1]]; 
        assert_eq!(search_matrix(nums, 1), true)
    }

    #[test]
    fn search_matrix_test_one_false() {
        let nums = vec![vec![1]]; 
        assert_eq!(search_matrix(nums, 0), false)
    }
}
