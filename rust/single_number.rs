//! <https://leetcode.com/problems/single-number/>
use std::collections::HashMap;
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    for num in nums {
        map.entry(num).and_modify(|e| *e = false).or_insert(true);
    }
    map.into_iter()
        .find(|&tuple| tuple.1)
        .expect("Guarantee to find exactly one true value")
        .0
}

fn main() {
    assert_eq!(single_number(vec![2, 2, 1]), 1);
    assert_eq!(single_number(vec![1]), 1);
}
