//! <https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/description/>
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut total = 0;
    for (i, price) in prices.iter().enumerate().skip(1) {
        total += 0.max(price - prices[i - 1]);
    }
    total
}

fn main() {
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
    assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}
