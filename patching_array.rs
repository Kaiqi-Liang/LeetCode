//! https://leetcode.com/problems/patching-array/
pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
    let n: i64 = n.into();
    let mut sum: i64 = 0;
    let mut count = 0;
    let mut i = 0;
    while sum < n {
        if i < nums.len() && i64::from(nums[i]) <= sum + 1 {
            sum += i64::from(nums[i]);
            i += 1;
        } else {
            sum = sum * 2 + 1;
            count += 1;
        }
    }
    count
}

fn main() {
    assert_eq!(min_patches(vec![1, 3], 6), 1);
    assert_eq!(min_patches(vec![1, 5, 10], 20), 2);
    assert_eq!(min_patches(vec![1, 2, 2], 5), 0);
    assert_eq!(min_patches(vec![1, 2], 10), 2);
    assert_eq!(min_patches(vec![2, 5], 25), 3);
    assert_eq!(min_patches(vec![2, 5], 26), 4);
    assert_eq!(min_patches(vec![1, 2, 31, 33], 2147483647), 28);
}
