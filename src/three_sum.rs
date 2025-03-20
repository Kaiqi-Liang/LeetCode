//! <https://leetcode.com/problems/three-sum/>
use std::collections::{HashMap, HashSet};
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = HashSet::new();
    let map: HashMap<i32, usize> = nums.iter().enumerate().map(|(i, &num)| (num, i)).collect();
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if let Some(&k) = map.get(&(0 - nums[i] - nums[j])) {
                if k > j {
                    let mut triplet = vec![nums[i], nums[j], nums[k]];
                    triplet.sort();
                    res.insert(triplet);
                }
            }
        }
    }
    res.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4])
                .into_iter()
                .collect::<HashSet<Vec<i32>>>(),
            HashSet::from([vec![-1, -1, 2], vec![-1, 0, 1]]),
        );
    }

    #[test]
    fn no_solution() {
        assert_eq!(
            three_sum(vec![0, 1, 1])
                .into_iter()
                .collect::<HashSet<Vec<i32>>>(),
            HashSet::new(),
        );
    }

    #[test]
    fn all_zeros() {
        assert_eq!(
            three_sum(vec![0, 0, 0])
                .into_iter()
                .collect::<HashSet<Vec<i32>>>(),
            HashSet::from([vec![0, 0, 0]]),
        );
    }
}
