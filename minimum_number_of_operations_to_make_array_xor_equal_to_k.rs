//! <https://leetcode.com/problems/minimum-number-of-operations-to-make-array-xor-equal-to-k/>
pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut curr = *nums.first().expect("nums.len() >= 1");
    for num in nums.iter().skip(1) {
        curr ^= num;
    }
    curr ^= k;
    curr.count_ones() as _
}

fn main() {
    assert_eq!(min_operations(vec![2, 1, 3, 4], 1), 2);
    assert_eq!(min_operations(vec![2, 0, 2, 0], 0), 0);
}
