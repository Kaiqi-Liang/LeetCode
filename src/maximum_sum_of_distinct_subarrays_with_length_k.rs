//! <https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/>
use std::collections::HashMap;
pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
    let k = k as usize;
    let mut map = HashMap::<i64, usize>::new();
    let mut running_sum = 0;
    for num in &nums[0..k] {
        let num = *num as i64;
        *map.entry(num).or_default() += 1;
        running_sum += num;
    }
    let mut max = if map.len() == k { running_sum } else { 0 };
    for (left, right) in (k..nums.len()).enumerate() {
        let prev = nums[left] as i64;
        let curr = nums[right] as i64;
        running_sum = running_sum - prev + curr;
        *map.entry(prev).or_default() -= 1;
        if let Some(&count) = map.get(&prev) {
            if count == 0 {
                map.remove(&prev);
            }
        }
        *map.entry(curr).or_default() += 1;
        if map.len() == k {
            max = max.max(running_sum);
        }
    }
    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn happy_path() {
        assert_eq!(maximum_subarray_sum(vec![9, 9, 9, 1, 2, 3], 3), 12);
    }

    #[test]
    fn duplicates_last() {
        assert_eq!(maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3), 15);
    }

    #[test]
    fn duplicates_first() {
        assert_eq!(maximum_subarray_sum(vec![1, 1, 1, 7, 8, 9], 3), 24);
    }

    #[test]
    fn multiple_duplicates() {
        assert_eq!(maximum_subarray_sum(vec![5, 3, 3, 1, 1], 3), 0);
    }

    #[test]
    fn no_subarray() {
        assert_eq!(maximum_subarray_sum(vec![4, 4, 4], 3), 0);
    }
}
