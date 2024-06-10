//! <https://leetcode.com/problems/best-time-to-buy-and-sell-stock/>
/// `O(n^2)` time complexity
mod loop_max {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        for i in 0..prices.len() {
            let buy = prices[i];
            if let Some(sell) = prices[i..].iter().filter(|&&price| price > buy).max() {
                let profit = sell - buy;
                max = std::cmp::max(max, profit);
            }
        }
        max
    }
}

/// `O(n)` time complexity
mod two_pointers {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut l = 0;
        let mut r = 1;
        while r < prices.len() {
            let buy = prices[l];
            let sell = prices[r];
            let profit = sell - buy;
            if profit > 0 {
                max = std::cmp::max(max, profit);
            } else {
                l = r;
            }
            r += 1;
        }
        max
    }
}

fn main() {
    assert_eq!(two_pointers::max_profit(vec![7, 1, 5, 3, 6, 4]), loop_max::max_profit(vec![7, 1, 5, 3, 6, 4]));
    assert_eq!(two_pointers::max_profit(vec![7, 6, 4, 3, 2, 1]), loop_max::max_profit(vec![7, 6, 4, 3, 2, 1]));
    assert_eq!(two_pointers::max_profit(vec![2, 1, 4]), loop_max::max_profit(vec![2, 1, 4]));
    assert_eq!(
        two_pointers::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0, 9]),
        loop_max::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0, 9]),
    );
}
