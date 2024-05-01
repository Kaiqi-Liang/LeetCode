//! <https://leetcode.com/problems/minimum_number_of_operations_to_make_array_xor_equal_to-k/>
pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    nums.into_iter().chain([k]).fold(0, |a, c| a ^ c).count_ones() as _
}

fn main() {
    assert_eq!(min_operations(vec![2, 1, 3, 4], 1), 2);
    assert_eq!(min_operations(vec![2, 0, 2, 0], 0), 0);
}
