//! <https://leetcode.com/problems/maximum-subarray/>
use std::cmp;
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut opt = Vec::new();
    for &num in &nums {
        opt.push(cmp::max(num, opt.last().unwrap_or(&0) + num));
    }
    *opt.iter().max().unwrap_or(&0)
}

fn main() {
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    assert_eq!(max_sub_array(vec![-2]), -2);
    assert_eq!(max_sub_array(vec![]), 0);
}
