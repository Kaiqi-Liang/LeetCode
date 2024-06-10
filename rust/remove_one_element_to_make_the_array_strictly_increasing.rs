//! <https://leetcode.com/problems/remove-one-element-to-make-the-array-strictly-increasing/>
pub fn can_be_increasing(nums: Vec<i32>) -> bool {
    let mut remove = 0;
    for i in 1..nums.len() {
        if nums[i - 1] >= nums[i] {
            if remove > 0 {
                return false;
            }
            remove = i;
        }
    }

    let is_increasing = |nums: &[i32]| nums.windows(2).all(|w| w[0] < w[1]);
    let mut nums = nums;
    let num = nums.remove(remove);
    if !is_increasing(&nums) {
        nums.insert(remove, num);
        nums.remove(remove - 1);
        return is_increasing(&nums);
    }
    true
}

fn main() {
    assert!(!can_be_increasing(vec![2, 3, 1, 2]));
    assert!(can_be_increasing(vec![2, 3, 1, 5]));
    assert!(can_be_increasing(vec![2, 1, 2, 5]));
    assert!(can_be_increasing(vec![1, 2, 10, 5, 7]));
    assert!(can_be_increasing(vec![2, 3]));
    assert!(can_be_increasing(vec![2, 2]));
    assert!(can_be_increasing(vec![2, 1]));
    assert!(!can_be_increasing(vec![2, 2, 2]));
    assert!(!can_be_increasing(vec![2, 3, 2, 3]));
}
