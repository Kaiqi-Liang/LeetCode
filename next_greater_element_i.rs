//! <https://leetcode.com/problems/next-greater-element-i/>
use std::collections::{HashMap, VecDeque};
pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let num_to_index: HashMap<i32, usize> =
        nums1.iter().enumerate().map(|(i, &num)| (num, i)).collect();
    let mut res = nums1;
    let mut stack = VecDeque::new();
    for num in nums2.into_iter().rev() {
        while !stack.is_empty() && stack.back().expect("!stack.is_empty()") <= &num {
            stack.pop_back();
        }
        if let Some(&index) = num_to_index.get(&num) {
            res[index] = if let Some(&top) = stack.back() {
                top
            } else {
                -1
            };
        }
        stack.push_back(num)
    }
    res
}

fn main() {
    assert_eq!(
        next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        vec![-1, 3, -1]
    );
    assert_eq!(
        next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        vec![3, -1]
    );
}
