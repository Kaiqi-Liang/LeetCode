//! <https://leetcode.com/problems/contains-duplicate/>
use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    nums.len() != nums.into_iter().collect::<HashSet<i32>>().len()
}

fn main() {
    assert!(contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!contains_duplicate(vec![]));
}
