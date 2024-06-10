//! <https://leetcode.com/problems/majority-element/>
use std::collections::HashMap;
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, usize> = HashMap::new();
    for &num in nums.iter() {
        map.entry(num).and_modify(|e| *e += 1).or_insert(1);
    }
    *map.iter()
        .find(|&tuple| *tuple.1 > nums.len() / 2)
        .expect("Guarantee to find exactly one true value")
        .0
}

fn main() {
    assert_eq!(majority_element(vec![3, 2, 3]), 3);
    assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
}
