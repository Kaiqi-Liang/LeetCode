//! <https://leetcode.com/problems/first-missing-positive/>
use std::collections::HashSet;
use std::iter::FromIterator;
pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let max = nums.len() + 1;
    let set: HashSet<i32> = HashSet::from_iter(nums);
    for i in 1..max {
        if !set.contains(&(i as i32)) {
            return i as _;
        }
    }
    max as _
}

fn main() {
    assert_eq!(first_missing_positive(vec![1, 3, 6, 4, 1, 2]), 5);
    assert_eq!(first_missing_positive(vec![1, 2, 3]), 4);
    assert_eq!(first_missing_positive(vec![-1, -3]), 1);
}
