//! <https://leetcode.com/problems/subarray-sum-equals-k/>
use std::collections::HashMap;
pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::from([(0, 1)]);
    let mut sum = 0;
    let mut count = 0;
    for num in nums {
        sum += num;
        count += map.get(&(sum - k)).unwrap_or(&0);
        map.entry(sum).and_modify(|v| *v += 1).or_insert(1);
    }
    count
}

fn main() {
    assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
    assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    assert_eq!(subarray_sum(vec![1, -1, 1, 1, 1, 1], 3), 4);
}
