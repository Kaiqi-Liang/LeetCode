//! <https://leetcode.com/problems/maximum-product-subarray/>
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let mut opt: Vec<Vec<i32>> = vec![vec![0; nums.len()]; nums.len()];
    for i in 0..nums.len() {
        for j in i..nums.len() {
            opt[i][j] = if i > 0 {
                nums[j] * opt[i - 1][j - 1]
            } else {
                nums[j]
            };
        }
    }
    opt.iter()
        .map(|vec| *vec.iter().max().unwrap_or(&0))
        .max()
        .unwrap_or(0)
}

fn main() {
    assert_eq!(max_sub_array(vec![2, 3, -2, 4]), 6);
    assert_eq!(max_sub_array(vec![-2, 0, -1]), 0);
    assert_eq!(max_sub_array(vec![-2, 3, -4]), 24);
    assert_eq!(max_sub_array(vec![-2]), -2);
    assert_eq!(max_sub_array(vec![]), 0);
}
