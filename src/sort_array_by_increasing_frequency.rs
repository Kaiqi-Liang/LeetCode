//! <https://leetcode.com/problems/sort-array-by-increasing-frequency/>
use std::{cmp::Ordering, collections::HashMap};
pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = nums.clone();
    let frequency = nums.into_iter().fold(HashMap::new(), |mut a, c| {
        a.entry(c).and_modify(|num| *num += 1).or_insert(1);
        a
    });
    sorted_nums.sort_by(|a, b| match frequency.get(a).cmp(&frequency.get(b)) {
        Ordering::Equal => b.cmp(a),
        other => other,
    });
    sorted_nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(
            frequency_sort(vec![1, 1, 2, 2, 2, 3]),
            vec![3, 1, 1, 2, 2, 2]
        );
    }

    #[test]
    fn same_frequency() {
        assert_eq!(frequency_sort(vec![2, 3, 1, 3, 2]), vec![1, 3, 3, 2, 2]);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(
            frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]),
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1]
        );
    }
}
