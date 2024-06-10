//! <https://leetcode.com/problems/third-maximum-number/>
/// `O(nlogn)` time complexity
mod sort {
    use std::collections::HashSet;
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut nums = nums
            .into_iter()
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect::<Vec<i32>>();
        nums.sort_by(|a, b| b.cmp(a));
        *nums.get(2).unwrap_or(&nums[0])
    }
}

/// `O(n)` time complexity
mod max {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let first = nums.iter().max().expect("nums.len() >= 1");
        let nums = nums.iter().filter(|&num| num != first);
        match nums.clone().max() {
            Some(second) => match nums.filter(|&num| num != second).max() {
                Some(third) => *third,
                None => *first,
            },
            None => *first,
        }
    }
}

fn main() {
    assert_eq!(
        sort::third_max(vec![3, 2, 1]),
        max::third_max(vec![3, 2, 1])
    );
    assert_eq!(sort::third_max(vec![1, 2]), max::third_max(vec![1, 2]));
    assert_eq!(
        sort::third_max(vec![2, 2, 3, 1]),
        max::third_max(vec![2, 2, 3, 1])
    );
    assert_eq!(
        sort::third_max(vec![1, 2, 2, 5, 3, 5]),
        max::third_max(vec![1, 2, 2, 5, 3, 5])
    );
}
