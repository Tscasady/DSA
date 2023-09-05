pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = 1;
    let mut profit = 0;
    while right < prices.len() {
        if prices[left] < prices[right] {
            left = right;
        } else {
            profit = std::cmp::max(prices[right] - prices[left], profit);
            right += 1;
        }
    }
    profit as i32
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
