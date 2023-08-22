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

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let mut right: i32 = *piles.iter().max().unwrap();
    let mut left: i32 = 1;
    let mut solution = right;
    
    while left <= right {
        let mid = (left + right) / 2;
        let hours: i64 = piles.iter().fold(0, |acc, pile| acc + div_ceil(pile, mid));
        println!("{}", hours);
        if hours <= h as i64 {
            solution = std::cmp::min(solution, mid);
            right = mid - 1
        } else {
            left = mid + 1
        }
    }
    return solution
    // match solutions.partition_point(|&num| banana_check(&piles, num, h)) {
    //     i if i == solutions.len() => max, 
    //     i => solutions[i]
    // }
}

// pub fn banana_check(piles: &Vec<i32>, num: i32, h: i32) -> bool {
//     let hours = piles.iter().fold(0, |acc, pile| acc + div_ceil(pile, num));
//     return hours > h
// }

pub fn div_ceil(divd: &i32, div: i32) -> i64 {
    let mut solution = divd / div;
    if divd % div != 0 {
        solution += 1
    };
    return solution.into()
}

pub fn find_min(nums: Vec<i32>) -> i32 {
    let mut left = 0;    
    let mut right = nums.len() - 1;
    while left <= right {
        let mid = (right + left) / 2;
        if nums[mid] < nums[right] {
            right = mid
        } else {
            left = mid + 1
        }
    }
    return nums[right]
}

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

    #[test]
    fn koko_test() {
        let piles = vec![3, 6, 7, 11];
        assert_eq!(4, min_eating_speed(piles, 8))
    }

    #[test]
    fn koko_test_two() {
        let piles = vec![312884470];
        assert_eq!(2, min_eating_speed(piles, 312884469))
    }
    #[test]
    fn koko_three() {
        let piles = vec![805306368,805306368,805306368];
        assert_eq!(3, min_eating_speed(piles, 1000000000))
    }
    #[test]
    fn rotated_test() {
        let nums = vec![4, 5, 1, 2, 3];
        assert_eq!(1, find_min(nums))
    }

}
