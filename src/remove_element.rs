//! <https://leetcode.com/problems/remove-element/>
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let filtered_nums = nums.clone();
    let filtered_nums = filtered_nums
        .iter()
        .filter(|&&num| num != val)
        .collect::<Vec<_>>();
    for i in 0..filtered_nums.len() {
        nums[i] = *filtered_nums[i];
    }
    filtered_nums.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    fn judge(nums: &mut Vec<i32>, val: i32, expected: Vec<i32>) {
        let result = remove_element(nums, val) as usize;
        assert_eq!(result, expected.len());
        let mut nums = nums.iter().take(result).collect::<Vec<_>>();
        nums.sort();
        for i in 0..result {
            assert_eq!(*nums[i], expected[i]);
        }
    }

    #[test]
    fn happy_path() {
        judge(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 0, 1, 3, 4]);
    }

    #[test]
    fn all_same_value_left_after_removal() {
        judge(&mut vec![3, 2, 2, 3], 3, vec![2, 2]);
    }
}
