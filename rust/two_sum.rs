//! <https://leetcode.com/problems/two-sum/>
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.into_iter().enumerate() {
        let key = target - num;
        if let Some(&index) = map.get(&key) {
            if i != index {
                return vec![index as _, i as _];
            }
        }
        map.insert(num, i);
    }
    vec![]
}

fn main() {
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
