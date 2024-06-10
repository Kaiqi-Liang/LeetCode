//! <https://leetcode.com/problems/three-sum/>
use std::collections::{HashMap, HashSet};
pub fn three_sum(nums: Vec<i32>) -> HashSet<(i32, i32, i32)> {
    let mut res = HashSet::new();
    let map: HashMap<i32, usize> = nums.iter().enumerate().map(|(i, &num)| (num, i)).collect();
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if let Some(&k) = map.get(&(0 - nums[i] - nums[j])) {
                if k > j {
                    let mut triplet = vec![nums[i], nums[j], nums[k]];
                    triplet.sort();
                    res.insert((triplet[0], triplet[1], triplet[2]));
                }
            }
        }
    }
    res
}

fn main() {
    assert_eq!(
        three_sum(vec![-1, 0, 1, 2, -1, -4]),
        HashSet::from([(-1, -1, 2), (-1, 0, 1)]),
    );
    assert_eq!(three_sum(vec![0, 1, 1]), HashSet::new());
    assert_eq!(three_sum(vec![0, 0, 0]), HashSet::from([(0, 0, 0)]));
}
