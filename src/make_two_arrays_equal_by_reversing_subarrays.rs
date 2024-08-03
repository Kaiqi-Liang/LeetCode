//! <https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/>
use std::collections::HashMap;
pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
    counter!(target.into_iter()) == counter!(arr.into_iter())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert!(can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3]));
    }

    #[test]
    fn one_element() {
        assert!(can_be_equal(vec![7], vec![7]));
    }

    #[test]
    fn not_reversable() {
        assert!(!can_be_equal(vec![3, 7, 9], vec![3, 7, 11]));
    }
}
