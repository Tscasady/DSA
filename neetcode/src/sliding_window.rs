use std::collections::HashSet;
use std::cmp;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 1;
    let mut profit = 0;
    while right < prices.len() {
        if prices[left] < prices[right] {
            left = right;
        } else {
            profit = cmp::max(prices[right] - prices[left], profit);
            right += 1;
        }
    }
    profit as i32
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut left = 0;
    let mut right = 1;
    let s: Vec<char> = s.chars().collect();
    let mut set: HashSet<char> = HashSet::new();
    let mut maxlength = 1;
    let mut curr_length = 1;
    if s.len() > 0 {
        set.insert(s[left]);
        while right < s.len() {
            if set.insert(s[right]) {
                curr_length += 1;
                maxlength = cmp::max(maxlength, curr_length);
                right += 1;
            } else {
                left += 1;
                right = left + 1;
                maxlength = cmp::max(maxlength, curr_length);
                curr_length = 1;
                set.clear();
                set.insert(s[left]);
            }      
        }
    } else {
        return 0
    }
    maxlength
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn max_profit_test() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(5, max_profit(prices))
    }
}
